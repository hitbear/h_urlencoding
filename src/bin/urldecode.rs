use std::env; //ToDo: std::env::args_os
use urlencoding::decode;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        hurlencoding::help(&args[0]);
    }

    let input = &args[1];
    let decoded = decode(input).expect("UTF-8");
    
    println!("{}", decoded);
}
