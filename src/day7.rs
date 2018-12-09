use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::vec::Vec;
use std::collections::HashMap;
use std::collections::HashSet;


pub fn day_seven_go_do_thing() {
    let file = File::open("C:\\Data\\Working\\X-Other\\aoc-2018\\day7.txt").unwrap();
    let reader = BufReader::new(&file);

    // char = me
    // Things I depend on
    let mut dependency_map:HashMap<char, HashSet<char>> = HashMap::new();

    for line in reader.lines() {
        let l = line.unwrap();
        let parsed = ParsedLine::parse(&l);
        dependency_map = ensure_record(dependency_map, parsed.dependency);
        dependency_map = add_dependency(dependency_map, parsed.step, parsed.dependency);
    }

    let mut met_dependencies:HashSet<char> = HashSet::new();
    let mut max_cycles = 50000;
    let mut seconds_spent = 0;
    let total_workers = 5;

    // Represents the amount of time remaining for a given job.
    let mut workers_currently_working:Vec<Work> = Vec::new();

    while dependency_map.len() > 0 && max_cycles > 0 {
        // Calculate which jobs are available based on met dependencies
        let mut ready_to_run:Vec<char> = dependency_map.iter()
            .filter(|(_, v)|  {
                v.iter().filter(|x| !met_dependencies.contains(&x)).count() == 0
            })
            .map(|(k, _)| k.clone())
            .collect();
        
        // Sort the list
        ready_to_run.sort();

        // Figure out how many (if any) workers are available to take on jobs.
        let available_workers = total_workers - workers_currently_working.len();
        let jobs_to_assign = available_workers.min(ready_to_run.len());

        // println!("{:?}", workers_currently_working);
        // println!("{} {} {}", ready_to_run.len(), available_workers, jobs_to_assign );

        // Dole out those jobs
        for _ in 0..jobs_to_assign {
            let letter = ready_to_run.first().expect("A job").clone();
            let letter_length = letter.to_string().bytes().last().unwrap() - 64;
            let job_length:i32 = 60 + i32::from(letter_length);
            
            // println!("{}", job_length);
            workers_currently_working.push(Work {
                id: letter.clone(),
                time_remaining: job_length
            });

            // Remove it from the read-to-run list
            ready_to_run.remove(0);
            dependency_map.remove(&letter);
        }

        // println!("{:?}", workers_currently_working);

        // Spend time on each of the items, remove worker if they only have 1 second remaining
        let mut new_time:Vec<Work> = Vec::new();
        while workers_currently_working.len() > 0 {
            let current_value = workers_currently_working.remove(0);
            match current_value.time_remaining {
                1 => {
                    // This dep is now met
                    met_dependencies.insert(current_value.id.clone());
                }
                v => new_time.push(Work {
                    id: current_value.id.clone(),
                    time_remaining: v - 1
                })
            }
        }

        workers_currently_working = new_time;

        max_cycles -= 1;
        seconds_spent += 1;
    }

    if max_cycles <= 1 {
        panic!("Infinite loop");
    }

    let remaining = workers_currently_working.iter().max_by_key(|g| g.time_remaining);
    let seconds_remaining = match remaining {
        Some(s) => s.time_remaining,
        None => 0
    };

    println!("{}", seconds_spent + seconds_remaining);
}

fn ensure_record(mut map:HashMap<char, HashSet<char>>, step:char) -> HashMap<char, HashSet<char>> {
    if !map.contains_key(&step) {
        map.insert(step, HashSet::new());
    }
    map
}


fn add_dependency(mut map:HashMap<char, HashSet<char>>, step:char, dep:char) -> HashMap<char, HashSet<char>> {
    let mut vector = match map.remove(&step) {
        Some(v) => v,
        None => HashSet::new()
    };
    vector.insert(dep);
    map.insert(step, vector);
    map
}

struct Work {
    id: char,
    time_remaining: i32
}

#[derive(Debug)]
struct ParsedLine {
    dependency: char,
    step: char
}

impl ParsedLine {
    fn parse(line:&str) -> ParsedLine {
        //           1         2         3
        // 0123456789012345678901234567890123456
        // Step X must be finished before step X can begin.
        let dependency = line.chars().skip(5).take(1).last().expect("Dependency");
        let step = line.chars().skip(36).take(1).last().expect("Step");
        ParsedLine { step, dependency }
    }
}