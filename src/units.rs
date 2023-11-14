use comrak::nodes::{AstNode, NodeValue};
use comrak::{format_html, parse_document, Arena, ComrakOptions};

fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
where
    F: Fn(&'a AstNode<'a>),
{
    f(node);
    for c in node.children() {
        iter_nodes(c, f);
    }
}

pub fn recipe_markdown_to_html(source: String) -> String {
    let arena = Arena::new();
    let root = parse_document(&arena, &source, &ComrakOptions::default());

    iter_nodes(root, &|node| match &mut node.data.borrow_mut().value {
        &mut NodeValue::Code(ref mut nodecode) => {
            nodecode.literal = units_to_standard_form(&nodecode.literal);
        }
        _ => (),
    });

    let mut result = vec![];
    format_html(root, &ComrakOptions::default(), &mut result).unwrap();

    return String::from_utf8(result).unwrap();
}

fn units_to_standard_form(unit_string: &String) -> String {
    return unit_string.clone();
}
