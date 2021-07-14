use super::utils;
use std::fmt;
use ansi_term::Colour;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Parameter{
    pub name: String,
    pub static_type: String
}
impl Parameter {
    pub fn new(name: String, static_type: String) -> Self{
        Parameter {
            name,
            static_type
        }
    }

}
impl fmt::Display for Parameter{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "  * {}: {}",
            Colour::Purple.paint(&self.name),
            Colour::Green.paint(&self.static_type)
        )
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Method{
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub output: String
}
impl Method {
    pub fn new(method_header: &String) -> Self {
        let method_name = match utils::extract_method_name(method_header) {
            Ok(name) => name,
            Err(err) => {
                println!("Error while extracting method name: {}", err);
                String::from("ENL")
            }
        };
        let method_output = match utils::extract_method_output(method_header) {
            Ok(output) => output,
            Err(_) => String::from("None")
        };
        Method {
            name: method_name,
            output: method_output,
            parameters: utils::extract_parameters(method_header)
        }
    }
}
impl fmt::Display for Method{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, ":: [{}] -> {}",
            Colour::Yellow.paint(&self.name),
            Colour::Cyan.paint(&self.output)
        )
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct PythonClass{
    pub name: String,
    pub methods: Vec<Method>,
    pub inheritance: Vec<String>
}
impl PythonClass {
    pub fn new(class_code: Vec<String>, name: String, inheritance: Vec<String>) -> Self {
        PythonClass {
            name: name,
            methods: utils::extract_methods(class_code),
            inheritance
        }
    }
}
impl fmt::Display for PythonClass{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut inheritance_display = String::from("");
        inheritance_display.push_str("[");
        for object in self.inheritance.iter() {
            inheritance_display.push_str(object);
            inheritance_display.push_str(", ");
        }
        inheritance_display.push_str("]");
        write!(f, "# Class :: [{}]\n* inherit -> {}\n\n# Methods\n-------",
            Colour::Cyan.paint(&self.name),
            Colour::Green.paint(inheritance_display)
        )
    }
}