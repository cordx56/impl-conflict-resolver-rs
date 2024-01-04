mod check;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let contents =
        std::fs::read_to_string(args.get(1).expect("Argument required")).expect("File read error");
    let program = check::parser::program(&contents).expect("Parse error").1;
    println!("program: {:?}", program);
    let result = check::checker::Checker::new().check(program).unwrap();
    println!("result: {:?}", result);
}
