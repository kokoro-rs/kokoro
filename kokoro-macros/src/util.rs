pub fn hash(s: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

pub fn get_pkg_name() -> String {
    let env = std::env::var("CARGO_PKG_NAME");
    env.unwrap_or("".to_string())
}
