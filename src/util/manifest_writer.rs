use std::{fs::{File, OpenOptions}, io::{BufRead, BufReader, BufWriter, Error, Read}, path::Path, rc::Rc, vec::IntoIter};

struct CharFileIter {
	// From https://stackoverflow.com/questions/47193584/is-there-an-owned-version-of-stringchars
	line : Option<IntoIter<char>>,
	reader : BufReader<File>,
}

impl CharFileIter {
	fn get_line(&mut self) -> Option<<CharFileIter as Iterator>::Item> {
		let mut line = String::new();

		let line_read = self.reader.read_line(&mut line);
		if line_read.is_err() {
			return Some(Err(line_read.err().unwrap()));
		}
		let bytes_read = line_read.unwrap();
		
		if bytes_read > 0 {
			self.line = Some(line.chars().collect::<Vec<_>>().into_iter());

			let chars = self.line.as_mut().unwrap();
			let char = chars.next();
			return Some(Ok(char.unwrap()));
		} else {
			return None;
		}
	}
}

impl<'a> Iterator for CharFileIter {
	type Item = std::io::Result<char>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.line.is_none() {
			return self.get_line();
		}

		let chars = self.line.as_mut().unwrap();

		let next_char = chars.next();
		if next_char.is_none() {
			return self.get_line();
		} else {
			Some(Ok(next_char.unwrap()))
		}
	}
}


pub struct ManifestWriter<'a> {
	read_iter : CharFileIter,
	read_path : &'a Path,

	writer : BufWriter<File>,
}

impl<'a> ManifestWriter<'a> {
	pub fn open(path : &'a Path) -> std::io::Result<Self> {
		let read = File::open(path)?;
		let tmp_path = path.with_extension(".tmp");
		let write = File::create(tmp_path)?;
		
		Ok(ManifestWriter {
			read_path: path,
			read_iter: CharFileIter {
				line: None,
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