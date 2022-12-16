use std::io::{BufWriter, Write};
use rand::Rng;

pub fn make_sample(stream: &mut dyn Write, split: &str, column: usize, row: usize) {
    let capacity = 8192;
    let mut buf_writer = BufWriter::new(stream);
    let numbers: [char;10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let mut buffer = String::new();
    // number [0-9] + split: &str as byte
    for _ in 0 .. row {
        let mut sum = 0;
        for _ in 0..column - 1 {
            let num = numbers[rand::thread_rng().gen_range(0..=9)];
            sum += num as u8 - b'0';

            // join split to numbers[random_index]
            // e.g. "3,"
            let tmp = String::from(num) + split;

            // check the capacity
            if buffer.len() > capacity {
                // write stream
                match buf_writer.write_all(buffer.as_bytes()) {
                    Ok(_) => {},
                    Err(e) => {
                        panic!("{}", e)
                    }
                }
                buffer = String::new();
            }
            // push to buffer
            buffer = buffer + &tmp;
        }
        let tmp = sum.to_string() + "\n";

        // check the capacity
        if buffer.len() > capacity {
            // write stream
            match buf_writer.write_all(buffer.as_bytes()) {
                Ok(_) => {},
                Err(e) => {
                    panic!("{}", e)
                }
            }
            buffer = String::new();
        }
        // push to buffer
        buffer = buffer + &tmp;
    }
    
    if !buffer.is_empty() {
        match buf_writer.write_all(buffer.as_bytes()) {
            Ok(_) => {},
            Err(e) => {
                panic!("{}", e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::stdout;

    #[test]
    fn make_samplestest() {
        let mut write = stdout().lock();
        make_sample(&mut write, ",", 10, 100000);
    }
}
