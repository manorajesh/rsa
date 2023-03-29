use num_bigint::{BigUint, RandBigInt, ToBigUint};
use rand::thread_rng;

fn main() {
    // primes (private key)
    let mut p: BigUint = thread_rng().gen_biguint(2048);
    let mut q: BigUint = thread_rng().gen_biguint(2048);

    let mut counter = 0;

    while !is_prime(&p, 10) {
        p += 1.to_biguint().unwrap();
        counter += 1;
        if counter % 10 == 0 {
            print!("\r{}\n", "*".to_string().repeat(counter / 50));
        }
    }
    counter = 0;
    while !is_prime(&q, 10) {
        q += 1.to_biguint().unwrap();
        counter += 1;
        if counter % 10 == 0 {
            print!("\r{}\n", "*".to_string().repeat(counter / 50));
        }
    }

    // public key
    let n = &p * &q;

    let data = 2.to_biguint().unwrap();
    let encrypted = encrypt(&data, &n);
    // println!("Encrypted: {:x}", encrypted);
    println!("public key: {:x}\n", n);
    println!("private key: {:x}\n{:x}\n", p, q);
    println!("Encrypted: {:x}", encrypted);

    let decrypted = decrypt(&data, &p, &q);
    println!("\nDecrypted: {:x}\n", decrypted);
}

fn encrypt(data: &BigUint, n: &BigUint) -> BigUint {
    data * n
}

fn decrypt(data: &BigUint, key_0: &BigUint, key_1: &BigUint) -> BigUint {
    data / key_0 / key_1
}

fn is_prime(n: &BigUint, k: u32) -> bool {
    if n <= &1.to_biguint().unwrap() || n == &4.to_biguint().unwrap() {
        return false;
    } else if n <= &3.to_biguint().unwrap() {
        return true;
    }

    let mut d = n - 1.to_biguint().unwrap();
    while d.clone() % 2.to_biguint().unwrap() == 0.to_biguint().unwrap() {
        d /= 2.to_biguint().unwrap();
    }

    for _ in 0..k {
        if !miller_rabin(&mut d, n) {
            return false;
        }
    }
    return true;
}

fn miller_rabin(d: &mut BigUint, n: &BigUint) -> bool {
    // Miller-Rabin Primality Test

    let a: BigUint = 2.to_biguint().unwrap()
        + thread_rng().gen_biguint_range(&1.to_biguint().unwrap(), &(n - 4.to_biguint().unwrap()));

    let mut x = a.modpow(&d, &n);
    if x == 1.to_biguint().unwrap() || x == n - 1.to_biguint().unwrap() {
        return true;
    }

    while *d != n - 1.to_biguint().unwrap() {
        x = &x * &x % n;
        *d *= 2.to_biguint().unwrap();

        if x == 1.to_biguint().unwrap() {
            return false;
        }
        if x == n - 1.to_biguint().unwrap() {
            return true;
        }
    }
    return false;
}

// fn expmod(base: BigUint, exp: BigUint, modulus: BigUint) -> BigUint {
//     // Modular Exponentiation
//     let mut result = 1.to_biguint().unwrap();
//     let mut base = base;
//     let mut exp = exp;

//     base = base % modulus;
//     while exp > 0.to_biguint().unwrap() {
//         if exp % 2.to_biguint().unwrap() == 1.to_biguint().unwrap() {
//             result = (result * base) % modulus;
//         }
//         exp = exp >> 1;
//         base = (base * base) % modulus;
//     }
//     result
// }
