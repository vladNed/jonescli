/*
JonesCLI

Author: Vlad Nedelcu
Date: Jul 2021
License: MIT

Copyright 2021 Vlad Nedelcu
*/

use ansi_term::Colour;
use std::fmt;

const SELF_PARAMETER: [&str; 2] = ["self", "cls"];

#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub static_type: String,
}
impl Parameter {
    pub fn new(name: String, static_type: String) -> Self {
        let annotation = if SELF_PARAMETER.contains(&&*name) {
            "Self".to_string()
        } else {
            static_type.clone()
        };

        Parameter {
            name,
            static_type: annotation,
        }
    }
}
impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "  * {}: {}",
            Colour::Purple.paint(&self.name),
            Colour::Green.paint(&self.static_type)
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Method {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub output: String,
}
impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            ":: [{}] -> {}",
            Colour::Yellow.paint(&self.name),
            Colour::Cyan.paint(&self.output)
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct PythonClass {
    pub name: String,
    pub methods: Vec<Method>,
    pub inheritance: Vec<String>,
    pub docstring: String,
}
impl fmt::Display for PythonClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let inheritance_display = self.inheritance.join(", ");
        write!(
            f,
            "# Name [{}]\n--------\n* docstring: {}\n* inherits -> {}\n\n# Methods\n-------",
            Colour::Cyan.paint(&self.name),
            Colour::Yellow.paint(&self.docstring),
            Colour::Green.paint(inheritance_display)
        )
    }
}
