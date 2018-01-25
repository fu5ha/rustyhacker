use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n_test_cases: i64 = buf.trim_right().parse().unwrap();
    for _ in 0..n_test_cases {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let n = buf.trim_right().parse().unwrap();
        println!("{}", solve(n));
    }
}

fn solve(n: i64) -> i64 {
    let n3 = (n - 1) / 3;
    let n5 = (n - 1) / 5;
    let n15 = (n - 1) / 15;
    let sn3 = 3 * n3 * (n3 + 1) / 2;
    let sn5 = 5 * n5 * (n5 + 1) / 2;
    let sn15 = 15 * n15 * (n15 + 1) / 2;
    sn3 + sn5 - sn15
}
