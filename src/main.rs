extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;

use docopt::Docopt;

use std::path::Path;
use std::io::{Read, ReadExt, BufReader, BufStream};
use std::fs::File;


// Write the Docopt usage string.
static USAGE: &'static str = "
Usage: dotaparse <file>
";

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_file: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);


	let mut f = File::open(&args.arg_file).unwrap();
	let mut stream = BufStream::new(f);
	let mut reader = BufReader::new(stream);
	let mut string = String::new();
	reader.take(8).read_to_string(&mut string);
	println!("{:?}", string);
    println!("Hello, world!")
}
