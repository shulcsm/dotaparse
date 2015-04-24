extern crate rustc_serialize;
extern crate docopt;
extern crate byteorder;

use docopt::Docopt;
use std::io::{Read, Seek, SeekFrom};
use std::fs::File;
use byteorder::{ReadBytesExt, LittleEndian};

use proto::demo;

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

	let mut reader = File::open(&args.arg_file).unwrap();

  // This seems wrong
	let mut header_buf = [0; 8];
  reader.read(&mut header_buf);
  let header = String::from_utf8_lossy(&mut header_buf);

  let game_info_offset = reader.read_u32::<LittleEndian>().unwrap();
  reader.seek(SeekFrom::Start(game_info_offset as u64));

  println!("{:?}", header);
	assert!("PBUFDEM\0" == header, "Doesn't look like a scrds demo file.");
  println!("{:?}", game_info_offset);

}
