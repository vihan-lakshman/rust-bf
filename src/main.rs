use std::hash::{Hash};

use fasthash::{metro, MetroHasher};

pub struct BloomFilter {
	bit_array: [bool; 256],
}

impl BloomFilter {
	pub fn new() -> BloomFilter {
		BloomFilter {bit_array: [false; 256]}
	}

	pub fn insert(&mut self, value: &str) {
		// For now, hard code the hash functions here
		// TODO: Make the number of hashes configurable

		let h = metro::hash64(value);
		let usize hash_value = h % 256;
		println!("The hash value is {}", h);
		self.bit_array[usize::from(h)] = true;
	}

	pub fn query(&mut self, value: &str) -> bool {
		true
	}
}


// tests are good
#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn basic_test() {
    	let mut bloom_filter = BloomFilter::new();
    	let input = "spam";
    	bloom_filter.insert(input);
    	assert_eq!(10, 5 + 5);
    	println!("The bit array is: {:?}", bloom_filter.bit_array);
    }

}


