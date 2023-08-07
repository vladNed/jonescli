/*
JonesCLI

Author: Vlad Nedelcu
Date: Jul 2021
License: MIT

Copyright 2021 Vlad Nedelcu
*/
use std::{path::PathBuf, process::Command};

use regex::{Regex, RegexBuilder};

use super::{objects::PythonClass, parser::parse_class};

static CLASS_NAME_PATTERN: &str = r"<Class> (\w+)";
static FILE_NAME_PATTERN: &str = r"<File> (.+)";
static METHODS_PATTERN: &str = r"<Methods> (\w+)";
static ATTRIBUTES_PATTERN: &str = r"<Args>\s\[(.*?)\]";
static DOCSTRING_PATTERN: &str = r"<DocString> <#(.*)#>";
static INHERITANCE_PATTERN: &str = r"<Inherit>\s\[(.*?)\]";
static OUTPUT_PATTERN: &str = r"<Output> (\w+)";

/// Loads all objects from a Python project, given through the python project path.
pub fn load_python_project(project_path: &PathBuf) -> Option<Vec<(String, String)>> {
    let class_name_pattern = Regex::new(CLASS_NAME_PATTERN).unwrap();
    let file_name_pattern = Regex::new(FILE_NAME_PATTERN).unwrap();

    let script_output = match run_python_script(&project_path) {
        Some(output) => String::from_utf8(output).unwrap(),
        None => return None,
    };

    let file_pattern_captures = file_name_pattern.captures_iter(&script_output);
    let found_classes = class_name_pattern
        .captures_iter(&script_output)
        .zip(file_pattern_captures)
        .map(|(class_name, file_name)| (class_name[1].to_string(), file_name[1].to_string()))
        .collect::<Vec<(String, String)>>();

    Some(found_classes)
}

pub fn load_python_object(file_path: &PathBuf, class_name: &String) -> Option<PythonClass> {
    let script_output = match run_python_class_script(file_path, class_name) {
        Some(output) => String::from_utf8(output).unwrap(),
        None => return None,
    };

    let methods_pattern = Regex::new(METHODS_PATTERN).unwrap();
    let attributes_pattern = Regex::new(ATTRIBUTES_PATTERN).unwrap();
    let class_pattern = Regex::new(CLASS_NAME_PATTERN).unwrap();
    let docstring_pattern = RegexBuilder::new(DOCSTRING_PATTERN)
        .dot_matches_new_line(true)
        .build()
        .unwrap();
    let inheritance_pattern = Regex::new(INHERITANCE_PATTERN).unwrap();
    let output_pattern = Regex::new(OUTPUT_PATTERN).unwrap();

    let found_parameters = attributes_pattern.captures_iter(&script_output);
    let found_outputs = output_pattern.captures_iter(&script_output);
    let found_methods = methods_pattern
        .captures_iter(&script_output)
        .zip(found_parameters)
        .zip(found_outputs)
        .map(|((method, params), output)| {
            (
                method[1].to_string(),
                params[1].to_string(),
                output[1].to_string(),
            )
        })
        .collect::<Vec<(String, String, String)>>();
    let found_class = class_pattern
        .captures_iter(&script_output)
        .map(|class_name| class_name[1].to_string())
        .collect::<Vec<String>>();
    let found_docstring = docstring_pattern
        .captures_iter(&script_output)
        .map(|docstring| docstring[1].to_string())
        .collect::<Vec<String>>();
    let found_inheritance = inheritance_pattern
        .captures_iter(&script_output)
        .map(|inheritance| inheritance[1].to_string())
        .collect::<Vec<String>>();

    let class_name = found_class[0].clone();
    let docstring = found_docstring[0].clone();
    let inheritance = found_inheritance[0]
        .clone()
        .split(", ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    Some(parse_class(
        class_name,
        found_methods,
        docstring,
        inheritance,
    ))
}

#[inline]
fn run_python_script(project_path: &PathBuf) -> Option<Vec<u8>> {
    let python_script = format!(
        r#"import os
import ast

for root, dirs, files in os.walk({:?}):
    for name in files:
        if name.endswith(".py"):
            with open(os.path.join(root, name), 'r') as file:
                tree = ast.parse(file.read())

            for node in ast.walk(tree):
                if isinstance(node, ast.ClassDef):
                    class_name = node.name
                    print("<Class> %s, <File> %s" % (class_name, os.path.join(root, name)))
"#,
        project_path.as_os_str()
    );

    let output = Command::new("python").arg("-c").arg(python_script).output();

    if let Ok(output) = output {
        Some(output.stdout)
    } else {
        None
    }
}

#[inline]
fn run_python_class_script(file_path: &PathBuf, class_name: &String) -> Option<Vec<u8>> {
    let python_script = format!(
        r#"import ast

with open({:?}, "r") as file:
    tree = ast.parse(file.read())

def get_method(node_method):
    method_name = node_method.name
    method_args = []
    for arg in node_method.args.args:
        arg_name = arg.arg
        if isinstance(arg.annotation, ast.Name):
            arg_type = arg.annotation.id
        elif isinstance(arg.annotation, ast.Subscript):
            arg_type = arg.annotation.value.id
        elif isinstance(arg.annotation, ast.Attribute):
            arg_type = arg.annotation.attr
        else:
            arg_type = None

        method_value = arg_name + "||" + str(arg_type)
        method_args.append(method_value)

    return method_name, method_args

def get_output(node_method):
    if isinstance(node_method.returns, ast.Name):
        return node_method.returns.id
    elif isinstance(node_method.returns, ast.Subscript):
        return node_method.returns.value.id
    elif isinstance(node_method.returns, ast.Attribute):
        return node_method.returns.attr
    else:
        return None

for node in ast.walk(tree):
    if isinstance(node, ast.ClassDef) and node.name == "{}":
        class_name = node.name
        for m in node.body:
            if isinstance(m, ast.FunctionDef) or isinstance(m, ast.AsyncFunctionDef):
                method_name, method_args = get_method(m)
                print("<Methods> %s, <Args> [%s], <Output> %s" % (method_name, ", ".join(method_args), get_output(m)))
        print("<Class> %s" % (class_name))
        print("<DocString> <#%s#>" % (ast.get_docstring(node)))
        print("<Inherit> [%s]" % (', '.join([b.attr if isinstance(b, ast.Attribute) else b.id for b in node.bases] )))
        break
"#,
        file_path.as_os_str(),
        class_name
    );

    let output = Command::new("python").arg("-c").arg(python_script).output();

    if let Ok(output) = output {
        println!("{}", String::from_utf8(output.stderr.clone()).unwrap());
        Some(output.stdout)
    } else {
        None
    }
}
