use super::utils;
use std::fmt;

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
        write!(f, "[{}: {}]",
            &self.name,
            &self.static_type
        )
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Method{
    pub name: String,
    pub parameters: Vec<Parameter>
}
impl Method {
    pub fn new(method_header: &String) -> Self {
        Method {
            name: utils::extract_method_name(method_header),
            parameters: utils::extract_parameters(method_header)
        }
    }
}
impl fmt::Display for Method{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Method :: {} -> {}",
            &self.name,
            &self.parameters[0]
        )
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct PythonClass{
    pub name: String,
    pub methods: Vec<Method>
}
impl PythonClass {
    pub fn new(class_code: Vec<String>, name: String) -> Self {
        PythonClass {
            name: name,
            methods: utils::extract_methods(class_code)
        }
    }
}
impl fmt::Display for PythonClass{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Class :: {}\n\n{}",
            &self.name,
            &self.methods[1]
        )
    }
}