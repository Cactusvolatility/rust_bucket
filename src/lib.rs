//use core::num;
use std::collections::{HashMap};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::sync::RwLock;

pub struct ConcurrentHashMap <Key, Val> {
    buckets: Vec<RwLock<HashMap<Key, Val>>>,
    // multiple RwLock hashmaps
    bucket_count: usize,
    // use usize for direct indexing into Vec collection for pointer size of whatever machine is running code
}

// we explicitly state variables
impl <Key, Val> ConcurrentHashMap<Key, Val>
where
    Key : Hash + Eq + Clone, // hashable, equatable, and clonable

    {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            let num_buckets = 16;
            let mut buckets = Vec::with_capacity(num_buckets);
                // reduce fragmentation
            
            for _ in 0..num_buckets {
                buckets.push(RwLock::new(HashMap::new()));
                // add a RwLocked Hashmap for each one
                    // init our hashamp
            }
            
            // init and make it usable
            ConcurrentHashMap { 
                buckets: buckets, 
                bucket_count: num_buckets, 
            }
        }

        // pass in reference to key - hash something and return the index
        fn bucket_index(&self, key: &Key) -> usize {
            let mut hasher = DefaultHasher::new();
            // hash the key with random hasher
            key.hash(&mut hasher);
            let hashed_number = hasher.finish() as usize;
            // hash it again by bucket_count
            hashed_number % self.bucket_count
        }

        // insert
        pub fn insert(&self, key: Key, value: Val) -> Option<Val> {
            // dummy
            None
        }
        // remove key
        // contains key -> bool
        // len() -> usize
        // is_empty()

    }