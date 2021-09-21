use fasthash::{metro};

pub struct BloomFilter<const N: usize> {
	bit_array: [bool; N],
}

impl<const N: usize> BloomFilter<N> {
	pub fn new() -> BloomFilter<N> {
		Self {bit_array: [false; N]}
	}

	pub fn insert(&mut self, value: &str) {
		// For now, hard code the hash functions here
		// TODO: Make the number of hashes configurable

		let h = metro::hash64(value);
		let hash_value = (h % 256) as usize;
		println!("The hash value is {}", hash_value);
		self.bit_array[hash_value] = true;
	}

	pub fn query(&mut self, value: &str) -> bool {
		let h = metro::hash64(value);
		let hash_value = (h % 256) as usize;
		return self.bit_array[hash_value];
	}
}


// tests are good
#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn basic_test() {
    	let mut bloom_filter: BloomFilter<256> = BloomFilter::new();
    	let input1 = "spam";
    	let input2 = "spam2";

    	bloom_filter.insert(input1);
    	assert!(bloom_filter.bit_array[93] == true);

    	assert!(bloom_filter.query(input1) == true);
    	assert!(bloom_filter.query(input2) == false);
    }
}
