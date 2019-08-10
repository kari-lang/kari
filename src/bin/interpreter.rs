use std::{
    fs::File,
    io::stdin,
    process::exit,
};

use acc_reader::AccReader;
use clap::{
    App,
    Arg,
};

use kari::interpreter::evaluator::Evaluator;


fn main() {
    let args = App::new("Kari")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Interpreter for the Kari prorgamming language")
        .arg(
            Arg::with_name("path")
                .value_name("PATH")
                .index(1)
                .help("The program to execute, without the \".kr\" extension.")
        )
        .get_matches();

    match args.value_of("path") {
        Some(name) => {
            let path = format!("kr/examples/{}.kr", name);

            let file = match File::open(&path) {
                Ok(file) => {
                    file
                }
                Err(error) => {
                    print!(
                        "\nERROR: Failed to open file {} ({})\n\n",
                        path,
                        error,
                    );
                    exit(1);
                }
            };

            Evaluator::run(path.into(), Box::new(file));
        }
        None => {
            Evaluator::run(
                "<stdin>".into(),
                Box::new(AccReader::new(stdin())),
            );
        }
    }
}
