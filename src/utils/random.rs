use rand::distributions::Alphanumeric;
use rand::thread_rng;
use rand::Rng;

// Can be used to make identity_id also.
pub fn alphanumeric_key(len: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric).take(len).collect()
}
