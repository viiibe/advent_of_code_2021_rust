use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut depths: Vec<u16> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(depth) = line {
                let d = depth.trim().parse::<u16>().expect("Error");
                depths.push(d);
            }
        }
    }

    let mut measure = depths[1];
    let mut depth_times: u16 = 0;

    for depth in depths {
        if depth > measure {
            depth_times += 1;
        }
        measure = depth;
    }

    println!{"Number of times depth measurement increases = {}", depth_times};
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
