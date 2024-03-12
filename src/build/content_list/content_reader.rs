use std::fs::File;

use regex::Regex;
use serde::Deserialize;
use serde_xml_rs::from_reader;

use super::ContentWindowItem;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all="kebab-case")]
struct ContentWindow {
	name: String,
	game_folder : String,
	content_list : ContentList,
}

#[derive(Debug, Deserialize, PartialEq)]
struct ContentList {
	#[serde(rename = "$value")]
	items: Vec<ContentItem>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all="kebab-case")]
struct ContentItem {
	name: String,
	#[serde(rename="type")]
	content_type: String,
	subcontent_list: SubcontentList,
}

#[derive(Debug, Deserialize, PartialEq)]
struct SubcontentList {
	#[serde(rename="$value")]
	items: Vec<SubcontentItem>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all="kebab-case")]
struct SubcontentItem {
	#[serde(rename="type")]
	subcontent_type: String,
	#[serde(rename="$value")]
	value: String,
}

pub fn read(loc : String) -> Result<ContentWindowItem, String> {
	let read = File::open(format!("src/content/{loc}"));
	if read.is_err() {
		panic!("Could not open src/content/{loc} : {}", read.err().unwrap());
	}

	println!("cargo:rerun-if-changed=src/content/{loc}");

	let in_xml = read.unwrap();
	let read_xml : Result<ContentWindow, serde_xml_rs::Error> = from_reader(in_xml);
	
	if read_xml.is_err() {
		return Err(read_xml.unwrap_err().to_string());
	}

	let content_window = read_xml.unwrap();

	let mod_path = Regex::new(r#"(?<path>[\w\/]+)\/\w+\.xml"#).unwrap();
	let path = mod_path.captures(&loc).unwrap();
	let relative_mod_path = path.name("path").unwrap().as_str().replace("/", "::");

	Ok(ContentWindowItem {
		xml_def_path : loc,
		mod_location : format!("{relative_mod_path}::{}", content_window.name),
	})
}