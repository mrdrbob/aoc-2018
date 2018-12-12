use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::vec::Vec;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn execute() {
    let file = File::open("C:\\Data\\Working\\X-Other\\aoc-2018\\day10.txt").unwrap();
    let reader = BufReader::new(&file);
    let mut particles:Vec<Particle> = Vec::new();
    let max_x = 79;
    let max_y = 16;

    let mut sum_x = 0;
    let mut sum_y = 0;
    let mut count = 0;

    for line in reader.lines() {
        let l = line.unwrap();
        let pos_x:i32 = l.chars().skip(10).take(6).collect::<String>().trim_left().parse().expect("pos_x");
        let pos_y:i32 = l.chars().skip(18).take(6).collect::<String>().trim_left().parse().expect("pos_y");
        
        let vel_x:i32 = l.chars().skip(36).take(2).collect::<String>().trim_left().parse().expect("vel_x");
        let vel_y:i32 = l.chars().skip(40).take(2).collect::<String>().trim_left().parse().expect("vel_y");

        let particle = Particle {
            pos: (pos_x, pos_y),
            velocity: (vel_x, vel_y)
        };

        sum_x += pos_x / vel_x;
        sum_y += pos_y / vel_y;
        count += 1;

        particles.push(particle);
    }

    println!("{}", sum_x / count);
    println!("{}", sum_y / count);
    // panic!();

    let mut time = 1000;
    let mut cont = true;
    while cont {

        let particle_pos:Vec<(i32, i32)> = particles.iter().map(|part| {
            let (x, y) = part.pos;
            let (vx, vy) = part.velocity;
            let new_x = x + (vx * time);
            let new_y = y + (vy * time);
            (new_x, new_y)
        }).collect();

        let max_x = particle_pos.iter().map(|(x,_)| x).max().expect("max x").clone();
        let min_x = particle_pos.iter().map(|(x,_)| x).min().expect("min x").clone();

        let max_y = particle_pos.iter().map(|(_,y)| y).max().expect("max y").clone();
        let min_y = particle_pos.iter().map(|(_,y)| y).min().expect("min y").clone();

        let width = max_x - min_x;
        let height = max_y - min_y;

        if width < 100 || height < 20 {
            println!("{} {} {}", time, width, height);
            let map:HashSet<(i32, i32)> = HashSet::from_iter(particle_pos);

            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    if map.contains(&(x, y)) {
                        print!("*");
                    } else {
                        print!(" ");
                    }
                }
                println!()
            }

            println!("{}", time);
            let mut tmp:String = String::new();
            std::io::stdin().read_line(&mut tmp).expect("user input");
        }
        
        time += 1;
    }

    println!("{}", time);
}

#[derive(Debug)]
struct Particle {
    pos:(i32, i32),
    velocity:(i32,i32)
}
