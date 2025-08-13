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
            // we keep this private - we don't want to expose which hash function we're using to hash values
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
            // decide which bucket I am putting it into
            let to_bucket = self.bucket_index(&key);
            // now I canuse this to index into my list of buckets - index() or just []?
                // how to aquire RwLock
            let mut bucket = self.buckets[to_bucket].write().unwrap();

            // we can directly return the result of insert which is an Option<T> of the old val.

                // if new insert then nothing happens and returns None
            bucket.insert(key, value)

            //None
        }
        // remove key
            // right now we are taking full ownership of the key and it will be dropped after this method
            // this is fine for the moment, since usually we don't care about keys that are dropped

        pub fn remove_key(&self, key: Key) -> Option<Val> {
            // if found something return true, else return false
            // how do I find the key the bucket that the key goes to?
            let from_bucket = self.bucket_index(&key);
            let mut bucket = self.buckets[from_bucket].write().unwrap();

            // we don't want to consume they key (ownership) in order to lookup - unlike in insert
                // so pass reference
            bucket.remove(&key)
            //None
        }

        // contains key -> bool
        pub fn contains_key(&self, key: &Key) -> bool {
            let from_bucket = self.bucket_index(key);
            // don't need it let here because read() returns a Readguard and not an Option
            let bucket = self.buckets[from_bucket].read().unwrap();

            // return bool (not Bool)
            bucket.contains_key(key)
        }

        // get (key)
            // different from contains_key because we need to send back a clone
            // pass a reference to a key since we don't want to handle exchanging ownership with a "lookup"
        pub fn get_key(&self, key: &Key) -> Option<Val>
        where
            Val: Clone, // make sure that it's clonable
        {
            let from_bucket = self.bucket_index(key);
            let bucket = self.buckets[from_bucket].read().unwrap();
            // clone it and send the key back
            bucket.get(key).cloned()
        }

        // len() -> usize
            // what do I need to do here.
            // I need to find all the entries inside of each bucket?
        
        pub fn len_dict(&self) -> usize {
            // loop through all the dicts in the current ConcurrentHashmap Vector
            let mut total = 0;
            for bucket_lock in &self.buckets {
                let bucket = bucket_lock.read().unwrap();

                // use Hashmap .len() method > returns usize and elemnts in the map
                total += bucket.len();
            }

            total
            // this will not be perfect is something is editing it at the same time but that's fine?
        }

        // is_empty() -> bool
        pub fn is_empty(&self) -> bool {
            for bucket_lock in &self.buckets {
                let bucket = bucket_lock.read().unwrap().is_empty();

                if !bucket {
                    return false
                }
            }

            // clippy is pretty annoying
            true
        }

    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_insert() {
        let _map = ConcurrentHashMap::new();
        _map.insert("Hello", "World");
    }
}