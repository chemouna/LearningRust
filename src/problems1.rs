/*
* Unique characters check:
 */

use std::env;

fn unique_chars(s: &str) -> bool {
    let v: Vec<char> = s.chars().collect();
    let mut y = v.clone();

    //TODO: check if we need to sort the vector first to remove all duplicates

    y.dedup();
    v.len() == y.len()
}

fn main() -> () {
    let args: Vec<String> = env::args().collect();

    match args.len {
        1 => {
            println!("No string argument to check is passed in, Please pass some arguments.")
        }
    }
    let sentence = args[1];

    match unique_chars(sentence) {
        true => println!("String contains only unique characters"),
        false => println!("String doesn't contain only unique characters")
    }
}
