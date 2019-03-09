/**
The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
**/

use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::time::Instant;
static NTHREADS: i32 = 60;

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

fn get_higher_div(num:u64, start_div:u64, end_div:u64, thread_id:i32) -> u64{
    
    let mut count:u64 = 0;
    for div in (start_div..end_div).rev(){
        if count % 10000000000 == 0 {
            println!("Checking num: {} in thread {}", div, thread_id);
        }

        if num % div == 0 && is_prime(div){
            return div;
        }
        count +=1;
    }    
    return 1;
}

fn main() {
    println!("Program start: {:?}",  Instant::now());

    //let num:u64 = 600851475143; 
    let num:u64 = 600851475143;

    // Make a vector to hold the children which are spawned.
    let mut children = vec![];
    let (tx, rx): (Sender<i32>, Receiver<i32>) = channel();

    let max_possible_div = num /2;

    for i in 0..NTHREADS {
        // The sender endpoint can be copied
        let thread_tx = tx.clone();

        // Spin up another thread
        children.push(thread::spawn(move || {
            let begin:u64 = (i as u64) * (max_possible_div/NTHREADS as u64);
            let end:u64 = ((i + 1)as u64) * (max_possible_div/NTHREADS as u64) - 1;
            let _ = thread_tx.send(get_higher_div(num, begin, end, i) as i32);
            // result.push(get_higher_div(num, begin, end, i));
            println!("this is thread number {}", i);
        }
        ));
    }

    // Here, all the messages are collected
    let mut result = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there are no messages available
        result.push(rx.recv());
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }

    println!("{:?}", result);
     println!("Program finish: {:?}",  Instant::now());  
}