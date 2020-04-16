use std::collections::HashMap;
use std::io::BufRead;
use std::sync::mpsc;
use std::thread;
use std::{env, fs, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let reader = match fs::File::open(file_path) {
        Err(e) => panic!(e),
        Ok(file) => io::BufReader::new(file),
    };
    let (tx, rx) = mpsc::channel();
    let mut count = 0;

    for (i, line) in reader.lines().enumerate() {
        count = count + 1;
        match line {
            Err(e) => panic!(e),
            Ok(line) => match line.parse::<u64>() {
                Err(e) => panic!(e),
                Ok(n) => {
                    let txc = tx.clone();
                    thread::spawn(move || {
                        let p = is_prime(n);
                        txc.send((i, p))
                    });
                }
            },
        }
    }
    let mut line_map = HashMap::new();
    for _ in 0..count {
        let (i, p) = match rx.recv() {
            Err(_e) => break,
            Ok(v) => v,
        };
        line_map.insert(i, p);
    }

    for i in 0..count {
        println!("{}", line_map[&i]);
    }
}
fn is_prime(n: u64) -> u8 {
    if n <= 1 {
        return 0;
    }

    if n == 2 || n == 3 {
        return 1;
    }

    let sqrt = (n as f64).sqrt() as u64;
    for i in 2..sqrt {
        if n % i == 0 {
            return 0;
        }
    }
    return 1;
}
