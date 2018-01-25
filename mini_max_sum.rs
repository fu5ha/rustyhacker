use std::io;

struct Sum {
    min: u64,
    max: u64,
    sum: u64
}

impl Sum {
    pub fn new() -> Self {
        Sum {
            min: u64::max_value(),
            max: 0,
            sum: 0
        }
    }
    
    pub fn add(&mut self, n: u64) {
        self.sum += n;
        self.max = std::cmp::max(self.max, n);
        self.min = std::cmp::min(self.min, n);
    }
    
    pub fn find_smallest_largest(&self) -> (u64, u64) {
        (self.sum-self.max, self.sum-self.min)
    }
}

fn main() {
    let mut sum = Sum::new();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    for ns in buf.split(' ') {
        let n = ns.parse::<u64>().unwrap();
        sum.add(n);
    }
    let (smallest, largest) = sum.find_smallest_largest();
    println!("{} {}", smallest, largest);
}
