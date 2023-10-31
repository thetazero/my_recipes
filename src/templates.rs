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
pub struct IndexTemplate<'a> {
    pub recipes: &'a Vec<Link>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Link {
    pub name: String,
    pub path: String,
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: {} path: {}", self.name, self.path)
    }
}

#[derive(Template)]
#[template(path = "navbar.html", escape = "none")]
pub struct NavbarTemplate {}

#[derive(Template)]
#[template(path = "index.js", escape = "none")]
pub struct IndexJsonTemplate<'a> {
    pub recipes: &'a Vec<Link>,
}