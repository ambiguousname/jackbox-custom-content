use std::{fs::{File, OpenOptions}, io::{BufRead, BufReader, BufWriter, Empty, Error, ErrorKind, Read, Write}, path::Path, rc::Rc, vec::IntoIter};

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

	/// Read [`ManifestWriter::read_iter`] until we find `key_to_match`.
	pub fn read_until_key(&mut self, key_to_match : String) -> std::io::Result<()> {
		let mut key_str = String::new();

		let formatted_key = format!(r#""{}":{{"#, key_to_match);

		while let Some(c) = self.read_iter.next() {
			let char = c?;
			
			// We're looking for a key string like "key":{
			if char == '"' || char == ':' || char == '{' || char == '_' || char.is_alphanumeric() {
				key_str.push(char);
			} else {
				if key_str.len() > 0 {
					self.writer.write(key_str.as_bytes())?;
					key_str.clear();
				}
				let mut char_out : Vec<u8> = Vec::new();
				char.encode_utf8(&mut char_out);
				self.writer.write(&char_out)?;
			}

			if key_str == formatted_key {
				return Ok(());
			}
		}
		Err(Error::new(ErrorKind::NotFound, format!("Could not find key {}", key_to_match)))
	}

	pub fn read_next_object(&mut self, writer : Option<&mut dyn Write>) -> std::io::Result<()> {
		let mut enclosing_braces: usize = 1;

		let writer_exists = writer.is_some();
		let mut empty = Empty::default();
		// ONLY use if writer_exists:
		let unwrapped_writer = writer.unwrap_or(&mut empty);
		while let Some(c) = self.read_iter.next() {
			let char = c?;
			if char == '{' {
				enclosing_braces += 1;
			} else if char == '}' {
				enclosing_braces -= 1;
			}

			if writer_exists {
				let mut to_write = Vec::new();
				char.encode_utf8(&mut to_write);
				unwrapped_writer.write(&to_write)?;
			}

			if enclosing_braces == 0 {
				return Ok(());
			}
		}
		Err(Error::new(ErrorKind::InvalidData, format!("Missing {} }}", enclosing_braces)))
	}

	pub fn insert(&mut self, key : String, value : serde_json::Value) -> std::io::Result<()> {
		// TODO: Allow for multiple key values.
		self.read_until_key(key)?;
		self.read_next_object(None::<&mut dyn std::io::Write>)?;

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