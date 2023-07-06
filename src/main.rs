mod bloom_filter;

fn main() {
    let bf = bloom_filter::BloomFilter::new(100000, 0.0001);
    let bf2 = bf.insert("kek");
    assert!(bf2.member("kek"));
    assert!(!bf2.member("pepe"));
    let bf3 = bf2.clear();
    assert!(!bf3.member("kek"));
}
