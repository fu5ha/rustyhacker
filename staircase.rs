use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let size = buf.trim_right().parse::<usize>().unwrap();
    
    for i in 1..size+1 {
        buf.clear();
        for _ in 0..(size - i){
            buf = format!("{} ", buf);
        }
        for _ in 0..i{
            buf = format!("{}#", buf);
        }
        println!("{}",buf);
    }
}
