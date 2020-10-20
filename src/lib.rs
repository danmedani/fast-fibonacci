//! # fast-fibonacci
//!
//! `fast-fibonacci` uses linear recurrence to quickly find fib(n, mod) in O(log(n)) time.
//!
//! Adapted from http://fusharblog.com/solving-linear-recurrence-for-programming-contest/

use ndarray::arr2;
use ndarray::Array2;
use num_bigint::BigUint;
use num::FromPrimitive;

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
        answer = (answer + (power_t[[0, i]] * f[i])) % modulo;
    }
    answer
}


/// BigUint version of fib_with_mod. Finds the nth fibonacci number with modulo. Runtime O(log(n))
///
/// Uses linear recurrence under the covers.
///
/// # Examples
/// ```
/// use num::FromPrimitive;
///
/// let ns               = vec![0, 1, 2, 3, 10, 1_000_000_000_000_000, 1_000_000_000_000_000, 1_955_995_342_096_516];
/// let modulos          = vec![10, 10, 20, 30, 100, 1_000_000, 1_000, u64::MAX];
/// let expected_results = vec![0, 1, 1, 2, 55, 546_875, 875, 2_886_946_313_980_141_317];
///
/// for i in 0..ns.len() {
///     assert_eq!(
///         fast_fibonacci::bigfib_with_mod(
///             &FromPrimitive::from_u64(ns[i]).unwrap(),
///             &FromPrimitive::from_u64(modulos[i]).unwrap()
///         ),
///         FromPrimitive::from_u64(expected_results[i]).unwrap()
///     );
/// }
/// ```
///
/// ```
/// use num_bigint::BigUint;
///
/// let big_n: BigUint = BigUint::from_slice(&[100u32, 100, 100, 100, 15129, 12319]);
/// let big_modulo: BigUint = BigUint::from_slice(&[14u32, 12, 1923876123, 12]);
/// let expected_result: BigUint = BigUint::from_slice(&[2743227343u32, 920986447, 1158660944, 5]);
///
/// assert_eq!(
///     fast_fibonacci::bigfib_with_mod(&big_n, &big_modulo),
///     expected_result
/// );
/// ```
pub fn bigfib_with_mod(n: &BigUint, modulo: &BigUint) -> BigUint {
    let ZERO: BigUint = FromPrimitive::from_u64(0).unwrap();
    let ONE: BigUint = FromPrimitive::from_u64(1).unwrap();
    if n == &ZERO || n == &ONE {
        return n.clone();
    }

    let f: Vec<BigUint> = vec![ZERO.clone(), ONE.clone()];
    let t: Array2<BigUint> = arr2(&[
        [ZERO.clone(), ONE.clone()],
        [ONE.clone(), ONE.clone()]
    ]);
    let power_t = bigfib_matrix_power(&t, n, modulo);
    let mut answer: BigUint = ZERO.clone();
    for i in 0..2 {
        answer = (answer + (&power_t[[0, i]] * &f[i])) % modulo;
    }
    return answer;
}


fn bigfib_matrix_power(mat: &Array2<BigUint>, pow: &BigUint, modulo: &BigUint) -> Array2<BigUint> {
    let ONE: BigUint = FromPrimitive::from_u64(1).unwrap();
    let TWO: BigUint = FromPrimitive::from_u64(2).unwrap();
    if pow == &ONE {
        return mat.clone();
    }
    if pow % &TWO == ONE {
        return bigfib_multiply(
            &mat, 
            &bigfib_matrix_power(mat, &(pow - ONE), modulo),
            modulo
        );
    }
    let x = bigfib_matrix_power(mat, &(pow / TWO), modulo);
    bigfib_multiply(&x, &x, modulo)
}


fn bigfib_multiply(a: &Array2<BigUint>, b: &Array2<BigUint>, modulo: &BigUint) -> Array2<BigUint> {
    let ZERO: BigUint = FromPrimitive::from_u64(0).unwrap();
    let mut return_mat: Array2<BigUint> = arr2(&[
        [ZERO.clone(), ZERO.clone()],
        [ZERO.clone(), ZERO.clone()]
    ]);

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let big_val: BigUint = &return_mat[[i, j]] + (&a[[i, k]] * &b[[k, j]]);
                return_mat[[i, j]] = big_val % modulo;
            }
        }
    }
    return_mat
}


fn multiply_with_mod(a: &Array2<u64>, b: &Array2<u64>, modulo: u64) -> Array2<u64> {
    let mut return_mat: Array2<u64> = Array2::zeros((2, 2));

    let big_mod: BigUint = FromPrimitive::from_u64(modulo).unwrap();
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let mat_ij: BigUint = FromPrimitive::from_u64(return_mat[[i, j]]).unwrap();
                let a_ik: BigUint = FromPrimitive::from_u64(a[[i, k]]).unwrap();
                let b_kj: BigUint = FromPrimitive::from_u64(b[[k, j]]).unwrap();

                let big_val: BigUint = (mat_ij + (
                    a_ik * b_kj
                )) % &big_mod;

                return_mat[[i, j]] = small_big_int_to_u64(&big_val);
            }
        }
    }
    return_mat
}


fn matrix_power_with_mod(mat: &Array2<u64>, pow: u64, modulo: u64) -> Array2<u64> {
    if pow == 1 {
        return mat.clone();
    }
    if pow % 2 == 1 {
        return multiply_with_mod(
            &mat, 
            &matrix_power_with_mod(
                mat, 
                pow - 1, 
                modulo
            ), 
            modulo
        );
    }
    let x = matrix_power_with_mod(mat, pow / 2, modulo);
    multiply_with_mod(&x, &x, modulo)
}


fn small_big_int_to_u64(big_int: &BigUint) -> u64 {
    let mut result: u64 = 0;

    let digits = big_int.to_radix_be(10);
	for i in 0..digits.len() - 1 {
		result = result + digits[i] as u64;
		result = result * 10;
	}
	result + digits[digits.len() - 1] as u64
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
        assert_eq!(fib_with_mod(1_955_995_342_096_516, u64::MAX), 2_886_946_313_980_141_317);
    }

    #[test]
    fn test_bigfib() {
        let ns = vec![0, 1, 2, 3, 10, 1_000_000_000_000_000, 1_955_995_342_096_516];
        let modulos = vec![10, 10, 20, 30, 100, 1_000_000, u64::MAX];
        let expected_results = vec![0, 1, 1, 2, 55, 546_875, 2_886_946_313_980_141_317];

        for i in 0..ns.len() {
            assert_eq!(
                bigfib_with_mod(
                    &FromPrimitive::from_u64(ns[i]).unwrap(), 
                    &FromPrimitive::from_u64(modulos[i]).unwrap()
                ),
                FromPrimitive::from_u64(expected_results[i]).unwrap()
            );
        }
    }

    #[test]
    fn test_large_bigfib() {
        let n: BigUint = BigUint::from_slice(&[100u32, 100, 100, 100, 15129, 12319]);
        let modulo: BigUint = BigUint::from_slice(&[14u32, 12, 1923876123, 12]);
        let expected_result: BigUint = BigUint::from_slice(&[2743227343u32, 920986447, 1158660944, 5]);
        assert_eq!( 
            bigfib_with_mod(&n, &modulo),
            expected_result
        );
    }
}
