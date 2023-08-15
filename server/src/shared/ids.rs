use nanoid::nanoid;

const ALPHABET: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub fn create_poll_id() -> String {
    nanoid!(6, &ALPHABET)
}

pub fn create_user_id() -> String {
    nanoid!()
}

pub fn create_nomination_id() -> String {
    nanoid!(8)
}
