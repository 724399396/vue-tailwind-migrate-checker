use glob::glob;
use regex::Regex;
use std::env;
use std::{collections::HashSet, fs};

fn main() {
    let re = Regex::new(r#"class="([^"]+)"#).unwrap();
    let tailwind_class_content = fs::read_to_string("tailwind-classes").unwrap();
    let tailwind_classes: HashSet<&str> = tailwind_class_content.split("\n").collect();
    let args: Vec<String> = env::args().collect();
    for entry in glob(&args[1])
        .expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => {
                if path.is_file() {
                    let content = fs::read_to_string(path.to_str().unwrap()).unwrap();
                    for cap in re.captures_iter(&content) {
                        let classes = &cap[1];
                        for class in classes.split(' ') {
                            if tailwind_classes.contains(class) && !class.is_empty() {
                                println!("{} {}", path.to_str().unwrap(), class);
                                break;
                            }
                        }
                    }
                }
            }
            Err(e) => panic!("{:?}", e),
        }
    }
}
