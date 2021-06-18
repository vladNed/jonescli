use std::fs;
mod finder;

fn main() {

    // TODO: This will become argumens
    let class_name: &str = "Humans";
    let file_name: &str = "test.py";

    // Read the file
    let content = fs::read_to_string(file_name).expect("Something went wrong");
    let lines: Vec<&str> = content.split("\n").collect();

    // Generate python class
    let class = finder::extractors::extract_python_class(lines, class_name);
    finder::objects::PythonClass::new(class, "Humans".to_string());

    finder::test();

}