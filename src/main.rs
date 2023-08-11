use std::fs;
use comrak::{markdown_to_html, ComrakOptions};
fn main() {
    println!("Hello, world!");

    compile_recipes("./recipes");
}

fn compile_recipes(path: &str) {
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        let content = fs::read_to_string(path.unwrap().path()).unwrap();
        let markdown = markdown_to_html(&content, &ComrakOptions::default());
        println!("{markdown}")
    }
}
