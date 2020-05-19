use roxmltree::*;

use std::fs;
enum MDLSError {
    Error(Error),
    ReadToString(std::io::Error),
}

#[derive(Debug, Clone)]
struct Input {
    position: i32,
    path: String,
}
#[derive(Debug, Clone, Default)]
struct NameSpace {
    path: String,
    names: Vec<String>,
    root: Cue,
    children: Vec<Cue>,
}
#[derive(Debug, Clone, Default)]
struct Cue {
    path: String,
    names: Vec<String>,
}

fn main() {
    let path = "D:/x4_extract_3.2b/md/genericmissions.xml";

    let input = Input {
        position: 3,
        path: path.to_owned(),
    };

    let current = get_current(&input);
}
fn get_current(input: &Input) {
    let mut first_cue = true;
    let mut current_space = Cue::default();
    let mut namespace = NameSpace::default();
    if let Ok(string) = std::fs::read_to_string(input.path.clone()) {
        if let Ok(doc) = roxmltree::Document::parse(&string) {
            for node in doc.root().descendants() {
                if node.tag_name().name() == "cue" {
                    if let Some(n) = node.attribute("name") {
                        if first_cue {
                            current_space.names.clear();
                            current_space.names.push(n.to_owned());
                            namespace.root = current_space.clone();
                            first_cue = false;
                        }
                    }
                    if let Some(n) = node.attribute("namespace") {
                        if n == "this" {
                            // current_space.names.clear();
                            current_space.names.push(n.to_owned());
                            namespace.root = current_space.clone();
                            first_cue = false;
                        }
                    }
                }
                println!("{:#?}", namespace);
            }
        }
    }
}
