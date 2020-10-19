//! # fast-fibonacci
//!
//! `fast-fibonacci` uses linear recurrence to quickly find fib(n, mod) in O(log(n)) time.
//!
//! Adapted from http://fusharblog.com/solving-linear-recurrence-for-programming-contest/

use ndarray::arr2;
use ndarray::Array2;


/// Finds the nth fibonacci number with modulo. Runtime O(log(n))
///
/// Uses linear recurrence under the covers.
/// # Examples
///
/// ```
/// assert_eq!(0, fast_fibonacci::fib_with_mod(0, 10));
/// assert_eq!(1, fast_fibonacci::fib_with_mod(1, 10));
/// assert_eq!(1, fast_fibonacci::fib_with_mod(2, 10));
/// assert_eq!(2, fast_fibonacci::fib_with_mod(3, 10));
/// assert_eq!(546_875, fast_fibonacci::fib_with_mod(1_000_000_000_000_000, 1_000_000));
/// assert_eq!(875, fast_fibonacci::fib_with_mod(1_000_000_000_000_000, 1_000));
/// ```
pub fn fib_with_mod(n: u64, modulo: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let f = vec![0, 1];
    let t = arr2(&[
        [0, 1], 
        [1, 1]
    ]);
    let power_t = matrix_power_with_mod(&t, n, modulo);
    let mut answer = 0;
    for i in 0..2 {
        answer += (answer + (power_t[[0, i]] * f[i])) % modulo;
    }
    answer
}

fn multiply_with_mod(a: &Array2<u64>, b: &Array2<u64>, modulo: u64) -> Array2<u64> {
    let mut return_mat = Array2::zeros((2, 2));
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                return_mat[[i, j]] = ( 
                    return_mat[[i, j]] + (
                        a[[i, k]] * b[[k, j]]
                    )
                ) % modulo;
            }
        }
    }
    return_mat
}

fn matrix_power_with_mod(mat: &Array2<u64>, pow: u64, modulo: u64) -> Array2<u64> {
    let return_mat = mat.clone();
    if pow == 1 {
        return return_mat;
    }
    if pow % 2 == 1 {
        return multiply_with_mod(&mat, &matrix_power_with_mod(mat, pow - 1, modulo), modulo);
    }
    let x = matrix_power_with_mod(mat, pow / 2, modulo);
    multiply_with_mod(&x, &x, modulo)
}

#[cfg(test)]
mod tests {
    use crate::*;
    
    #[test]
    fn test_first_few() {
        assert_eq!(fib_with_mod(0, 10), 0);
        assert_eq!(fib_with_mod(1, 10), 1);
        assert_eq!(fib_with_mod(2, 10), 1);
        assert_eq!(fib_with_mod(3, 10), 2);
        assert_eq!(fib_with_mod(4, 10), 3);
        assert_eq!(fib_with_mod(5, 10), 5);
    }

    #[test]
    fn test_modulo() {
        assert_eq!(fib_with_mod(100, 1_000_000_000), 261_915_075);
        assert_eq!(fib_with_mod(100, 1_000_000), 915_075);
        assert_eq!(fib_with_mod(100, 1_000), 75);
        assert_eq!(fib_with_mod(100, 10), 5);
    }
    
    #[test]
    fn test_big() {
        assert_eq!(fib_with_mod(1_000_000_000_000_000, 1_000_000), 546_875);
        assert_eq!(fib_with_mod(1_000_000_000_000_001, 1_000_000), 937_501);
    }
}
