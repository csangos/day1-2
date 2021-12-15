use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn load_from_file(file_path: &str) -> Vec<i32> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let numbers: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();

    numbers
}

fn sonar_sweep(numbers: Vec<i32>) {
    let mut total = 0;
    let max_loops = numbers.len() - 3;
    for n in 0..max_loops {
        let sum1 = numbers[n + 0] + numbers[n + 1] + numbers[n + 2];
        let sum2 = numbers[n + 1 + 0] + numbers[n + 1 + 1] + numbers[n + 1 + 2];
        if sum2 > sum1 {
            total += 1;
        }
    }
    println!("Total: {}", total);
}

fn main() {
    let numbers = load_from_file("src/depths.txt");
    sonar_sweep(numbers); // 1-2: sum three depths and comapre to a sliding three depth sum, print total
}
