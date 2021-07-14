/*
JonesCLI

Author: Vlad Nedelcu
Date: Jul 2021
License: MIT

Copyright 2021 Vlad Nedelcu
*/
use super::objects;
use super::ClassMatch;
use ansi_term::Colour;

pub fn output_class(python_class: &objects::PythonClass) {
    println!("{}", python_class);

    for method in python_class.methods.iter() {
        println!("{}", method);
        for parameter in method.parameters.iter() {
            println!("{}", parameter);
        }
    }
}

pub fn not_found_message() -> () {
    println!(
        "{}: {}",
        Colour::Green.paint("Output"),
        Colour::Yellow.paint("Searched class was not found in project")
    )
}

pub fn class_matches(found_match_classes: Vec<ClassMatch>) -> () {
    println!("> [{}]", Colour::Cyan.paint("FOUND MATCHES"));
    for line in found_match_classes.iter() {
        println!(
            ":: {} -> {}",
            Colour::Yellow.paint(&line.0.replace("\r", "")),
            Colour::Purple.paint(&line.1)
        )
    }
}