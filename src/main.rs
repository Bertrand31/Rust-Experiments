use fixedbitset::FixedBitSet;

struct BloomFilter {
    n: u32,
    p: f32,
    bitset: FixedBitSet,
    m: u32,
    k: u32,
    hash_seed: u32
}

fn main() {
    println!("Hello, world!");
}
