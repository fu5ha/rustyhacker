use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let l = buf.trim_right().parse::<u32>().unwrap();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let r = buf.trim_right().parse::<u32>().unwrap();
    let mut max = 0;
    for i in l..(r+1) {
        for j in i..(r+1) {
            max = std::cmp::max(i^j, max);
        }
    }
    println!("{}", max);
}
