use std::io::BufRead;
use std::{env, fs, io};
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    match fs::File::open(file_path) {
        Err(e) => panic!(e),
        Ok(file) => {
            let f = io::BufReader::new(file);
            for line in f.lines() {
                match line {
                    Err(e) => panic!(e),
                    Ok(line) => match line.parse::<u64>() {
                        Err(e) => panic!(e),
                        Ok(n) => println!("{}", is_prime(n)),
                    },
                }
            }
        }
    }
}
fn is_prime(n: u64) -> u8 {
    let sqrt = (n as f64).sqrt() as u64;
    for i in 2..sqrt {
        if n % i == 0 {
            return 0;
        }
    }
    return 1;
}
