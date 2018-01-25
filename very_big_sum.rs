use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let sum = buf.split(' ')
        .map(|ns| ns.parse::<u64>().unwrap())
        .fold(0, |acc, n| acc + n);
    println!("{}", sum);
}
