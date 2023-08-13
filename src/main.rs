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
    compile_recipes("./recipes");
    copy_assets();
}

fn compile_recipes(path: &str) {
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let content = fs::read_to_string(&path).unwrap();
        let markdown = markdown_to_html(&content, &ComrakOptions::default());

        let recipe_name = path.file_name().unwrap().to_str().unwrap();
        let nice_name = procees_title_string(recipe_name);

        let rendered_page = RecipeTemplate {
            name: &nice_name,
            markdown: &markdown,
        }
        .to_string();
        fs::write(
            "./built/".to_owned() + recipe_name + ".html",
            &rendered_page,
        )
        .unwrap();
    }
}

fn copy_assets() {
    let paths = fs::read_dir("./assets").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        fs::copy(&path, "./built/".to_owned() + file_name).expect("Failed to copy asset");
    }
}

fn procees_title_string(title: &str) -> String {
    let mut result = String::new();
    result.push_str(&title[0..1].to_uppercase());
    for c in title[1..].chars(){
        match c {
            '_' => {
                result.push(' ');
            }
            '.' => {
                break;
            }
            _ => {
                result.push(c);
            }
        }
    }
    result
}
