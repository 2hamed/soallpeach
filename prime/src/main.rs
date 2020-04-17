use std::collections::HashMap;
use std::io::BufRead;
use std::{env, fs, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let reader = match fs::File::open(file_path) {
        Err(e) => panic!(e),
        Ok(file) => io::BufReader::new(file),
    };
    let mut map: HashMap<u64, u8> = HashMap::new();
    for line in reader.lines() {
        match line {
            Err(e) => panic!(e),
            Ok(line) => match line.parse::<u64>() {
                Err(e) => panic!(e),
                Ok(n) => println!("{}", is_prime(n, &mut map)),
            },
        }
    }
}
fn is_prime(n: u64, map: &mut HashMap<u64, u8>) -> u8 {
    if n <= 1 {
        return 0;
    }

    if n == 2 || n == 3 {
        return 1;
    }

    if map.contains_key(&n) {
        return map[&n];
    }

    let sqrt = (n as f64).sqrt() as u64;
    for i in 2..sqrt {
        if n % i == 0 {
            map.insert(n, 0);
            return 0;
        }
    }
    map.insert(n, 1);
    return 1;
}
