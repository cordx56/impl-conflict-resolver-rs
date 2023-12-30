mod checker;
mod common;
mod parser;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let contents = std::fs::read_to_string(args.get(1).unwrap()).unwrap();
    let program = parser::program(&contents).unwrap();
    println!("{:?}", program);
}
