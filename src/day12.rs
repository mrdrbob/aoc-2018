use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::VecDeque;

const INIT_STATE:&str = "##..##..#.##.###....###.###.#.#.######.#.#.#.#.##.###.####..#.###...#######.####.##...#######.##..#";

pub fn execute() {
    let file = File::open("C:\\Data\\Working\\X-Other\\aoc-2018\\day12.txt").unwrap();
    let reader = BufReader::new(&file);

    let mut rules:HashMap<[bool; 5], bool> = HashMap::new();

    // Create a map of the rules using 5 item arrays as the keys
    for line in reader.lines() {
        let l = line.unwrap();
        let mut iter = l.chars();
        let key:[bool; 5] = [
            iter.next().expect("") == '#',
            iter.next().expect("") == '#',
            iter.next().expect("") == '#',
            iter.next().expect("") == '#',
            iter.next().expect("") == '#'
        ];
        let result = l.chars().skip(9).take(1).last().expect("result") == '#';
        rules.insert(key, result);
    }
    
    let mut state:VecDeque<bool> = VecDeque::new();
    for c in INIT_STATE.chars() {
        state.push_back(c == '#');
    }

    let mut index_offset:i32 = 0;
    let mut last_score = 0;
    let times_to_run:u64 = 5000;
    let mut diff = 0;

    for _ in 0..5000 {
        // Trim off all empty pots from the ends
        index_offset += trim_left(&mut state);
        trim_right(&mut state);

        //print_state(&state);
        //println!("{}", index_offset);

        // Pad out the ends with empty pots
        index_offset -= pad_left(&mut state, 4);
        pad_right(&mut state, 4);

        //print_state(&state);
        //println!("{}", index_offset);

        // apply this generation
        state = {
            // Create a new state
            let mut new_state:VecDeque<bool> = VecDeque::with_capacity(state.len());
            new_state.push_back(false);
            new_state.push_back(false);

            // Create the first key, consume first 5 pots
            let mut it = state.iter();
            let mut key:[bool; 5] = [
                it.next().expect("item").clone(),
                it.next().expect("item").clone(),
                it.next().expect("item").clone(),
                it.next().expect("item").clone(),
                it.next().expect("item").clone()
            ];

            // Iterate remaining pots, shifting the key each time, applying result to new state
            let mut cont = true;
            while cont {
                // apply rule, get result and apply to new state
                let rule = rules[&key];
                new_state.push_back(rule);

                // create new key and decide if not to continue
                cont = match it.next() {
                    Some(s) => {
                        key = [ key[1],key[2],key[3],key[4], s.clone() ];
                        true
                    },
                    None => false
                }
            }

            new_state
        };

        let sum = calculate_score(&state, index_offset);
        diff = sum - last_score;
        println!("Diff: {}", diff);
        last_score = sum;
    }

    // After here, score always goes up by the last diff
    let remaining_runs:u64 = 50000000000 - times_to_run;
    let sum = calculate_score(&state, index_offset) as u64;
    let final_score = sum + (remaining_runs * (diff as u64));

    println!("Sum: {}", sum);
    println!("Final score: {}", final_score);
}

fn calculate_score(state:&VecDeque<bool>, index_offset:i32) -> i32 {
    let mut index = index_offset;
    let mut score = 0;
    for t in state.iter() {
        if t == &true {
            score += index;
        }
        index += 1;
    }
    score
}

fn print_state(state:&VecDeque<bool>) {
    for t in state.iter() {
        // t == &true ? or t.clone() ? or ... ?
        print!("{}", if t == &true { '#' } else { '.' });
    }
    println!();
}

fn pad_left(state:&mut VecDeque<bool>, count:i32) -> i32 {
    for _ in 0..count {
        state.push_front(false);
    }
    count
}

fn pad_right(state:&mut VecDeque<bool>, count:i32) -> i32 {
    for _ in 0..count {
        state.push_back(false);
    }
    count
}

fn trim_left(state:&mut VecDeque<bool>) -> i32 {
    let mut removed = 0;
    loop {
        let left = state.pop_front().expect("A value");
        if left {
            state.push_front(left);
            return removed;
        }
        removed += 1;
    }
}

fn trim_right(state:&mut VecDeque<bool>) -> i32 {
    let mut removed = 0;
    loop {
        let right = state.pop_back().expect("A value");
        if right {
            state.push_back(right);
            return removed;
        }
        removed += 1;
    }
}
