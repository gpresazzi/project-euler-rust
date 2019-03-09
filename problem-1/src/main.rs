use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        println!("No number selected. Specify an input number");
        process::exit(0x0100);
    }
    let number_str = &args[1];
    println!("Input number {}", number_str);

    let input_number: i32 = number_str.parse().unwrap();

    let mut sum = 0;
    for i in 1..input_number{
        let mut multiple = false;
        if i % 3 == 0 {
            multiple = true;
        }
        else if i % 5 == 0 {
            multiple = true;   
        }

        if multiple{
            sum += i;
        }
    }
    println!("Sum {}", sum);

}