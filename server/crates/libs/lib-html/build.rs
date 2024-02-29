use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;

pub fn main() {
    if Ok("release".to_owned()) != env::var("PROFILE") {
        return;
    }

    let templates_dir = "templates";

    for entry in fs::read_dir(templates_dir).unwrap() {
        let entry = entry.unwrap();
        let template_path = entry.path();

        if template_path.extension().map_or(false, |ext| ext == "html") {
            process_html_template(&template_path);
        }
    }
}

fn process_html_template(input_path: &Path) {
    let template_content = fs::read_to_string(input_path).unwrap();
    let re = Regex::new(r"<!--([\s\S]*?)-->").unwrap();
    let processed_content = re.replace_all(&template_content, "").to_string();

    // Write processed file to templates
    fs::write(input_path, processed_content).unwrap();
}
