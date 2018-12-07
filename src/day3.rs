use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;
use std::collections::HashMap;
use std::vec::Vec;
use std::iter::Iterator;
use std::cmp;

pub fn day_three_part_two() {
    let file = File::open("C:\\Data\\Working\\X-Other\\aoc-2018\\day3.txt").unwrap();
    let reader = BufReader::new(&file);

    let mut patches:Vec<Patch> = Vec::new();

    let mut max_x = 0;
    let mut max_y = 0;

    for line in reader.lines() {
        let l = line.unwrap();
        let parsed = Patch::parse(&l);
        max_x = cmp::max(max_x, parsed.offset_x + parsed.width);
        max_y = cmp::max(max_y, parsed.offset_y + parsed.height);

        patches.push(parsed);
    }

    let mut viable_patches:HashSet<String> = HashSet::new();
    let mut hits:HashMap<(i32, i32), Vec<String>> = HashMap::new();

    for patch in &patches {
        viable_patches.insert(patch.id.clone());

        for w in 0..patch.width {
            for h in 0..patch.height {
                let tuple = (patch.offset_x + w, patch.offset_y + h);
                // Probably not the most efficient to remove and re-insert...
                let mut list = match hits.remove(&tuple) {
                    None => Vec::new(),
                    Some(current) => current
                };
                list.push(patch.id.clone());
                hits.insert(tuple, list);
            }
        }
    }

    for id in hits.values().filter(|c| c.len() > 1 ).flat_map(|c| c) {
        viable_patches.remove(id);
    }

    for patch in viable_patches {
        println!("{}", patch);
    }
}

pub fn day_three() {
    let file = File::open("C:\\Data\\Working\\X-Other\\aoc-2018\\day3.txt").unwrap();
    let reader = BufReader::new(&file);

    let mut patches:Vec<Patch> = Vec::new();

    let mut max_x = 0;
    let mut max_y = 0;

    for line in reader.lines() {
        let l = line.unwrap();
        let parsed = Patch::parse(&l);
        max_x = cmp::max(max_x, parsed.offset_x + parsed.width);
        max_y = cmp::max(max_y, parsed.offset_y + parsed.height);

        patches.push(parsed);
    }

    let mut hits:HashMap<(i32, i32), i32> = HashMap::new();
    for patch in &patches {
        for w in 0..patch.width {
            for h in 0..patch.height {
                let tuple = (patch.offset_x + w, patch.offset_y + h);
                let count = match hits.remove(&tuple) {
                    None => 1,
                    Some(current) => current + 1
                };
                hits.insert(tuple, count);
            }
        }
    }

    let total = hits.values().filter(|c| c > &&1 ).count();
    println!("{}", total);

    /* Slow as fuuuuuuuuuuuuuudge
    let mut patches_with_two_hits = 0;
    println!("{} {}", max_x, max_y);
    for x in 0..max_x {
        println!("{}", x);
        for y in 0..max_y {
            let mut hits = 0;
            for patch in &patches {
                if patch.is_hit(x, y) {
                    hits += 1;
                    if hits > 1 {
                        patches_with_two_hits += 1;
                        break;
                    }
                }
            }
        }
    }

    println!("{}", patches_with_two_hits);
    */
}

#[derive(Debug)]
struct Patch {
    id: String,
    offset_x: i32,
    offset_y: i32,
    width: i32,
    height: i32
}

impl Patch {
    fn parse(raw:&str) -> Patch {
        // #1 @ 912,277: 27x20
        let mut t = raw.split(" ");
        let id = t.next().unwrap();
        let _at = t.next().unwrap();
        let offset = t.next().unwrap();
        let size = t.next().unwrap();

        let offset_trimmed = offset.trim_end_matches(':');
        let mut offset_split = offset_trimmed.split(",");
        let offset_x = offset_split.next().unwrap();
        let offset_y = offset_split.next().unwrap();

        let mut size_split = size.split("x");
        let size_x = size_split.next().unwrap();
        let size_y = size_split.next().unwrap();

        Patch {
            id: String::from(id),
            offset_x: offset_x.parse().unwrap(),
            offset_y: offset_y.parse().unwrap(),
            width: size_x.parse().unwrap(),
            height: size_y.parse().unwrap()
        }
    }

    fn is_hit(&self, x:i32, y:i32) -> bool {
        x >= self.offset_x && x < (self.offset_x + self.width) &&
        y >= self.offset_y && y < (self.offset_y + self.height)
    }
}