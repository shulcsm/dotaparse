// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct CDOTAMsg_LocationPing {
    // message fields
    x: ::std::option::Option<i32>,
    y: ::std::option::Option<i32>,
    target: ::std::option::Option<i32>,
    direct_ping: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAMsg_LocationPing {}

impl CDOTAMsg_LocationPing {
    pub fn new() -> CDOTAMsg_LocationPing {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAMsg_LocationPing {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAMsg_LocationPing> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAMsg_LocationPing,
        };
        unsafe {
            instance.get(|| {
                CDOTAMsg_LocationPing {
                    x: ::std::option::Option::None,
                    y: ::std::option::Option::None,
                    target: ::std::option::Option::None,
                    direct_ping: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 x = 1;

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: i32) {
        self.x = ::std::option::Option::Some(v);
    }

    pub fn get_x(&self) -> i32 {
        self.x.unwrap_or(0)
    }

    // optional int32 y = 2;

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: i32) {
        self.y = ::std::option::Option::Some(v);
    }

    pub fn get_y(&self) -> i32 {
        self.y.unwrap_or(0)
    }

    // optional int32 target = 3;

    pub fn clear_target(&mut self) {
        self.target = ::std::option::Option::None;
    }

    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target(&mut self, v: i32) {
        self.target = ::std::option::Option::Some(v);
    }

    pub fn get_target(&self) -> i32 {
        self.target.unwrap_or(0)
    }

    // optional bool direct_ping = 4;

    pub fn clear_direct_ping(&mut self) {
        self.direct_ping = ::std::option::Option::None;
    }

    pub fn has_direct_ping(&self) -> bool {
        self.direct_ping.is_some()
    }

    // Param is passed by value, moved
    pub fn set_direct_ping(&mut self, v: bool) {
        self.direct_ping = ::std::option::Option::Some(v);
    }

    pub fn get_direct_ping(&self) -> bool {
        self.direct_ping.unwrap_or(false)
    }
}

impl ::protobuf::Message for CDOTAMsg_LocationPing {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.y = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.target = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.direct_ping = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.x {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.y {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.target {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.direct_ping.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.y {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.target {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.direct_ping {
            try!(os.write_bool(4, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<CDOTAMsg_LocationPing>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAMsg_LocationPing {
    fn new() -> CDOTAMsg_LocationPing {
        CDOTAMsg_LocationPing::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAMsg_LocationPing>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "x",
                    CDOTAMsg_LocationPing::has_x,
                    CDOTAMsg_LocationPing::get_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "y",
                    CDOTAMsg_LocationPing::has_y,
                    CDOTAMsg_LocationPing::get_y,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "target",
                    CDOTAMsg_LocationPing::has_target,
                    CDOTAMsg_LocationPing::get_target,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "direct_ping",
                    CDOTAMsg_LocationPing::has_direct_ping,
                    CDOTAMsg_LocationPing::get_direct_ping,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAMsg_LocationPing>(
                    "CDOTAMsg_LocationPing",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
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

impl ::std::cmp::PartialEq for CDOTAMsg_LocationPing {
    fn eq(&self, other: &CDOTAMsg_LocationPing) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.target == other.target &&
        self.direct_ping == other.direct_ping &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAMsg_LocationPing {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAMsg_MapLine {
    // message fields
    x: ::std::option::Option<i32>,
    y: ::std::option::Option<i32>,
    initial: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAMsg_MapLine {}

impl CDOTAMsg_MapLine {
    pub fn new() -> CDOTAMsg_MapLine {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAMsg_MapLine {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAMsg_MapLine> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAMsg_MapLine,
        };
        unsafe {
            instance.get(|| {
                CDOTAMsg_MapLine {
                    x: ::std::option::Option::None,
                    y: ::std::option::Option::None,
                    initial: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 x = 1;

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: i32) {
        self.x = ::std::option::Option::Some(v);
    }

    pub fn get_x(&self) -> i32 {
        self.x.unwrap_or(0)
    }

    // optional int32 y = 2;

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: i32) {
        self.y = ::std::option::Option::Some(v);
    }

    pub fn get_y(&self) -> i32 {
        self.y.unwrap_or(0)
    }

    // optional bool initial = 3;

    pub fn clear_initial(&mut self) {
        self.initial = ::std::option::Option::None;
    }

    pub fn has_initial(&self) -> bool {
        self.initial.is_some()
    }

    // Param is passed by value, moved
    pub fn set_initial(&mut self, v: bool) {
        self.initial = ::std::option::Option::Some(v);
    }

    pub fn get_initial(&self) -> bool {
        self.initial.unwrap_or(false)
    }
}

impl ::protobuf::Message for CDOTAMsg_MapLine {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.y = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.initial = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.x {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.y {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.initial.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.y {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.initial {
            try!(os.write_bool(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<CDOTAMsg_MapLine>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAMsg_MapLine {
    fn new() -> CDOTAMsg_MapLine {
        CDOTAMsg_MapLine::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAMsg_MapLine>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "x",
                    CDOTAMsg_MapLine::has_x,
                    CDOTAMsg_MapLine::get_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "y",
                    CDOTAMsg_MapLine::has_y,
                    CDOTAMsg_MapLine::get_y,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "initial",
                    CDOTAMsg_MapLine::has_initial,
                    CDOTAMsg_MapLine::get_initial,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAMsg_MapLine>(
                    "CDOTAMsg_MapLine",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
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

impl ::std::cmp::PartialEq for CDOTAMsg_MapLine {
    fn eq(&self, other: &CDOTAMsg_MapLine) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.initial == other.initial &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAMsg_MapLine {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

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
    0x05, 0x48, 0x01, 0x80, 0x01, 0x00,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
