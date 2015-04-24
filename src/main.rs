extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use std::path::Path;
use std::io::{Read, BufReader, BufStream};
use std::fs::File;


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

	let f = File::open(&args.arg_file).unwrap();
	let stream = BufStream::new(f);
	let reader = BufReader::new(stream);

	let mut header = String::new();
	reader.take(8).read_to_string(&mut header);
	assert!("PBUFDEM\0" == header, "Doesn't look like a scrds demo file.");

	println!("{:?}", header);
}
