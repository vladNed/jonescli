/*
JonesCLI

Author: Vlad Nedelcu
Date: Jul 2021
License: MIT

Copyright 2021 Vlad Nedelcu
*/
use super::objects::{Parameter, Method, PythonClass};

static PARAMETER_SEPARATOR: &str = "||";

pub fn parse_method_parameter(parameters: Vec<String>) -> Vec<Parameter>{
    let mut parsed_parameters = Vec::new();
    for parameter in parameters.iter() {
        let values = parameter.split(PARAMETER_SEPARATOR).collect::<Vec<&str>>();
        let (param_name, param_annotation) = (values[0], values[1]);
        parsed_parameters.push(Parameter::new(param_name.to_string(), param_annotation.to_string()));
    }

    parsed_parameters
}

pub fn parse_method(methods_data: Vec<(String, String, String)>) -> Vec<Method> {
    let mut parsed_methods = Vec::new();
    for method in methods_data.iter() {
        let (method_name, method_parameters, method_output) = (method.0.clone(), method.1.clone(), method.2.clone());
        let raw_parameters = method_parameters.split(",").map(|x| x.to_string()).collect::<Vec<String>>();
        let parameters = parse_method_parameter(raw_parameters);
        parsed_methods.push(
            Method {
                name: method_name.to_string(),
                parameters,
                output: method_output
            }
        );
    }

    parsed_methods
}

pub fn parse_class(
    name: String,
    methods: Vec<(String, String, String)>,
    docstring: String,
    inheritance: Vec<String>,
) -> PythonClass {
    let methods = parse_method(methods);
    PythonClass {
        name,
        methods,
        docstring,
        inheritance
    }
}