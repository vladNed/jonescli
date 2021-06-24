mod joneslib;

fn main() {

    // TODO: This will become argumens
    let class_name = String::from("Human");

    // Generate python class
    let class = joneslib::project_traversal(&"./testdir".to_string(), &class_name);
    match class {
        Some(class) => joneslib::display::output_class(&class),
        None => println!("Found none")
    }

}

