use rand::{distributions::Alphanumeric, Rng};

pub fn generate_random_password() -> String {
    let password: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    password
}
