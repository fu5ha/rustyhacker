use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n_test_cases = buf.trim_right().parse::<usize>().unwrap();
    for _ in 0..n_test_cases {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let n = buf.trim_right().parse::<u64>().unwrap();
        let result = find_palindrome(n);
        println!("{}", result);
    }
}

fn find_palindrome(lt: u64) -> u64 {
    let mut largest = 0;
    
    for i in (100..std::cmp::min(lt/100, 999)).rev() {
        for j in (100..std::cmp::min(lt/100, 999)).rev() {
            let product = i*j;
            if product.check_palindrome() && product > largest && product < lt {
                largest = product;
            }
        }
    }
    largest
}

trait Reversible {
    fn reverse(&self) -> Self;
}

impl Reversible for u64 {
    fn reverse(&self) -> u64 {
        let mut m = 0;
        let mut n = *self;
        while n > 0 {
            let d = n % 10;
            m = m * 10 + d;
            n /= 10;
        }
        m
    }
}

trait PossiblePalindrome {
    fn check_palindrome(&self) -> bool;
}

impl PossiblePalindrome for u64 {
    fn check_palindrome(&self) -> bool {
        *self == self.reverse()
    }
}
