use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;

pub fn day_one_part_two() -> i32 {
    let mut total = 0;
    let mut seen_frequences:HashSet<i32>  = HashSet::new();
    let mut total_runs = 0;
    
    while total_runs < 1000 {
        let file = File::open("C:\\Data\\Working\\X-Other\\aoc-2018\\day1.txt").unwrap();
        let reader = BufReader::new(&file);

        for line in reader.lines() {
            let l = line.unwrap();
            let parsed = l.parse::<i32>().unwrap();
            total += parsed;
            if seen_frequences.contains(&total) {
                // println!("{}", seen_frequences.len());
                return total;
            }else {
                seen_frequences.insert(total);
            }
        }

        total_runs += 1;
    }
    panic!();
}

pub fn day_one() -> i32 {
    let file = File::open("C:\\Data\\Working\\X-Other\\aoc-2018\\day1.txt").unwrap();
    let mut total = 0;
    let reader = BufReader::new(&file);
    for line in reader.lines() {
        let l = line.unwrap();
        let parsed = l.parse::<i32>().unwrap();
        total += parsed;
    }
    total
}
