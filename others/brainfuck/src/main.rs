extern crate brainfuck;

use brainfuck::interpreter::Executor;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        println!("Usage : {} <bf_file>", args[0]);
        return;
    }

    let path = &args[1];
    let code = match std::fs::read_to_string(path) {
        Ok(x) => x,
        Err(_) => {
            println!("Could not open file : {}", &path);
            return;
        }
    };

    run_bf(code.as_str());
}

fn run_bf(code: &str) {
    let mut reader = std::io::stdin();
    let mut writer = std::io::stdout();
    let mut executor = Executor::new(&mut reader, &mut writer);
    executor.execute(code);
}
