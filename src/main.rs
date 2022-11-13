// read file line by line

use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::os::unix::prelude::FileExt;
use std::path::Path;
use std::time::Instant;

mod sanitize;
use sanitize::to_md;

use crate::sanitize::text_from;

fn main() -> std::io::Result<()> {
    let base_path = "/home/tronfy/source/wikirun/data/";
    let dump = "ptwiki-articles.xml";
    let file = File::open(base_path.to_owned() + dump).unwrap();
    let reader = BufReader::new(file);

    // fs::create_dir_all("xml")?;
    fs::create_dir_all(base_path.to_owned() + "wiki")?;

    let mut page = String::new();
    let mut title = String::new();
    let mut start = Instant::now();

    for line in reader.lines() {
        let line = line? + "\n";
        // println!("{}", line;

        if line.trim().starts_with("<page>") {
            page = String::new();
            title = String::new();
            start = Instant::now();
        } else if line.trim().starts_with("<title>") {
            title = line
                .replace("<title>", "")
                .replace("</title>", "")
                .replace("/", "-")
                .trim()
                .to_string();

            let v = title.chars().collect::<Vec<_>>(); // I □ unicode
            if v.len() > 100 {
                title = v[..100].iter().cloned().collect::<String>();
            }
        } else if line.trim().starts_with("</page>") {
            print!("{} ", title);

            let f = format!("{}wiki/{}.md", base_path.to_owned(), title);

            // if file exists, skip
            if Path::new(&f).exists() {
                println!("(skipped, file exists)");
            } else if title.contains(":") {
                // todo?
                println!("(skipped, contains ':')");
            } else {
                let file = File::create(f)?;
                file.write_all_at(to_md(text_from(&page), &title).as_bytes(), 0)?;

                println!("{:.2?}", start.elapsed());
            }
        }

        page += &line;
    }

    Ok(())
}
