/**
2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
**/

use std::time::Instant;

fn is_divisible_for_any_num(n:u32, max:u32) -> bool{
    for i in 1..max + 1 {
        if n % i != 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    let start_time = Instant::now();
    println!("Program start at: {:?}", start_time);

    assert!(is_divisible_for_any_num(2520, 10));
    
    let max_div:u32 = 20;
    let mut number = max_div;
    let mut found = false;
    while found == false {
        found = is_divisible_for_any_num(number, max_div);
        number += 2; //the number must be even.
    }
    number -= 2;

    println!("Number divisible for any number to 1 and {} is : {}", max_div, number);
    let elapsed = Instant::now() - start_time;
    println!("Program end at: {:?}, elapsed time: {:?}",  Instant::now(), elapsed);
}