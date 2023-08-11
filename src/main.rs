use askama::Template;
use comrak::{markdown_to_html, ComrakOptions};
use std::fs; // bring trait in scope

#[derive(Template)] // this will generate the code...
#[template(path = "recipe.html", escape = "none")] // using the template in this path, relative
struct RecipeTemplate<'a> {
    name: &'a str,
    markdown: &'a str,
}
fn main() {
    println!("Hello, world!");

    compile_recipes("./recipes");
}

fn compile_recipes(path: &str) {
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let content = fs::read_to_string(&path).unwrap();
        let markdown = markdown_to_html(&content, &ComrakOptions::default());

        let recipe_name = path.file_name().unwrap().to_str().unwrap();

        let rendered_page = RecipeTemplate {
            name: recipe_name,
            markdown: &markdown,
        }.to_string();
        fs::write("./built/".to_owned() + recipe_name + ".html", &rendered_page).unwrap();
    }
}
