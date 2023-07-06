use fixedbitset::FixedBitSet;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::RngCore;
use std::hash::Hash;
use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;

#[derive(PartialEq, Debug)]
struct BloomFilter {
    n: u32,
    p: f32,
    bitset: FixedBitSet,
    m: u64,
    k: u64,
    hash_seed: u32
}

impl BloomFilter {

    fn get_max_size(n: u32, p: f32) -> u64 {
        ((n as f32 * p.log2()).abs() as u64 / (1 / u64::pow(2_u8.ilog2() as u64, 2))) as u64 + 1
    }

    fn get_number_of_hash_fns(n: u32, m: u64) -> u64 {
        (m / n as u64) * 2_u8.ilog2() as u64
    }

    pub fn new(n: u32, p: f32, mut rand_gen: SmallRng) -> Self {
        let m = Self::get_max_size(n, p);
        BloomFilter {
            n: n,
            p: p,
            bitset: FixedBitSet::with_capacity(m as usize),
            m: m,
            k: Self::get_number_of_hash_fns(n, m),
            hash_seed: rand_gen.next_u32()
        }
    }

    fn get_hashes<T: Hash>(&self, item: &T) -> Vec<u64> {
        (1..self.k).map(|i| {
            let mut s = DefaultHasher::new();
            item.hash(&mut s);
            (s.finish() + i + self.hash_seed as u64) % self.m
        }).collect()
    }

    pub fn insert<T: Hash>(self, item: T) -> Self {
        let hashes = self.get_hashes(&item);
        BloomFilter {
            bitset: hashes.iter().fold(self.bitset, |mut curr, &hash| {
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
}

fn main() {
    let bf = BloomFilter::new(100000, 0.0001, SmallRng::from_entropy());
    let bf2 = bf.insert("kek");
    println!("{}", bf2.member("kek"));
    println!("{}", bf2.member("pepe"));
}
