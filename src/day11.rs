
const MAX_X:usize = 299;
const MAX_Y:usize = 299;

pub fn execute() {

    let mut scores:Vec<i32> = vec![0; MAX_X * MAX_Y];
    let serial_numer = 5034;

    // Calculate all scores
    for x in 0..MAX_X {
        for  y in 0..MAX_Y {
            let rack_id = (x as i32) + 11;
            let power_level = rack_id * ((y + 1) as i32);
            let with_serial = power_level + serial_numer;
            let with_rack_id = with_serial * rack_id;
            let hundreds_digit = if with_rack_id > 100 {
                let t = with_rack_id / 100;
                t % 10
            } else {
                0
            };
            // println!("{} {}", with_rack_id, hundreds_digit );
            let power_level = hundreds_digit - 5;
            let pos = MAX_X * y + x;
            scores[pos] = power_level;
        }
    }

    // Calculate best grid
    // Being lazy here and not using Option<T>
    let mut best_score = i32::min_value();
    let mut best_grid = (0, 0);
    let mut best_size:usize = 0;

    for size in 3..30 {
        for x in 0..(MAX_X - size) {
            for y in 0..(MAX_Y - size) {
                let this_score = calculate_grid_score(&scores, x, y, size);
                if this_score > best_score {
                    best_score = this_score;
                    best_grid = (x, y);
                    best_size = size;
                }
            }
        }

        println!("Size: {}, Score: {}", size, best_score);
    }

    println!("Best Score: {}", best_score);
    let (best_x, best_y) = best_grid;
    println!("Best Pos: {} {} {}", best_x + 1, best_y + 1, best_size);
}

fn calculate_grid_score(scores:&Vec<i32>, x:usize, y:usize, size:usize) -> i32 {
    let mut score = 0;

    for sub_x in x..(x + size) {
        for sub_y in y..(y + size) {
            let pos = MAX_X * sub_y + sub_x;
            score += scores[pos];
        }
    }

    score
}

