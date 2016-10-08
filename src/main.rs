extern crate rustc_serialize;
extern crate docopt;
extern crate byteorder;
extern crate protobuf;
extern crate snap;

use docopt::Docopt;
use std::io::{BufReader, Error, ErrorKind};
use std::fs::File;
use std::process;

use protobuf::{CodedInputStream, ProtobufEnum, parse_from_bytes, ProtobufResult, Message};

mod proto;
use proto::demo::{self, EDemoCommands};

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

    println!("raw format: {:?}", raw_format); 
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


fn read_command(stream: &mut CodedInputStream) {
    let kind = stream.read_int32().unwrap();

    let comp = (kind & EDemoCommands::DEM_IsCompressed as i32) > 0;

    let kind = if comp {
        EDemoCommands::from_i32(kind & !(EDemoCommands::DEM_IsCompressed as i32))
    } else {
        EDemoCommands::from_i32(kind)
    }.unwrap();
    
    let tick = stream.read_uint32().unwrap();
    let size = stream.read_uint32().unwrap();

    let mut message = stream.read_raw_bytes(size).unwrap();

    if comp {
        let mut decoded_message = vec![0; decompress_len(message.as_slice()).unwrap()];
        let mut decoder = Decoder::new();
    
        decoder.decompress(message.as_slice(), &mut decoded_message).unwrap();
        message = decoded_message;
    };

    match kind {
        EDemoCommands::DEM_Error => panic!("Demo error"),
        EDemoCommands::DEM_Stop => process::exit(0),
        EDemoCommands::DEM_FileHeader => println!("{:?}", parse_from_bytes::<demo::CDemoFileHeader>(message.as_slice()).unwrap()),
        EDemoCommands::DEM_FileInfo => println!("{:?}", parse_from_bytes::<demo::CDemoFileInfo>(message.as_slice()).unwrap()),
        EDemoCommands::DEM_SyncTick => println!("{:?}", parse_from_bytes::<demo::CDemoSyncTick>(message.as_slice()).unwrap()),
        EDemoCommands::DEM_SendTables => println!("{:?}", parse_from_bytes::<demo::CDemoSendTables>(message.as_slice()).unwrap()),
        EDemoCommands::DEM_ClassInfo => println!("{:?}", parse_from_bytes::<demo::CDemoClassInfo>(message.as_slice()).unwrap()),
        EDemoCommands::DEM_StringTables => println!("{:?}", parse_from_bytes::<demo::CDemoStringTables>(message.as_slice()).unwrap()),
        EDemoCommands::DEM_Packet => println!("{:?}", parse_from_bytes::<demo::CDemoPacket>(message.as_slice()).unwrap()),
        EDemoCommands::DEM_SignonPacket => println!("Signon"),
        EDemoCommands::DEM_ConsoleCmd => println!("{:?}", parse_from_bytes::<demo::CDemoConsoleCmd>(message.as_slice()).unwrap()),
        EDemoCommands::DEM_CustomData => println!("{:?}", parse_from_bytes::<demo::CDemoCustomData>(message.as_slice()).unwrap()),
        EDemoCommands::DEM_CustomDataCallbacks => println!("CustomDataCallbacks"),
        EDemoCommands::DEM_UserCmd => println!("{:?}", parse_from_bytes::<demo::CDemoUserCmd>(message.as_slice()).unwrap()),
        EDemoCommands::DEM_FullPacket => println!("{:?}", parse_from_bytes::<demo::CDemoFullPacket>(message.as_slice()).unwrap()),
        EDemoCommands::DEM_Max => println!("Dem max"), // What is this?
        _ => ()
    }

    // println!("comp: {:?}, kind: {:?}, tick: {:?}, size: {:?}", comp, kind, tick, size);
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

    // Skip some bytes (s2 format)
    let _b = stream.skip_raw_bytes(4);

    loop {
        read_command(&mut stream);
    }


}
