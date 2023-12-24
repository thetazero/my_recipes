use std::{fs, path::PathBuf}; // bring trait in scope

pub mod templates;
use templates::{IndexJsonTemplate, IndexTemplate, Link, RecipeTemplate};

pub mod units;
use units::recipe_markdown_to_html;
fn main() {
    if !fs::metadata("./built").is_ok() {
        fs::create_dir("./built").expect("Failed to create built directory");
    }

    render_all("./recipes");

    copy_folder("./assets/", "./built/");
    if !fs::metadata("./built/images").is_ok() {
        fs::create_dir("./built/images").expect("Failed to create images directory");
    }
    copy_folder("./images/", "./built/images/");
}

fn render_all(path: &str) {
    let paths = fs::read_dir(path).unwrap();
    let mut recipe_list: Vec<Link> = Vec::new();
    for path in paths {
        let path = path.unwrap().path();
        if !is_valid_recipe_path(&path) {
            continue;
        }
        let recipe_link = make_link(&path);

        let rendered_page = compile_recipe(&recipe_link, &path);
        fs::write("./built/".to_owned() + &recipe_link.path, &rendered_page).unwrap();

        recipe_list.push(recipe_link);
    }
    recipe_list.sort();

    let index_html = compile_index(&recipe_list);
    fs::write("./built/index.html", &index_html).unwrap();

    let index_json_html = compile_index_json(&recipe_list);
    fs::write("./built/index.js", &index_json_html).unwrap();
}

fn is_valid_recipe_path(path: &PathBuf) -> bool {
    let file_name = path.file_name().unwrap().to_str().unwrap();
    file_name.ends_with(".md")
}

fn compile_index_json(recipe_list: &Vec<Link>) -> String {
    IndexJsonTemplate {
        recipes: recipe_list,
    }
    .to_string()
}

fn compile_index(recipe_list: &Vec<Link>) -> String {
    IndexTemplate {
        recipes: recipe_list,
    }
    .to_string()
}

fn make_link(path: &PathBuf) -> Link {
    let recipe_name = path.file_name().unwrap().to_str().unwrap();
    let nice_name = procees_title_string(recipe_name);

    Link {
        name: nice_name,
        path: recipe_name.to_owned() + ".html",
    }
}

fn compile_recipe(link: &Link, source: &PathBuf) -> String {
    let content = fs::read_to_string(source).unwrap();
    let markdown = recipe_markdown_to_html(content);

    RecipeTemplate {
        name: &link.name,
        markdown: &markdown,
    }
    .to_string()
}

fn copy_folder(from: &str, to: &str) {
    let paths = fs::read_dir(from).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        fs::copy(&path, to.to_owned() + file_name).expect("Failed to copy asset");
    }
}

fn procees_title_string(title: &str) -> String {
    let mut result = String::new();
    result.push_str(&title[0..1].to_uppercase());
    for c in title[1..].chars() {
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
