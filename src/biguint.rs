use rand::Rng;
use std::{fmt, ops};

#[derive(Debug)]
struct BigUInt {
    data: Vec<u128>,
}

impl BigUInt {
    fn is_even(&self) -> bool {
        self.data[0] % 2 == 0
    }

    fn new_random(size: usize) -> BigUInt {
        let mut rng = rand::thread_rng();
        let mut data = Vec::new();
        for _ in 0..size {
            data.push(rng.gen_range(0..u128::MAX));
        }
        BigUInt { data }
    }

    fn from(data: Vec<u128>) -> BigUInt {
        BigUInt { data }
    }

    fn new() -> BigUInt {
        BigUInt { data: Vec::new() }
    }
}

impl fmt::Display for BigUInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.data.iter().rev() {
            write!(f, "{}", byte)?;
        }
        Ok(())
    }
}

impl ops::Add for BigUInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = BigUInt::new();
        let mut carry = 0;
        for i in 0..rhs.data.len() {
            let sum = self.data[i] + rhs.data[i] + carry;
            result.data.push(sum);
            carry = sum / u128::MAX;
        }
        result
    }
}


fn main() {
    // // primes
    // let p = 3;
    // let q = 7;

    // // public key
    // let n = p * q;

    // let data = 2;
    // let encrypted = encrypt(data, n);
    // println!("Encrypted: {:x}", encrypted);

    let a = BigUInt::from(vec![3, 2, 1]);
    println!("a: {}", a);
    println!("a is even: {}", a.is_even());
    println!("a: {}", a+BigUInt::from(vec![2]));
}

fn encrypt(data: i32, n: i32) -> i32 {
    data * n
}