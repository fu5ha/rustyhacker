use std::io;

fn largest_prime_factor(n: u64) -> u64 {
    let mut n = n;
    if n % 2 == 0 {
        while n % 2 == 0 {
            n /= 2;
        }
        if n == 1 {
            return 2;
        }
    }
    let mut sqrtn = (n as f64).sqrt().ceil() as u64;
    let mut i = 3;
    while i <= sqrtn {
        if n % i == 0 {
            n /= i;
            i = 3;
            sqrtn = (n as f64).sqrt().ceil() as u64;
            continue;
        }
        i += 2;
    }
    if n > 2 {
        return n;
    } else {
        return i;
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n_test_cases = buf.trim_right().parse::<usize>().unwrap();
    for _ in 0..n_test_cases {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let n = buf.trim_right().parse::<u64>().unwrap();
    
        println!("{}", largest_prime_factor(n));
    }
}
