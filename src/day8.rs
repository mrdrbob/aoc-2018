use std::fs::File;
use std::io::Read;
use std::vec::Vec;
use std::str::Split;

pub fn day_eight_the_ocho() {
    let mut file = File::open("C:\\Data\\Working\\X-Other\\aoc-2018\\day8.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    let as_integers:Vec<i32> = contents.split(" ").map(|g| g.parse::<i32>().expect("valid integer")).collect();

    let (entry, final_index) = parse_entry(&as_integers, 0);
    println!("Final index: {}", final_index);

    let meta_sum = sum_meta_values(&entry);
    println!("Meta sum: {}", meta_sum);
    
    let part2_sum = sum_meta_part_2(&entry);
    println!("Part 2 Sum: {}", part2_sum);
}

fn sum_meta_values(entry:&Entry) -> i32 {
    let mut my_sum = 0;
    for child in entry.child_nodes.iter() {
        my_sum += sum_meta_values(child);
    }

    for meta in entry.meta_elements.iter() {
        my_sum += meta.clone();
    }

    my_sum
}

fn sum_meta_part_2(entry:&Entry) -> i32 {
    if entry.child_nodes.len() == 0 {
        let sum:i32 = entry.meta_elements.iter().map(|g| g.clone()).sum();
        return sum;
    }

    let mut child_sum = 0;
    for idx in entry.meta_elements.iter().map(|g| g.clone() - 1) {
        let index = idx as usize;
        if index < entry.child_nodes.len() {
            let child = entry.child_nodes.get(index).expect("child node");
            child_sum += sum_meta_part_2(child);
        }
    }

    child_sum
}

fn parse_entry(list:&Vec<i32>, current_index:usize) -> (Entry, usize) {
    let child_count = list.get(current_index).expect("child count").clone();
    let meta_count = list.get(current_index + 1).expect("meta count").clone();

    let mut kids:Vec<Entry> = Vec::new();
    let mut index = current_index + 2;
    for _ in 0..child_count {
        let (entry, new_index) = parse_entry(list, index);
        kids.push(entry);
        index = new_index;
    }

    let mut meta:Vec<i32> = Vec::new();
    for _ in 0..meta_count {
        meta.push(list.get(index).expect("meta element").clone());
        index += 1;
    }

    let final_entry = Entry {
        child_nodes: kids,
        meta_elements: meta
    };

    (final_entry, index)
}

#[derive(Debug)]
struct Entry {
    child_nodes:Vec<Entry>,
    meta_elements:Vec<i32>
}

