use std::env;
use std::process;
use ansi_term::Colour;

mod joneslib;
mod commands;

fn main() {

    let args: Vec<String> = env::args().collect();
    let coms = commands::Config::new(&args).unwrap_or_else(|err| {
        println!(
            "{}: {}",
            Colour::Red.paint("ARGS ERROR"),
            Colour::Yellow.paint(err)
        );
        process::exit(1);
    });

    // Generate python class
    let class = joneslib::project_traversal(&"./testdir".to_string(), &coms.class_name);
    match class {
        Some(class) => joneslib::display::output_class(&class),
        None => println!(
            "{}: {}",
            Colour::Green.paint("Output"),
            Colour::Yellow.paint("Searched class was not found")
        )
    }

}

