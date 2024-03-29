/*
JonesCLI

Author: Vlad Nedelcu
Date: Jul 2021
License: MIT

Copyright 2021 Vlad Nedelcu
*/

mod commands;
mod joneslib;

use joneslib::display;
use structopt::StructOpt;

fn main() {
    let comms = commands::CLI::from_args();
    if comms.grep {
        // Search for a keyword in class name
        match joneslib::search(&comms.path, &comms.class_name) {
            Some(matches) => display::class_matches(matches),
            None => display::not_found_message(),
        }
    } else {
        // Generate python class
        match joneslib::fetch_object_details(&comms.path, &comms.class_name) {
            Some(class) => joneslib::display::output_class(&class),
            None => display::not_found_message(),
        }
    }
}
