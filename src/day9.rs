use std::collections::VecDeque;
use std::collections::HashMap;

pub fn execute() {
    // too lazy to parse the input, deal with it.
    // /*
    let total_players:usize = 427;
    let final_marble_worth = 70723 * 100;
    // */
    /*
    let total_players:usize = 30;
    let final_marble_worth = 5807;
    // */

    let mut marble_circle:VecDeque<i64> = VecDeque::new();
    let mut player_scores:HashMap<usize, i64> = HashMap::new();
    let mut current_marble:i64 = 1;
    let mut current_player:usize = 0;

    // Push the zero marble
    marble_circle.push_front(0);

    let mut cont = true;
    while cont {
        /*
        print!("[{}] ", current_player);
        let mut x = 0;
        for t in &marble_circle {
            if x == current_marble_index {
                print!("({}) ", t);
            } else {
                print!("{} ", t);
            }
            x += 1;
        }
        println!();
        // */

        let is_marble_special = (current_marble % 23) == 0;
        match is_marble_special {
            false => {
                marble_circle = rotate_back(marble_circle, 2);
                marble_circle.push_front(current_marble);
            }
            true => {
                let score = match player_scores.remove(&current_player) {
                    Some(c) => c,
                    None => 0
                };
                marble_circle = rotate_forward(marble_circle, 7);
                let removed_marble = marble_circle.pop_front().expect("valid marble");
                let score = score + removed_marble + current_marble;
                player_scores.insert(current_player, score);
            }
        };
        
        cont = current_marble != final_marble_worth;

        current_marble += 1;
        current_player = mod_add(current_player, 1, total_players);
    }

    let (_, winning_score) = player_scores.iter().max_by_key(|(_, &v)| v).unwrap();

    println!("Winning score: {}", winning_score);
}

fn rotate_back(mut queue:VecDeque<i64>, count:i32) -> VecDeque<i64> {
    for _ in 0..count {
        let value = queue.pop_front().expect("rotate back value");
        queue.push_back(value);
    }
    queue
}

fn rotate_forward(mut queue:VecDeque<i64>, count:i32) -> VecDeque<i64> {
    for _ in 0..count {
        let value = queue.pop_back().expect("rotate forward value");
        queue.push_front(value);
    }
    queue
}

fn mod_add(left:usize, right:usize, max_size:usize) -> usize {
    let new_size = left + right;
    if new_size > max_size {
        new_size - max_size
    } else {
        new_size
    }
}
