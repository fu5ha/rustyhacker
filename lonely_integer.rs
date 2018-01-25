use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let unique = buf.split(' ')
        .map(|s| {
            s.parse::<i8>().unwrap()
        })
        .fold(0, |acc, n| {
            n ^ acc
        });
    println!("{}", unique);
}
