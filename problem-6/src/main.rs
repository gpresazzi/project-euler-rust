/**
The sum of the squares of the first ten natural numbers is,

1^2 + 2^2 + ... + 10^2 = 385
The square of the sum of the first ten natural numbers is,

(1 + 2 + ... + 10)2 = 55^2 = 3025
Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.**/

use std::time::Instant;


fn main() {
    let start_time = Instant::now();
    println!("Program start at: {:?}", start_time);

    let mut sum_squared:u64 = 0;
    let mut sum:u64 = 0;
    for i in 1..101 {
        sum_squared += u64::pow(i,2);
        sum += i as u64;
    }

    let number = u64::pow(sum,2) - sum_squared;

    println!("Result is : {}", number);
    let elapsed = Instant::now() - start_time;
    println!("Program end at: {:?}, elapsed time: {:?}",  Instant::now(), elapsed);
}