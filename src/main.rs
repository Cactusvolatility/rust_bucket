use bucket_map::ConcurrentHashMap;

fn main() {
    println!("Hello, world!");
    let map: ConcurrentHashMap<i32, String> = ConcurrentHashMap::new();
    println!("initialized new hashmap");
}
