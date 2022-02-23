use std::env; //ToDo: std::env::args_os
use urlencoding::encode;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        hurlencoding::help(&args[0]);
    }

    let input = &args[1];
    let encoded = encode(input);
    
    println!("{}", encoded);
}
