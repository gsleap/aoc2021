use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let filename = "src/day01/input.txt";

    let data = read_iter(filename);

    let mut increases = 0;

    for i in 3..data.len() {
        if data[i] + data[i - 1] + data[i - 2] > data[i - 1] + data[i - 2] + data[i - 3] {
            increases += 1;
        }
    }

    println!("{} increases\n", increases);
}

// Read file into vector
fn read_iter(filename: &str) -> Vec<i32> {
    let file = File::open(filename).expect("file not found!");
    let reader = BufReader::new(file);
    let mut vec: Vec<i32> = vec![];

    for line in reader.lines() {
        let val: i32 = line.expect("Error").parse().expect("error parsing");
        vec.push(val);
    }

    vec
}
