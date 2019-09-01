pub mod builtins;
pub mod context;
pub mod data;
pub mod functions;
pub mod interpreter;
pub mod pipeline;


use std::{
	fs::File,
	io,
};


pub fn prelude() -> Result<Box<File>, io::Error> {
	let prelude = File::open("kr/src/prelude.kr")?;
	Ok(Box::new(prelude))
}
