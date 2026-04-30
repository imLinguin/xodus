use rand::RngExt;

pub fn generate_suid() -> String {
    format!("S-1-5-21-0000000000-0000000000-0000000000-1001")
}

pub fn generate_string(length: usize) -> String {
    rand::rng()
        .sample_iter(rand::distr::Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
