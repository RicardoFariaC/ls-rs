use echo_reimpl::ls::{self, DirArg};
use std::process;

fn main() {
    let arg: DirArg = DirArg::new();
    if let Err(ref e) = ls::execute(&arg.path) {
        println!("{}", e);
        process::exit(1);
    }
}
