use std::fs;

fn main() {
    let content = fs::read_to_string("hello.txt").expect("Can't read file from filesystem");
    println!("{content}");
}
