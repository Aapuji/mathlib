use std::mem::swap;

/// Returns the Greatest Common Denominator (or greatest common factor) of it's two arguments.
pub fn euclid_gcd(x: u32, y: u32) -> u32 {
    let mut a = x;
    let mut b = y;

    if b == 0 {
        return a;
    }

    while a != 0 {
        if a >= b {
            a %= b;
        } else {
            swap(&mut a, &mut b);
        }
    }

    return b;
}

#[cfg(test)]
mod test {
    use crate::algo::euclid_gcd;

    #[test]
    fn gcd_x_and_zero_is_x() {
        for a in [1, 2, 63, 67, 0, 123, 5] {
            assert_eq!(a, euclid_gcd(a, 0));
            assert_eq!(a, euclid_gcd(0, a));
        }
    }

    #[test]
    fn gcd_prime_and_prime_is_one() {
        let primes = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];
        for i in 0..primes.len() {
            for j in 0..primes.len() {
                if i == j {
                    continue;
                }
                assert_eq!(1, euclid_gcd(primes[i], primes[j]));
                assert_eq!(1, euclid_gcd(primes[j], primes[i]));
            }
        }
    }
    #[test]
    fn gcd_xn_and_yn_is_n_for_coprime_x_and_y() {
        let primes = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];
        for n in [1, 2, 3, 4, 5, 9, 344, 25246, 3732, 232, 445, 13] {
            for i in 0..primes.len() {
                for j in 0..primes.len() {
                    if i == j {
                        continue;
                    }
                    assert_eq!(n, euclid_gcd(n * primes[i], n * primes[j]));
                    assert_eq!(n, euclid_gcd(n * primes[j], n * primes[i]));
                }
            }
        }
    }
}
