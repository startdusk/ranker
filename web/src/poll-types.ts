export type Participants = {
  [participantId: string]: string;
};

export type Nomination = {
  userId: string;
  text: string;
};

type NominationId = string;

export type Nominations = {
  [nominationId: NominationId]: Nomination;
};

export type Rankings = {
  [userId: string]: NominationId[];
};

export type Results = Array<{
  nominationId: NominationId;
  nominationText: string;
  score: number;
}>;

export type Poll = {
  id: string;
  topic: string;
  votesPerVoter: number;
  participants: Participants;
  adminId: string;
  nominations: Nominations;
  rankings: Rankings;
  results: Results;
  hasStarted: boolean;
};

export type NotificationMessage = {
  notify_type: "join_poll";
  username: string;
  poll_id: string;
  topic: string;
};
