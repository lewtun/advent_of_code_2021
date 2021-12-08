use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut last_seen_depth = 0;
    // Start counter at -1 since we don't compare the first depth
    let mut _number_of_increases = -1;

    if let Ok(lines) = read_lines("data/sonar_sweep.txt") {
        for line in lines {
            if let Ok(line) = line {
                let depth = line.parse::<i32>().unwrap();
                if depth > last_seen_depth {
                    _number_of_increases += 1;
                }
                last_seen_depth = depth;
            }
        }
    }

    println!("Number of depth increases: {}", _number_of_increases);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
