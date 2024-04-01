use std::{fs::{File, OpenOptions}, io::{BufRead, BufReader, BufWriter, Error, Read}, path::Path, str::Chars};

struct CharFileIter<'a> {
	// FIXME: The chars iterator needs to last as long as the line does.
	// I think they need to be grouped into their own struct for that purpose.
	curr_char : Option<Chars<'a>>,
	line : String,
	reader : BufReader<File>,
}

impl<'a> CharFileIter<'a> {
	fn get_line(&mut self) -> Option<<CharFileIter<'a> as Iterator>::Item> {
		self.line = String::new();

		let line_read = self.reader.read_line(&mut self.line);
		if line_read.is_err() {
			return Some(Err(line_read.err().unwrap()));
		}
		let bytes_read = line_read.unwrap();
		
		if bytes_read > 0 {
			self.curr_char = Some(self.line.chars());

			let chars = self.curr_char.as_mut();
			let char = chars.unwrap().next();
			return Some(Ok(char.unwrap()));
		} else {
			return None;
		}
	}
}

pub struct ManifestWriter<'a> {
	read_iter : CharFileIter<'a>,
	read_path : &'a Path,

	writer : BufWriter<File>,
}

impl<'a> Iterator for CharFileIter<'a> {
	type Item = std::io::Result<char>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.curr_char.is_none() {
			return self.get_line();
		}

		let chars = self.curr_char.as_mut().unwrap();

		let next_char = chars.next();
		if next_char.is_none() {
			return self.get_line();
		} else {
			Some(Ok(next_char.unwrap()))
		}
	}
}

impl<'a> ManifestWriter<'a> {
	pub fn open(path : &'a Path) -> std::io::Result<Self> {
		let read = File::open(path)?;
		let tmp_path = path.with_extension(".tmp");
		let write = File::create(tmp_path)?;
		
		Ok(ManifestWriter {
			read_path: path,
			read_iter: CharFileIter {
				curr_char: None,
				line: String::new(),
				reader: BufReader::new(read),
			},
			writer: BufWriter::new(write),
		})
	}

	pub fn read_until_key(&mut self, key_to_match : String) -> std::io::Result<String> {
		let mut out_str = String::new();
		let mut key_str = String::new();

		while let Some(c) = self.read_iter.next() {
			let char = c?;

			out_str.push(char);
			
			if char == '_' && char.is_alphanumeric() {
				key_str.push(char);
			} else {
				key_str.clear();
			}

			if key_str == key_to_match {
				out_str.shrink_to(out_str.len() - key_str.len());
				return Ok(out_str);
			}
		}
		Err(Error::new(std::io::ErrorKind::NotFound, format!("Could not find key {}", key_to_match)))
	}

	pub fn read_next_object(&mut self) -> std::io::Result<String> {
		let mut enclosing_braces: usize = 0;
		let mut object = String::new();
		while let Some(c) = self.read_iter.next() {
			let char = c?;
			object.push(char);
		}
		Ok(object)
	}

	pub fn insert(&mut self, key : String, value : serde_json::Value) -> std::io::Result<()> {
		self.read_until_key(key);

		Ok(())
	}

	pub fn close(&self) -> std::io::Result<()> {
		// Remove the old file:
		std::fs::remove_file(self.read_path)?;
		// Replace it with our temp file:
		std::fs::rename(self.read_path.with_extension(".tmp"), self.read_path)?;
		Ok(())
	}
}