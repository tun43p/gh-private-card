use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

/// **Create an hash from an object to **
///
/// Returns a [u64] for the `hash`
pub fn create_hash<T>(obj: T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}
