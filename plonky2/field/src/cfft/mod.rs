use plonky2_util::log2_strict;

use crate::types::Field;

#[cfg(feature = "parallel")]
mod concurrent;

mod serial;

const USIZE_BITS: usize = 0_usize.count_zeros() as usize;
const MIN_CONCURRENT_SIZE: usize = 1024;

pub fn get_twiddles<F>(domain_size: usize) -> Vec<F>
where
    F: Field,
{
    assert!(
        domain_size.is_power_of_two(),
        "domain size must be a power of 2"
    );
    assert!(
        log2_strict(domain_size) <= F::TWO_ADICITY,
        "multiplicative subgroup of size {} does not exist in the specified base field",
        domain_size
    );
    let root = F::primitive_root_of_unity(log2_strict(domain_size));
    let mut twiddles = root.powers().take(domain_size / 2).collect::<Vec<F>>();
    permute(&mut twiddles);
    twiddles
}

pub fn get_inv_twiddles<F>(domain_size: usize) -> Vec<F>
where
    F: Field,
{
    assert!(
        domain_size.is_power_of_two(),
        "domain size must be a power of 2"
    );
    assert!(
        log2_strict(domain_size) <= F::TWO_ADICITY,
        "multiplicative subgroup of size {} does not exist in the specified base field",
        domain_size
    );
    let root = F::primitive_root_of_unity(log2_strict(domain_size));
    // let inv_root = root.inverse();
    let inv_root = root.exp_u64(domain_size as u64 - 1);
    let mut inv_twiddles = inv_root.powers().take(domain_size / 2).collect::<Vec<F>>();
    permute(&mut inv_twiddles);
    inv_twiddles
}

fn permute<F: Field>(v: &mut [F]) {
    if cfg!(feature = "parallel") && v.len() >= MIN_CONCURRENT_SIZE {
        #[cfg(feature = "parallel")]
        concurrent::permute(v);
    } else {
        serial::permute(v);
    }
}

fn permute_index(size: usize, index: usize) -> usize {
    debug_assert!(index < size);
    if size == 1 {
        return 0;
    }
    debug_assert!(size.is_power_of_two());
    let bits = size.trailing_zeros() as usize;
    index.reverse_bits() >> (USIZE_BITS - bits)
}

#[allow(clippy::uninit_vec)]
pub unsafe fn uninit_vector<T>(length: usize) -> Vec<T> {
    let mut vector = Vec::with_capacity(length);
    vector.set_len(length);
    vector
}
