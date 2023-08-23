use core::fmt;

use askama::Template;

#[derive(Template)] // this will generate the code...
#[template(path = "recipe.html", escape = "none")] // using the template in this path, relative
pub struct RecipeTemplate<'a> {
    pub name: &'a str,
    pub markdown: &'a str,
}

#[derive(Template)]
#[template(path = "index.html", escape = "none")]
pub struct IndexTemplate {
    pub recipes: Vec<Recipe>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Recipe {
    pub name: String,
    pub path: String,
}

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: {} path: {}", self.name, self.path)
    }
}