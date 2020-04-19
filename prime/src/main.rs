use std::collections::HashMap;
use std::io::BufRead;
use std::{env, fs, io};

fn main() {
    let mut output = String::new();
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let reader = match fs::File::open(file_path) {
        Err(e) => panic!(e),
        Ok(file) => io::BufReader::new(file),
    };
    let mut map: HashMap<u32, char> = HashMap::new();
    for line in reader.lines() {
        match line {
            Err(e) => panic!(e),
            Ok(line) => match line.parse::<u32>() {
                Err(e) => panic!(e),
                Ok(n) => {
                    output.push(is_prime(n, &mut map));
                    output.push('\n');
                }
            },
        }
    }
    print!("{}", output);
}
fn is_prime(n: u32, map: &mut HashMap<u32, char>) -> char {
    if n <= 1 {
        return '0';
    }

    if n == 2 || n == 3 {
        return '1';
    }

    if map.contains_key(&n) {
        return map[&n];
    }

    let mut i = 2;
    while i * i < n {
        if n % i == 0 {
            map.insert(n, '0');
            return '0';
        }
        i += 1;
    }
    map.insert(n, '1');
    return '1';
}
