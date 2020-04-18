use std::collections::HashMap;
use std::io::BufRead;
use std::{env, fs, io};

fn main() {
    let mut sieve: HashMap<i64, bool> = HashMap::new();
    let mut limit_cache = 1;

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
                Ok(n) => {
                    sieve_of_atkin(n, &mut sieve, &mut limit_cache);
                    println!(
                        "{}",
                        if sieve.contains_key(&n) && sieve[&n] {
                            1
                        } else {
                            0
                        }
                    )
                }
            },
        }
    }
}

fn sieve_of_atkin(limit: i64, sieve: &mut HashMap<i64, bool>, limit_cache: &mut i64) {
    if limit < *limit_cache {
        return;
    }

    let mut x = (*limit_cache as f64).sqrt() as i64;
    while x * x < limit {
        let mut y = 1;
        while y * y < limit {
            let n = (4 * x * x) + (y * y);
            if n <= limit && (n % 12 == 1 || n % 12 == 5) {
                sieve.insert(n, true);
            }
            let n = (3 * x * x) + (y * y);
            if n <= limit && n % 12 == 7 {
                sieve.insert(n, true);
            }
            let n = (3 * x * x) - (y * y);
            if x > y && n <= limit && n % 12 == 11 {
                sieve.insert(n, true);
            }
            y += 1;
        }
        x += 1;
    }
    let mut r = 5;
    while r * r < limit {
        if sieve.contains_key(&r) && sieve[&r] {
            for i in (r * r..limit).step_by((r * r) as usize) {
                sieve.insert(i, false);
            }
        }
        r += 1;
    }
    *limit_cache = limit;
}
