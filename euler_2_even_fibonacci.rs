use std::io;

struct Fibonacci {
    prev: u64,
    cur: u64,
    max: u64
}

impl Fibonacci {
    pub fn new(max: u64) -> Self {
        Fibonacci {
            prev: 0,
            cur: 1,
            max
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;
    
    fn next(&mut self) -> Option<u64> {
        let prev = self.prev;
        let cur = self.cur;
        self.cur = prev + cur;
        self.prev = cur;
        if self.cur < self.max {
            Some(self.cur)  
        } else {
            None
        }
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n_test_cases: i64 = buf.trim_right().parse().unwrap();
    for _ in 0..n_test_cases {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let max = buf.trim_right().parse::<u64>().unwrap();
        let fib = Fibonacci::new(max);
        let mut sum = 0;
        for n in fib {
            if n%2 == 0 {
                sum += n;
            }
        }
        println!("{}", sum);
    }
}
