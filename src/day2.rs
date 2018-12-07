use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;
use std::vec::Vec;
use std::iter::Iterator;


pub fn day_two() {
    let file = File::open("C:\\Data\\Working\\X-Other\\aoc-2018\\day2.txt").unwrap();
    let reader = BufReader::new(&file);
    
    let mut two = 0;
    let mut three = 0;

    for line in reader.lines() {
        let l = line.unwrap();
        // Yeah, this is a weird way to do it.
        let mut matched_one:HashSet<char> = HashSet::new();
        let mut matched_two:HashSet<char> = HashSet::new();
        let mut matched_three:HashSet<char> = HashSet::new();
        let mut invalid:HashSet<char> = HashSet::new();

        for c in l.chars() {
            if invalid.contains(&c) {
                // println!("Invalid: {}", &c);
                continue;
            }
            
            if matched_three.contains(&c) {
                // println!("-> invalid: {}", &c);
                matched_three.remove(&c);
                invalid.insert(c);
                continue;
            }
            if matched_two.contains(&c) {
                // println!("-> three: {}", &c);
                matched_two.remove(&c);
                matched_three.insert(c);
                continue;
            }
            if matched_one.contains(&c) {
                // println!("-> two: {}", &c);
                matched_one.remove(&c);
                matched_two.insert(c);
                continue;
            }
            // println!("one: {}", &c);
            matched_one.insert(c);
        }
        if matched_two.len() > 0 {
            two += 1;
        }
        if matched_three.len() > 0 {
            three += 1;
        }
        // println!();
    }

    let total = two * three;
    println!("{}", total);
}


pub fn day_two_part_two() {
    let file = File::open("C:\\Data\\Working\\X-Other\\aoc-2018\\day2.txt").unwrap();
    let reader = BufReader::new(&file);
    
    let mut list:Vec<String> = Vec::new();
    for line in reader.lines() {
        let l = line.unwrap();
        list.push(l);
    }

    for x in 0..list.len() {
        for y in 0..list.len() {
            if x == y {
                continue;
            }
            let in_common = common_letters(&list[x], &list[y]);
            if in_common.len() == (&list[x].len() - 1) {
                let t:String = in_common.iter().collect();

                println!("{}", t);
                return;
            }
        }
    }
}

fn common_letters(left:&String, right:&String) -> Vec<char> {
    let mut left_iter = left.chars();
    let mut right_iter = right.chars();
    let mut cont = true;
    let mut result:Vec<char> = Vec::new();

    while cont {
        // I have no idea what I'm doing...
        let diff = match left_iter.next() {
            Some(left_val) => match right_iter.next() {
                Some(right_val) => if left_val.eq(&right_val) { Some(left_val) } else { None },
                None => {cont = false; None }
            },
            None => {cont = false; None }
        };
        match diff {
            Some(v) => result.push(v),
            _ => ()
        };
    }

    result
}