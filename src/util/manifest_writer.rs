use std::{borrow::BorrowMut, fs::{File, OpenOptions}, io::{BufRead, BufReader, BufWriter, Empty, Error, ErrorKind, Read, Write}, path::{Path, PathBuf}, rc::Rc, vec::IntoIter};

struct CharFileIter {
	// From https://stackoverflow.com/questions/47193584/is-there-an-owned-version-of-stringchars
	line : Option<IntoIter<char>>,
	reader : BufReader<File>,
}

enum ManifestError {
	/// An error thrown by the writer or reader.
	StdErr(Error),
	SerdeJsonErr(serde_json::Error),
	/// If we've found a value that shouldn't be there, like an unexpected }
	UnexpectedValue(String),
	/// If we've left the file unexpectedly.
	UnexpectedEOF(),
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

/// During a read of the whole file, where are we?
#[derive(PartialEq)]
enum ManifestParseState {
	/// JSON accepts many possible characters to start with: https://www.json.org/json-en.html
	/// But for our utility purposes, there's no way anyone would want to just edit a file with a number or a string.
	/// Instead, we're expecting it to start with an object or an array.
	/// In this state, we have yet to read anything to determine whether or not we're in an array or object.
	Uninitialized,
	/// [`ManifestWriter::initialize`] will pop off [`Uninitialized`] and set this instead.
	/// Used by the reader to determine that there's nothing else to read, and we're at EOF.
	Empty,
	/// We've determined ourselves to be inside an object `{}`.
	ObjectParse,
	/// We've determined ourselves to be inside an array `[]`.
	ArrayParse,
	/// We found a key previously, and now we want to read the value associated with that key:
	KeyParsed,
}

/// A utility structure for going through manifest (i.e., JSON files), and reading/writing data to/from them.
/// Meant for fast and dirty writing rather than parsing the whole thing.
/// It has some limitations for this reason. For example, it is not a fully-fledged linter. It assumes that the JSON is mostly accurate, but it won't look for things like only one object in a JSON. It's on you to provide correctly written JSON.
pub struct ManifestWriter<'a> {
	read_iter : CharFileIter,
	read_path : &'a Path,

	writer : BufWriter<File>,
	/// Where we currently are in the JSON (relative to objects).
	curr_path : Vec<String>,
	/// A stack FSM for reading through JSON:
	parse_state: Vec<ManifestParseState>,
	/// The key associated with the value we'll next read.
	key_buf : String,
}

/// The types of nodes we support reading.
/// Could be expanded in the future, but [`ManifestWriter`] is mostly meant to look for key values
enum ManifestNode {
	/// A key, formatted as "key":
	Key(String),
	/// A value. This doesn't match ALL of the JSON value types, just anything that isn't an array or object start. (i.e., true, false, "string", etc.)
	Value(String),
	/// Start of an object `{`
	ObjectStart,
	/// Close of an object `}`
	ObjectClose,
	/// Start of an array `[`
	ArrayStart,
	/// End of an array `]`
	ArrayClose,
	EOF
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
			parse_state: vec![ManifestParseState::Uninitialized],
			key_buf: String::new(),
		})
	}

	fn start_object(&mut self) -> ManifestNode {
		if self.curr_path.len() <= 0 {
			self.curr_path.push(String::from("/"));
		} else {
			self.curr_path.push(self.key_buf.clone());
			self.key_buf.clear();
		}
		self.parse_state.push(ManifestParseState::ObjectParse);
		ManifestNode::ObjectStart
	}

	fn end_object(&mut self) -> ManifestNode {
		self.curr_path.pop();
		self.parse_state.pop();
		ManifestNode::ObjectClose
	}

	fn start_array(&mut self) -> ManifestNode {
		if self.key_buf.len() > 0 {
			self.curr_path.push(self.key_buf.clone());
			self.key_buf.clear();
		}
		self.parse_state.push(ManifestParseState::ArrayParse);
		ManifestNode::ArrayStart
	}

	fn end_array(&mut self) -> ManifestNode {
		self.parse_state.pop();
		let last_state = self.parse_state.last();
		if last_state.is_some() && ManifestParseState::ObjectParse == *last_state.unwrap() {
			self.curr_path.pop();
		}
		ManifestNode::ArrayClose
	}

	fn expect_value(&mut self, first_char : char, rest_of_value : &str) -> Result<ManifestNode, ManifestError> {
		let mut chars = rest_of_value.chars();


		while let Some(c) = self.read_iter.next() {
			let ch = c.map_err(|e| {
				ManifestError::StdErr(e)
			})?;

			let next = chars.next();

			if ch == ',' && next.is_none() {
				let full_str = vec![first_char.to_string(), rest_of_value.to_string()].join("");
				return Ok(ManifestNode::Value(full_str));	
			} else if next.is_none() {
				return Err(ManifestError::UnexpectedValue(format!("Expected `,`, got {ch}")));
			}

			let next_ch = next.unwrap();
			if next_ch != ch {
				return Err(ManifestError::UnexpectedValue(format!("Expected `{next_ch}`, got `{ch}`")))
			}
		}
		
		Err(ManifestError::UnexpectedEOF())
	}

	fn get_numeric(&mut self) -> Result<ManifestNode, ManifestError> {
		let mut number_val = String::new();
		while let Some(c) = self.read_iter.next() {
			let ch = c.map_err(|e| {
				ManifestError::StdErr(e)
			})?;

			if ch.is_numeric() {
				number_val.push(ch);
			} else if ch == ',' {
				return Ok(ManifestNode::Value(number_val));
			} else {
				return Err(ManifestError::UnexpectedValue(format!("Expected a digit or `,`, got `{ch}`")));
			}
		}

		Err(ManifestError::UnexpectedEOF())
	}

	fn get_string(&mut self) -> Result<ManifestNode, ManifestError> {
		let mut string = String::new();
		let mut backslash = false;
		while let Some(c) = self.read_iter.next() {
			let ch = c.map_err(|e| {
				ManifestError::StdErr(e)
			})?;

			if backslash {
				string.push(ch);
			} else {
				match ch {
					'\\' => {string.push(ch); backslash = true;},
					'"' => return Ok(ManifestNode::Value(string)),
					_ => string.push(ch),
				}
			}
		}
		Err(ManifestError::UnexpectedEOF())
	}

	/// When we have a :, we need to find the next value after that.
	fn verify_value(&mut self) -> Result<ManifestNode, ManifestError> {
		let mut value_out = String::new();

		while let Some(c) = self.read_iter.next() {
			let ch = c.map_err(|e| {
				ManifestError::StdErr(e)
			})?;

			if ch.is_whitespace() {
				continue;
			}

			if ch.is_numeric() {
				let number = self.get_numeric()?;
				if let ManifestNode::Value(n) = number {
					return Ok(ManifestNode::Value(vec![ch.to_string(), n].join("")));
				} else {
					unreachable!("ManifestWriter::get_numeric returned a non-ManifestNode success.");
				}
			}

			return match ch {
				'"' => self.get_string(),
				'{' => Ok(self.start_object()),
				'[' => Ok(self.start_array()),
				't' => self.expect_value('t', "rue"),
				'f' => self.expect_value('f', "alse"),
				'n' => self.expect_value('n', "ull"),
				_ => Err(ManifestError::UnexpectedValue(format!("Unexpected value character start: {ch}"))),
			}
		}
		
		Err(ManifestError::UnexpectedEOF())
	}

	/// Assuming we're inside an object and we've discovered a `"` character,
	/// continue going until we find the full key.
	fn verify_key(&mut self) -> Result<ManifestNode, ManifestError> {
		let mut key_value = String::new();

		let mut end_quote = false;

		let key = self.get_string()?;
		if let ManifestNode::Value(key_name) = key {
			while let Some(c) = self.read_iter.next() {
				let ch = c.map_err(|e| {
					ManifestError::StdErr(e)
				})?;
	
				if ch.is_whitespace() {
					continue;
				}

				return match ch {
					':' => Ok(ManifestNode::Key(key_value)),
					_ => Err(ManifestError::UnexpectedValue(format!("Expected : not {ch}"))),
				}
			}
			
			return Err(ManifestError::UnexpectedEOF());
		} else {
			unreachable!("ManifestWriter::get_string returned a non-value on success. This should not be possible.");
		}
	}

	fn read_array(&mut self) -> Result<ManifestNode, ManifestError> {
		while let Some(c) = self.read_iter.next() {
			let ch = c.map_err(|e| {
				ManifestError::StdErr(e)
			})?;

			if ch.is_whitespace() {
				continue;
			}
			match ch {
				']' => {
					return Ok(self.end_array());
				},
				_ => { return self.verify_value(); }
			}
		}
		
		Err(ManifestError::UnexpectedEOF())
	}

	fn read_object(&mut self) -> Result<ManifestNode, ManifestError> {
		while let Some(c) = self.read_iter.next() {
			let ch = c.map_err(|e| {
				ManifestError::StdErr(e)
			})?;

			if ch.is_whitespace() {
				continue;
			}

			match ch {
				'"' => {return self.verify_key(); },
				// A linter might want to check the veracity of commas, but I think we're fine.
				',' => continue,
				'}' => {
					return Ok(self.end_object());
				},
				_ => { return Err(ManifestError::UnexpectedValue(format!("Unexpected value reading object: {ch}"))); }
			}
		}
		
		Err(ManifestError::UnexpectedEOF())
	}

	fn initialize(&mut self) -> Result<ManifestNode, ManifestError> {
		self.parse_state = vec![ManifestParseState::Empty];
		while let Some(c) = self.read_iter.next() {
			let ch = c.map_err(|e| {
				ManifestError::StdErr(e)
			})?;

			if ch.is_whitespace() {
				continue;
			}
			match ch {
				'{' => {Ok(self.start_object())},
				'[' => {Ok(self.start_array())},
				_ => Err(ManifestError::UnexpectedValue(format!("Expected [ or {{, found {ch}"))),
			};
		}
		Err(ManifestError::UnexpectedEOF())
	}
	// TODO: Output read values from parse_node to the writer.

	/// Based on https://www.json.org/json-en.html
	/// Not an actual AST parser, but this does enough to look through JSON.
	/// Returns whenever ANY of the [`ManifestNode`] types are found.
	pub fn parse_node(&mut self) -> Result<ManifestNode, ManifestError> {
		let state = self.parse_state.last().expect("Could not get parse_state value.");
		let value = match state {
			ManifestParseState::Uninitialized => {
				self.initialize()
			},
			ManifestParseState::Empty => {
				Ok(ManifestNode::EOF)
			},
			ManifestParseState::ObjectParse => {
				self.read_object()
			},
			ManifestParseState::ArrayParse => {
				self.read_array()
			},
			ManifestParseState::KeyParsed => {
				self.verify_value()
			},
		}?;
		return Ok(value);
	}

	pub fn insert(&mut self, key : String, value : serde_json::Value) -> Result<(), ManifestError> {
		let mut written_values = false;

		// TODO: Allow for multiple key values.
		// self.find_key(key)?;
		// self.read_object(None::<&mut dyn std::io::Write>).map_err(|e| {
		// 	ManifestError::StdErr(e)
		// })?;

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