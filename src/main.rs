use ansi_term::Colour;
use structopt::StructOpt;


mod joneslib;
mod commands;

fn main() {
    let comms = commands::CLI::from_args();

    // Generate python class
    let class = joneslib::project_traversal(&comms.dir_path, &comms.class_name);
    match class {
        Some(class) => joneslib::display::output_class(&class),
        None => println!(
            "{}: {}",
            Colour::Green.paint("Output"),
            Colour::Yellow.paint("Searched class was not found in project")
        )
    }


}

