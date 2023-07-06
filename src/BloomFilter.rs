use fixedbitset::FixedBitSet;
use std::hash::Hash;
use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;

#[derive(PartialEq, Debug)]
struct BloomFilter {
    n: u32,
    p: f32,
    bitset: FixedBitSet,
    m: u64,
    k: u64
}

impl BloomFilter {

    fn get_max_size(n: u32, p: f32) -> u64 {
        ((n as f32 * p.log2()).abs() as u64 / (1 / u64::pow(2_u8.ilog2() as u64, 2))) as u64 + 1
    }

    fn get_number_of_hash_fns(n: u32, m: u64) -> u64 {
        (m / n as u64) * 2_u8.ilog2() as u64
    }

    pub fn new(n: u32, p: f32) -> Self {
        let m = Self::get_max_size(n, p);
        BloomFilter {
            n: n,
            p: p,
            bitset: FixedBitSet::with_capacity(m as usize),
            m: m,
            k: Self::get_number_of_hash_fns(n, m)
        }
    }

    fn get_hashes<T: Hash>(&self, item: &T) -> Vec<u64> {
        (1..self.k).map(|i| {
            let mut s = DefaultHasher::new();
            item.hash(&mut s);
            (s.finish() + i) % self.m
        }).collect()
    }

    pub fn insert<T: Hash>(self, item: T) -> Self {
        let hashes = self.get_hashes(&item);
        BloomFilter {
            bitset: hashes.iter().fold(self.bitset.clone(), |mut curr, &hash| {
                curr.insert(hash as usize);
                curr
            }),
            ..self
        }
    }

    pub fn member<T: Hash>(&self, item: T) -> bool {
        self.get_hashes(&item)
            .iter()
            .all(|&hash| self.bitset.contains(hash as usize))
    }

    pub fn clear(mut self) -> Self {
        self.bitset.clear();
        self
    }
}

fn main() {
    let bf = BloomFilter::new(100000, 0.0001);
    let bf2 = bf.insert("kek");
    assert!(bf2.member("kek"));
    assert!(!bf2.member("pepe"));
    let bf3 = bf2.clear();
    assert!(!bf3.member("kek"));
}
