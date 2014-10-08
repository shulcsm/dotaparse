// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]

use descriptor::*;
use netmessages::*;

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x19, 0x64, 0x6f, 0x74, 0x61, 0x5f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x20, 0x67, 0x6f, 0x6f,
    0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x64, 0x65, 0x73,
    0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x11, 0x6e,
    0x65, 0x74, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x22, 0x52, 0x0a, 0x15, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x4d, 0x73, 0x67, 0x5f, 0x4c, 0x6f, 0x63,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x69, 0x6e, 0x67, 0x12, 0x09, 0x0a, 0x01, 0x78, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x05, 0x12, 0x09, 0x0a, 0x01, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12,
    0x0e, 0x0a, 0x06, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x12,
    0x13, 0x0a, 0x0b, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x5f, 0x70, 0x69, 0x6e, 0x67, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x08, 0x22, 0x39, 0x0a, 0x10, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x4d, 0x73, 0x67,
    0x5f, 0x4d, 0x61, 0x70, 0x4c, 0x69, 0x6e, 0x65, 0x12, 0x09, 0x0a, 0x01, 0x78, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x05, 0x12, 0x09, 0x0a, 0x01, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0f,
    0x0a, 0x07, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x61, 0x6c, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x42,
    0x05, 0x48, 0x01, 0x80, 0x01, 0x00, 0x4a, 0xd0, 0x06, 0x0a, 0x06, 0x12, 0x04, 0x1e, 0x00, 0x50,
    0x01, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x1e, 0x00, 0x1c, 0x0a, 0x37, 0x0a, 0x04, 0x08,
    0xe7, 0x07, 0x00, 0x12, 0x03, 0x1e, 0x00, 0x1c, 0x1a, 0x2a, 0x20, 0x57, 0x65, 0x20, 0x63, 0x61,
    0x72, 0x65, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x73, 0x70,
    0x65, 0x65, 0x64, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x20, 0x63, 0x6f, 0x64, 0x65, 0x20, 0x73, 0x69,
    0x7a, 0x65, 0x0d, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x1e,
    0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1e, 0x07,
    0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x07,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x1e, 0x16, 0x1b, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x21, 0x00, 0x23, 0x0a, 0x41, 0x0a, 0x04, 0x08, 0xe7, 0x07,
    0x01, 0x12, 0x03, 0x21, 0x00, 0x23, 0x1a, 0x34, 0x20, 0x57, 0x65, 0x20, 0x64, 0x6f, 0x6e, 0x27,
    0x74, 0x20, 0x75, 0x73, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63,
    0x65, 0x20, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x75, 0x6e,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x0d, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x21, 0x07, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7,
    0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x21, 0x07, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x21, 0x07, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07,
    0x01, 0x03, 0x12, 0x03, 0x21, 0x1d, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x3a,
    0x07, 0x29, 0x0a, 0x22, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x3d, 0x07, 0x1a, 0x1a, 0x17, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x2c, 0x20,
    0x65, 0x74, 0x63, 0x2e, 0x0d, 0x0a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x43, 0x00,
    0x49, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x43, 0x08, 0x1d, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x45, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x45, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x45, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x45, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x45, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x46, 0x08, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x46, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x46, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x46, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x46, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02,
    0x12, 0x03, 0x47, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x47, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x47, 0x11,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x47, 0x17, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x47, 0x20, 0x21, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x48, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x48, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x48, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x48, 0x16, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x48,
    0x24, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x4b, 0x00, 0x50, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x4b, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x00, 0x12, 0x03, 0x4d, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x4d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x4d, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4d, 0x17,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4d, 0x1b, 0x1c, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x4e, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x4e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x4e, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x4e, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x4e, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x4f, 0x08,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x4f, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x4f, 0x11, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4f, 0x16, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x4f, 0x20, 0x21,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto };

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data)
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct CDOTAMsg_LocationPing {
    x: Option<i32>,
    y: Option<i32>,
    target: Option<i32>,
    direct_ping: Option<bool>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> CDOTAMsg_LocationPing {
    pub fn new() -> CDOTAMsg_LocationPing {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAMsg_LocationPing {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAMsg_LocationPing> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const CDOTAMsg_LocationPing };
        unsafe {
            instance.get(|| {
                CDOTAMsg_LocationPing {
                    x: None,
                    y: None,
                    target: None,
                    direct_ping: None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    pub fn clear_x(&mut self) {
        self.x = None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: i32) {
        self.x = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_x(&'a mut self) -> &'a mut i32 {
        if self.x.is_none() {
            self.x = Some(0);
        };
        self.x.as_mut().unwrap()
    }

    pub fn get_x(&self) -> i32 {
        self.x.unwrap_or_else(|| 0)
    }

    pub fn clear_y(&mut self) {
        self.y = None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: i32) {
        self.y = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_y(&'a mut self) -> &'a mut i32 {
        if self.y.is_none() {
            self.y = Some(0);
        };
        self.y.as_mut().unwrap()
    }

    pub fn get_y(&self) -> i32 {
        self.y.unwrap_or_else(|| 0)
    }

    pub fn clear_target(&mut self) {
        self.target = None;
    }

    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target(&mut self, v: i32) {
        self.target = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_target(&'a mut self) -> &'a mut i32 {
        if self.target.is_none() {
            self.target = Some(0);
        };
        self.target.as_mut().unwrap()
    }

    pub fn get_target(&self) -> i32 {
        self.target.unwrap_or_else(|| 0)
    }

    pub fn clear_direct_ping(&mut self) {
        self.direct_ping = None;
    }

    pub fn has_direct_ping(&self) -> bool {
        self.direct_ping.is_some()
    }

    // Param is passed by value, moved
    pub fn set_direct_ping(&mut self, v: bool) {
        self.direct_ping = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_direct_ping(&'a mut self) -> &'a mut bool {
        if self.direct_ping.is_none() {
            self.direct_ping = Some(false);
        };
        self.direct_ping.as_mut().unwrap()
    }

    pub fn get_direct_ping(&self) -> bool {
        self.direct_ping.unwrap_or_else(|| false)
    }
}

impl ::protobuf::Message for CDOTAMsg_LocationPing {
    fn new() -> CDOTAMsg_LocationPing {
        CDOTAMsg_LocationPing::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(::protobuf::wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int32();
                    self.x = Some(tmp);
                },
                2 => {
                    assert_eq!(::protobuf::wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int32();
                    self.y = Some(tmp);
                },
                3 => {
                    assert_eq!(::protobuf::wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int32();
                    self.target = Some(tmp);
                },
                4 => {
                    assert_eq!(::protobuf::wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_bool();
                    self.direct_ping = Some(tmp);
                },
                _ => {
                    let unknown = is.read_unknown(wire_type);
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.x.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.y.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.target.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.direct_ping.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        *sizes.get_mut(pos) = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variable)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        use protobuf::{Message};
        match self.x {
            Some(ref v) => {
                os.write_int32(1, *v);
            },
            None => {},
        };
        match self.y {
            Some(ref v) => {
                os.write_int32(2, *v);
            },
            None => {},
        };
        match self.target {
            Some(ref v) => {
                os.write_int32(3, *v);
            },
            None => {},
        };
        match self.direct_ping {
            Some(ref v) => {
                os.write_bool(4, *v);
            },
            None => {},
        };
        os.write_unknown_fields(self.get_unknown_fields());
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: Option<CDOTAMsg_LocationPing>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::MessageDescriptor };
        unsafe {
            descriptor.get(|| {
                let mut fields: Vec<&'static ::protobuf::reflect::FieldAccessor<CDOTAMsg_LocationPing>> = Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&CDOTAMsg_LocationPing_x_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAMsg_LocationPing>) });
                fields.push(unsafe { ::std::mem::transmute(&CDOTAMsg_LocationPing_y_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAMsg_LocationPing>) });
                fields.push(unsafe { ::std::mem::transmute(&CDOTAMsg_LocationPing_target_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAMsg_LocationPing>) });
                fields.push(unsafe { ::std::mem::transmute(&CDOTAMsg_LocationPing_direct_ping_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAMsg_LocationPing>) });
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAMsg_LocationPing>(
                    "CDOTAMsg_LocationPing",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<CDOTAMsg_LocationPing>()
    }
}

impl ::protobuf::Clear for CDOTAMsg_LocationPing {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.clear_target();
        self.clear_direct_ping();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for CDOTAMsg_LocationPing {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct CDOTAMsg_LocationPing_x_acc_type;
static CDOTAMsg_LocationPing_x_acc: CDOTAMsg_LocationPing_x_acc_type = CDOTAMsg_LocationPing_x_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAMsg_LocationPing> for CDOTAMsg_LocationPing_x_acc_type {
    fn name(&self) -> &'static str {
        "x"
    }

    fn has_field(&self, m: &CDOTAMsg_LocationPing) -> bool {
        m.has_x()
    }

    fn get_i32(&self, m: &CDOTAMsg_LocationPing) -> i32 {
        m.get_x()
    }
}

#[allow(non_camel_case_types)]
struct CDOTAMsg_LocationPing_y_acc_type;
static CDOTAMsg_LocationPing_y_acc: CDOTAMsg_LocationPing_y_acc_type = CDOTAMsg_LocationPing_y_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAMsg_LocationPing> for CDOTAMsg_LocationPing_y_acc_type {
    fn name(&self) -> &'static str {
        "y"
    }

    fn has_field(&self, m: &CDOTAMsg_LocationPing) -> bool {
        m.has_y()
    }

    fn get_i32(&self, m: &CDOTAMsg_LocationPing) -> i32 {
        m.get_y()
    }
}

#[allow(non_camel_case_types)]
struct CDOTAMsg_LocationPing_target_acc_type;
static CDOTAMsg_LocationPing_target_acc: CDOTAMsg_LocationPing_target_acc_type = CDOTAMsg_LocationPing_target_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAMsg_LocationPing> for CDOTAMsg_LocationPing_target_acc_type {
    fn name(&self) -> &'static str {
        "target"
    }

    fn has_field(&self, m: &CDOTAMsg_LocationPing) -> bool {
        m.has_target()
    }

    fn get_i32(&self, m: &CDOTAMsg_LocationPing) -> i32 {
        m.get_target()
    }
}

#[allow(non_camel_case_types)]
struct CDOTAMsg_LocationPing_direct_ping_acc_type;
static CDOTAMsg_LocationPing_direct_ping_acc: CDOTAMsg_LocationPing_direct_ping_acc_type = CDOTAMsg_LocationPing_direct_ping_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAMsg_LocationPing> for CDOTAMsg_LocationPing_direct_ping_acc_type {
    fn name(&self) -> &'static str {
        "direct_ping"
    }

    fn has_field(&self, m: &CDOTAMsg_LocationPing) -> bool {
        m.has_direct_ping()
    }

    fn get_bool(&self, m: &CDOTAMsg_LocationPing) -> bool {
        m.get_direct_ping()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct CDOTAMsg_MapLine {
    x: Option<i32>,
    y: Option<i32>,
    initial: Option<bool>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> CDOTAMsg_MapLine {
    pub fn new() -> CDOTAMsg_MapLine {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAMsg_MapLine {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAMsg_MapLine> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const CDOTAMsg_MapLine };
        unsafe {
            instance.get(|| {
                CDOTAMsg_MapLine {
                    x: None,
                    y: None,
                    initial: None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    pub fn clear_x(&mut self) {
        self.x = None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: i32) {
        self.x = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_x(&'a mut self) -> &'a mut i32 {
        if self.x.is_none() {
            self.x = Some(0);
        };
        self.x.as_mut().unwrap()
    }

    pub fn get_x(&self) -> i32 {
        self.x.unwrap_or_else(|| 0)
    }

    pub fn clear_y(&mut self) {
        self.y = None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: i32) {
        self.y = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_y(&'a mut self) -> &'a mut i32 {
        if self.y.is_none() {
            self.y = Some(0);
        };
        self.y.as_mut().unwrap()
    }

    pub fn get_y(&self) -> i32 {
        self.y.unwrap_or_else(|| 0)
    }

    pub fn clear_initial(&mut self) {
        self.initial = None;
    }

    pub fn has_initial(&self) -> bool {
        self.initial.is_some()
    }

    // Param is passed by value, moved
    pub fn set_initial(&mut self, v: bool) {
        self.initial = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_initial(&'a mut self) -> &'a mut bool {
        if self.initial.is_none() {
            self.initial = Some(false);
        };
        self.initial.as_mut().unwrap()
    }

    pub fn get_initial(&self) -> bool {
        self.initial.unwrap_or_else(|| false)
    }
}

impl ::protobuf::Message for CDOTAMsg_MapLine {
    fn new() -> CDOTAMsg_MapLine {
        CDOTAMsg_MapLine::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(::protobuf::wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int32();
                    self.x = Some(tmp);
                },
                2 => {
                    assert_eq!(::protobuf::wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int32();
                    self.y = Some(tmp);
                },
                3 => {
                    assert_eq!(::protobuf::wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_bool();
                    self.initial = Some(tmp);
                },
                _ => {
                    let unknown = is.read_unknown(wire_type);
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.x.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.y.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.initial.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        *sizes.get_mut(pos) = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variable)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        use protobuf::{Message};
        match self.x {
            Some(ref v) => {
                os.write_int32(1, *v);
            },
            None => {},
        };
        match self.y {
            Some(ref v) => {
                os.write_int32(2, *v);
            },
            None => {},
        };
        match self.initial {
            Some(ref v) => {
                os.write_bool(3, *v);
            },
            None => {},
        };
        os.write_unknown_fields(self.get_unknown_fields());
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: Option<CDOTAMsg_MapLine>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::MessageDescriptor };
        unsafe {
            descriptor.get(|| {
                let mut fields: Vec<&'static ::protobuf::reflect::FieldAccessor<CDOTAMsg_MapLine>> = Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&CDOTAMsg_MapLine_x_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAMsg_MapLine>) });
                fields.push(unsafe { ::std::mem::transmute(&CDOTAMsg_MapLine_y_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAMsg_MapLine>) });
                fields.push(unsafe { ::std::mem::transmute(&CDOTAMsg_MapLine_initial_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAMsg_MapLine>) });
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAMsg_MapLine>(
                    "CDOTAMsg_MapLine",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<CDOTAMsg_MapLine>()
    }
}

impl ::protobuf::Clear for CDOTAMsg_MapLine {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.clear_initial();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for CDOTAMsg_MapLine {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct CDOTAMsg_MapLine_x_acc_type;
static CDOTAMsg_MapLine_x_acc: CDOTAMsg_MapLine_x_acc_type = CDOTAMsg_MapLine_x_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAMsg_MapLine> for CDOTAMsg_MapLine_x_acc_type {
    fn name(&self) -> &'static str {
        "x"
    }

    fn has_field(&self, m: &CDOTAMsg_MapLine) -> bool {
        m.has_x()
    }

    fn get_i32(&self, m: &CDOTAMsg_MapLine) -> i32 {
        m.get_x()
    }
}

#[allow(non_camel_case_types)]
struct CDOTAMsg_MapLine_y_acc_type;
static CDOTAMsg_MapLine_y_acc: CDOTAMsg_MapLine_y_acc_type = CDOTAMsg_MapLine_y_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAMsg_MapLine> for CDOTAMsg_MapLine_y_acc_type {
    fn name(&self) -> &'static str {
        "y"
    }

    fn has_field(&self, m: &CDOTAMsg_MapLine) -> bool {
        m.has_y()
    }

    fn get_i32(&self, m: &CDOTAMsg_MapLine) -> i32 {
        m.get_y()
    }
}

#[allow(non_camel_case_types)]
struct CDOTAMsg_MapLine_initial_acc_type;
static CDOTAMsg_MapLine_initial_acc: CDOTAMsg_MapLine_initial_acc_type = CDOTAMsg_MapLine_initial_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAMsg_MapLine> for CDOTAMsg_MapLine_initial_acc_type {
    fn name(&self) -> &'static str {
        "initial"
    }

    fn has_field(&self, m: &CDOTAMsg_MapLine) -> bool {
        m.has_initial()
    }

    fn get_bool(&self, m: &CDOTAMsg_MapLine) -> bool {
        m.get_initial()
    }
}
