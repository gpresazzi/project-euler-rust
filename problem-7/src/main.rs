/**
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
What is the 10 001st prime number?
**/

use std::time::Instant;

fn is_prime(n:u64) -> bool{

    let mut prime = true;
    let mid = n/2;
    for d in 2..mid{
        if n % d == 0 {
            prime = false;
            break;
        }
    }
    return prime;
}

fn main() {
    let start_time = Instant::now();
    println!("Program start at: {:?}", start_time);

    let mut count_prime = 0;
    let mut number = 1;
    while count_prime <= 10001 {
        number +=1;
        if is_prime(number){
            count_prime += 1;
        }
    }

    println!("Result is : {}", number);
    let elapsed = Instant::now() - start_time;
    println!("Program end at: {:?}, elapsed time: {:?}",  Instant::now(), elapsed);
}