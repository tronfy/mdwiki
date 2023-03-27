use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::os::unix::prelude::FileExt;
use std::path::Path;

mod sanitize;
use sanitize::to_md;

use crate::sanitize::text_from;

fn main() -> std::io::Result<()> {
    let base_path = "../data/out/";
    let dump = "../raw/ptwiki-20230320-pages-articles.xml";
    let file = File::open(base_path.to_owned() + dump).unwrap();

    fs::create_dir_all(base_path.to_owned() + "wiki")?;

    let mut page = String::new();
    let mut title = String::new();
    let mut id: u32 = 0;

    let mut indexes: HashMap<u32, String> = HashMap::new();

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line? + "\n";

        if line.trim().starts_with("<page>") {
            page = String::new();
            title = String::new();
            id = 0;
        } else if line.trim().starts_with("<title>") {
            title = line
                .replace("<title>", "")
                .replace("</title>", "")
                .trim()
                .replace("/", "\\")
                .replace(" ", "_")
                .to_string();
        } else if line.trim().starts_with("<id>") && id == 0 {
            id = line
                .replace("<id>", "")
                .replace("</id>", "")
                .trim()
                .parse::<u32>()
                .unwrap();
        } else if line.trim().starts_with("</page>") {
            if !title.contains(":") {
                let f = format!("{}wiki/{}.md", base_path.to_owned(), id);

                if !Path::new(&f).exists() {
                    indexes.insert(id, title.clone());
                    let file = File::create(f)?;
                    file.write_all_at(
                        to_md(text_from(&page), &title.replace("_", " ")).as_bytes(),
                        0,
                    )?;
                }
            }
        }

        page += &line;
    }

    let mut index_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(base_path.to_owned() + "indexes.csv")
        .unwrap();

    writeln!(index_file, "id,title").unwrap();
    for (id, title) in indexes {
        writeln!(index_file, "{},{}", id, title).unwrap();
    }

    Ok(())
}
