/// Returns a sum with less precision loss than with summing an iterator as normal.
///
/// Replication of the [ASPN cookbook recipe for accurate float summation](https://code.activestate.com/recipes/393090/).
pub fn fsum(nums: &[f64]) -> f64 {
    let mut partials: Vec<f64> = vec![];
    #[allow(clippy::needless_range_loop)]
    // Intentional because of problems with mutable references and destructuring them
    for h in 0..nums.len() {
        let mut x = nums[h];
        let mut i: usize = 0;
        for g in 0..partials.len() {
            let mut y = partials[g];
            if x.abs() < y.abs() {
                (x, y) = (y, x);
            }
            let hi = x + y;
            let lo = y - (hi - x);
            if lo != 0.0_f64 {
                partials[i] = lo;
                i += 1;
            }
            x = hi;
        }
        partials.splice(i.., &mut [x].iter().cloned());
    }
    partials.iter().sum()
}

/// Returns the number of digits for a given integer.
pub fn ndigits(num: i64) -> u8 {
    1 + match num.abs().checked_ilog10() {
        Some(n) => n as u8,
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::{fsum, ndigits};
    #[test]
    fn fsum_works() {
        let ari = [0.1_f64; 10];
        let sum = fsum(&ari);
        assert_eq!(sum, 1.0_f64);
    }

    #[test]
    fn ndigits_works() {
        for i in -5_i32..=5 {
            let x: i32 = i.signum() * 10_i32.pow(i.abs() as u32);
            let nd = ndigits(x.into());
            assert_eq!((i.abs() + 1) as u8, nd);
        }
    }
}
