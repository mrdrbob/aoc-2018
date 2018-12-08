use std::fs::File;
use std::io::Read;
use std::vec::Vec;

pub fn execute_day_5() {
    let mut file = File::open("C:\\Data\\Working\\X-Other\\aoc-2018\\day5.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");


    let original_count = process_reactions(&contents);
    println!("Original count: {}", original_count);
    
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut smallest:Option<usize> = None;
    for letter in alphabet.chars() {
        // I'm sure there are worse ways to do this, but probably not many.
        let as_byte = letter.to_string().bytes().last().unwrap();

        let shortened = remove_letters(&contents, as_byte);
        let new_size = process_reactions(&shortened);
        smallest = match smallest {
            None => Some(new_size),
            Some(x) => {
                if new_size < x { 
                    println!("{} is smaller at {}", letter, new_size);
                    Some(new_size) 
                } else { 
                    Some(x) 
                }
            }
        };
    }
    

}

fn remove_letters(contents:&str, letter:u8) -> String {
    let mut stack:Vec<u8> = Vec::new();

    for c in contents.bytes() {
        if c != letter && !is_reactive(c, letter) {
            stack.push(c);
        }
    }

    String::from_utf8(stack).expect("valid utf8")
}

fn process_reactions(contents:&str) -> usize {
    let mut stack:Vec<u8> = Vec::new();

    for c in contents.bytes() {
        match stack.pop() {
            Some(x) => {
                match is_reactive(x, c) {
                    false => {
                        stack.push(x);
                        stack.push(c);
                    },
                    _ => {}
                };
            },
            None => { stack.push(c); }
        };
    }

    stack.len()
}

fn is_reactive(left:u8, right:u8) -> bool {
    left.wrapping_add(32) == right || right.wrapping_add(32) == left
}
