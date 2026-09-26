use std::io;
use std::io::Read;
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut kusok = input.split_whitespace();
    let x: i64 = kusok.next().unwrap().parse().unwrap();
    let y: i64 = kusok.next().unwrap().parse().unwrap();
    println!("{}", x + y);
}
