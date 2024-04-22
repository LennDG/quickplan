use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn new_url_id() -> String {
    let rng = thread_rng();
    rng.sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}
