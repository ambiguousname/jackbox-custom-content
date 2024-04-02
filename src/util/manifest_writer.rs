use std::{fs::{File, OpenOptions}, io::{BufRead, BufReader, BufWriter, Empty, Error, ErrorKind, Read, Write}, path::{Path, PathBuf}, rc::Rc, vec::IntoIter};

use regex::Regex;

struct CharFileIter {
	// From https://stackoverflow.com/questions/47193584/is-there-an-owned-version-of-stringchars
	line : Option<IntoIter<char>>,
	reader : BufReader<File>,
}

enum ManifestError {
	/// An error thrown by the writer or reader.
	StdErr(Error),
	SerdeJsonErr(serde_json::Error),
	/// If we've exited out of the object we're searching (i.e., a closing `}`):
	ExitedObject(),
	/// If we've exited out of the array we're searching (i.e., a closing `]`):
	ExitedArray(),
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
	/// Where we currently are in the JSON (relative to objects).
	curr_path : Vec<String>,
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
			curr_path: vec![],
		})
	}

	pub fn read_until_key(&mut self) -> Result<String, ManifestError> {
		let mut key_str = String::new();
		
		// Simple way to track if our key_str matches "key":{ without using Regex.
		let mut key_fsm = 0;

		while let Some(c) = self.read_iter.next() {
			if c.is_err() {
				return Err(ManifestError::StdErr(c.unwrap_err()));
			}
			let char = c.unwrap();

			if char == '"' || char == ':' || char == '{' || char == '_' || char.is_alphanumeric() {
				key_str.push(char);
				match key_fsm {
					0 => if char == '"' {key_fsm += 1} else {key_fsm = 0},
					1 => if char == '"' {key_fsm += 1},
					2 => if char == '{' {return Ok(key_str)},
					_ => panic!("Unexpected key fsm value of {key_fsm}."),
				}
			} else {
				if key_str.len() > 0 {
					key_fsm = 0;
					self.writer.write(key_str.as_bytes()).map_err(|e| {
						ManifestError::StdErr(e)
					})?;
					key_str.clear();
				}

				let mut char_out : Vec<u8> = Vec::new();
				char.encode_utf8(&mut char_out);
				
				self.writer.write(&char_out).map_err(|e| {
					ManifestError::StdErr(e)
				})?;

				if char == '}' {
					return Err(ManifestError::ExitedObject());
				}
			}
		}
		Err(ManifestError::StdErr(Error::new(ErrorKind::NotFound, format!("Unexpected end of input for read_until_key."))))
	}

	/// Read [`ManifestWriter::read_iter`] until we find `key_to_match`.
	pub fn find_key(&mut self, key_to_match : String) -> Result<(), ManifestError> {
		let formatted_key = format!(r#""{}":{{"#, key_to_match);

		loop {
			let key = self.read_until_key()?;
			if formatted_key == key {
				return Ok(())
			}
		}
	}

	pub fn read_object(&mut self, writer : Option<&mut dyn Write>) -> std::io::Result<()> {
		let writer_exists = writer.is_some();
		let mut empty = Empty::default();
		let start_depth = self.curr_path.len();
		let mut curr_depth = start_depth;
		// ONLY use if writer_exists:
		let unwrapped_writer = writer.unwrap_or(&mut empty);
		while let Some(c) = self.read_iter.next() {
			let char = c?;
			if char == '{' {
				curr_depth += 1;
			} else if char == '}' {
				curr_depth -= 1;
			}

			if writer_exists {
				let mut to_write = Vec::new();
				char.encode_utf8(&mut to_write);
				unwrapped_writer.write(&to_write)?;
			}

			if curr_depth == start_depth {
				return Ok(());
			}
		}
		Err(Error::new(ErrorKind::InvalidData, format!("Missing {} }}", curr_depth)))
	}

	pub fn insert(&mut self, key : String, value : serde_json::Value) -> Result<(), ManifestError> {
		let mut written_values = false;

		// TODO: Allow for multiple key values.
		self.find_key(key)?;
		self.read_object(None::<&mut dyn std::io::Write>).map_err(|e| {
			ManifestError::StdErr(e)
		})?;

		if !written_values {
			let buf = serde_json::to_vec(&value).map_err(|e| {
				ManifestError::SerdeJsonErr(e)
			})?;
			self.writer.write(&buf).map_err(|e| {
				ManifestError::StdErr(e)
			})?;
		}

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