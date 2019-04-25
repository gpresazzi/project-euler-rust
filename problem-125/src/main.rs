/**
The palindromic number 595 is interesting because it can be written as the sum of consecutive squares: 
6^2 + 7^2 + 8^2 + 9^2 + 10^2 + 11^2 + 12^2.
There are exactly eleven palindromes below one-thousand that can be written as consecutive square sums, and the sum of these palindromes is 4164. 
Note that 1 = 0^2 + 1^2 has not been included as this problem is concerned with the squares of positive integers.

Find the sum of all the numbers less than 10^8 that are both palindromic and can be written as the sum of consecutive squares.
**/

use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use std::{thread, time};
use std::time::Instant;
static NTHREADS: i32 = 50;

fn sum_cons_squares(n:u64) -> bool{

    let mut is_over = true;
    let mut start = 1;
    let mut sum:u64 = 0; //contains current sum
    let mut current_n:u64 = start; // contins current value
    if n % 50000 == 0 {
        println!("*** Running check on {:?} ", n);
    }
    while is_over == true && start <= ((n as f64).sqrt() as u64) {
        is_over = false;
        sum = 0; //contains current sum
        current_n = start;
        if start > ((n as f64).sqrt() as u64) {
            return false;
        }

        while sum < n {
            sum = sum + current_n.pow(2);
            current_n = current_n + 1;
        }
        if sum > n {
            is_over = true;
        }
        if sum == n && current_n - start > 1 {
            //got it, we need more than 1 number
            //println!("{:?} can be written as the sum of consecutive squares. From {:?} to {:?}", n, start, current_n -1);
            return true;
        }

        start = start + 1;
        thread::sleep(time::Duration::from_micros(150));
    }
    return false;
}

fn is_palindrome(n:u64) -> bool{
    let s: String = n.to_string();

    for i in 0..s.len() / 2 {
        if s.chars().nth(i).unwrap() != s.chars().nth(s.len() - i - 1).unwrap() {
            return false;
        }
    }
    return true;
}

fn sum_palindromic_before(n:u64) -> u64{

    let (tx, rx): (Sender<u64>, Receiver<u64>) = channel();
    let mut children = vec![];

    for i in 0..NTHREADS {
        // The sender endpoint can be copied
        let thread_tx = tx.clone();

        // Spin up another thread
        children.push(thread::spawn(move || {
            let begin:u64 = (i as u64) * (n/NTHREADS as u64);
            let mut end:u64 = ((i + 1)as u64) * (n/NTHREADS as u64);
            if i == NTHREADS - 1{
                end = n;
            }
            let mut sum:u64 = 0;
            for _d in begin..end{
                if is_palindrome(_d) {
                    if sum_cons_squares(_d) {
                        println!("Palindrome {:?} can be written as the sum of consecutive squares.", _d);
                        sum = sum + _d;
                    }
                }
            }
            let _ = thread_tx.send(sum);
            // result.push(get_higher_div(num, begin, end, i));
            //println!("this is thread number {}. From {} to {}", i, begin, end);
        }
        ));
    }

    // Here, all the messages are collected
    let mut result:u64 = 0;
    for _ in 0..NTHREADS {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there are no messages available
        result = result + rx.recv().unwrap();
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }

    return result;
}

fn main() {
    let start_time = Instant::now();
    println!("Program start at: {:?}", start_time);

    let sum = sum_palindromic_before(10_u64.pow(8));

    println!("Result is : {}", sum);
    let elapsed = Instant::now() - start_time;
    println!("Program end at: {:?}, elapsed time: {:?}",  Instant::now(), elapsed);
}


// ################################# TEST #########################
#[test]
fn it_works() {
    let sum = sum_palindromic_before(1000);
    assert_eq!(sum, 4164);
}