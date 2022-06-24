use std::io::{self, BufRead};

fn main() {
    for result in filepicker::find_files(unwrapped_stdio_lines()) {
        println!("{}", result);
    }
}

fn unwrapped_stdio_lines() -> Vec<String> {
    let stdin = io::stdin();
    let mut results = Vec::new();

    for line in stdin.lock().lines() {
        results.push(line.unwrap())
    }

    results
}
