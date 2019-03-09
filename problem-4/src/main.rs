/**
A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
**/

use std::time::Instant;

fn is_palindrome(n:u32) -> bool{

    let s: String = n.to_string();

    for i in 0..s.len() / 2 {
        if s.chars().nth(i).unwrap() != s.chars().nth(s.len() - i - 1).unwrap() {
            return false;
        }
    }
    return true;
}

fn main() {
    println!("Program start at: {:?}",  Instant::now());

    let mut max_palindrome = 0;
    //generate all the number with 3 digit
    for x in 100..999 {
        for y in 100..999 {
            let mult = x * y;
            if is_palindrome(mult) && max_palindrome < mult{
                max_palindrome = mult;
            }
        }
    }

    println!("Found max palindromic number: {}",  max_palindrome);
    println!("Program end at: {:?}",  Instant::now());
}