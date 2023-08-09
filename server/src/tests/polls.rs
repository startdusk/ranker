use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

use crate::data::redis::polls::{
    add_nomination, add_participant, add_poll, add_results, del_poll, get_poll, remove_nomination,
    remove_participant, start_poll,
};
use crate::models::{Nomination, Poll, Result};

#[tokio::test]
async fn test_polls_lifecycle() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = redis::aio::ConnectionManager::new(client).await.unwrap();
    let ttl = 1; // expire 1s
    let poll_id = "iBOY-JBDILBW3aWQwFTES".to_string();
    let user_id = "T7EYUQ".to_string();
    let topic = "test_tpoic".to_string();
    let name = "mynameisben".to_string();
    let votes_per_voter = 1;
    let poll = Poll::new(
        poll_id.clone(),
        topic.clone(),
        votes_per_voter,
        user_id.clone(),
    );

    // 1.add poll
    let adding_poll = add_poll(
        &mut con,
        ttl,
        poll_id.clone(),
        topic.clone(),
        votes_per_voter,
        user_id.clone(),
    )
    .await
    .unwrap();
    assert_eq!(poll, adding_poll);

    // 2.get poll
    let first_get_poll = get_poll(&mut con, poll_id.clone()).await.unwrap();
    assert_eq!(poll, first_get_poll);

    // 3.add participant
    let add_participant_poll =
        add_participant(&mut con, poll_id.clone(), user_id.clone(), name.clone())
            .await
            .unwrap();
    let mut expect_add_participant_poll = poll.clone();
    expect_add_participant_poll.participants =
        HashMap::from([(poll.admin_id.clone(), name.clone())]);
    assert_eq!(expect_add_participant_poll, add_participant_poll);

    // remove participant get error(poll no start)
    let Err(_) = remove_participant(&mut con, add_participant_poll.id.clone(), add_participant_poll.admin_id.clone()).await else {
        panic!("Should be got an error")
    };

    // 4.start poll
    let started_poll = start_poll(&mut con, poll_id.clone()).await.unwrap();
    let mut expect_started_poll = expect_add_participant_poll.clone();
    expect_started_poll.has_started = true;
    assert_eq!(expect_started_poll, started_poll);

    // 5.remove participant
    let remove_participant_poll = remove_participant(
        &mut con,
        add_participant_poll.id,
        add_participant_poll.admin_id,
    )
    .await
    .unwrap();
    let mut expect_remove_participant_poll = expect_started_poll.clone();
    expect_remove_participant_poll.participants = HashMap::new();
    assert_eq!(expect_remove_participant_poll, remove_participant_poll);

    // 6.add nomination
    let nomination_id = "nominati".to_string();
    let text = "this is a text".to_string();
    let nomination = Nomination {
        user_id: user_id.clone(),
        text: text.clone(),
    };
    let add_nomination_poll = add_nomination(
        &mut con,
        poll_id.clone(),
        nomination_id.clone(),
        nomination.clone(),
    )
    .await
    .unwrap();
    let mut expect_add_nomination_poll = expect_remove_participant_poll.clone();
    expect_add_nomination_poll.nominations = HashMap::from([(nomination_id.clone(), nomination)]);
    assert_eq!(expect_add_nomination_poll, add_nomination_poll);

    // 7.remove nomination
    let remove_nomination_poll =
        remove_nomination(&mut con, poll_id.clone(), nomination_id.clone())
            .await
            .unwrap();

    let mut expect_remove_nomination_poll = expect_add_nomination_poll.clone();
    expect_remove_nomination_poll.nominations = HashMap::new();
    assert_eq!(expect_remove_nomination_poll, remove_nomination_poll);

    // 8.add results
    let results = vec![Result {
        nomination_id,
        nomination_text: text,
        score: 1,
    }];
    let add_results_poll = add_results(&mut con, poll_id.clone(), results.clone())
        .await
        .unwrap();
    let mut expect_add_results_poll = expect_remove_nomination_poll;
    expect_add_results_poll.results = results;
    assert_eq!(expect_add_results_poll, add_results_poll);

    // 9.remove poll
    del_poll(&mut con, poll_id.clone()).await.unwrap();
    let Err(_) = get_poll(&mut con, poll_id.clone()).await else {
        panic!("Should be got an error")
    };

    // 10.wait for expired
    let _ = add_poll(
        &mut con,
        ttl,
        poll_id.clone(),
        topic,
        votes_per_voter,
        user_id.clone(),
    )
    .await
    .unwrap();
    sleep(Duration::from_secs(ttl as u64));

    let Err(_) = get_poll(&mut con, poll_id.clone()).await else {
        panic!("Should be got an error")
    };
}
