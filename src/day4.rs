use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn execute_day_4() {
    let file = File::open("C:\\Data\\Working\\X-Other\\aoc-2018\\day4.txt").unwrap();
    let reader = BufReader::new(&file);
    let mut lines:Vec<Line> = Vec::new();

    for line in reader.lines() {
        let l = line.unwrap();
        let parsed = Line::parse(&l);
        lines.push(parsed);
    }

    lines.sort();

    let mut guards:HashMap<String, Guard> = HashMap::new();
    let mut state = State::InbetweenGuards;

    for line in &lines {
        // This seems... more complicated than it needs to be?
        state = match state {
            State::InbetweenGuards => {
                match &line.data {
                    LineType::BeginsShift(guard) => State::InShift(Guard { id: guard.to_string(), total_minutes: 0, minute_dict: HashMap::new() }),
                    _ => panic!()
                }
            },
            State::InShift(in_shift_guard) => {
                match &line.data {
                    LineType::BeginsShift(new_guard) => {
                        guards = add_to_map(in_shift_guard, guards);
                        State::InShift(Guard { id: new_guard.to_string(), total_minutes: 0, minute_dict: HashMap::new() })
                    },
                    LineType::FallsAsleep => {
                        State::Sleeping(in_shift_guard, Moment { year: line.year, month: line.month, day: line.day, hour: line.hour, min: line.min })
                    },
                    _ => unimplemented!()
                }
            },
            State::Sleeping(sleeping_guard, start) => {
                match &line.data {
                    LineType::WakesUp => {
                        let now = Moment { year: line.year, month: line.month, day: line.day, hour: line.hour, min: line.min };
                        let minutes_slept = now.diff_minutes(&start);
                        let new_minutes = sleeping_guard.total_minutes + minutes_slept;
                        let mut minute_map = sleeping_guard.minute_dict;
                        for min in start.min..now.min {
                            let new_count = match minute_map.remove(&min) {
                                None => 1,
                                Some(val) => val + 1
                            };
                            minute_map.insert(min, new_count);
                        }

                        State::InShift(Guard { id: sleeping_guard.id.to_string(), total_minutes: new_minutes, minute_dict: minute_map })
                    },
                    _ => panic!()
                }
            }
        };
    }
    match state {
        State::InShift(in_shift_guard) => {
            guards = add_to_map(in_shift_guard, guards);
        },
        _ => panic!()
    };
    

    let sleepiest_guard = guards.values().max_by_key(|g| g.total_minutes ).unwrap();
    println!("Sleepiest guard: {}", sleepiest_guard.id);
    println!("Sleepiest guard time: {}", sleepiest_guard.total_minutes);
    println!("Sleepiest dict size: {}", sleepiest_guard.minute_dict.len());


    let (sleepiest_minute, time_spent) = sleepiest_guard.minute_dict.iter().max_by_key(|(&_, &val)| val).unwrap();
    println!("Sleepiest minute: {}", sleepiest_minute);
    println!("Time in minute: {}", time_spent);


    // Part 2, which guard is most frequently asleep on the same minute?
    let most_on_a_single_minute = guards.values().max_by_key(|g| {
        match g.minute_dict.iter().max_by_key(|(_, v)| v.clone() ) {
            None => 0,
            Some((_, v)) => v.clone()
        }
    }).unwrap();

    let (most_req_minute, minute_time) = most_on_a_single_minute.minute_dict.iter().max_by_key(|(_, v)| v.clone() ).unwrap().clone();

    println!("Most on a single minute: {}", most_on_a_single_minute.id);
    println!("Most freq minute: {}", most_req_minute);
    println!("Time on said minute: {}", minute_time);
}

fn add_to_map(g:Guard, mut map:HashMap<String, Guard>) -> HashMap<String, Guard> {
    // This is nuts.
    let to_insert = match map.remove(&g.id) {
        None => g,
        Some(existing) => {
            let mut new_minute_dict:HashMap<i32, i32> = HashMap::new();
            for (k, v) in existing.minute_dict {
                new_minute_dict.insert(k, v);
            }
            for (k, v) in g.minute_dict {
                let new_count = match new_minute_dict.remove(&k) {
                    Some(s) => s + v,
                    None => v
                };
                new_minute_dict.insert(k, new_count);
            }
            Guard {
                id: g.id,
                total_minutes: existing.total_minutes + g.total_minutes,
                minute_dict: new_minute_dict
            }
        }
    };
    map.insert(to_insert.id.clone(), to_insert);

    // Borrowing guard was causing problems, so I'm just taking ownership and
    // returning ownership until I figure out wtf I'm doing.
    map
}

enum State {
    InbetweenGuards,
    InShift(Guard),
    Sleeping(Guard, Moment)
}

struct Guard {
    id: String,
    total_minutes: i32,
    minute_dict:HashMap<i32, i32>
}

#[derive(Copy, Clone)]
struct Moment {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    min: i32
}

impl Moment {
    fn diff_minutes(&self, other:&Moment) -> i32 {
        self.min - other.min
    }
}

#[derive(Eq)]
struct Line {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    min: i32,
    data: LineType
}

#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
enum LineType {
    FallsAsleep,
    WakesUp,
    BeginsShift(String)
}


impl Line {
    fn parse(raw:&str) -> Line {
        // 012345678910234567890
        // [1518-09-06 00:44] falls asleep
        let year:i32 = raw.chars().skip(1).take(4).collect::<String>().parse().unwrap();
        let month:i32 = raw.chars().skip(6).take(2).collect::<String>().parse().unwrap();
        let day:i32 = raw.chars().skip(9).take(2).collect::<String>().parse().unwrap();
        let hour:i32 = raw.chars().skip(12).take(2).collect::<String>().parse().unwrap();
        let min:i32 = raw.chars().skip(15).take(2).collect::<String>().parse().unwrap();
        let data = raw.chars().skip(19).collect::<String>();
        let line = match data.as_ref() {
            "falls asleep" => LineType::FallsAsleep,
            "wakes up" => LineType::WakesUp,
            other => LineType::BeginsShift(other.to_string())
        };

        Line {
            year,
            month,
            day,
            hour,
            min,
            data: line
        }
    }
}

impl Ord for Line {
    fn cmp(&self, other: &Line) -> Ordering {
        // I'm sure there's a better way to write this.
        // Lot of hoops to jump through to implement sorting...
        match self.year.cmp(&other.year) {
            Ordering::Equal => match self.month.cmp(&other.month) {
                Ordering::Equal => match self.day.cmp(&other.day) {
                    Ordering::Equal => match self.hour.cmp(&other.hour) {
                        Ordering::Equal => self.min.cmp(&other.min),
                        other4 => other4
                    },
                    other3 => other3
                },
                other2 => other2
            },
            other1 => other1
        }
    }
}

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Line) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Line) -> bool {
        self.year == other.year
    }
}

impl Ord for LineType {
    fn cmp(&self, _other: &LineType) -> Ordering {
        Ordering::Equal
    }
}

impl PartialOrd for LineType {
    fn partial_cmp(&self, other: &LineType) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
