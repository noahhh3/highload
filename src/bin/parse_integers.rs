use std::io::{self, BufRead};
use std::os::unix::io::FromRawFd;
fn parse(bytes: &[u8]) -> u32 {
    let mut num = 0;
    for i in bytes.iter() {
        num = num * 10 + (*i - b'0') as u32;
    }
    return num;
}

fn main() {
    let stdin = unsafe { std::fs::File::from_raw_fd(0) };
    let mut reader = io::BufReader::new(stdin);
    let mut sum: u64 = 0;
    let mut buffer = vec![b'0';10];

    while reader.read_until(b'\n', &mut buffer).unwrap() > 0 {
        let last = buffer.len() - 1;
        let num = parse(&buffer[..last]);
        sum += num as u64;
        buffer.clear();
    }

    println!("{}", sum);
}