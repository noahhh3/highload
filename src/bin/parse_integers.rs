use std::io::{self, BufRead};
use std::os::unix::io::FromRawFd;
fn parse(bytes: &[u8]) -> u32 {
    let mut num = 0;
    // for ijj
    //     num = num * 10 + (*i - b'0') as u32;
    // }
    let mut num1 = 0;
    let mut num2 = 0;
    let mut num3 = 0;
    let num1_vals = [0,0,1_000_000_000,100_000_000];
    let num2_vals = [100000000,1000000,100000,10000];
    let num3_vals = [1000,100,10,1];
    let mut j = 0; 
    for i in (3..16).step_by(4) {
        println!("{} {}", i, bytes[i]);
        num1 = num1 * num1_vals[j] + (bytes[i] - b'0') as u32;
        num2 = num2 * num2_vals[j] + (bytes[i] - b'0') as u32;
        num3 = num3 * num3_vals[j] + (bytes[i] - b'0') as u32;

        j+=1;
    }
    println!("{} {} {}", num1, num2, num3);
    return num1 + num2 + num3;
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
        break;
    }

    println!("sum: {}", sum);
}