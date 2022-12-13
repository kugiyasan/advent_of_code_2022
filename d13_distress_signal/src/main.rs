use std::fs;

fn main() {
    // let path = "input";
    let path = "example";
    let buf = fs::read_to_string(path).unwrap();

    let input: Vec<_> = buf.trim().split("\n\n").collect();

    println!("{:?}", &input[0..5]);
}
