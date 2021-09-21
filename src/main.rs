use fasthash::{metro};

const MAX_NUM_INSERTIONS: u32 = 10000;

// We specify the error rate as a decimal between 0 and 1
pub struct BloomFilter<const N: usize> {
	bit_array: [bool; N],
	num_hash_funcs: u8,
	insertion_count: u64
}

impl<const N: usize> BloomFilter<N> {
	// Do note: error_rate is out of 1, *not* in percentage points
	pub fn new(false_positive_rate: f64) -> BloomFilter<N> {
		if false_positive_rate > 1.0 {
			panic!("Error rate cannot be over 1");
		}

		let num_hashes: u8 = false_positive_rate.powf(-1.0).log(10.0).ceil() as u8;

		Self {
			bit_array: [false; N],
			num_hash_funcs: num_hashes,
			insertion_count: 0
		}
	}

	pub fn insert(&mut self, value: &str) {
		self.insertion_count += 1;
		if self.insertion_count > MAX_NUM_INSERTIONS.into() {
			panic!("Too many insertions!"); // Probably shouldn't panic, should return
		}
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
    	let mut bloom_filter: BloomFilter<256> = BloomFilter::new(0.01);
    	let input1 = "spam";
    	let input2 = "spam2";

    	bloom_filter.insert(input1);
    	assert!(bloom_filter.bit_array[93] == true);

    	assert!(bloom_filter.query(input1) == true);
    	assert!(bloom_filter.query(input2) == false);
    }
}
