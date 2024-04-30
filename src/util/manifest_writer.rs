use std::{fs::File, io::{BufRead, BufReader, BufWriter, Error, Read, Seek, SeekFrom, Write}, path::Path, vec::IntoIter};


#[derive(Debug)]
pub enum ManifestError {
	/// An error thrown by the writer or reader.
	StdErr(Error),
	SerdeJsonErr(serde_json::Error),
	/// If we've found a value that shouldn't be there, like an unexpected }
	UnexpectedValue(String),
	/// If we've left the file unexpectedly.
	UnexpectedEOF(),
	/// If our Manifest is in a state we don't expect it to be in.
	UnexpectedState(String),
}

impl ManifestError {
	pub fn to_string(&self) -> String {
		match self {
			Self::StdErr(e) => {
				format!("stderr trying to write to manifest: {}", e.to_string())
			},
			Self::SerdeJsonErr(e) => {
				format!("serde error trying to write to manifest: {}", e.to_string())
			},
			Self::UnexpectedEOF() => {
				String::from("Unexpected end of file when reading manifest.")
			},
			Self::UnexpectedState(s) |
			Self::UnexpectedValue(s) => {
				s.to_string()
			},
		}
	}
}

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

	fn read_to_end(&mut self, buf : &mut Vec<u8>) -> std::io::Result<()> {
		let string : String = self.line.as_mut().unwrap().collect();
		buf.write_all(string.as_bytes())?;
		self.reader.read_to_end(buf)?;
		Ok(())
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
#[derive(PartialEq, Debug)]
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

pub enum WriteTo<T : Write> {
	OutFile,
	None,
	/// Allow us to set whatever kind of custom 
	CustomWriter(T),
}

/// A utility structure for going through manifest (i.e., JSON files), and reading/writing data to/from them.
/// Meant for fast and dirty writing rather than parsing the whole thing.
/// It has some limitations for this reason. For example, it is not a fully-fledged linter. It assumes that the JSON is mostly accurate, but it won't look for things like only one object in a JSON. It's on you to provide correctly written JSON.
/// 
/// `T` Represents something we might want to use when not directly outputting to the writer.
pub struct ManifestWriter<'a, T : Write> {
	read_iter : CharFileIter,
	read_path : &'a Path,

	writer : BufWriter<File>,
	/// Where should we be writing?
	pub active_writer : WriteTo<T>,
	
	/// Where we currently are in the JSON (relative to objects).
	curr_path : Vec<String>,
	/// A stack FSM for reading through JSON:
	parse_state: Vec<ManifestParseState>,
	/// The key associated with the value we'll next read.
	key_buf : String,
}

/// The types of nodes we support reading.
/// Could be expanded in the future, but [`ManifestWriter`] is mostly meant to look for key values
#[derive(PartialEq, Debug)]
pub enum ManifestNode {
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

macro_rules! map_err {
	($e:expr) => {
		$e.map_err(|err| {
			ManifestError::StdErr(err)
		})
	};
	(serde, $e:expr) => {
		$e.map_err(|err| {
			ManifestError::SerdeJsonErr(err)
		})
	}
}

impl<'a, T: Write> ManifestWriter<'a, T> {
	pub fn open(path : &'a Path) -> std::io::Result<Self> {
		let read = File::open(path)?;
		let tmp_path = path.with_extension("tmp");
		let write = File::create(tmp_path)?;
		
		Ok(ManifestWriter {
			read_path: path,
			read_iter: CharFileIter {
				line: None,
				reader: BufReader::new(read),
			},
			
			writer: BufWriter::new(write),
			active_writer: WriteTo::OutFile,

			curr_path: vec![],
			parse_state: vec![ManifestParseState::Uninitialized],
			key_buf: String::new(),
		})
	}

	pub fn next(&mut self) -> Result<char, ManifestError> {
		let char = self.read_iter.next();
		
		if char.is_some() {
			return map_err!(char.unwrap()).and_then(|c| {
				let mut out_bytes = vec![0];
				c.encode_utf8(&mut out_bytes);
				map_err!(self.write(&out_bytes))?;
				Ok(c)
			});

		}

		Err(ManifestError::UnexpectedEOF())
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
		} else {
			self.curr_path.push(String::from("$ARRAY$"));
		}
		self.parse_state.push(ManifestParseState::ArrayParse);
		ManifestNode::ArrayStart
	}

	fn end_array(&mut self) -> ManifestNode {
		self.parse_state.pop();
		self.curr_path.pop();
		ManifestNode::ArrayClose
	}

	fn expect_value(&mut self, first_char : char, rest_of_value : &str) -> Result<ManifestNode, ManifestError> {
		let mut chars = rest_of_value.chars();

		loop {
			let ch = self.next()?;

			let next = chars.next();

			if (ch == ',' || ch.is_whitespace()) && next.is_none() {
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
	}

	fn get_numeric(&mut self, starting_digit : char) -> Result<ManifestNode, ManifestError> {
		let mut number_val = String::from(starting_digit);
		loop {
			let ch = self.next()?;

			if ch.is_numeric() {
				number_val.push(ch);
			} else if ch == ',' || ch.is_whitespace() {
				return Ok(ManifestNode::Value(number_val));
			} else if ch == ']' {
				// Are we closing an array? Right now our parser can't rewind reading, even though this is a valid JSON format.
				// So return an error.
				return Err(ManifestError::UnexpectedValue(format!("Closing ] while reading a number. This is valid, but the parser does not support reading this right now. Try inserting a whitespace to avoid these errors.")));
			} else {
				return Err(ManifestError::UnexpectedValue(format!("Expected a digit, whitespace, or `,` got `{ch}`")));
			}
		}
	}

	fn get_string(&mut self) -> Result<ManifestNode, ManifestError> {
		let mut string = String::new();
		let mut backslash = false;

		loop {
			let ch = self.next()?;

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
	}

	/// If we know we're about to read a value with a starting character, use that character to parse the value.
	fn get_value(&mut self, ch : char) -> Result<ManifestNode, ManifestError> {
		if ch.is_numeric() || ch == '-' {
			let number = self.get_numeric(ch)?;
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

	/// When we have a :, we need to find the next value after that.
	fn verify_value(&mut self) -> Result<ManifestNode, ManifestError> {
		// Remove KeyParsed if it exists since we're now trying to read a value:
		if self.parse_state.last() == Some(&ManifestParseState::KeyParsed) {
			self.parse_state.pop();
		}
		loop {
			let ch = self.next()?;

			if ch.is_whitespace() {
				continue;
			}

			return self.get_value(ch);
		}
	}

	/// Assuming we're inside an object and we've discovered a `"` character,
	/// continue going until we find the full key.
	fn verify_key(&mut self) -> Result<ManifestNode, ManifestError> {
		let key = self.get_string()?;
		if let ManifestNode::Value(key_value) = key {
			loop {
				let ch = self.next()?;

				if ch.is_whitespace() {
					continue;
				}

				return match ch {
					':' => {self.parse_state.push(ManifestParseState::KeyParsed); Ok(ManifestNode::Key(key_value))},
					_ => Err(ManifestError::UnexpectedValue(format!("Expected : not {ch}"))),
				}
			}
		} else {
			unreachable!("ManifestWriter::get_string returned a non-value on success. This should not be possible.");
		}
	}

	fn read_array(&mut self) -> Result<ManifestNode, ManifestError> {
		loop {
			let ch = self.next()?;

			// Ignore extra commas because we're not a linter.
			if ch.is_whitespace() || ch == ',' {
				continue;
			}
			match ch {
				']' => {
					return Ok(self.end_array());
				},
				_ => { return self.get_value(ch); }
			}
		}
	}

	fn read_object(&mut self) -> Result<ManifestNode, ManifestError> {
		loop {
			let ch = self.next()?;

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
	}

	/// Start actually 
	pub fn initialize(&mut self) -> Result<ManifestNode, ManifestError> {
		if self.parse_state.last() != Some(&ManifestParseState::Uninitialized) {
			return Err(ManifestError::UnexpectedState(String::from("Expected an uninitialized manifest to initialize.")));
		}
		self.parse_state = vec![ManifestParseState::Empty];
		loop {
			let ch = self.next()?;

			if ch.is_whitespace() {
				continue;
			}
			return match ch {
				'{' => {Ok(self.start_object())},
				'[' => {Ok(self.start_array())},
				_ => Err(ManifestError::UnexpectedValue(format!("Expected [ or {{, found {ch}"))),
			};
		}
	}

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

	/// Skip over characters on our current depth level until we find a node of a certain type.
	/// We use depth_offset for the caller to let us know what relative depth we should be looking for.
	/// Like if we've already opened an ObjectOpen or ArrayOpen node, and so the depth is affected because of that.
	fn skip_node(&mut self, node_type : ManifestNode, depth_offset : isize) -> Result<ManifestNode, ManifestError> {
		let mut prev_writer = WriteTo::None;
		std::mem::swap(&mut prev_writer, &mut self.active_writer);
		let curr_depth = self.curr_path.len().checked_add_signed(depth_offset).expect("depth_offset provided to skip_node leads to overflow.");

		loop {
			let node = self.parse_node()?;

			if node_type == node && curr_depth == self.curr_path.len() {
				self.active_writer = prev_writer;
				return Ok(node);
			}
		}
	}


	/// Seek by [`char`].
	pub fn write_search_seek(&mut self, offset : SeekFrom) -> Result<(), ManifestError> {
		// Per https://doc.rust-lang.org/std/mem/fn.size_of.html, the size for char will always be 4.
		let new_offset : SeekFrom;
		match offset {
			SeekFrom::Current(i) => new_offset = SeekFrom::Current(i * 4),
			SeekFrom::End(i) => new_offset = SeekFrom::End(i * 4),
			SeekFrom::Start(i) => new_offset = SeekFrom::Start(i * 4),
		}

		map_err!(self.writer.seek(new_offset))?;
		Ok(())
	}

	/// Helper function for [`Self::insert`]
	fn write_insert(&mut self, written_val : &mut bool, value : &serde_json::Value) -> Result<(), ManifestError> {
		let buf = map_err!(serde, serde_json::to_vec(value))?;
		map_err!(self.write(&buf))?;

		*written_val = true;
		Ok(())
	}

	/// Insert an object into a given key, assuming that we are presently in an object.
	pub fn insert(&mut self, key : String, value : serde_json::Value) -> Result<(), ManifestError> {
		if self.parse_state.last() != Some(&ManifestParseState::ObjectParse) {
			return Err(ManifestError::UnexpectedState(String::from("Cannot insert, Manifest is not parsing an object.")));
		}
		let mut written_values = false;


		let current_depth = self.curr_path.len();

		loop {
			let node = self.parse_node()?;
			if node == ManifestNode::EOF {
				return Err(ManifestError::UnexpectedEOF())
			}

			if ManifestNode::Key(key.clone()) == node {
				let mut prev_writer= WriteTo::None;
				std::mem::swap(&mut prev_writer, &mut self.active_writer);
				
				let next_value = self.parse_node()?;

				self.active_writer = prev_writer;

				if next_value == ManifestNode::ObjectStart {
					self.skip_node(ManifestNode::ObjectClose, -1)?;
				} else if next_value == ManifestNode::ArrayStart {
					self.skip_node(ManifestNode::ArrayClose, -1)?;
				}
				
				if !written_values {
					self.write_insert(&mut written_values, &value)?;
					map_err!(self.write(b","))?;
				}
			}

			if node == ManifestNode::ObjectClose && self.curr_path.len() == current_depth - 1 {
				if !written_values { 
					// Go back from object close:
					self.write_search_seek(SeekFrom::Current(-1))?;
					
					// Write our key:
					map_err!(self.writer.write(format!(r#""{key}": "#).as_bytes()))?;
					
					// Then re-write our value:
					self.write_insert(&mut written_values, &value)?;
					
					// And re-write the end of the object we just exited:
					map_err!(self.writer.write(b"\n}"))?;
				}
				return Ok(());
			}
		}
	}

	/// Assuming we're inside an array, a value from within that array.
	/// Will return [`None`] when no value is done.
	pub fn read_array_item(&mut self) -> Option<Result<serde_json::Value, ManifestError>> {
		if self.parse_state.last() != Some(&ManifestParseState::ArrayParse) {
			return Some(Err(ManifestError::UnexpectedState(String::from("Could not parse array item, Manifest is not in an array."))));
		}
		let curr_depth = self.curr_path.len();
		loop {
			let node_result = self.parse_node();
			if node_result.is_err() {
				return Some(node_result.map(|v| {serde_json::Value::Null}))
			}

			let node = node_result.expect("Could not unwrap ManifestNode.");
			if node == ManifestNode::EOF {
				return Some(Err(ManifestError::UnexpectedEOF()));
			}

			if node == ManifestNode::ArrayClose && curr_depth - 1 == self.curr_path.len() {
				return None;
			}
		}
	}

	/// Based on [`Self::active_writer`], write a buffer to a writer.
	pub fn write(&mut self, buf : &[u8]) -> std::io::Result<()> {
		return match &mut self.active_writer {
			WriteTo::OutFile => self.write_to_outfile(buf),
			WriteTo::CustomWriter(w) => w.write_all(buf),
			_ => Ok(()),
		};
	}

	/// Ignore the [`Self::active_writer`], just write directly to the outfile writer ([`Self::writer`]).
	pub fn write_to_outfile(&mut self, buf : &[u8]) -> std::io::Result<()> {
		self.writer.write_all(buf)
	}

	/// Fully finish reading/writing our manifest file, flush everything from the reader into the active writer.
	pub fn flush(&mut self) -> std::io::Result<()> {
		let mut buf = Vec::<u8>::new();
		self.read_iter.read_to_end(&mut buf)?;
		self.write(&buf)?;
		Ok(())
	}

	fn close(&self) -> std::io::Result<()> {
		// Remove the old file:
		std::fs::remove_file(self.read_path)?;
		// Replace it with our temp file:
		std::fs::rename(self.read_path.with_extension("tmp"), self.read_path)?;
		Ok(())
	}
}

impl<T> Drop for ManifestWriter<'_, T> where T: Write {
	fn drop(&mut self) {
		let _ = self.close();
	}
}

#[cfg(test)]
mod tests {

	use std::path::PathBuf;

	use serde_json::{Map, Number, Value};

use super::*;

	struct TestFile {
		pub file : File,
		file_pth : PathBuf
	}
	impl TestFile {
		fn create(path : &Path) -> Self {
			return TestFile {
				file: File::create(path).expect(format!("Could not open {}", path.display()).as_str()),
				file_pth: path.into()
			}
		}
	}

	impl Drop for TestFile {
		fn drop(&mut self) {
			std::fs::remove_file(self.file_pth.clone()).expect(format!("Could not remove {}", &self.file_pth.display()).as_str());
		}
	}

	#[test]
	fn test_write_close() {
		let test_json = Path::new("test.json");
		let file = TestFile::create(test_json);
	
		let test_tmp_json = Path::new("test.tmp");
		{
			assert!(test_json.exists(), "test.json does not exist.");

			let manifest = ManifestWriter::<std::io::Empty>::open(test_json);
			assert!(manifest.is_ok(), "{}", manifest.err().unwrap());
			assert!(test_tmp_json.exists(), "test.tmp does not exist.");
		}
		assert!(!test_tmp_json.exists(), "test.tmp exists.");
	}

	fn get_manifest<T : Write>(path : &Path) -> ManifestWriter<T> {
		let manifest_res = ManifestWriter::<T>::open(path);
		assert!(manifest_res.is_ok(), "{}", manifest_res.err().unwrap());
		manifest_res.unwrap()
	}

	fn write_something(path : &Path, buf : &[u8]) {
		let mut manifest = get_manifest::<std::io::Empty>(path);
		let write_out = manifest.write(buf);
		assert!(write_out.is_ok(), "{}", write_out.err().unwrap());
	}

	fn assert_file_matches(path : &Path, buf : String) {
		let read = File::open(path);
		assert!(read.is_ok(), "{}", read.err().unwrap());

		let mut reader = read.unwrap();
		let mut out_str = String::new();
		let write = reader.read_to_string(&mut out_str);
		assert!(write.is_ok(), "{}", write.err().unwrap());
		assert_eq!(out_str, buf);
	}

	#[test]
	fn test_write() {
		let path = Path::new("array.json");
		let file = TestFile::create(path);

		self::write_something(path, b"[]");
		self::assert_file_matches(path, String::from("[]"));
	}

	#[test]
	fn test_write_search_seek() {
		let path = Path::new("seek.json");
		let file = TestFile::create(path);

		self::write_something(path, b"[012, \"test\"]");
		{
			let mut manifest = get_manifest::<std::io::Empty>(path);
			
			loop {
				let n = manifest.parse_node();
				assert!(n.is_ok(), "{:?}", n.unwrap_err());

				if n.is_ok_and(|v| v == ManifestNode::EOF) {
					break;
				}
			}
			let seek = manifest.write_search_seek(SeekFrom::Current(-4));
			assert!(seek.is_ok(), "{:?}", seek.unwrap_err());

			let out_res = manifest.write(b"5");
			assert!(out_res.is_ok(), "{:?}", out_res.unwrap_err());
		}

		assert_file_matches(path, String::from("[012, \"te5t\"]"));
	}

	#[test]
	fn test_object_insert() {
		let path = Path::new("object.json");
		let file = TestFile::create(path);
		
		self::write_something(path, br#"
{
	"test": 0,
	"five": null,
	"three": "four"
}"#);
		
		{
			let mut manifest = get_manifest::<std::io::Empty>(path);
			let initialize_result = manifest.initialize();
			assert!(initialize_result.is_ok(), "{:?}", initialize_result.err().unwrap());

			let insert_result = manifest.insert("five".to_string(), serde_json::Value::Array(vec![]));
			assert!(insert_result.is_ok(), "{:?}", insert_result.err().unwrap());

			let flush_result = manifest.flush();
			assert!(flush_result.is_ok(), "{}", flush_result.err().unwrap());
		}
		assert_file_matches(path, String::from(r#"
{
	"test": 0,
	"five":[],
	"three": "four"
}"#));
	}

	#[test]
	fn test_read_array_item() {
		let path = Path::new("array_items.json");
		let file = TestFile::create(path);
		self::write_something(path, br#"
[
	"string test",
	{"id": "b"},
	1243
]"#);

		{
			let mut manifest = get_manifest::<std::io::Empty>(path);
			let initialize_result = manifest.initialize();
			assert!(initialize_result.is_ok(), "{:?}", initialize_result.err().unwrap());

			let expected_items = vec![Value::String("string test".to_string()), serde_json::from_str(r#"{"id": "b"}"#).unwrap(), Value::Number(Number::from(1243))];
			let mut i = 0;
			while let Some(val) = manifest.read_array_item() {
				assert!(val.is_ok(), "{:?}", val.unwrap_err());
				assert_eq!(expected_items[i], val.unwrap());
				i += 1;
			}
		}
	}

	#[test]
	fn test_custom_output() {
		let path = Path::new("custom_output.json");
		let file = TestFile::create(path);
		let out_str =
String::from(r#"
{
	"some": {
		"body": []
	},
	"once": {
		"told me": "the world was gonna roll me"
	},
	"I": [ "a", 1, "nt" ],
	"the": 5,
	"harp3st": {
		"tool": {
			"in": {
				"the": "shed."
			}
		}
	}
}"#);
		self::write_something(path, out_str.as_bytes());
		{
			let out = std::io::Cursor::new(Vec::new());
			let mut manifest = get_manifest::<std::io::Cursor<Vec<u8>>>(path);
			manifest.active_writer = WriteTo::CustomWriter(out);
			loop {
				let n = manifest.parse_node();
				if n.is_ok_and(|v| v == ManifestNode::EOF) {
					break;
				}
			}
			let mut string = String::new();
			match &mut manifest.active_writer {
				WriteTo::CustomWriter(o) => {
					o.set_position(0);
					let res = o.read_to_string(&mut string);
					assert!(res.is_ok(), "{:?}", res.unwrap_err());
				},
				_ => unreachable!()
			}
			assert_eq!(out_str, string);
		}
		// Our output should be blank:
		assert_file_matches(path, String::from(""));
	}
}