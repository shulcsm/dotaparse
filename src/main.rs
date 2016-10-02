extern crate rustc_serialize;
extern crate docopt;
extern crate byteorder;
extern crate protobuf;
extern crate snap;

use docopt::Docopt;
use std::io::{BufReader, Error, ErrorKind};
use std::fs::File;
use protobuf::{CodedInputStream, parse_from_bytes};

mod proto;
use proto::demo::{EDemoCommands, CDemoFileInfo};

use snap::{Decoder, decompress_len};

static USAGE: &'static str = "
Usage: dotaparse <file>
";

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_file: String,
}

#[derive(Debug)]
enum DemoFormat {
    Source1,
    Source2
}

#[derive(Debug)]
struct FileHeader {
    pub format: DemoFormat,
    pub file_info_offset: u32,
}

impl FileHeader {
    pub fn new(format: DemoFormat, file_info_offset: u32) -> FileHeader {
        FileHeader { format: format, file_info_offset: file_info_offset }
    }
}

fn read_header(stream: &mut CodedInputStream) -> Result<(FileHeader), Error> {
    let raw_format = String::from_utf8(stream.read_raw_bytes(8).unwrap()).unwrap();
    
    let format = match raw_format.as_ref() {
        "PBUFDEM\0" => Some(DemoFormat::Source1),
        "PBDEMS2\0" => Some(DemoFormat::Source2),
        _ => None
    };

    let file_info_offset = stream.read_fixed32().unwrap();
        
    match format {
        Some(demo_format) => Ok(FileHeader::new(demo_format, file_info_offset)),
        None => Err(Error::new(ErrorKind::Other, format!("Unknown format: {}", raw_format)))
    }
}


fn main() {
    let args: Args = Docopt::new(USAGE)
                        .and_then(|d| d.decode())
                        .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);

    let f = File::open(&args.arg_file).unwrap();
    let mut f = BufReader::new(f);
  
    let mut stream = CodedInputStream::from_buffered_reader(&mut f);
    let header = read_header(&mut stream).unwrap();
    println!("{:?}", header);

    
    let pos = stream.pos();
    stream.skip_raw_bytes(header.file_info_offset - pos).unwrap();
    
    let mut kind = stream.read_raw_varint64().unwrap() as u64;
    
    let comp = (kind & EDemoCommands::DEM_IsCompressed as u64) > 0;

    if comp {
        kind = kind & !(EDemoCommands::DEM_IsCompressed as u64);
    }
        
    let tick = stream.read_raw_varint64().unwrap();
    let size = stream.read_raw_varint64().unwrap() as u32;

    println!("comp: {:?}, kind: {:?}, tick: {:?}, size: {:?}", comp, kind, tick, size);

    let message = stream.read_raw_bytes(size).unwrap();

    let mut decoded_message = vec![0; decompress_len(message.as_slice()).unwrap()];
    let mut decoder = Decoder::new();
    
    decoder.decompress(message.as_slice(), &mut decoded_message).unwrap();
    let file_info = parse_from_bytes::<CDemoFileInfo>(decoded_message.as_slice()).unwrap();

    println!("Playback time: {}", file_info.get_playback_time());
}
