use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::vec::Vec;
use std::collections::HashMap;
use std::collections::HashSet;


pub fn execute_six() {
    let file = File::open("C:\\Data\\Working\\X-Other\\aoc-2018\\day6.txt").unwrap();
    let reader = BufReader::new(&file);
    let mut coords:Vec<(i32, i32)> = Vec::new();
    let mut lower_bound_x:Option<i32> = None;
    let mut upper_bound_x:Option<i32> = None;
    let mut lower_bound_y:Option<i32> = None;
    let mut upper_bound_y:Option<i32> = None;

    // Read in all the coordinates, calculate the upper and lower boundaries
    for line in reader.lines() {
        let l = line.unwrap();
        let (x, y) = parse_cords(&l);
        lower_bound_x = take_min(lower_bound_x, x);
        upper_bound_x = take_max(upper_bound_x, x);
        lower_bound_y = take_min(lower_bound_y, y);
        upper_bound_y = take_max(upper_bound_y, y);

        coords.push((x, y));
    }

    let origin_x = lower_bound_x.unwrap();
    let origin_y = lower_bound_y.unwrap();
    let upper_x = upper_bound_x.unwrap();
    let upper_y = upper_bound_y.unwrap();

    let width = upper_x - origin_x;
    let height = upper_y - origin_x;
    let offset = (origin_x, origin_y);

    let normalized_coords:Vec<(i32, i32)> = coords.iter().map(|c| diff(c, &offset)).collect();

    let mut coord_scores:HashMap<(i32, i32), i32> = HashMap::new();
    let mut infinite_coords:HashSet<(i32, i32)> = HashSet::new();

    // Calculate all the distances
    for x in 0..width {
        for y in 0..height {
            let pos = (x, y);

            let mut shortest_dist:Option<i32> = None;
            let mut closest_coord:Option<(i32, i32)> = None;
            
            // Loop through all know coordinates, calculating the closest coordinate
            // If two coordinates are equadistance, ignore them
            for coord in &normalized_coords {
                let dist = manhattan_distance(coord, &pos);
                shortest_dist = match shortest_dist {
                    None => {
                        closest_coord = Some(coord.clone());
                        Some(dist)
                    },
                    Some(current) => {
                        if current == dist {
                            closest_coord = None;
                            Some(current)
                        } else if dist < current {
                            closest_coord = Some(coord.clone());
                            Some(dist)
                        } else {
                            Some(current)
                        }
                    }
                };
                // println!("{:?} {:?} {}, {:?}", pos, coord, dist, shortest_dist);
            }

            // Add a score for the closest coord if found.
            coord_scores = match closest_coord {
                Some(c) => {
                    // literal edge case, if we're at a boundary and this coord is the closest,
                    // it is considered infinite.
                    let at_boundary = x == 0 || x == width - 1 || y == 0 || y == height - 1;
                    if at_boundary {
                        infinite_coords.insert(c);
                    }
                    let new_score = match coord_scores.remove(&c) {
                        Some(score) => score + 1,
                        None => 1
                    };
                    coord_scores.insert(c, new_score);
                    coord_scores
                },
                _ => coord_scores
            }
        }
    }

    // Now that all the scores are counted, need to remove infinite scores
    for c in &infinite_coords {
        coord_scores.remove(c);
    }

    println!("{} {}", infinite_coords.len(), coord_scores.len() );
    println!("{:?}", coord_scores);

    // Get the largest score from remaining
    let largest_score = coord_scores.values().max_by_key(|&g| g).unwrap();
    println!("Largest area: {}", largest_score);

}

fn manhattan_distance(t1:&(i32, i32), t2:&(i32, i32)) -> i32 {
    let (x, y) = diff(&t1, &t2);
    x.abs() + y.abs()
}

fn diff(t1:&(i32, i32), t2:&(i32, i32)) -> (i32, i32) {
    let (x1, y1) = t1;
    let (x2, y2) = t2;
    (x1 - x2, y1 - y2)
}

fn apply(current:Option<i32>, next:i32, cmp:fn(i32, i32)->i32) -> Option<i32> {
    match current {
        None => Some(next),
        Some(c) => Some(cmp(c, next))
    }
}

fn take_min(current:Option<i32>, next:i32) -> Option<i32> {
    apply(current, next, |x,y| x.min(y))
}

fn take_max(current:Option<i32>, next:i32) -> Option<i32> {
    apply(current, next, |x,y| x.max(y))
}

fn parse_cords(line:&str) -> (i32, i32) {
    let mut t = line.split(",");

    let first_segment = t.next().expect("No first segment");
    let x:i32 = first_segment.parse().expect("First segment is not int");
    let next_segment:String = t.next().expect("No second segment").chars().skip(1).collect();
    let y:i32 = next_segment.parse().expect("Second segment is not int");
    (x, y)
}