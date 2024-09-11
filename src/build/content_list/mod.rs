use std::{collections::HashMap, env, fs::File, io::Write, iter::Map, path::Path};

use regex::Regex;

mod content_reader;

#[derive(Debug)]
struct ContentWindowItem {
    xml_def_path: String,
    mod_location: String,
    window_name: String,

    party_pack: String,
    game_folder: String,

    content_info: Vec<ContentInfo>,
}

#[derive(Debug)]
struct ContentInfo {
    content_type: String,
    subcontent_info: Vec<SubcontentInfo>,
}

#[derive(Debug)]
struct SubcontentInfo {
    args: Vec<String>,
}

pub fn compile_content_list() {
    let content_list = include_str!("../../content/content_list.ui");

    let content_tag: Regex =
        Regex::new(r#"<property name="xml-definition">\W*(?<def>[\w\/.]+)\W*<\/property>"#)
            .unwrap();

    let content: Vec<ContentWindowItem> = content_tag
        .captures_iter(content_list)
        .map(move |caps| {
            let def = caps.name("def").unwrap();
            let result = ContentWindowItem::read(def.as_str().to_string());
            if result.is_err() {
                panic!("Could not read {}: {}", def.as_str(), result.unwrap_err());
            }
            result.unwrap()
        })
        .collect();

    let out_dir = env::var_os("OUT_DIR").unwrap();

    let content_list_pth = Path::new(&out_dir).join("content_list.rs");
    let mut content_list_out = File::create(content_list_pth).expect("Could not create file.");

    content_list_out.write(b"pub fn create_window(xml_def_path : &str) -> ContentWindow {\n\tmatch xml_def_path {\n").expect("Could not write bytes.");
    for c in &content {
        let out = format!("\t\t\"{}\" => {{crate::content::{}::ensure_all_types(); gtk::glib::Object::new::<crate::content::{}>().upcast()}},\n", c.xml_def_path, c.mod_location, c.mod_location);
        content_list_out
            .write(out.as_bytes())
            .expect("Could not write bytes.");
    }
    content_list_out
        .write(b"\t\t_=>panic!(\"XML definition of path {xml_def_path} not found.\")\n\t}\n}\n")
        .expect("Could not write bytes.");

    content_list_out.write(b"pub fn get_subcontent_args(xml_def_path : &str, content_type : &str) -> Vec<Vec<&'static str>> {\n\tmatch xml_def_path {\n").expect("Could not write bytes.");
    for c in &content {
        let info = &c.content_info;
        let window_match = format!("\t\t\"{}\" => match content_type {{\n", c.xml_def_path);
        content_list_out
            .write(window_match.as_bytes())
            .expect("Could not write bytes.");
        for i in info {
            let i_out = format!("\t\t\t\"{}\" => vec![", i.content_type);
            content_list_out
                .write(i_out.as_bytes())
                .expect("Could not write bytes.");
            let mut subcontent_iter = i.subcontent_info.iter().peekable();
            while let Some(s) = subcontent_iter.next() {
                let s_out = format!(
                    "vec![\"{}\"]{}",
                    s.args.join("\",\""),
                    if subcontent_iter.peek().is_none() {
                        ""
                    } else {
                        ","
                    }
                );
                content_list_out
                    .write(s_out.as_bytes())
                    .expect("Could not write subcontent info.");
            }
            content_list_out
                .write(b"],\n")
                .expect("Could not write bytes.");
        }
        content_list_out
            .write(b"\t\t\t_=>panic!(\"content_type {content_type} not found.\"),\n\t\t},\n")
            .expect("Could not write bytes.");
    }
    content_list_out
        .write(b"\t\t_=>panic!(\"XML definition of path {xml_def_path} not found.\"),\n\t}\n}\n")
        .expect("Could not write bytes.");

    content_list_out.write(b"\npub fn get_relative_folder(xml_def_path : &str) -> &'static std::path::Path {\n\tmatch xml_def_path {\n").expect("Could not write bytes.");
    for c in &content {
        let out = format!(
            "\t\t\"{}\" => std::path::Path::new(\"{}/{}\"),\n",
            c.xml_def_path, c.party_pack, c.game_folder
        );
        content_list_out
            .write(out.as_bytes())
            .expect("Could not write bytes.");
    }
    content_list_out
        .write(b"\t\t_ => panic!(\"XML definition of path {xml_def_path} not found.\"),\n\t}\n}\n")
        .expect("Could not write bytes.");

    println!("cargo:rerun-if-changed=src/build/content_list/mod.rs");
    println!("cargo:rerun-if-changed=src/build/content_list/content_reader.rs");
    println!("cargo:rerun-if-changed=src/content/content_list.ui");
}
