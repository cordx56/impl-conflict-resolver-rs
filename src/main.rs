mod check;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let contents =
        std::fs::read_to_string(args.get(1).expect("Argument required")).expect("File read error");
    let program = check::parser::program(&contents).expect("Parse error").1;
    // println!("program: {:?}", program);
    let results = check::checker::Checker::new().check(program).unwrap();
    for result in results {
        println!("{:?}:\n\t{}\n\t{}", result.0, result.1, result.2);
    }
}
