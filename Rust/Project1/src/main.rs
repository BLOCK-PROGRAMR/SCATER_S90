use std::fs;
use std::path::Path;
use std::env;

use regex::Regex;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: solidity-docgen <path-to-solidity-file>");
        return;
    }

    let path = &args[1];
    let content = fs::read_to_string(path).expect("Failed to read file");

    let contract_name = extract_contract_name(&content);
    let functions = extract_functions(&content);

    let mut markdown = format!("# Documentation for `{}`\n\n", contract_name);
    for (sig, desc) in functions {
        markdown.push_str(&format!("## `{}`\n\n{}\n\n---\n\n", sig, desc));
    }

    let out_path = Path::new(path).with_extension("md");
    let mut file = fs::File::create(out_path).expect("Failed to write markdown file");
    file.write_all(markdown.as_bytes()).unwrap();

    println!("📄 Markdown documentation generated.");
}

fn extract_contract_name(content: &str) -> String {
    let re = Regex::new(r"contract\s+(\w+)").unwrap();
    re.captures(content)
        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
        .unwrap_or_else(|| "UnknownContract".to_string())
}

fn extract_functions(content: &str) -> Vec<(String, String)> {
    let re = Regex::new(r"(?s)(?:/\*\*(.*?)\*/)?\s*function\s+(\w+\s*\([^)]*\))").unwrap();
    let mut functions = vec![];

    for cap in re.captures_iter(content) {
        let desc = cap.get(1).map(|m| m.as_str().trim().replace("*", "")).unwrap_or_default();
        let sig = cap.get(2).unwrap().as_str().to_string();
        functions.push((sig, desc));
    }
    functions
}
