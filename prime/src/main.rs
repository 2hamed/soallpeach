use std::io::BufRead;
use std::{env, fs, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let reader = match fs::File::open(file_path) {
        Err(e) => panic!(e),
        Ok(file) => io::BufReader::new(file),
    };
    for line in reader.lines() {
        match line {
            Err(e) => panic!(e),
            Ok(line) => match line.parse::<i64>() {
                Err(e) => panic!(e),
                Ok(n) => println!(
                    "{}",
                    if *sieve_of_atkin(n).last().unwrap() {
                        1
                    } else {
                        0
                    }
                ),
            },
        }
    }
}

fn sieve_of_atkin(limit: i64) -> Vec<bool> {
    let mut sieve: Vec<bool> = (0..(limit + 1))
        .map(|i| {
            if i == 2 || i == 3 || i == 5 {
                return true;
            }
            return false;
        })
        .collect();

    let mut x = 1;
    while x * x < limit {
        let mut y = 1;
        while y * y < limit {
            let n = (4 * x * x) + (y * y);
            if n <= limit && (n % 12 == 1 || n % 12 == 5) {
                sieve[n as usize] ^= true;
            }
            let n = (3 * x * x) + (y * y);
            if n <= limit && n % 12 == 7 {
                sieve[n as usize] ^= true;
            }
            let n = (3 * x * x) - (y * y);
            if x > y && n <= limit && n % 12 == 11 {
                sieve[n as usize] ^= true;
            }
            y += 1;
        }
        x += 1;
    }
    let mut r = 5;
    while r * r < limit {
        if sieve[r as usize] {
            for i in (r * r..limit).step_by((r * r) as usize) {
                sieve[i as usize] = false;
            }
        }
        r += 1;
    }
    sieve
}
