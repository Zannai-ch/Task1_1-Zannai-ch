use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut kusok = input.split_whitespace();
    let x: i32 = kusok.next().unwrap().parse().unwrap();
    let y: i32 = kusok.next().unwrap().parse().unwrap();
    println!("{}", x + y);
}
