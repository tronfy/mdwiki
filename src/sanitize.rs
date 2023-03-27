use lazy_static::lazy_static;

use regex::{Regex, RegexBuilder};

pub fn to_md(xml: String, title: &str) -> String {
    lazy_static! {
        static ref REF: Regex = RegexBuilder::new("&lt;ref.+?(&lt;/ref&gt;|/&gt;)")
            .dot_matches_new_line(true)
            .build()
            .unwrap();
        static ref MULTI_LF: Regex = Regex::new(r"\n\n+").unwrap();
    }

    let xml = remove_after_see_also(&xml);

    // build headings, add title
    let xml = build_headings(&xml, title.to_string());

    // remove references
    let xml = REF.replace_all(&xml, "").to_string();

    // macros {{ ... }}
    let xml = build_macros(&xml);

    // images
    let xml = build_images(&xml);

    // mediawiki links to markdown links
    let xml = build_links(&xml);

    // styling
    let xml = xml.replace("'''", "**"); // bold
    let xml = xml.replace("''", "*"); // italic

    // remove multiple blank lines
    let xml = MULTI_LF.replace_all(&xml, "\n\n");
    let xml = xml.trim().to_owned() + "\n";

    xml.to_string()
}

pub fn text_from(xml: &String) -> String {
    lazy_static! {
        static ref TEXT_ALL: Regex = RegexBuilder::new("<text.+?>.+?</text>")
            .dot_matches_new_line(true)
            .build()
            .unwrap();
        static ref TAG_START: Regex = Regex::new("<text.+?>").unwrap();
        static ref TAG_END: Regex = Regex::new("</text>").unwrap();
    }

    let text = TEXT_ALL.captures(&xml).unwrap().get(0).unwrap().as_str();
    let text = &TAG_START.replace(text, "");
    let text = &TAG_END.replace(text, "");

    text.to_string()
}

fn remove_after_see_also(xml: &str) -> String {
    lazy_static! {
        // todo: i18n
        static ref SEE_ALSO: Regex = RegexBuilder::new("== Ver também ==.+")
            .dot_matches_new_line(true)
            .build()
            .unwrap();
    }

    let xml = SEE_ALSO.replace(xml, "");
    xml.to_string()
}

fn build_headings(xml: &str, title: String) -> String {
    lazy_static! {
        static ref H4: Regex = Regex::new("====.+?====").unwrap();
        static ref H3: Regex = Regex::new("===.+?===").unwrap();
        static ref H2: Regex = Regex::new("==.+?==").unwrap();
    }

    let xml = H4.replace_all(xml, |caps: &regex::Captures| {
        format!("#### {}\n\n", &caps[0][4..caps[0].len() - 4].trim())
    });

    let xml = H3.replace_all(&xml, |caps: &regex::Captures| {
        format!("### {}\n\n", &caps[0][3..caps[0].len() - 3].trim())
    });

    let xml = H2.replace_all(&xml, |caps: &regex::Captures| {
        format!("## {}\n\n", &caps[0][2..caps[0].len() - 2].trim())
    });

    let xml = format!("# {}\n\n", title.trim()) + &xml;
    xml.to_string()
}

fn build_images(xml: &String) -> String {
    lazy_static! {
        // assumes image takes the whole line
        // todo: i18n
        static ref IMG: Regex = RegexBuilder::new(r"^\[\[Imagem:.+\]\]$")
            .multi_line(true)
            .build()
            .unwrap();
    }

    // todo: deal with images
    let xml = IMG.replace_all(&xml, "");

    xml.to_string()
}

fn build_links(xml: &String) -> String {
    lazy_static! {
        static ref LINK: Regex = RegexBuilder::new("\\[\\[.+?\\]\\]")
            .dot_matches_new_line(true)
            .build()
            .unwrap();
    }

    let xml = LINK.replace_all(&xml, |caps: &regex::Captures| {
        let link = caps.get(0).unwrap().as_str();
        let link = link.replace("[[", "").replace("]]", "");
        let link = link.split("|").collect::<Vec<&str>>();
        // todo: format properly?
        let link = if link.len() > 1 {
            format!("[{}]({})", link[1], link[0].replace(" ", "_"))
        } else {
            format!("[{}]({})", link[0], link[0].replace(" ", "_"))
        };
        link
    });

    xml.to_string()
}

fn build_macros(xml: &String) -> String {
    lazy_static! {
        // static ref MACRO: Regex = Regex::new(r"\{\{.+?\}\}").unwrap();
        static ref MACRO: Regex = RegexBuilder::new(r"\{\{.+?\}\}")
            .dot_matches_new_line(true)
            .build()
            .unwrap();
    }

    // todo
    let xml = MACRO.replace_all(&xml, "");

    xml.to_string()
}
