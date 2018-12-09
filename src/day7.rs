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
    let mut max_cycles = 5000;
    while dependency_map.len() > 0 && max_cycles > 0 {
        let mut ready_to_run:Vec<char> = dependency_map.iter()
            .filter(|(_, v)|  {
                v.iter().filter(|x| !met_dependencies.contains(&x)).count() == 0
            })
            .map(|(k, _)| k.clone())
            .collect();
        
        if ready_to_run.len() == 0 {
            panic!();
        }

        ready_to_run.sort();

        // for c in ready_to_run {
        let c = ready_to_run.first().unwrap();
        print!("{}", c);
        met_dependencies.insert(c.clone());

        met_dependencies.insert(c.clone());
        dependency_map.remove(c);

    

        max_cycles -= 1;
    }

    if max_cycles <= 1 {
        panic!("Infinite loop");
    }


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