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
pub struct CDOTAUserMsg_AIDebugLine {
    // message fields
    message: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_AIDebugLine {}

impl CDOTAUserMsg_AIDebugLine {
    pub fn new() -> CDOTAUserMsg_AIDebugLine {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_AIDebugLine {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_AIDebugLine> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_AIDebugLine,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_AIDebugLine {
                    message: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string message = 1;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        if self.message.is_none() {
            self.message.set_default();
        };
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        self.message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        match self.message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for CDOTAUserMsg_AIDebugLine {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.message));
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
        for value in &self.message {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.message.as_ref() {
            try!(os.write_string(1, &v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_AIDebugLine>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_AIDebugLine {
    fn new() -> CDOTAUserMsg_AIDebugLine {
        CDOTAUserMsg_AIDebugLine::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_AIDebugLine>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "message",
                    CDOTAUserMsg_AIDebugLine::has_message,
                    CDOTAUserMsg_AIDebugLine::get_message,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_AIDebugLine>(
                    "CDOTAUserMsg_AIDebugLine",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_AIDebugLine {
    fn clear(&mut self) {
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_AIDebugLine {
    fn eq(&self, other: &CDOTAUserMsg_AIDebugLine) -> bool {
        self.message == other.message &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_AIDebugLine {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_Ping {
    // message fields
    message: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_Ping {}

impl CDOTAUserMsg_Ping {
    pub fn new() -> CDOTAUserMsg_Ping {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_Ping {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_Ping> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_Ping,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_Ping {
                    message: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string message = 1;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        if self.message.is_none() {
            self.message.set_default();
        };
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        self.message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        match self.message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for CDOTAUserMsg_Ping {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.message));
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
        for value in &self.message {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.message.as_ref() {
            try!(os.write_string(1, &v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_Ping>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_Ping {
    fn new() -> CDOTAUserMsg_Ping {
        CDOTAUserMsg_Ping::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_Ping>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "message",
                    CDOTAUserMsg_Ping::has_message,
                    CDOTAUserMsg_Ping::get_message,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_Ping>(
                    "CDOTAUserMsg_Ping",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_Ping {
    fn clear(&mut self) {
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_Ping {
    fn eq(&self, other: &CDOTAUserMsg_Ping) -> bool {
        self.message == other.message &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_Ping {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_ChatEvent {
    // message fields
    field_type: ::std::option::Option<DOTA_CHAT_MESSAGE>,
    value: ::std::option::Option<u32>,
    playerid_1: ::std::option::Option<i32>,
    playerid_2: ::std::option::Option<i32>,
    playerid_3: ::std::option::Option<i32>,
    playerid_4: ::std::option::Option<i32>,
    playerid_5: ::std::option::Option<i32>,
    playerid_6: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_ChatEvent {}

impl CDOTAUserMsg_ChatEvent {
    pub fn new() -> CDOTAUserMsg_ChatEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_ChatEvent {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_ChatEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_ChatEvent,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_ChatEvent {
                    field_type: ::std::option::Option::None,
                    value: ::std::option::Option::None,
                    playerid_1: ::std::option::Option::None,
                    playerid_2: ::std::option::Option::None,
                    playerid_3: ::std::option::Option::None,
                    playerid_4: ::std::option::Option::None,
                    playerid_5: ::std::option::Option::None,
                    playerid_6: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .DOTA_CHAT_MESSAGE type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: DOTA_CHAT_MESSAGE) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> DOTA_CHAT_MESSAGE {
        self.field_type.unwrap_or(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_INVALID)
    }

    // optional uint32 value = 2;

    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: u32) {
        self.value = ::std::option::Option::Some(v);
    }

    pub fn get_value(&self) -> u32 {
        self.value.unwrap_or(0)
    }

    // optional sint32 playerid_1 = 3;

    pub fn clear_playerid_1(&mut self) {
        self.playerid_1 = ::std::option::Option::None;
    }

    pub fn has_playerid_1(&self) -> bool {
        self.playerid_1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_playerid_1(&mut self, v: i32) {
        self.playerid_1 = ::std::option::Option::Some(v);
    }

    pub fn get_playerid_1(&self) -> i32 {
        self.playerid_1.unwrap_or(-1i32)
    }

    // optional sint32 playerid_2 = 4;

    pub fn clear_playerid_2(&mut self) {
        self.playerid_2 = ::std::option::Option::None;
    }

    pub fn has_playerid_2(&self) -> bool {
        self.playerid_2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_playerid_2(&mut self, v: i32) {
        self.playerid_2 = ::std::option::Option::Some(v);
    }

    pub fn get_playerid_2(&self) -> i32 {
        self.playerid_2.unwrap_or(-1i32)
    }

    // optional sint32 playerid_3 = 5;

    pub fn clear_playerid_3(&mut self) {
        self.playerid_3 = ::std::option::Option::None;
    }

    pub fn has_playerid_3(&self) -> bool {
        self.playerid_3.is_some()
    }

    // Param is passed by value, moved
    pub fn set_playerid_3(&mut self, v: i32) {
        self.playerid_3 = ::std::option::Option::Some(v);
    }

    pub fn get_playerid_3(&self) -> i32 {
        self.playerid_3.unwrap_or(-1i32)
    }

    // optional sint32 playerid_4 = 6;

    pub fn clear_playerid_4(&mut self) {
        self.playerid_4 = ::std::option::Option::None;
    }

    pub fn has_playerid_4(&self) -> bool {
        self.playerid_4.is_some()
    }

    // Param is passed by value, moved
    pub fn set_playerid_4(&mut self, v: i32) {
        self.playerid_4 = ::std::option::Option::Some(v);
    }

    pub fn get_playerid_4(&self) -> i32 {
        self.playerid_4.unwrap_or(-1i32)
    }

    // optional sint32 playerid_5 = 7;

    pub fn clear_playerid_5(&mut self) {
        self.playerid_5 = ::std::option::Option::None;
    }

    pub fn has_playerid_5(&self) -> bool {
        self.playerid_5.is_some()
    }

    // Param is passed by value, moved
    pub fn set_playerid_5(&mut self, v: i32) {
        self.playerid_5 = ::std::option::Option::Some(v);
    }

    pub fn get_playerid_5(&self) -> i32 {
        self.playerid_5.unwrap_or(-1i32)
    }

    // optional sint32 playerid_6 = 8;

    pub fn clear_playerid_6(&mut self) {
        self.playerid_6 = ::std::option::Option::None;
    }

    pub fn has_playerid_6(&self) -> bool {
        self.playerid_6.is_some()
    }

    // Param is passed by value, moved
    pub fn set_playerid_6(&mut self, v: i32) {
        self.playerid_6 = ::std::option::Option::Some(v);
    }

    pub fn get_playerid_6(&self) -> i32 {
        self.playerid_6.unwrap_or(-1i32)
    }
}

impl ::protobuf::Message for CDOTAUserMsg_ChatEvent {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        };
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
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.value = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint32());
                    self.playerid_1 = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint32());
                    self.playerid_2 = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint32());
                    self.playerid_3 = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint32());
                    self.playerid_4 = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint32());
                    self.playerid_5 = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint32());
                    self.playerid_6 = ::std::option::Option::Some(tmp);
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
        for value in &self.field_type {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.value {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.playerid_1 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, *value);
        };
        for value in &self.playerid_2 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(4, *value);
        };
        for value in &self.playerid_3 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(5, *value);
        };
        for value in &self.playerid_4 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(6, *value);
        };
        for value in &self.playerid_5 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(7, *value);
        };
        for value in &self.playerid_6 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(8, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.value {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.playerid_1 {
            try!(os.write_sint32(3, v));
        };
        if let Some(v) = self.playerid_2 {
            try!(os.write_sint32(4, v));
        };
        if let Some(v) = self.playerid_3 {
            try!(os.write_sint32(5, v));
        };
        if let Some(v) = self.playerid_4 {
            try!(os.write_sint32(6, v));
        };
        if let Some(v) = self.playerid_5 {
            try!(os.write_sint32(7, v));
        };
        if let Some(v) = self.playerid_6 {
            try!(os.write_sint32(8, v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_ChatEvent>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_ChatEvent {
    fn new() -> CDOTAUserMsg_ChatEvent {
        CDOTAUserMsg_ChatEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_ChatEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    CDOTAUserMsg_ChatEvent::has_field_type,
                    CDOTAUserMsg_ChatEvent::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "value",
                    CDOTAUserMsg_ChatEvent::has_value,
                    CDOTAUserMsg_ChatEvent::get_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "playerid_1",
                    CDOTAUserMsg_ChatEvent::has_playerid_1,
                    CDOTAUserMsg_ChatEvent::get_playerid_1,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "playerid_2",
                    CDOTAUserMsg_ChatEvent::has_playerid_2,
                    CDOTAUserMsg_ChatEvent::get_playerid_2,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "playerid_3",
                    CDOTAUserMsg_ChatEvent::has_playerid_3,
                    CDOTAUserMsg_ChatEvent::get_playerid_3,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "playerid_4",
                    CDOTAUserMsg_ChatEvent::has_playerid_4,
                    CDOTAUserMsg_ChatEvent::get_playerid_4,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "playerid_5",
                    CDOTAUserMsg_ChatEvent::has_playerid_5,
                    CDOTAUserMsg_ChatEvent::get_playerid_5,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "playerid_6",
                    CDOTAUserMsg_ChatEvent::has_playerid_6,
                    CDOTAUserMsg_ChatEvent::get_playerid_6,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_ChatEvent>(
                    "CDOTAUserMsg_ChatEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_ChatEvent {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_value();
        self.clear_playerid_1();
        self.clear_playerid_2();
        self.clear_playerid_3();
        self.clear_playerid_4();
        self.clear_playerid_5();
        self.clear_playerid_6();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_ChatEvent {
    fn eq(&self, other: &CDOTAUserMsg_ChatEvent) -> bool {
        self.field_type == other.field_type &&
        self.value == other.value &&
        self.playerid_1 == other.playerid_1 &&
        self.playerid_2 == other.playerid_2 &&
        self.playerid_3 == other.playerid_3 &&
        self.playerid_4 == other.playerid_4 &&
        self.playerid_5 == other.playerid_5 &&
        self.playerid_6 == other.playerid_6 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_ChatEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_CombatLogData {
    // message fields
    field_type: ::std::option::Option<u32>,
    target_name: ::std::option::Option<u32>,
    attacker_name: ::std::option::Option<u32>,
    attacker_illusion: ::std::option::Option<bool>,
    target_illusion: ::std::option::Option<bool>,
    inflictor_name: ::std::option::Option<u32>,
    value: ::std::option::Option<i32>,
    health: ::std::option::Option<i32>,
    time: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_CombatLogData {}

impl CDOTAUserMsg_CombatLogData {
    pub fn new() -> CDOTAUserMsg_CombatLogData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_CombatLogData {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_CombatLogData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_CombatLogData,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_CombatLogData {
                    field_type: ::std::option::Option::None,
                    target_name: ::std::option::Option::None,
                    attacker_name: ::std::option::Option::None,
                    attacker_illusion: ::std::option::Option::None,
                    target_illusion: ::std::option::Option::None,
                    inflictor_name: ::std::option::Option::None,
                    value: ::std::option::Option::None,
                    health: ::std::option::Option::None,
                    time: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: u32) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> u32 {
        self.field_type.unwrap_or(0)
    }

    // optional uint32 target_name = 2;

    pub fn clear_target_name(&mut self) {
        self.target_name = ::std::option::Option::None;
    }

    pub fn has_target_name(&self) -> bool {
        self.target_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target_name(&mut self, v: u32) {
        self.target_name = ::std::option::Option::Some(v);
    }

    pub fn get_target_name(&self) -> u32 {
        self.target_name.unwrap_or(0)
    }

    // optional uint32 attacker_name = 3;

    pub fn clear_attacker_name(&mut self) {
        self.attacker_name = ::std::option::Option::None;
    }

    pub fn has_attacker_name(&self) -> bool {
        self.attacker_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attacker_name(&mut self, v: u32) {
        self.attacker_name = ::std::option::Option::Some(v);
    }

    pub fn get_attacker_name(&self) -> u32 {
        self.attacker_name.unwrap_or(0)
    }

    // optional bool attacker_illusion = 4;

    pub fn clear_attacker_illusion(&mut self) {
        self.attacker_illusion = ::std::option::Option::None;
    }

    pub fn has_attacker_illusion(&self) -> bool {
        self.attacker_illusion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attacker_illusion(&mut self, v: bool) {
        self.attacker_illusion = ::std::option::Option::Some(v);
    }

    pub fn get_attacker_illusion(&self) -> bool {
        self.attacker_illusion.unwrap_or(false)
    }

    // optional bool target_illusion = 5;

    pub fn clear_target_illusion(&mut self) {
        self.target_illusion = ::std::option::Option::None;
    }

    pub fn has_target_illusion(&self) -> bool {
        self.target_illusion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target_illusion(&mut self, v: bool) {
        self.target_illusion = ::std::option::Option::Some(v);
    }

    pub fn get_target_illusion(&self) -> bool {
        self.target_illusion.unwrap_or(false)
    }

    // optional uint32 inflictor_name = 6;

    pub fn clear_inflictor_name(&mut self) {
        self.inflictor_name = ::std::option::Option::None;
    }

    pub fn has_inflictor_name(&self) -> bool {
        self.inflictor_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inflictor_name(&mut self, v: u32) {
        self.inflictor_name = ::std::option::Option::Some(v);
    }

    pub fn get_inflictor_name(&self) -> u32 {
        self.inflictor_name.unwrap_or(0)
    }

    // optional int32 value = 7;

    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: i32) {
        self.value = ::std::option::Option::Some(v);
    }

    pub fn get_value(&self) -> i32 {
        self.value.unwrap_or(0)
    }

    // optional int32 health = 8;

    pub fn clear_health(&mut self) {
        self.health = ::std::option::Option::None;
    }

    pub fn has_health(&self) -> bool {
        self.health.is_some()
    }

    // Param is passed by value, moved
    pub fn set_health(&mut self, v: i32) {
        self.health = ::std::option::Option::Some(v);
    }

    pub fn get_health(&self) -> i32 {
        self.health.unwrap_or(0)
    }

    // optional float time = 9;

    pub fn clear_time(&mut self) {
        self.time = ::std::option::Option::None;
    }

    pub fn has_time(&self) -> bool {
        self.time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: f32) {
        self.time = ::std::option::Option::Some(v);
    }

    pub fn get_time(&self) -> f32 {
        self.time.unwrap_or(0.)
    }
}

impl ::protobuf::Message for CDOTAUserMsg_CombatLogData {
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
                    let tmp = try!(is.read_uint32());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.target_name = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.attacker_name = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.attacker_illusion = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.target_illusion = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.inflictor_name = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.value = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.health = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.time = ::std::option::Option::Some(tmp);
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
        for value in &self.field_type {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.target_name {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.attacker_name {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.attacker_illusion.is_some() {
            my_size += 2;
        };
        if self.target_illusion.is_some() {
            my_size += 2;
        };
        for value in &self.inflictor_name {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.value {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.health {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.time.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.target_name {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.attacker_name {
            try!(os.write_uint32(3, v));
        };
        if let Some(v) = self.attacker_illusion {
            try!(os.write_bool(4, v));
        };
        if let Some(v) = self.target_illusion {
            try!(os.write_bool(5, v));
        };
        if let Some(v) = self.inflictor_name {
            try!(os.write_uint32(6, v));
        };
        if let Some(v) = self.value {
            try!(os.write_int32(7, v));
        };
        if let Some(v) = self.health {
            try!(os.write_int32(8, v));
        };
        if let Some(v) = self.time {
            try!(os.write_float(9, v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_CombatLogData>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_CombatLogData {
    fn new() -> CDOTAUserMsg_CombatLogData {
        CDOTAUserMsg_CombatLogData::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_CombatLogData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "type",
                    CDOTAUserMsg_CombatLogData::has_field_type,
                    CDOTAUserMsg_CombatLogData::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "target_name",
                    CDOTAUserMsg_CombatLogData::has_target_name,
                    CDOTAUserMsg_CombatLogData::get_target_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "attacker_name",
                    CDOTAUserMsg_CombatLogData::has_attacker_name,
                    CDOTAUserMsg_CombatLogData::get_attacker_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "attacker_illusion",
                    CDOTAUserMsg_CombatLogData::has_attacker_illusion,
                    CDOTAUserMsg_CombatLogData::get_attacker_illusion,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "target_illusion",
                    CDOTAUserMsg_CombatLogData::has_target_illusion,
                    CDOTAUserMsg_CombatLogData::get_target_illusion,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "inflictor_name",
                    CDOTAUserMsg_CombatLogData::has_inflictor_name,
                    CDOTAUserMsg_CombatLogData::get_inflictor_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "value",
                    CDOTAUserMsg_CombatLogData::has_value,
                    CDOTAUserMsg_CombatLogData::get_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "health",
                    CDOTAUserMsg_CombatLogData::has_health,
                    CDOTAUserMsg_CombatLogData::get_health,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "time",
                    CDOTAUserMsg_CombatLogData::has_time,
                    CDOTAUserMsg_CombatLogData::get_time,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_CombatLogData>(
                    "CDOTAUserMsg_CombatLogData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_CombatLogData {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_target_name();
        self.clear_attacker_name();
        self.clear_attacker_illusion();
        self.clear_target_illusion();
        self.clear_inflictor_name();
        self.clear_value();
        self.clear_health();
        self.clear_time();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_CombatLogData {
    fn eq(&self, other: &CDOTAUserMsg_CombatLogData) -> bool {
        self.field_type == other.field_type &&
        self.target_name == other.target_name &&
        self.attacker_name == other.attacker_name &&
        self.attacker_illusion == other.attacker_illusion &&
        self.target_illusion == other.target_illusion &&
        self.inflictor_name == other.inflictor_name &&
        self.value == other.value &&
        self.health == other.health &&
        self.time == other.time &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_CombatLogData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_CombatLogShowDeath {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_CombatLogShowDeath {}

impl CDOTAUserMsg_CombatLogShowDeath {
    pub fn new() -> CDOTAUserMsg_CombatLogShowDeath {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_CombatLogShowDeath {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_CombatLogShowDeath> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_CombatLogShowDeath,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_CombatLogShowDeath {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for CDOTAUserMsg_CombatLogShowDeath {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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
        ::std::any::TypeId::of::<CDOTAUserMsg_CombatLogShowDeath>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_CombatLogShowDeath {
    fn new() -> CDOTAUserMsg_CombatLogShowDeath {
        CDOTAUserMsg_CombatLogShowDeath::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_CombatLogShowDeath>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_CombatLogShowDeath>(
                    "CDOTAUserMsg_CombatLogShowDeath",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_CombatLogShowDeath {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_CombatLogShowDeath {
    fn eq(&self, other: &CDOTAUserMsg_CombatLogShowDeath) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_CombatLogShowDeath {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_BotChat {
    // message fields
    player_id: ::std::option::Option<u32>,
    format: ::protobuf::SingularField<::std::string::String>,
    message: ::protobuf::SingularField<::std::string::String>,
    target: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_BotChat {}

impl CDOTAUserMsg_BotChat {
    pub fn new() -> CDOTAUserMsg_BotChat {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_BotChat {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_BotChat> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_BotChat,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_BotChat {
                    player_id: ::std::option::Option::None,
                    format: ::protobuf::SingularField::none(),
                    message: ::protobuf::SingularField::none(),
                    target: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 player_id = 1;

    pub fn clear_player_id(&mut self) {
        self.player_id = ::std::option::Option::None;
    }

    pub fn has_player_id(&self) -> bool {
        self.player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_id(&mut self, v: u32) {
        self.player_id = ::std::option::Option::Some(v);
    }

    pub fn get_player_id(&self) -> u32 {
        self.player_id.unwrap_or(0)
    }

    // optional string format = 2;

    pub fn clear_format(&mut self) {
        self.format.clear();
    }

    pub fn has_format(&self) -> bool {
        self.format.is_some()
    }

    // Param is passed by value, moved
    pub fn set_format(&mut self, v: ::std::string::String) {
        self.format = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_format(&mut self) -> &mut ::std::string::String {
        if self.format.is_none() {
            self.format.set_default();
        };
        self.format.as_mut().unwrap()
    }

    // Take field
    pub fn take_format(&mut self) -> ::std::string::String {
        self.format.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_format(&self) -> &str {
        match self.format.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string message = 3;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        if self.message.is_none() {
            self.message.set_default();
        };
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        self.message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        match self.message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string target = 4;

    pub fn clear_target(&mut self) {
        self.target.clear();
    }

    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target(&mut self, v: ::std::string::String) {
        self.target = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_target(&mut self) -> &mut ::std::string::String {
        if self.target.is_none() {
            self.target.set_default();
        };
        self.target.as_mut().unwrap()
    }

    // Take field
    pub fn take_target(&mut self) -> ::std::string::String {
        self.target.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_target(&self) -> &str {
        match self.target.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for CDOTAUserMsg_BotChat {
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
                    let tmp = try!(is.read_uint32());
                    self.player_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.format));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.message));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.target));
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
        for value in &self.player_id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.format {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.message {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.target {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.player_id {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.format.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.message.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.target.as_ref() {
            try!(os.write_string(4, &v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_BotChat>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_BotChat {
    fn new() -> CDOTAUserMsg_BotChat {
        CDOTAUserMsg_BotChat::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_BotChat>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "player_id",
                    CDOTAUserMsg_BotChat::has_player_id,
                    CDOTAUserMsg_BotChat::get_player_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "format",
                    CDOTAUserMsg_BotChat::has_format,
                    CDOTAUserMsg_BotChat::get_format,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "message",
                    CDOTAUserMsg_BotChat::has_message,
                    CDOTAUserMsg_BotChat::get_message,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "target",
                    CDOTAUserMsg_BotChat::has_target,
                    CDOTAUserMsg_BotChat::get_target,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_BotChat>(
                    "CDOTAUserMsg_BotChat",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_BotChat {
    fn clear(&mut self) {
        self.clear_player_id();
        self.clear_format();
        self.clear_message();
        self.clear_target();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_BotChat {
    fn eq(&self, other: &CDOTAUserMsg_BotChat) -> bool {
        self.player_id == other.player_id &&
        self.format == other.format &&
        self.message == other.message &&
        self.target == other.target &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_BotChat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_CombatHeroPositions {
    // message fields
    index: ::std::option::Option<u32>,
    time: ::std::option::Option<i32>,
    world_pos: ::protobuf::SingularPtrField<super::netmessages::CMsgVector2D>,
    health: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_CombatHeroPositions {}

impl CDOTAUserMsg_CombatHeroPositions {
    pub fn new() -> CDOTAUserMsg_CombatHeroPositions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_CombatHeroPositions {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_CombatHeroPositions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_CombatHeroPositions,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_CombatHeroPositions {
                    index: ::std::option::Option::None,
                    time: ::std::option::Option::None,
                    world_pos: ::protobuf::SingularPtrField::none(),
                    health: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 index = 1;

    pub fn clear_index(&mut self) {
        self.index = ::std::option::Option::None;
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: u32) {
        self.index = ::std::option::Option::Some(v);
    }

    pub fn get_index(&self) -> u32 {
        self.index.unwrap_or(0)
    }

    // optional int32 time = 2;

    pub fn clear_time(&mut self) {
        self.time = ::std::option::Option::None;
    }

    pub fn has_time(&self) -> bool {
        self.time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: i32) {
        self.time = ::std::option::Option::Some(v);
    }

    pub fn get_time(&self) -> i32 {
        self.time.unwrap_or(0)
    }

    // optional .CMsgVector2D world_pos = 3;

    pub fn clear_world_pos(&mut self) {
        self.world_pos.clear();
    }

    pub fn has_world_pos(&self) -> bool {
        self.world_pos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_world_pos(&mut self, v: super::netmessages::CMsgVector2D) {
        self.world_pos = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_world_pos(&mut self) -> &mut super::netmessages::CMsgVector2D {
        if self.world_pos.is_none() {
            self.world_pos.set_default();
        };
        self.world_pos.as_mut().unwrap()
    }

    // Take field
    pub fn take_world_pos(&mut self) -> super::netmessages::CMsgVector2D {
        self.world_pos.take().unwrap_or_else(|| super::netmessages::CMsgVector2D::new())
    }

    pub fn get_world_pos(&self) -> &super::netmessages::CMsgVector2D {
        self.world_pos.as_ref().unwrap_or_else(|| super::netmessages::CMsgVector2D::default_instance())
    }

    // optional int32 health = 4;

    pub fn clear_health(&mut self) {
        self.health = ::std::option::Option::None;
    }

    pub fn has_health(&self) -> bool {
        self.health.is_some()
    }

    // Param is passed by value, moved
    pub fn set_health(&mut self, v: i32) {
        self.health = ::std::option::Option::Some(v);
    }

    pub fn get_health(&self) -> i32 {
        self.health.unwrap_or(0)
    }
}

impl ::protobuf::Message for CDOTAUserMsg_CombatHeroPositions {
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
                    let tmp = try!(is.read_uint32());
                    self.index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.time = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.world_pos));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.health = ::std::option::Option::Some(tmp);
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
        for value in &self.index {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.time {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.world_pos {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.health {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.index {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.time {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.world_pos.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.health {
            try!(os.write_int32(4, v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_CombatHeroPositions>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_CombatHeroPositions {
    fn new() -> CDOTAUserMsg_CombatHeroPositions {
        CDOTAUserMsg_CombatHeroPositions::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_CombatHeroPositions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "index",
                    CDOTAUserMsg_CombatHeroPositions::has_index,
                    CDOTAUserMsg_CombatHeroPositions::get_index,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "time",
                    CDOTAUserMsg_CombatHeroPositions::has_time,
                    CDOTAUserMsg_CombatHeroPositions::get_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "world_pos",
                    CDOTAUserMsg_CombatHeroPositions::has_world_pos,
                    CDOTAUserMsg_CombatHeroPositions::get_world_pos,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "health",
                    CDOTAUserMsg_CombatHeroPositions::has_health,
                    CDOTAUserMsg_CombatHeroPositions::get_health,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_CombatHeroPositions>(
                    "CDOTAUserMsg_CombatHeroPositions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_CombatHeroPositions {
    fn clear(&mut self) {
        self.clear_index();
        self.clear_time();
        self.clear_world_pos();
        self.clear_health();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_CombatHeroPositions {
    fn eq(&self, other: &CDOTAUserMsg_CombatHeroPositions) -> bool {
        self.index == other.index &&
        self.time == other.time &&
        self.world_pos == other.world_pos &&
        self.health == other.health &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_CombatHeroPositions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_MiniKillCamInfo {
    // message fields
    attackers: ::protobuf::RepeatedField<CDOTAUserMsg_MiniKillCamInfo_Attacker>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_MiniKillCamInfo {}

impl CDOTAUserMsg_MiniKillCamInfo {
    pub fn new() -> CDOTAUserMsg_MiniKillCamInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_MiniKillCamInfo {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_MiniKillCamInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_MiniKillCamInfo,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_MiniKillCamInfo {
                    attackers: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .CDOTAUserMsg_MiniKillCamInfo.Attacker attackers = 1;

    pub fn clear_attackers(&mut self) {
        self.attackers.clear();
    }

    // Param is passed by value, moved
    pub fn set_attackers(&mut self, v: ::protobuf::RepeatedField<CDOTAUserMsg_MiniKillCamInfo_Attacker>) {
        self.attackers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_attackers(&mut self) -> &mut ::protobuf::RepeatedField<CDOTAUserMsg_MiniKillCamInfo_Attacker> {
        &mut self.attackers
    }

    // Take field
    pub fn take_attackers(&mut self) -> ::protobuf::RepeatedField<CDOTAUserMsg_MiniKillCamInfo_Attacker> {
        ::std::mem::replace(&mut self.attackers, ::protobuf::RepeatedField::new())
    }

    pub fn get_attackers(&self) -> &[CDOTAUserMsg_MiniKillCamInfo_Attacker] {
        &self.attackers
    }
}

impl ::protobuf::Message for CDOTAUserMsg_MiniKillCamInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.attackers));
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
        for value in &self.attackers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.attackers {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_MiniKillCamInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_MiniKillCamInfo {
    fn new() -> CDOTAUserMsg_MiniKillCamInfo {
        CDOTAUserMsg_MiniKillCamInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_MiniKillCamInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "attackers",
                    CDOTAUserMsg_MiniKillCamInfo::get_attackers,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_MiniKillCamInfo>(
                    "CDOTAUserMsg_MiniKillCamInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_MiniKillCamInfo {
    fn clear(&mut self) {
        self.clear_attackers();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_MiniKillCamInfo {
    fn eq(&self, other: &CDOTAUserMsg_MiniKillCamInfo) -> bool {
        self.attackers == other.attackers &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_MiniKillCamInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_MiniKillCamInfo_Attacker {
    // message fields
    attacker: ::std::option::Option<u32>,
    total_damage: ::std::option::Option<i32>,
    abilities: ::protobuf::RepeatedField<CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_MiniKillCamInfo_Attacker {}

impl CDOTAUserMsg_MiniKillCamInfo_Attacker {
    pub fn new() -> CDOTAUserMsg_MiniKillCamInfo_Attacker {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_MiniKillCamInfo_Attacker {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_MiniKillCamInfo_Attacker> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_MiniKillCamInfo_Attacker,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_MiniKillCamInfo_Attacker {
                    attacker: ::std::option::Option::None,
                    total_damage: ::std::option::Option::None,
                    abilities: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 attacker = 1;

    pub fn clear_attacker(&mut self) {
        self.attacker = ::std::option::Option::None;
    }

    pub fn has_attacker(&self) -> bool {
        self.attacker.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attacker(&mut self, v: u32) {
        self.attacker = ::std::option::Option::Some(v);
    }

    pub fn get_attacker(&self) -> u32 {
        self.attacker.unwrap_or(0)
    }

    // optional int32 total_damage = 2;

    pub fn clear_total_damage(&mut self) {
        self.total_damage = ::std::option::Option::None;
    }

    pub fn has_total_damage(&self) -> bool {
        self.total_damage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_total_damage(&mut self, v: i32) {
        self.total_damage = ::std::option::Option::Some(v);
    }

    pub fn get_total_damage(&self) -> i32 {
        self.total_damage.unwrap_or(0)
    }

    // repeated .CDOTAUserMsg_MiniKillCamInfo.Attacker.Ability abilities = 3;

    pub fn clear_abilities(&mut self) {
        self.abilities.clear();
    }

    // Param is passed by value, moved
    pub fn set_abilities(&mut self, v: ::protobuf::RepeatedField<CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability>) {
        self.abilities = v;
    }

    // Mutable pointer to the field.
    pub fn mut_abilities(&mut self) -> &mut ::protobuf::RepeatedField<CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability> {
        &mut self.abilities
    }

    // Take field
    pub fn take_abilities(&mut self) -> ::protobuf::RepeatedField<CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability> {
        ::std::mem::replace(&mut self.abilities, ::protobuf::RepeatedField::new())
    }

    pub fn get_abilities(&self) -> &[CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability] {
        &self.abilities
    }
}

impl ::protobuf::Message for CDOTAUserMsg_MiniKillCamInfo_Attacker {
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
                    let tmp = try!(is.read_uint32());
                    self.attacker = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.total_damage = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.abilities));
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
        for value in &self.attacker {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.total_damage {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.abilities {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.attacker {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.total_damage {
            try!(os.write_int32(2, v));
        };
        for v in &self.abilities {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_MiniKillCamInfo_Attacker>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_MiniKillCamInfo_Attacker {
    fn new() -> CDOTAUserMsg_MiniKillCamInfo_Attacker {
        CDOTAUserMsg_MiniKillCamInfo_Attacker::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_MiniKillCamInfo_Attacker>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "attacker",
                    CDOTAUserMsg_MiniKillCamInfo_Attacker::has_attacker,
                    CDOTAUserMsg_MiniKillCamInfo_Attacker::get_attacker,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "total_damage",
                    CDOTAUserMsg_MiniKillCamInfo_Attacker::has_total_damage,
                    CDOTAUserMsg_MiniKillCamInfo_Attacker::get_total_damage,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "abilities",
                    CDOTAUserMsg_MiniKillCamInfo_Attacker::get_abilities,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_MiniKillCamInfo_Attacker>(
                    "CDOTAUserMsg_MiniKillCamInfo_Attacker",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_MiniKillCamInfo_Attacker {
    fn clear(&mut self) {
        self.clear_attacker();
        self.clear_total_damage();
        self.clear_abilities();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_MiniKillCamInfo_Attacker {
    fn eq(&self, other: &CDOTAUserMsg_MiniKillCamInfo_Attacker) -> bool {
        self.attacker == other.attacker &&
        self.total_damage == other.total_damage &&
        self.abilities == other.abilities &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_MiniKillCamInfo_Attacker {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability {
    // message fields
    ability: ::std::option::Option<u32>,
    damage: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability {}

impl CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability {
    pub fn new() -> CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability {
                    ability: ::std::option::Option::None,
                    damage: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 ability = 1;

    pub fn clear_ability(&mut self) {
        self.ability = ::std::option::Option::None;
    }

    pub fn has_ability(&self) -> bool {
        self.ability.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ability(&mut self, v: u32) {
        self.ability = ::std::option::Option::Some(v);
    }

    pub fn get_ability(&self) -> u32 {
        self.ability.unwrap_or(0)
    }

    // optional int32 damage = 2;

    pub fn clear_damage(&mut self) {
        self.damage = ::std::option::Option::None;
    }

    pub fn has_damage(&self) -> bool {
        self.damage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_damage(&mut self, v: i32) {
        self.damage = ::std::option::Option::Some(v);
    }

    pub fn get_damage(&self) -> i32 {
        self.damage.unwrap_or(0)
    }
}

impl ::protobuf::Message for CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability {
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
                    let tmp = try!(is.read_uint32());
                    self.ability = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.damage = ::std::option::Option::Some(tmp);
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
        for value in &self.ability {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.damage {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ability {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.damage {
            try!(os.write_int32(2, v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability {
    fn new() -> CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability {
        CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "ability",
                    CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability::has_ability,
                    CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability::get_ability,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "damage",
                    CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability::has_damage,
                    CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability::get_damage,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability>(
                    "CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability {
    fn clear(&mut self) {
        self.clear_ability();
        self.clear_damage();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability {
    fn eq(&self, other: &CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability) -> bool {
        self.ability == other.ability &&
        self.damage == other.damage &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_MiniKillCamInfo_Attacker_Ability {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_GlobalLightColor {
    // message fields
    color: ::std::option::Option<u32>,
    duration: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_GlobalLightColor {}

impl CDOTAUserMsg_GlobalLightColor {
    pub fn new() -> CDOTAUserMsg_GlobalLightColor {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_GlobalLightColor {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_GlobalLightColor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_GlobalLightColor,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_GlobalLightColor {
                    color: ::std::option::Option::None,
                    duration: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 color = 1;

    pub fn clear_color(&mut self) {
        self.color = ::std::option::Option::None;
    }

    pub fn has_color(&self) -> bool {
        self.color.is_some()
    }

    // Param is passed by value, moved
    pub fn set_color(&mut self, v: u32) {
        self.color = ::std::option::Option::Some(v);
    }

    pub fn get_color(&self) -> u32 {
        self.color.unwrap_or(0)
    }

    // optional float duration = 2;

    pub fn clear_duration(&mut self) {
        self.duration = ::std::option::Option::None;
    }

    pub fn has_duration(&self) -> bool {
        self.duration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration(&mut self, v: f32) {
        self.duration = ::std::option::Option::Some(v);
    }

    pub fn get_duration(&self) -> f32 {
        self.duration.unwrap_or(0.)
    }
}

impl ::protobuf::Message for CDOTAUserMsg_GlobalLightColor {
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
                    let tmp = try!(is.read_uint32());
                    self.color = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.duration = ::std::option::Option::Some(tmp);
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
        for value in &self.color {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.duration.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.color {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.duration {
            try!(os.write_float(2, v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_GlobalLightColor>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_GlobalLightColor {
    fn new() -> CDOTAUserMsg_GlobalLightColor {
        CDOTAUserMsg_GlobalLightColor::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_GlobalLightColor>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "color",
                    CDOTAUserMsg_GlobalLightColor::has_color,
                    CDOTAUserMsg_GlobalLightColor::get_color,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "duration",
                    CDOTAUserMsg_GlobalLightColor::has_duration,
                    CDOTAUserMsg_GlobalLightColor::get_duration,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_GlobalLightColor>(
                    "CDOTAUserMsg_GlobalLightColor",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_GlobalLightColor {
    fn clear(&mut self) {
        self.clear_color();
        self.clear_duration();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_GlobalLightColor {
    fn eq(&self, other: &CDOTAUserMsg_GlobalLightColor) -> bool {
        self.color == other.color &&
        self.duration == other.duration &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_GlobalLightColor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_GlobalLightDirection {
    // message fields
    direction: ::protobuf::SingularPtrField<super::netmessages::CMsgVector>,
    duration: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_GlobalLightDirection {}

impl CDOTAUserMsg_GlobalLightDirection {
    pub fn new() -> CDOTAUserMsg_GlobalLightDirection {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_GlobalLightDirection {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_GlobalLightDirection> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_GlobalLightDirection,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_GlobalLightDirection {
                    direction: ::protobuf::SingularPtrField::none(),
                    duration: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .CMsgVector direction = 1;

    pub fn clear_direction(&mut self) {
        self.direction.clear();
    }

    pub fn has_direction(&self) -> bool {
        self.direction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_direction(&mut self, v: super::netmessages::CMsgVector) {
        self.direction = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_direction(&mut self) -> &mut super::netmessages::CMsgVector {
        if self.direction.is_none() {
            self.direction.set_default();
        };
        self.direction.as_mut().unwrap()
    }

    // Take field
    pub fn take_direction(&mut self) -> super::netmessages::CMsgVector {
        self.direction.take().unwrap_or_else(|| super::netmessages::CMsgVector::new())
    }

    pub fn get_direction(&self) -> &super::netmessages::CMsgVector {
        self.direction.as_ref().unwrap_or_else(|| super::netmessages::CMsgVector::default_instance())
    }

    // optional float duration = 2;

    pub fn clear_duration(&mut self) {
        self.duration = ::std::option::Option::None;
    }

    pub fn has_duration(&self) -> bool {
        self.duration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration(&mut self, v: f32) {
        self.duration = ::std::option::Option::Some(v);
    }

    pub fn get_duration(&self) -> f32 {
        self.duration.unwrap_or(0.)
    }
}

impl ::protobuf::Message for CDOTAUserMsg_GlobalLightDirection {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.direction));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.duration = ::std::option::Option::Some(tmp);
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
        for value in &self.direction {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.duration.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.direction.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.duration {
            try!(os.write_float(2, v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_GlobalLightDirection>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_GlobalLightDirection {
    fn new() -> CDOTAUserMsg_GlobalLightDirection {
        CDOTAUserMsg_GlobalLightDirection::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_GlobalLightDirection>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "direction",
                    CDOTAUserMsg_GlobalLightDirection::has_direction,
                    CDOTAUserMsg_GlobalLightDirection::get_direction,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "duration",
                    CDOTAUserMsg_GlobalLightDirection::has_duration,
                    CDOTAUserMsg_GlobalLightDirection::get_duration,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_GlobalLightDirection>(
                    "CDOTAUserMsg_GlobalLightDirection",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_GlobalLightDirection {
    fn clear(&mut self) {
        self.clear_direction();
        self.clear_duration();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_GlobalLightDirection {
    fn eq(&self, other: &CDOTAUserMsg_GlobalLightDirection) -> bool {
        self.direction == other.direction &&
        self.duration == other.duration &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_GlobalLightDirection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_LocationPing {
    // message fields
    player_id: ::std::option::Option<u32>,
    location_ping: ::protobuf::SingularPtrField<super::dota_commonmessages::CDOTAMsg_LocationPing>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_LocationPing {}

impl CDOTAUserMsg_LocationPing {
    pub fn new() -> CDOTAUserMsg_LocationPing {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_LocationPing {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_LocationPing> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_LocationPing,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_LocationPing {
                    player_id: ::std::option::Option::None,
                    location_ping: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 player_id = 1;

    pub fn clear_player_id(&mut self) {
        self.player_id = ::std::option::Option::None;
    }

    pub fn has_player_id(&self) -> bool {
        self.player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_id(&mut self, v: u32) {
        self.player_id = ::std::option::Option::Some(v);
    }

    pub fn get_player_id(&self) -> u32 {
        self.player_id.unwrap_or(0)
    }

    // optional .CDOTAMsg_LocationPing location_ping = 2;

    pub fn clear_location_ping(&mut self) {
        self.location_ping.clear();
    }

    pub fn has_location_ping(&self) -> bool {
        self.location_ping.is_some()
    }

    // Param is passed by value, moved
    pub fn set_location_ping(&mut self, v: super::dota_commonmessages::CDOTAMsg_LocationPing) {
        self.location_ping = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_location_ping(&mut self) -> &mut super::dota_commonmessages::CDOTAMsg_LocationPing {
        if self.location_ping.is_none() {
            self.location_ping.set_default();
        };
        self.location_ping.as_mut().unwrap()
    }

    // Take field
    pub fn take_location_ping(&mut self) -> super::dota_commonmessages::CDOTAMsg_LocationPing {
        self.location_ping.take().unwrap_or_else(|| super::dota_commonmessages::CDOTAMsg_LocationPing::new())
    }

    pub fn get_location_ping(&self) -> &super::dota_commonmessages::CDOTAMsg_LocationPing {
        self.location_ping.as_ref().unwrap_or_else(|| super::dota_commonmessages::CDOTAMsg_LocationPing::default_instance())
    }
}

impl ::protobuf::Message for CDOTAUserMsg_LocationPing {
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
                    let tmp = try!(is.read_uint32());
                    self.player_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.location_ping));
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
        for value in &self.player_id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.location_ping {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.player_id {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.location_ping.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_LocationPing>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_LocationPing {
    fn new() -> CDOTAUserMsg_LocationPing {
        CDOTAUserMsg_LocationPing::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_LocationPing>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "player_id",
                    CDOTAUserMsg_LocationPing::has_player_id,
                    CDOTAUserMsg_LocationPing::get_player_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "location_ping",
                    CDOTAUserMsg_LocationPing::has_location_ping,
                    CDOTAUserMsg_LocationPing::get_location_ping,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_LocationPing>(
                    "CDOTAUserMsg_LocationPing",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_LocationPing {
    fn clear(&mut self) {
        self.clear_player_id();
        self.clear_location_ping();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_LocationPing {
    fn eq(&self, other: &CDOTAUserMsg_LocationPing) -> bool {
        self.player_id == other.player_id &&
        self.location_ping == other.location_ping &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_LocationPing {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_MinimapEvent {
    // message fields
    event_type: ::std::option::Option<i32>,
    entity_handle: ::std::option::Option<i32>,
    x: ::std::option::Option<i32>,
    y: ::std::option::Option<i32>,
    duration: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_MinimapEvent {}

impl CDOTAUserMsg_MinimapEvent {
    pub fn new() -> CDOTAUserMsg_MinimapEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_MinimapEvent {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_MinimapEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_MinimapEvent,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_MinimapEvent {
                    event_type: ::std::option::Option::None,
                    entity_handle: ::std::option::Option::None,
                    x: ::std::option::Option::None,
                    y: ::std::option::Option::None,
                    duration: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 event_type = 1;

    pub fn clear_event_type(&mut self) {
        self.event_type = ::std::option::Option::None;
    }

    pub fn has_event_type(&self) -> bool {
        self.event_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_type(&mut self, v: i32) {
        self.event_type = ::std::option::Option::Some(v);
    }

    pub fn get_event_type(&self) -> i32 {
        self.event_type.unwrap_or(0)
    }

    // optional int32 entity_handle = 2;

    pub fn clear_entity_handle(&mut self) {
        self.entity_handle = ::std::option::Option::None;
    }

    pub fn has_entity_handle(&self) -> bool {
        self.entity_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity_handle(&mut self, v: i32) {
        self.entity_handle = ::std::option::Option::Some(v);
    }

    pub fn get_entity_handle(&self) -> i32 {
        self.entity_handle.unwrap_or(0)
    }

    // optional int32 x = 3;

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

    // optional int32 y = 4;

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

    // optional int32 duration = 5;

    pub fn clear_duration(&mut self) {
        self.duration = ::std::option::Option::None;
    }

    pub fn has_duration(&self) -> bool {
        self.duration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration(&mut self, v: i32) {
        self.duration = ::std::option::Option::Some(v);
    }

    pub fn get_duration(&self) -> i32 {
        self.duration.unwrap_or(0)
    }
}

impl ::protobuf::Message for CDOTAUserMsg_MinimapEvent {
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
                    self.event_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.entity_handle = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.x = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.y = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.duration = ::std::option::Option::Some(tmp);
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
        for value in &self.event_type {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.entity_handle {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.x {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.y {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.duration {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.event_type {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.entity_handle {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.x {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.y {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.duration {
            try!(os.write_int32(5, v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_MinimapEvent>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_MinimapEvent {
    fn new() -> CDOTAUserMsg_MinimapEvent {
        CDOTAUserMsg_MinimapEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_MinimapEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "event_type",
                    CDOTAUserMsg_MinimapEvent::has_event_type,
                    CDOTAUserMsg_MinimapEvent::get_event_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "entity_handle",
                    CDOTAUserMsg_MinimapEvent::has_entity_handle,
                    CDOTAUserMsg_MinimapEvent::get_entity_handle,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "x",
                    CDOTAUserMsg_MinimapEvent::has_x,
                    CDOTAUserMsg_MinimapEvent::get_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "y",
                    CDOTAUserMsg_MinimapEvent::has_y,
                    CDOTAUserMsg_MinimapEvent::get_y,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "duration",
                    CDOTAUserMsg_MinimapEvent::has_duration,
                    CDOTAUserMsg_MinimapEvent::get_duration,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_MinimapEvent>(
                    "CDOTAUserMsg_MinimapEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_MinimapEvent {
    fn clear(&mut self) {
        self.clear_event_type();
        self.clear_entity_handle();
        self.clear_x();
        self.clear_y();
        self.clear_duration();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_MinimapEvent {
    fn eq(&self, other: &CDOTAUserMsg_MinimapEvent) -> bool {
        self.event_type == other.event_type &&
        self.entity_handle == other.entity_handle &&
        self.x == other.x &&
        self.y == other.y &&
        self.duration == other.duration &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_MinimapEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_MapLine {
    // message fields
    player_id: ::std::option::Option<i32>,
    mapline: ::protobuf::SingularPtrField<super::dota_commonmessages::CDOTAMsg_MapLine>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_MapLine {}

impl CDOTAUserMsg_MapLine {
    pub fn new() -> CDOTAUserMsg_MapLine {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_MapLine {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_MapLine> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_MapLine,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_MapLine {
                    player_id: ::std::option::Option::None,
                    mapline: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 player_id = 1;

    pub fn clear_player_id(&mut self) {
        self.player_id = ::std::option::Option::None;
    }

    pub fn has_player_id(&self) -> bool {
        self.player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_id(&mut self, v: i32) {
        self.player_id = ::std::option::Option::Some(v);
    }

    pub fn get_player_id(&self) -> i32 {
        self.player_id.unwrap_or(0)
    }

    // optional .CDOTAMsg_MapLine mapline = 2;

    pub fn clear_mapline(&mut self) {
        self.mapline.clear();
    }

    pub fn has_mapline(&self) -> bool {
        self.mapline.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mapline(&mut self, v: super::dota_commonmessages::CDOTAMsg_MapLine) {
        self.mapline = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mapline(&mut self) -> &mut super::dota_commonmessages::CDOTAMsg_MapLine {
        if self.mapline.is_none() {
            self.mapline.set_default();
        };
        self.mapline.as_mut().unwrap()
    }

    // Take field
    pub fn take_mapline(&mut self) -> super::dota_commonmessages::CDOTAMsg_MapLine {
        self.mapline.take().unwrap_or_else(|| super::dota_commonmessages::CDOTAMsg_MapLine::new())
    }

    pub fn get_mapline(&self) -> &super::dota_commonmessages::CDOTAMsg_MapLine {
        self.mapline.as_ref().unwrap_or_else(|| super::dota_commonmessages::CDOTAMsg_MapLine::default_instance())
    }
}

impl ::protobuf::Message for CDOTAUserMsg_MapLine {
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
                    self.player_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.mapline));
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
        for value in &self.player_id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.mapline {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.player_id {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.mapline.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_MapLine>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_MapLine {
    fn new() -> CDOTAUserMsg_MapLine {
        CDOTAUserMsg_MapLine::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_MapLine>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "player_id",
                    CDOTAUserMsg_MapLine::has_player_id,
                    CDOTAUserMsg_MapLine::get_player_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "mapline",
                    CDOTAUserMsg_MapLine::has_mapline,
                    CDOTAUserMsg_MapLine::get_mapline,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_MapLine>(
                    "CDOTAUserMsg_MapLine",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_MapLine {
    fn clear(&mut self) {
        self.clear_player_id();
        self.clear_mapline();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_MapLine {
    fn eq(&self, other: &CDOTAUserMsg_MapLine) -> bool {
        self.player_id == other.player_id &&
        self.mapline == other.mapline &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_MapLine {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_MinimapDebugPoint {
    // message fields
    location: ::protobuf::SingularPtrField<super::netmessages::CMsgVector>,
    color: ::std::option::Option<u32>,
    size: ::std::option::Option<i32>,
    duration: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_MinimapDebugPoint {}

impl CDOTAUserMsg_MinimapDebugPoint {
    pub fn new() -> CDOTAUserMsg_MinimapDebugPoint {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_MinimapDebugPoint {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_MinimapDebugPoint> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_MinimapDebugPoint,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_MinimapDebugPoint {
                    location: ::protobuf::SingularPtrField::none(),
                    color: ::std::option::Option::None,
                    size: ::std::option::Option::None,
                    duration: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .CMsgVector location = 1;

    pub fn clear_location(&mut self) {
        self.location.clear();
    }

    pub fn has_location(&self) -> bool {
        self.location.is_some()
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: super::netmessages::CMsgVector) {
        self.location = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_location(&mut self) -> &mut super::netmessages::CMsgVector {
        if self.location.is_none() {
            self.location.set_default();
        };
        self.location.as_mut().unwrap()
    }

    // Take field
    pub fn take_location(&mut self) -> super::netmessages::CMsgVector {
        self.location.take().unwrap_or_else(|| super::netmessages::CMsgVector::new())
    }

    pub fn get_location(&self) -> &super::netmessages::CMsgVector {
        self.location.as_ref().unwrap_or_else(|| super::netmessages::CMsgVector::default_instance())
    }

    // optional uint32 color = 2;

    pub fn clear_color(&mut self) {
        self.color = ::std::option::Option::None;
    }

    pub fn has_color(&self) -> bool {
        self.color.is_some()
    }

    // Param is passed by value, moved
    pub fn set_color(&mut self, v: u32) {
        self.color = ::std::option::Option::Some(v);
    }

    pub fn get_color(&self) -> u32 {
        self.color.unwrap_or(0)
    }

    // optional int32 size = 3;

    pub fn clear_size(&mut self) {
        self.size = ::std::option::Option::None;
    }

    pub fn has_size(&self) -> bool {
        self.size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: i32) {
        self.size = ::std::option::Option::Some(v);
    }

    pub fn get_size(&self) -> i32 {
        self.size.unwrap_or(0)
    }

    // optional float duration = 4;

    pub fn clear_duration(&mut self) {
        self.duration = ::std::option::Option::None;
    }

    pub fn has_duration(&self) -> bool {
        self.duration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration(&mut self, v: f32) {
        self.duration = ::std::option::Option::Some(v);
    }

    pub fn get_duration(&self) -> f32 {
        self.duration.unwrap_or(0.)
    }
}

impl ::protobuf::Message for CDOTAUserMsg_MinimapDebugPoint {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.location));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.color = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.size = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.duration = ::std::option::Option::Some(tmp);
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
        for value in &self.location {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.color {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.size {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.duration.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.location.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.color {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.size {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.duration {
            try!(os.write_float(4, v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_MinimapDebugPoint>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_MinimapDebugPoint {
    fn new() -> CDOTAUserMsg_MinimapDebugPoint {
        CDOTAUserMsg_MinimapDebugPoint::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_MinimapDebugPoint>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "location",
                    CDOTAUserMsg_MinimapDebugPoint::has_location,
                    CDOTAUserMsg_MinimapDebugPoint::get_location,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "color",
                    CDOTAUserMsg_MinimapDebugPoint::has_color,
                    CDOTAUserMsg_MinimapDebugPoint::get_color,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "size",
                    CDOTAUserMsg_MinimapDebugPoint::has_size,
                    CDOTAUserMsg_MinimapDebugPoint::get_size,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "duration",
                    CDOTAUserMsg_MinimapDebugPoint::has_duration,
                    CDOTAUserMsg_MinimapDebugPoint::get_duration,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_MinimapDebugPoint>(
                    "CDOTAUserMsg_MinimapDebugPoint",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_MinimapDebugPoint {
    fn clear(&mut self) {
        self.clear_location();
        self.clear_color();
        self.clear_size();
        self.clear_duration();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_MinimapDebugPoint {
    fn eq(&self, other: &CDOTAUserMsg_MinimapDebugPoint) -> bool {
        self.location == other.location &&
        self.color == other.color &&
        self.size == other.size &&
        self.duration == other.duration &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_MinimapDebugPoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_CreateLinearProjectile {
    // message fields
    origin: ::protobuf::SingularPtrField<super::netmessages::CMsgVector>,
    velocity: ::protobuf::SingularPtrField<super::netmessages::CMsgVector2D>,
    latency: ::std::option::Option<i32>,
    entindex: ::std::option::Option<i32>,
    particle_index: ::std::option::Option<i32>,
    handle: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_CreateLinearProjectile {}

impl CDOTAUserMsg_CreateLinearProjectile {
    pub fn new() -> CDOTAUserMsg_CreateLinearProjectile {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_CreateLinearProjectile {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_CreateLinearProjectile> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_CreateLinearProjectile,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_CreateLinearProjectile {
                    origin: ::protobuf::SingularPtrField::none(),
                    velocity: ::protobuf::SingularPtrField::none(),
                    latency: ::std::option::Option::None,
                    entindex: ::std::option::Option::None,
                    particle_index: ::std::option::Option::None,
                    handle: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .CMsgVector origin = 1;

    pub fn clear_origin(&mut self) {
        self.origin.clear();
    }

    pub fn has_origin(&self) -> bool {
        self.origin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin(&mut self, v: super::netmessages::CMsgVector) {
        self.origin = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin(&mut self) -> &mut super::netmessages::CMsgVector {
        if self.origin.is_none() {
            self.origin.set_default();
        };
        self.origin.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin(&mut self) -> super::netmessages::CMsgVector {
        self.origin.take().unwrap_or_else(|| super::netmessages::CMsgVector::new())
    }

    pub fn get_origin(&self) -> &super::netmessages::CMsgVector {
        self.origin.as_ref().unwrap_or_else(|| super::netmessages::CMsgVector::default_instance())
    }

    // optional .CMsgVector2D velocity = 2;

    pub fn clear_velocity(&mut self) {
        self.velocity.clear();
    }

    pub fn has_velocity(&self) -> bool {
        self.velocity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_velocity(&mut self, v: super::netmessages::CMsgVector2D) {
        self.velocity = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_velocity(&mut self) -> &mut super::netmessages::CMsgVector2D {
        if self.velocity.is_none() {
            self.velocity.set_default();
        };
        self.velocity.as_mut().unwrap()
    }

    // Take field
    pub fn take_velocity(&mut self) -> super::netmessages::CMsgVector2D {
        self.velocity.take().unwrap_or_else(|| super::netmessages::CMsgVector2D::new())
    }

    pub fn get_velocity(&self) -> &super::netmessages::CMsgVector2D {
        self.velocity.as_ref().unwrap_or_else(|| super::netmessages::CMsgVector2D::default_instance())
    }

    // optional int32 latency = 3;

    pub fn clear_latency(&mut self) {
        self.latency = ::std::option::Option::None;
    }

    pub fn has_latency(&self) -> bool {
        self.latency.is_some()
    }

    // Param is passed by value, moved
    pub fn set_latency(&mut self, v: i32) {
        self.latency = ::std::option::Option::Some(v);
    }

    pub fn get_latency(&self) -> i32 {
        self.latency.unwrap_or(0)
    }

    // optional int32 entindex = 4;

    pub fn clear_entindex(&mut self) {
        self.entindex = ::std::option::Option::None;
    }

    pub fn has_entindex(&self) -> bool {
        self.entindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entindex(&mut self, v: i32) {
        self.entindex = ::std::option::Option::Some(v);
    }

    pub fn get_entindex(&self) -> i32 {
        self.entindex.unwrap_or(0)
    }

    // optional int32 particle_index = 5;

    pub fn clear_particle_index(&mut self) {
        self.particle_index = ::std::option::Option::None;
    }

    pub fn has_particle_index(&self) -> bool {
        self.particle_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_particle_index(&mut self, v: i32) {
        self.particle_index = ::std::option::Option::Some(v);
    }

    pub fn get_particle_index(&self) -> i32 {
        self.particle_index.unwrap_or(0)
    }

    // optional int32 handle = 6;

    pub fn clear_handle(&mut self) {
        self.handle = ::std::option::Option::None;
    }

    pub fn has_handle(&self) -> bool {
        self.handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_handle(&mut self, v: i32) {
        self.handle = ::std::option::Option::Some(v);
    }

    pub fn get_handle(&self) -> i32 {
        self.handle.unwrap_or(0)
    }
}

impl ::protobuf::Message for CDOTAUserMsg_CreateLinearProjectile {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.origin));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.velocity));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.latency = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.entindex = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.particle_index = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.handle = ::std::option::Option::Some(tmp);
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
        for value in &self.origin {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.velocity {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.latency {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.entindex {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.particle_index {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.handle {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.velocity.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.latency {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.entindex {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.particle_index {
            try!(os.write_int32(5, v));
        };
        if let Some(v) = self.handle {
            try!(os.write_int32(6, v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_CreateLinearProjectile>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_CreateLinearProjectile {
    fn new() -> CDOTAUserMsg_CreateLinearProjectile {
        CDOTAUserMsg_CreateLinearProjectile::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_CreateLinearProjectile>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "origin",
                    CDOTAUserMsg_CreateLinearProjectile::has_origin,
                    CDOTAUserMsg_CreateLinearProjectile::get_origin,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "velocity",
                    CDOTAUserMsg_CreateLinearProjectile::has_velocity,
                    CDOTAUserMsg_CreateLinearProjectile::get_velocity,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "latency",
                    CDOTAUserMsg_CreateLinearProjectile::has_latency,
                    CDOTAUserMsg_CreateLinearProjectile::get_latency,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "entindex",
                    CDOTAUserMsg_CreateLinearProjectile::has_entindex,
                    CDOTAUserMsg_CreateLinearProjectile::get_entindex,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "particle_index",
                    CDOTAUserMsg_CreateLinearProjectile::has_particle_index,
                    CDOTAUserMsg_CreateLinearProjectile::get_particle_index,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "handle",
                    CDOTAUserMsg_CreateLinearProjectile::has_handle,
                    CDOTAUserMsg_CreateLinearProjectile::get_handle,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_CreateLinearProjectile>(
                    "CDOTAUserMsg_CreateLinearProjectile",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_CreateLinearProjectile {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_velocity();
        self.clear_latency();
        self.clear_entindex();
        self.clear_particle_index();
        self.clear_handle();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_CreateLinearProjectile {
    fn eq(&self, other: &CDOTAUserMsg_CreateLinearProjectile) -> bool {
        self.origin == other.origin &&
        self.velocity == other.velocity &&
        self.latency == other.latency &&
        self.entindex == other.entindex &&
        self.particle_index == other.particle_index &&
        self.handle == other.handle &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_CreateLinearProjectile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_DestroyLinearProjectile {
    // message fields
    handle: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_DestroyLinearProjectile {}

impl CDOTAUserMsg_DestroyLinearProjectile {
    pub fn new() -> CDOTAUserMsg_DestroyLinearProjectile {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_DestroyLinearProjectile {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_DestroyLinearProjectile> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_DestroyLinearProjectile,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_DestroyLinearProjectile {
                    handle: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 handle = 1;

    pub fn clear_handle(&mut self) {
        self.handle = ::std::option::Option::None;
    }

    pub fn has_handle(&self) -> bool {
        self.handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_handle(&mut self, v: i32) {
        self.handle = ::std::option::Option::Some(v);
    }

    pub fn get_handle(&self) -> i32 {
        self.handle.unwrap_or(0)
    }
}

impl ::protobuf::Message for CDOTAUserMsg_DestroyLinearProjectile {
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
                    self.handle = ::std::option::Option::Some(tmp);
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
        for value in &self.handle {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.handle {
            try!(os.write_int32(1, v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_DestroyLinearProjectile>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_DestroyLinearProjectile {
    fn new() -> CDOTAUserMsg_DestroyLinearProjectile {
        CDOTAUserMsg_DestroyLinearProjectile::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_DestroyLinearProjectile>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "handle",
                    CDOTAUserMsg_DestroyLinearProjectile::has_handle,
                    CDOTAUserMsg_DestroyLinearProjectile::get_handle,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_DestroyLinearProjectile>(
                    "CDOTAUserMsg_DestroyLinearProjectile",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_DestroyLinearProjectile {
    fn clear(&mut self) {
        self.clear_handle();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_DestroyLinearProjectile {
    fn eq(&self, other: &CDOTAUserMsg_DestroyLinearProjectile) -> bool {
        self.handle == other.handle &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_DestroyLinearProjectile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_DodgeTrackingProjectiles {
    // message fields
    entindex: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_DodgeTrackingProjectiles {}

impl CDOTAUserMsg_DodgeTrackingProjectiles {
    pub fn new() -> CDOTAUserMsg_DodgeTrackingProjectiles {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_DodgeTrackingProjectiles {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_DodgeTrackingProjectiles> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_DodgeTrackingProjectiles,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_DodgeTrackingProjectiles {
                    entindex: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int32 entindex = 1;

    pub fn clear_entindex(&mut self) {
        self.entindex = ::std::option::Option::None;
    }

    pub fn has_entindex(&self) -> bool {
        self.entindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entindex(&mut self, v: i32) {
        self.entindex = ::std::option::Option::Some(v);
    }

    pub fn get_entindex(&self) -> i32 {
        self.entindex.unwrap_or(0)
    }
}

impl ::protobuf::Message for CDOTAUserMsg_DodgeTrackingProjectiles {
    fn is_initialized(&self) -> bool {
        if self.entindex.is_none() {
            return false;
        };
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
                    self.entindex = ::std::option::Option::Some(tmp);
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
        for value in &self.entindex {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.entindex {
            try!(os.write_int32(1, v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_DodgeTrackingProjectiles>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_DodgeTrackingProjectiles {
    fn new() -> CDOTAUserMsg_DodgeTrackingProjectiles {
        CDOTAUserMsg_DodgeTrackingProjectiles::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_DodgeTrackingProjectiles>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "entindex",
                    CDOTAUserMsg_DodgeTrackingProjectiles::has_entindex,
                    CDOTAUserMsg_DodgeTrackingProjectiles::get_entindex,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_DodgeTrackingProjectiles>(
                    "CDOTAUserMsg_DodgeTrackingProjectiles",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_DodgeTrackingProjectiles {
    fn clear(&mut self) {
        self.clear_entindex();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_DodgeTrackingProjectiles {
    fn eq(&self, other: &CDOTAUserMsg_DodgeTrackingProjectiles) -> bool {
        self.entindex == other.entindex &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_DodgeTrackingProjectiles {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_SpectatorPlayerClick {
    // message fields
    entindex: ::std::option::Option<i32>,
    order_type: ::std::option::Option<i32>,
    target_index: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_SpectatorPlayerClick {}

impl CDOTAUserMsg_SpectatorPlayerClick {
    pub fn new() -> CDOTAUserMsg_SpectatorPlayerClick {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_SpectatorPlayerClick {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_SpectatorPlayerClick> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_SpectatorPlayerClick,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_SpectatorPlayerClick {
                    entindex: ::std::option::Option::None,
                    order_type: ::std::option::Option::None,
                    target_index: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int32 entindex = 1;

    pub fn clear_entindex(&mut self) {
        self.entindex = ::std::option::Option::None;
    }

    pub fn has_entindex(&self) -> bool {
        self.entindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entindex(&mut self, v: i32) {
        self.entindex = ::std::option::Option::Some(v);
    }

    pub fn get_entindex(&self) -> i32 {
        self.entindex.unwrap_or(0)
    }

    // optional int32 order_type = 2;

    pub fn clear_order_type(&mut self) {
        self.order_type = ::std::option::Option::None;
    }

    pub fn has_order_type(&self) -> bool {
        self.order_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_order_type(&mut self, v: i32) {
        self.order_type = ::std::option::Option::Some(v);
    }

    pub fn get_order_type(&self) -> i32 {
        self.order_type.unwrap_or(0)
    }

    // optional int32 target_index = 3;

    pub fn clear_target_index(&mut self) {
        self.target_index = ::std::option::Option::None;
    }

    pub fn has_target_index(&self) -> bool {
        self.target_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target_index(&mut self, v: i32) {
        self.target_index = ::std::option::Option::Some(v);
    }

    pub fn get_target_index(&self) -> i32 {
        self.target_index.unwrap_or(0)
    }
}

impl ::protobuf::Message for CDOTAUserMsg_SpectatorPlayerClick {
    fn is_initialized(&self) -> bool {
        if self.entindex.is_none() {
            return false;
        };
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
                    self.entindex = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.order_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.target_index = ::std::option::Option::Some(tmp);
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
        for value in &self.entindex {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.order_type {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.target_index {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.entindex {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.order_type {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.target_index {
            try!(os.write_int32(3, v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_SpectatorPlayerClick>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_SpectatorPlayerClick {
    fn new() -> CDOTAUserMsg_SpectatorPlayerClick {
        CDOTAUserMsg_SpectatorPlayerClick::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_SpectatorPlayerClick>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "entindex",
                    CDOTAUserMsg_SpectatorPlayerClick::has_entindex,
                    CDOTAUserMsg_SpectatorPlayerClick::get_entindex,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "order_type",
                    CDOTAUserMsg_SpectatorPlayerClick::has_order_type,
                    CDOTAUserMsg_SpectatorPlayerClick::get_order_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "target_index",
                    CDOTAUserMsg_SpectatorPlayerClick::has_target_index,
                    CDOTAUserMsg_SpectatorPlayerClick::get_target_index,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_SpectatorPlayerClick>(
                    "CDOTAUserMsg_SpectatorPlayerClick",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_SpectatorPlayerClick {
    fn clear(&mut self) {
        self.clear_entindex();
        self.clear_order_type();
        self.clear_target_index();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_SpectatorPlayerClick {
    fn eq(&self, other: &CDOTAUserMsg_SpectatorPlayerClick) -> bool {
        self.entindex == other.entindex &&
        self.order_type == other.order_type &&
        self.target_index == other.target_index &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_SpectatorPlayerClick {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_NevermoreRequiem {
    // message fields
    entity_handle: ::std::option::Option<i32>,
    lines: ::std::option::Option<i32>,
    origin: ::protobuf::SingularPtrField<super::netmessages::CMsgVector>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_NevermoreRequiem {}

impl CDOTAUserMsg_NevermoreRequiem {
    pub fn new() -> CDOTAUserMsg_NevermoreRequiem {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_NevermoreRequiem {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_NevermoreRequiem> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_NevermoreRequiem,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_NevermoreRequiem {
                    entity_handle: ::std::option::Option::None,
                    lines: ::std::option::Option::None,
                    origin: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 entity_handle = 1;

    pub fn clear_entity_handle(&mut self) {
        self.entity_handle = ::std::option::Option::None;
    }

    pub fn has_entity_handle(&self) -> bool {
        self.entity_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity_handle(&mut self, v: i32) {
        self.entity_handle = ::std::option::Option::Some(v);
    }

    pub fn get_entity_handle(&self) -> i32 {
        self.entity_handle.unwrap_or(0)
    }

    // optional int32 lines = 2;

    pub fn clear_lines(&mut self) {
        self.lines = ::std::option::Option::None;
    }

    pub fn has_lines(&self) -> bool {
        self.lines.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lines(&mut self, v: i32) {
        self.lines = ::std::option::Option::Some(v);
    }

    pub fn get_lines(&self) -> i32 {
        self.lines.unwrap_or(0)
    }

    // optional .CMsgVector origin = 3;

    pub fn clear_origin(&mut self) {
        self.origin.clear();
    }

    pub fn has_origin(&self) -> bool {
        self.origin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin(&mut self, v: super::netmessages::CMsgVector) {
        self.origin = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin(&mut self) -> &mut super::netmessages::CMsgVector {
        if self.origin.is_none() {
            self.origin.set_default();
        };
        self.origin.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin(&mut self) -> super::netmessages::CMsgVector {
        self.origin.take().unwrap_or_else(|| super::netmessages::CMsgVector::new())
    }

    pub fn get_origin(&self) -> &super::netmessages::CMsgVector {
        self.origin.as_ref().unwrap_or_else(|| super::netmessages::CMsgVector::default_instance())
    }
}

impl ::protobuf::Message for CDOTAUserMsg_NevermoreRequiem {
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
                    self.entity_handle = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.lines = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.origin));
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
        for value in &self.entity_handle {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.lines {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.origin {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.entity_handle {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.lines {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.origin.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_NevermoreRequiem>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_NevermoreRequiem {
    fn new() -> CDOTAUserMsg_NevermoreRequiem {
        CDOTAUserMsg_NevermoreRequiem::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_NevermoreRequiem>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "entity_handle",
                    CDOTAUserMsg_NevermoreRequiem::has_entity_handle,
                    CDOTAUserMsg_NevermoreRequiem::get_entity_handle,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "lines",
                    CDOTAUserMsg_NevermoreRequiem::has_lines,
                    CDOTAUserMsg_NevermoreRequiem::get_lines,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "origin",
                    CDOTAUserMsg_NevermoreRequiem::has_origin,
                    CDOTAUserMsg_NevermoreRequiem::get_origin,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_NevermoreRequiem>(
                    "CDOTAUserMsg_NevermoreRequiem",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_NevermoreRequiem {
    fn clear(&mut self) {
        self.clear_entity_handle();
        self.clear_lines();
        self.clear_origin();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_NevermoreRequiem {
    fn eq(&self, other: &CDOTAUserMsg_NevermoreRequiem) -> bool {
        self.entity_handle == other.entity_handle &&
        self.lines == other.lines &&
        self.origin == other.origin &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_NevermoreRequiem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_InvalidCommand {
    // message fields
    message: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_InvalidCommand {}

impl CDOTAUserMsg_InvalidCommand {
    pub fn new() -> CDOTAUserMsg_InvalidCommand {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_InvalidCommand {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_InvalidCommand> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_InvalidCommand,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_InvalidCommand {
                    message: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string message = 1;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        if self.message.is_none() {
            self.message.set_default();
        };
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        self.message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        match self.message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for CDOTAUserMsg_InvalidCommand {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.message));
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
        for value in &self.message {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.message.as_ref() {
            try!(os.write_string(1, &v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_InvalidCommand>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_InvalidCommand {
    fn new() -> CDOTAUserMsg_InvalidCommand {
        CDOTAUserMsg_InvalidCommand::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_InvalidCommand>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "message",
                    CDOTAUserMsg_InvalidCommand::has_message,
                    CDOTAUserMsg_InvalidCommand::get_message,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_InvalidCommand>(
                    "CDOTAUserMsg_InvalidCommand",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_InvalidCommand {
    fn clear(&mut self) {
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_InvalidCommand {
    fn eq(&self, other: &CDOTAUserMsg_InvalidCommand) -> bool {
        self.message == other.message &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_InvalidCommand {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_HudError {
    // message fields
    order_id: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_HudError {}

impl CDOTAUserMsg_HudError {
    pub fn new() -> CDOTAUserMsg_HudError {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_HudError {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_HudError> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_HudError,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_HudError {
                    order_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 order_id = 1;

    pub fn clear_order_id(&mut self) {
        self.order_id = ::std::option::Option::None;
    }

    pub fn has_order_id(&self) -> bool {
        self.order_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_order_id(&mut self, v: i32) {
        self.order_id = ::std::option::Option::Some(v);
    }

    pub fn get_order_id(&self) -> i32 {
        self.order_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for CDOTAUserMsg_HudError {
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
                    self.order_id = ::std::option::Option::Some(tmp);
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
        for value in &self.order_id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.order_id {
            try!(os.write_int32(1, v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_HudError>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_HudError {
    fn new() -> CDOTAUserMsg_HudError {
        CDOTAUserMsg_HudError::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_HudError>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "order_id",
                    CDOTAUserMsg_HudError::has_order_id,
                    CDOTAUserMsg_HudError::get_order_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_HudError>(
                    "CDOTAUserMsg_HudError",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_HudError {
    fn clear(&mut self) {
        self.clear_order_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_HudError {
    fn eq(&self, other: &CDOTAUserMsg_HudError) -> bool {
        self.order_id == other.order_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_HudError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_SharedCooldown {
    // message fields
    entindex: ::std::option::Option<i32>,
    name: ::protobuf::SingularField<::std::string::String>,
    cooldown: ::std::option::Option<f32>,
    name_index: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_SharedCooldown {}

impl CDOTAUserMsg_SharedCooldown {
    pub fn new() -> CDOTAUserMsg_SharedCooldown {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_SharedCooldown {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_SharedCooldown> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_SharedCooldown,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_SharedCooldown {
                    entindex: ::std::option::Option::None,
                    name: ::protobuf::SingularField::none(),
                    cooldown: ::std::option::Option::None,
                    name_index: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 entindex = 1;

    pub fn clear_entindex(&mut self) {
        self.entindex = ::std::option::Option::None;
    }

    pub fn has_entindex(&self) -> bool {
        self.entindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entindex(&mut self, v: i32) {
        self.entindex = ::std::option::Option::Some(v);
    }

    pub fn get_entindex(&self) -> i32 {
        self.entindex.unwrap_or(0)
    }

    // optional string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional float cooldown = 3;

    pub fn clear_cooldown(&mut self) {
        self.cooldown = ::std::option::Option::None;
    }

    pub fn has_cooldown(&self) -> bool {
        self.cooldown.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cooldown(&mut self, v: f32) {
        self.cooldown = ::std::option::Option::Some(v);
    }

    pub fn get_cooldown(&self) -> f32 {
        self.cooldown.unwrap_or(0.)
    }

    // optional int32 name_index = 4;

    pub fn clear_name_index(&mut self) {
        self.name_index = ::std::option::Option::None;
    }

    pub fn has_name_index(&self) -> bool {
        self.name_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name_index(&mut self, v: i32) {
        self.name_index = ::std::option::Option::Some(v);
    }

    pub fn get_name_index(&self) -> i32 {
        self.name_index.unwrap_or(0)
    }
}

impl ::protobuf::Message for CDOTAUserMsg_SharedCooldown {
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
                    self.entindex = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.cooldown = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.name_index = ::std::option::Option::Some(tmp);
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
        for value in &self.entindex {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if self.cooldown.is_some() {
            my_size += 5;
        };
        for value in &self.name_index {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.entindex {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.cooldown {
            try!(os.write_float(3, v));
        };
        if let Some(v) = self.name_index {
            try!(os.write_int32(4, v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_SharedCooldown>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_SharedCooldown {
    fn new() -> CDOTAUserMsg_SharedCooldown {
        CDOTAUserMsg_SharedCooldown::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_SharedCooldown>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "entindex",
                    CDOTAUserMsg_SharedCooldown::has_entindex,
                    CDOTAUserMsg_SharedCooldown::get_entindex,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    CDOTAUserMsg_SharedCooldown::has_name,
                    CDOTAUserMsg_SharedCooldown::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "cooldown",
                    CDOTAUserMsg_SharedCooldown::has_cooldown,
                    CDOTAUserMsg_SharedCooldown::get_cooldown,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "name_index",
                    CDOTAUserMsg_SharedCooldown::has_name_index,
                    CDOTAUserMsg_SharedCooldown::get_name_index,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_SharedCooldown>(
                    "CDOTAUserMsg_SharedCooldown",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_SharedCooldown {
    fn clear(&mut self) {
        self.clear_entindex();
        self.clear_name();
        self.clear_cooldown();
        self.clear_name_index();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_SharedCooldown {
    fn eq(&self, other: &CDOTAUserMsg_SharedCooldown) -> bool {
        self.entindex == other.entindex &&
        self.name == other.name &&
        self.cooldown == other.cooldown &&
        self.name_index == other.name_index &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_SharedCooldown {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_SetNextAutobuyItem {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_SetNextAutobuyItem {}

impl CDOTAUserMsg_SetNextAutobuyItem {
    pub fn new() -> CDOTAUserMsg_SetNextAutobuyItem {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_SetNextAutobuyItem {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_SetNextAutobuyItem> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_SetNextAutobuyItem,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_SetNextAutobuyItem {
                    name: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for CDOTAUserMsg_SetNextAutobuyItem {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
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
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_SetNextAutobuyItem>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_SetNextAutobuyItem {
    fn new() -> CDOTAUserMsg_SetNextAutobuyItem {
        CDOTAUserMsg_SetNextAutobuyItem::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_SetNextAutobuyItem>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    CDOTAUserMsg_SetNextAutobuyItem::has_name,
                    CDOTAUserMsg_SetNextAutobuyItem::get_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_SetNextAutobuyItem>(
                    "CDOTAUserMsg_SetNextAutobuyItem",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_SetNextAutobuyItem {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_SetNextAutobuyItem {
    fn eq(&self, other: &CDOTAUserMsg_SetNextAutobuyItem) -> bool {
        self.name == other.name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_SetNextAutobuyItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_UnitEvent {
    // message fields
    msg_type: ::std::option::Option<EDotaEntityMessages>,
    entity_index: ::std::option::Option<i32>,
    speech: ::protobuf::SingularPtrField<CDOTAUserMsg_UnitEvent_Speech>,
    speech_mute: ::protobuf::SingularPtrField<CDOTAUserMsg_UnitEvent_SpeechMute>,
    add_gesture: ::protobuf::SingularPtrField<CDOTAUserMsg_UnitEvent_AddGesture>,
    remove_gesture: ::protobuf::SingularPtrField<CDOTAUserMsg_UnitEvent_RemoveGesture>,
    blood_impact: ::protobuf::SingularPtrField<CDOTAUserMsg_UnitEvent_BloodImpact>,
    fade_gesture: ::protobuf::SingularPtrField<CDOTAUserMsg_UnitEvent_FadeGesture>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_UnitEvent {}

impl CDOTAUserMsg_UnitEvent {
    pub fn new() -> CDOTAUserMsg_UnitEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_UnitEvent {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_UnitEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_UnitEvent,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_UnitEvent {
                    msg_type: ::std::option::Option::None,
                    entity_index: ::std::option::Option::None,
                    speech: ::protobuf::SingularPtrField::none(),
                    speech_mute: ::protobuf::SingularPtrField::none(),
                    add_gesture: ::protobuf::SingularPtrField::none(),
                    remove_gesture: ::protobuf::SingularPtrField::none(),
                    blood_impact: ::protobuf::SingularPtrField::none(),
                    fade_gesture: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .EDotaEntityMessages msg_type = 1;

    pub fn clear_msg_type(&mut self) {
        self.msg_type = ::std::option::Option::None;
    }

    pub fn has_msg_type(&self) -> bool {
        self.msg_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg_type(&mut self, v: EDotaEntityMessages) {
        self.msg_type = ::std::option::Option::Some(v);
    }

    pub fn get_msg_type(&self) -> EDotaEntityMessages {
        self.msg_type.unwrap_or(EDotaEntityMessages::DOTA_UNIT_SPEECH)
    }

    // required int32 entity_index = 2;

    pub fn clear_entity_index(&mut self) {
        self.entity_index = ::std::option::Option::None;
    }

    pub fn has_entity_index(&self) -> bool {
        self.entity_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity_index(&mut self, v: i32) {
        self.entity_index = ::std::option::Option::Some(v);
    }

    pub fn get_entity_index(&self) -> i32 {
        self.entity_index.unwrap_or(0)
    }

    // optional .CDOTAUserMsg_UnitEvent.Speech speech = 3;

    pub fn clear_speech(&mut self) {
        self.speech.clear();
    }

    pub fn has_speech(&self) -> bool {
        self.speech.is_some()
    }

    // Param is passed by value, moved
    pub fn set_speech(&mut self, v: CDOTAUserMsg_UnitEvent_Speech) {
        self.speech = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_speech(&mut self) -> &mut CDOTAUserMsg_UnitEvent_Speech {
        if self.speech.is_none() {
            self.speech.set_default();
        };
        self.speech.as_mut().unwrap()
    }

    // Take field
    pub fn take_speech(&mut self) -> CDOTAUserMsg_UnitEvent_Speech {
        self.speech.take().unwrap_or_else(|| CDOTAUserMsg_UnitEvent_Speech::new())
    }

    pub fn get_speech(&self) -> &CDOTAUserMsg_UnitEvent_Speech {
        self.speech.as_ref().unwrap_or_else(|| CDOTAUserMsg_UnitEvent_Speech::default_instance())
    }

    // optional .CDOTAUserMsg_UnitEvent.SpeechMute speech_mute = 4;

    pub fn clear_speech_mute(&mut self) {
        self.speech_mute.clear();
    }

    pub fn has_speech_mute(&self) -> bool {
        self.speech_mute.is_some()
    }

    // Param is passed by value, moved
    pub fn set_speech_mute(&mut self, v: CDOTAUserMsg_UnitEvent_SpeechMute) {
        self.speech_mute = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_speech_mute(&mut self) -> &mut CDOTAUserMsg_UnitEvent_SpeechMute {
        if self.speech_mute.is_none() {
            self.speech_mute.set_default();
        };
        self.speech_mute.as_mut().unwrap()
    }

    // Take field
    pub fn take_speech_mute(&mut self) -> CDOTAUserMsg_UnitEvent_SpeechMute {
        self.speech_mute.take().unwrap_or_else(|| CDOTAUserMsg_UnitEvent_SpeechMute::new())
    }

    pub fn get_speech_mute(&self) -> &CDOTAUserMsg_UnitEvent_SpeechMute {
        self.speech_mute.as_ref().unwrap_or_else(|| CDOTAUserMsg_UnitEvent_SpeechMute::default_instance())
    }

    // optional .CDOTAUserMsg_UnitEvent.AddGesture add_gesture = 5;

    pub fn clear_add_gesture(&mut self) {
        self.add_gesture.clear();
    }

    pub fn has_add_gesture(&self) -> bool {
        self.add_gesture.is_some()
    }

    // Param is passed by value, moved
    pub fn set_add_gesture(&mut self, v: CDOTAUserMsg_UnitEvent_AddGesture) {
        self.add_gesture = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_add_gesture(&mut self) -> &mut CDOTAUserMsg_UnitEvent_AddGesture {
        if self.add_gesture.is_none() {
            self.add_gesture.set_default();
        };
        self.add_gesture.as_mut().unwrap()
    }

    // Take field
    pub fn take_add_gesture(&mut self) -> CDOTAUserMsg_UnitEvent_AddGesture {
        self.add_gesture.take().unwrap_or_else(|| CDOTAUserMsg_UnitEvent_AddGesture::new())
    }

    pub fn get_add_gesture(&self) -> &CDOTAUserMsg_UnitEvent_AddGesture {
        self.add_gesture.as_ref().unwrap_or_else(|| CDOTAUserMsg_UnitEvent_AddGesture::default_instance())
    }

    // optional .CDOTAUserMsg_UnitEvent.RemoveGesture remove_gesture = 6;

    pub fn clear_remove_gesture(&mut self) {
        self.remove_gesture.clear();
    }

    pub fn has_remove_gesture(&self) -> bool {
        self.remove_gesture.is_some()
    }

    // Param is passed by value, moved
    pub fn set_remove_gesture(&mut self, v: CDOTAUserMsg_UnitEvent_RemoveGesture) {
        self.remove_gesture = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_remove_gesture(&mut self) -> &mut CDOTAUserMsg_UnitEvent_RemoveGesture {
        if self.remove_gesture.is_none() {
            self.remove_gesture.set_default();
        };
        self.remove_gesture.as_mut().unwrap()
    }

    // Take field
    pub fn take_remove_gesture(&mut self) -> CDOTAUserMsg_UnitEvent_RemoveGesture {
        self.remove_gesture.take().unwrap_or_else(|| CDOTAUserMsg_UnitEvent_RemoveGesture::new())
    }

    pub fn get_remove_gesture(&self) -> &CDOTAUserMsg_UnitEvent_RemoveGesture {
        self.remove_gesture.as_ref().unwrap_or_else(|| CDOTAUserMsg_UnitEvent_RemoveGesture::default_instance())
    }

    // optional .CDOTAUserMsg_UnitEvent.BloodImpact blood_impact = 7;

    pub fn clear_blood_impact(&mut self) {
        self.blood_impact.clear();
    }

    pub fn has_blood_impact(&self) -> bool {
        self.blood_impact.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blood_impact(&mut self, v: CDOTAUserMsg_UnitEvent_BloodImpact) {
        self.blood_impact = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blood_impact(&mut self) -> &mut CDOTAUserMsg_UnitEvent_BloodImpact {
        if self.blood_impact.is_none() {
            self.blood_impact.set_default();
        };
        self.blood_impact.as_mut().unwrap()
    }

    // Take field
    pub fn take_blood_impact(&mut self) -> CDOTAUserMsg_UnitEvent_BloodImpact {
        self.blood_impact.take().unwrap_or_else(|| CDOTAUserMsg_UnitEvent_BloodImpact::new())
    }

    pub fn get_blood_impact(&self) -> &CDOTAUserMsg_UnitEvent_BloodImpact {
        self.blood_impact.as_ref().unwrap_or_else(|| CDOTAUserMsg_UnitEvent_BloodImpact::default_instance())
    }

    // optional .CDOTAUserMsg_UnitEvent.FadeGesture fade_gesture = 8;

    pub fn clear_fade_gesture(&mut self) {
        self.fade_gesture.clear();
    }

    pub fn has_fade_gesture(&self) -> bool {
        self.fade_gesture.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fade_gesture(&mut self, v: CDOTAUserMsg_UnitEvent_FadeGesture) {
        self.fade_gesture = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fade_gesture(&mut self) -> &mut CDOTAUserMsg_UnitEvent_FadeGesture {
        if self.fade_gesture.is_none() {
            self.fade_gesture.set_default();
        };
        self.fade_gesture.as_mut().unwrap()
    }

    // Take field
    pub fn take_fade_gesture(&mut self) -> CDOTAUserMsg_UnitEvent_FadeGesture {
        self.fade_gesture.take().unwrap_or_else(|| CDOTAUserMsg_UnitEvent_FadeGesture::new())
    }

    pub fn get_fade_gesture(&self) -> &CDOTAUserMsg_UnitEvent_FadeGesture {
        self.fade_gesture.as_ref().unwrap_or_else(|| CDOTAUserMsg_UnitEvent_FadeGesture::default_instance())
    }
}

impl ::protobuf::Message for CDOTAUserMsg_UnitEvent {
    fn is_initialized(&self) -> bool {
        if self.msg_type.is_none() {
            return false;
        };
        if self.entity_index.is_none() {
            return false;
        };
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
                    let tmp = try!(is.read_enum());
                    self.msg_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.entity_index = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.speech));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.speech_mute));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.add_gesture));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.remove_gesture));
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.blood_impact));
                },
                8 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fade_gesture));
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
        for value in &self.msg_type {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.entity_index {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.speech {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.speech_mute {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.add_gesture {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.remove_gesture {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.blood_impact {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.fade_gesture {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.msg_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.entity_index {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.speech.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.speech_mute.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.add_gesture.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.remove_gesture.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.blood_impact.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.fade_gesture.as_ref() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_UnitEvent>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_UnitEvent {
    fn new() -> CDOTAUserMsg_UnitEvent {
        CDOTAUserMsg_UnitEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_UnitEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "msg_type",
                    CDOTAUserMsg_UnitEvent::has_msg_type,
                    CDOTAUserMsg_UnitEvent::get_msg_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "entity_index",
                    CDOTAUserMsg_UnitEvent::has_entity_index,
                    CDOTAUserMsg_UnitEvent::get_entity_index,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "speech",
                    CDOTAUserMsg_UnitEvent::has_speech,
                    CDOTAUserMsg_UnitEvent::get_speech,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "speech_mute",
                    CDOTAUserMsg_UnitEvent::has_speech_mute,
                    CDOTAUserMsg_UnitEvent::get_speech_mute,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "add_gesture",
                    CDOTAUserMsg_UnitEvent::has_add_gesture,
                    CDOTAUserMsg_UnitEvent::get_add_gesture,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "remove_gesture",
                    CDOTAUserMsg_UnitEvent::has_remove_gesture,
                    CDOTAUserMsg_UnitEvent::get_remove_gesture,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "blood_impact",
                    CDOTAUserMsg_UnitEvent::has_blood_impact,
                    CDOTAUserMsg_UnitEvent::get_blood_impact,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "fade_gesture",
                    CDOTAUserMsg_UnitEvent::has_fade_gesture,
                    CDOTAUserMsg_UnitEvent::get_fade_gesture,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_UnitEvent>(
                    "CDOTAUserMsg_UnitEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_UnitEvent {
    fn clear(&mut self) {
        self.clear_msg_type();
        self.clear_entity_index();
        self.clear_speech();
        self.clear_speech_mute();
        self.clear_add_gesture();
        self.clear_remove_gesture();
        self.clear_blood_impact();
        self.clear_fade_gesture();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_UnitEvent {
    fn eq(&self, other: &CDOTAUserMsg_UnitEvent) -> bool {
        self.msg_type == other.msg_type &&
        self.entity_index == other.entity_index &&
        self.speech == other.speech &&
        self.speech_mute == other.speech_mute &&
        self.add_gesture == other.add_gesture &&
        self.remove_gesture == other.remove_gesture &&
        self.blood_impact == other.blood_impact &&
        self.fade_gesture == other.fade_gesture &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_UnitEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_UnitEvent_Speech {
    // message fields
    concept: ::std::option::Option<i32>,
    response: ::protobuf::SingularField<::std::string::String>,
    recipient_type: ::std::option::Option<i32>,
    level: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_UnitEvent_Speech {}

impl CDOTAUserMsg_UnitEvent_Speech {
    pub fn new() -> CDOTAUserMsg_UnitEvent_Speech {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_UnitEvent_Speech {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_UnitEvent_Speech> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_UnitEvent_Speech,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_UnitEvent_Speech {
                    concept: ::std::option::Option::None,
                    response: ::protobuf::SingularField::none(),
                    recipient_type: ::std::option::Option::None,
                    level: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 concept = 1;

    pub fn clear_concept(&mut self) {
        self.concept = ::std::option::Option::None;
    }

    pub fn has_concept(&self) -> bool {
        self.concept.is_some()
    }

    // Param is passed by value, moved
    pub fn set_concept(&mut self, v: i32) {
        self.concept = ::std::option::Option::Some(v);
    }

    pub fn get_concept(&self) -> i32 {
        self.concept.unwrap_or(0)
    }

    // optional string response = 2;

    pub fn clear_response(&mut self) {
        self.response.clear();
    }

    pub fn has_response(&self) -> bool {
        self.response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_response(&mut self, v: ::std::string::String) {
        self.response = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_response(&mut self) -> &mut ::std::string::String {
        if self.response.is_none() {
            self.response.set_default();
        };
        self.response.as_mut().unwrap()
    }

    // Take field
    pub fn take_response(&mut self) -> ::std::string::String {
        self.response.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_response(&self) -> &str {
        match self.response.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional int32 recipient_type = 3;

    pub fn clear_recipient_type(&mut self) {
        self.recipient_type = ::std::option::Option::None;
    }

    pub fn has_recipient_type(&self) -> bool {
        self.recipient_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_recipient_type(&mut self, v: i32) {
        self.recipient_type = ::std::option::Option::Some(v);
    }

    pub fn get_recipient_type(&self) -> i32 {
        self.recipient_type.unwrap_or(0)
    }

    // optional int32 level = 4;

    pub fn clear_level(&mut self) {
        self.level = ::std::option::Option::None;
    }

    pub fn has_level(&self) -> bool {
        self.level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_level(&mut self, v: i32) {
        self.level = ::std::option::Option::Some(v);
    }

    pub fn get_level(&self) -> i32 {
        self.level.unwrap_or(0)
    }
}

impl ::protobuf::Message for CDOTAUserMsg_UnitEvent_Speech {
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
                    self.concept = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.response));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.recipient_type = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.level = ::std::option::Option::Some(tmp);
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
        for value in &self.concept {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.response {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.recipient_type {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.level {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.concept {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.response.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.recipient_type {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.level {
            try!(os.write_int32(4, v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_UnitEvent_Speech>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_UnitEvent_Speech {
    fn new() -> CDOTAUserMsg_UnitEvent_Speech {
        CDOTAUserMsg_UnitEvent_Speech::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_UnitEvent_Speech>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "concept",
                    CDOTAUserMsg_UnitEvent_Speech::has_concept,
                    CDOTAUserMsg_UnitEvent_Speech::get_concept,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "response",
                    CDOTAUserMsg_UnitEvent_Speech::has_response,
                    CDOTAUserMsg_UnitEvent_Speech::get_response,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "recipient_type",
                    CDOTAUserMsg_UnitEvent_Speech::has_recipient_type,
                    CDOTAUserMsg_UnitEvent_Speech::get_recipient_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "level",
                    CDOTAUserMsg_UnitEvent_Speech::has_level,
                    CDOTAUserMsg_UnitEvent_Speech::get_level,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_UnitEvent_Speech>(
                    "CDOTAUserMsg_UnitEvent_Speech",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_UnitEvent_Speech {
    fn clear(&mut self) {
        self.clear_concept();
        self.clear_response();
        self.clear_recipient_type();
        self.clear_level();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_UnitEvent_Speech {
    fn eq(&self, other: &CDOTAUserMsg_UnitEvent_Speech) -> bool {
        self.concept == other.concept &&
        self.response == other.response &&
        self.recipient_type == other.recipient_type &&
        self.level == other.level &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_UnitEvent_Speech {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_UnitEvent_SpeechMute {
    // message fields
    delay: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_UnitEvent_SpeechMute {}

impl CDOTAUserMsg_UnitEvent_SpeechMute {
    pub fn new() -> CDOTAUserMsg_UnitEvent_SpeechMute {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_UnitEvent_SpeechMute {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_UnitEvent_SpeechMute> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_UnitEvent_SpeechMute,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_UnitEvent_SpeechMute {
                    delay: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional float delay = 1;

    pub fn clear_delay(&mut self) {
        self.delay = ::std::option::Option::None;
    }

    pub fn has_delay(&self) -> bool {
        self.delay.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delay(&mut self, v: f32) {
        self.delay = ::std::option::Option::Some(v);
    }

    pub fn get_delay(&self) -> f32 {
        self.delay.unwrap_or(0.5f32)
    }
}

impl ::protobuf::Message for CDOTAUserMsg_UnitEvent_SpeechMute {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.delay = ::std::option::Option::Some(tmp);
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
        if self.delay.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.delay {
            try!(os.write_float(1, v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_UnitEvent_SpeechMute>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_UnitEvent_SpeechMute {
    fn new() -> CDOTAUserMsg_UnitEvent_SpeechMute {
        CDOTAUserMsg_UnitEvent_SpeechMute::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_UnitEvent_SpeechMute>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "delay",
                    CDOTAUserMsg_UnitEvent_SpeechMute::has_delay,
                    CDOTAUserMsg_UnitEvent_SpeechMute::get_delay,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_UnitEvent_SpeechMute>(
                    "CDOTAUserMsg_UnitEvent_SpeechMute",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_UnitEvent_SpeechMute {
    fn clear(&mut self) {
        self.clear_delay();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_UnitEvent_SpeechMute {
    fn eq(&self, other: &CDOTAUserMsg_UnitEvent_SpeechMute) -> bool {
        self.delay == other.delay &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_UnitEvent_SpeechMute {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_UnitEvent_AddGesture {
    // message fields
    activity: ::std::option::Option<super::ai_activity::Activity>,
    slot: ::std::option::Option<i32>,
    fade_in: ::std::option::Option<f32>,
    fade_out: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_UnitEvent_AddGesture {}

impl CDOTAUserMsg_UnitEvent_AddGesture {
    pub fn new() -> CDOTAUserMsg_UnitEvent_AddGesture {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_UnitEvent_AddGesture {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_UnitEvent_AddGesture> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_UnitEvent_AddGesture,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_UnitEvent_AddGesture {
                    activity: ::std::option::Option::None,
                    slot: ::std::option::Option::None,
                    fade_in: ::std::option::Option::None,
                    fade_out: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .Activity activity = 1;

    pub fn clear_activity(&mut self) {
        self.activity = ::std::option::Option::None;
    }

    pub fn has_activity(&self) -> bool {
        self.activity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_activity(&mut self, v: super::ai_activity::Activity) {
        self.activity = ::std::option::Option::Some(v);
    }

    pub fn get_activity(&self) -> super::ai_activity::Activity {
        self.activity.unwrap_or(super::ai_activity::Activity::ACT_INVALID)
    }

    // optional int32 slot = 2;

    pub fn clear_slot(&mut self) {
        self.slot = ::std::option::Option::None;
    }

    pub fn has_slot(&self) -> bool {
        self.slot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slot(&mut self, v: i32) {
        self.slot = ::std::option::Option::Some(v);
    }

    pub fn get_slot(&self) -> i32 {
        self.slot.unwrap_or(0)
    }

    // optional float fade_in = 3;

    pub fn clear_fade_in(&mut self) {
        self.fade_in = ::std::option::Option::None;
    }

    pub fn has_fade_in(&self) -> bool {
        self.fade_in.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fade_in(&mut self, v: f32) {
        self.fade_in = ::std::option::Option::Some(v);
    }

    pub fn get_fade_in(&self) -> f32 {
        self.fade_in.unwrap_or(0f32)
    }

    // optional float fade_out = 4;

    pub fn clear_fade_out(&mut self) {
        self.fade_out = ::std::option::Option::None;
    }

    pub fn has_fade_out(&self) -> bool {
        self.fade_out.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fade_out(&mut self, v: f32) {
        self.fade_out = ::std::option::Option::Some(v);
    }

    pub fn get_fade_out(&self) -> f32 {
        self.fade_out.unwrap_or(0.1f32)
    }
}

impl ::protobuf::Message for CDOTAUserMsg_UnitEvent_AddGesture {
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
                    let tmp = try!(is.read_enum());
                    self.activity = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.slot = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.fade_in = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.fade_out = ::std::option::Option::Some(tmp);
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
        for value in &self.activity {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.slot {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.fade_in.is_some() {
            my_size += 5;
        };
        if self.fade_out.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.activity {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.slot {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.fade_in {
            try!(os.write_float(3, v));
        };
        if let Some(v) = self.fade_out {
            try!(os.write_float(4, v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_UnitEvent_AddGesture>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_UnitEvent_AddGesture {
    fn new() -> CDOTAUserMsg_UnitEvent_AddGesture {
        CDOTAUserMsg_UnitEvent_AddGesture::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_UnitEvent_AddGesture>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "activity",
                    CDOTAUserMsg_UnitEvent_AddGesture::has_activity,
                    CDOTAUserMsg_UnitEvent_AddGesture::get_activity,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "slot",
                    CDOTAUserMsg_UnitEvent_AddGesture::has_slot,
                    CDOTAUserMsg_UnitEvent_AddGesture::get_slot,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "fade_in",
                    CDOTAUserMsg_UnitEvent_AddGesture::has_fade_in,
                    CDOTAUserMsg_UnitEvent_AddGesture::get_fade_in,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "fade_out",
                    CDOTAUserMsg_UnitEvent_AddGesture::has_fade_out,
                    CDOTAUserMsg_UnitEvent_AddGesture::get_fade_out,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_UnitEvent_AddGesture>(
                    "CDOTAUserMsg_UnitEvent_AddGesture",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_UnitEvent_AddGesture {
    fn clear(&mut self) {
        self.clear_activity();
        self.clear_slot();
        self.clear_fade_in();
        self.clear_fade_out();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_UnitEvent_AddGesture {
    fn eq(&self, other: &CDOTAUserMsg_UnitEvent_AddGesture) -> bool {
        self.activity == other.activity &&
        self.slot == other.slot &&
        self.fade_in == other.fade_in &&
        self.fade_out == other.fade_out &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_UnitEvent_AddGesture {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_UnitEvent_RemoveGesture {
    // message fields
    activity: ::std::option::Option<super::ai_activity::Activity>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_UnitEvent_RemoveGesture {}

impl CDOTAUserMsg_UnitEvent_RemoveGesture {
    pub fn new() -> CDOTAUserMsg_UnitEvent_RemoveGesture {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_UnitEvent_RemoveGesture {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_UnitEvent_RemoveGesture> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_UnitEvent_RemoveGesture,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_UnitEvent_RemoveGesture {
                    activity: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .Activity activity = 1;

    pub fn clear_activity(&mut self) {
        self.activity = ::std::option::Option::None;
    }

    pub fn has_activity(&self) -> bool {
        self.activity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_activity(&mut self, v: super::ai_activity::Activity) {
        self.activity = ::std::option::Option::Some(v);
    }

    pub fn get_activity(&self) -> super::ai_activity::Activity {
        self.activity.unwrap_or(super::ai_activity::Activity::ACT_INVALID)
    }
}

impl ::protobuf::Message for CDOTAUserMsg_UnitEvent_RemoveGesture {
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
                    let tmp = try!(is.read_enum());
                    self.activity = ::std::option::Option::Some(tmp);
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
        for value in &self.activity {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.activity {
            try!(os.write_enum(1, v.value()));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_UnitEvent_RemoveGesture>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_UnitEvent_RemoveGesture {
    fn new() -> CDOTAUserMsg_UnitEvent_RemoveGesture {
        CDOTAUserMsg_UnitEvent_RemoveGesture::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_UnitEvent_RemoveGesture>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "activity",
                    CDOTAUserMsg_UnitEvent_RemoveGesture::has_activity,
                    CDOTAUserMsg_UnitEvent_RemoveGesture::get_activity,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_UnitEvent_RemoveGesture>(
                    "CDOTAUserMsg_UnitEvent_RemoveGesture",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_UnitEvent_RemoveGesture {
    fn clear(&mut self) {
        self.clear_activity();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_UnitEvent_RemoveGesture {
    fn eq(&self, other: &CDOTAUserMsg_UnitEvent_RemoveGesture) -> bool {
        self.activity == other.activity &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_UnitEvent_RemoveGesture {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_UnitEvent_BloodImpact {
    // message fields
    scale: ::std::option::Option<i32>,
    x_normal: ::std::option::Option<i32>,
    y_normal: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_UnitEvent_BloodImpact {}

impl CDOTAUserMsg_UnitEvent_BloodImpact {
    pub fn new() -> CDOTAUserMsg_UnitEvent_BloodImpact {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_UnitEvent_BloodImpact {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_UnitEvent_BloodImpact> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_UnitEvent_BloodImpact,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_UnitEvent_BloodImpact {
                    scale: ::std::option::Option::None,
                    x_normal: ::std::option::Option::None,
                    y_normal: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 scale = 1;

    pub fn clear_scale(&mut self) {
        self.scale = ::std::option::Option::None;
    }

    pub fn has_scale(&self) -> bool {
        self.scale.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scale(&mut self, v: i32) {
        self.scale = ::std::option::Option::Some(v);
    }

    pub fn get_scale(&self) -> i32 {
        self.scale.unwrap_or(0)
    }

    // optional int32 x_normal = 2;

    pub fn clear_x_normal(&mut self) {
        self.x_normal = ::std::option::Option::None;
    }

    pub fn has_x_normal(&self) -> bool {
        self.x_normal.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x_normal(&mut self, v: i32) {
        self.x_normal = ::std::option::Option::Some(v);
    }

    pub fn get_x_normal(&self) -> i32 {
        self.x_normal.unwrap_or(0)
    }

    // optional int32 y_normal = 3;

    pub fn clear_y_normal(&mut self) {
        self.y_normal = ::std::option::Option::None;
    }

    pub fn has_y_normal(&self) -> bool {
        self.y_normal.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y_normal(&mut self, v: i32) {
        self.y_normal = ::std::option::Option::Some(v);
    }

    pub fn get_y_normal(&self) -> i32 {
        self.y_normal.unwrap_or(0)
    }
}

impl ::protobuf::Message for CDOTAUserMsg_UnitEvent_BloodImpact {
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
                    self.scale = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.x_normal = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.y_normal = ::std::option::Option::Some(tmp);
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
        for value in &self.scale {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.x_normal {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.y_normal {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.scale {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.x_normal {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.y_normal {
            try!(os.write_int32(3, v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_UnitEvent_BloodImpact>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_UnitEvent_BloodImpact {
    fn new() -> CDOTAUserMsg_UnitEvent_BloodImpact {
        CDOTAUserMsg_UnitEvent_BloodImpact::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_UnitEvent_BloodImpact>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "scale",
                    CDOTAUserMsg_UnitEvent_BloodImpact::has_scale,
                    CDOTAUserMsg_UnitEvent_BloodImpact::get_scale,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "x_normal",
                    CDOTAUserMsg_UnitEvent_BloodImpact::has_x_normal,
                    CDOTAUserMsg_UnitEvent_BloodImpact::get_x_normal,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "y_normal",
                    CDOTAUserMsg_UnitEvent_BloodImpact::has_y_normal,
                    CDOTAUserMsg_UnitEvent_BloodImpact::get_y_normal,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_UnitEvent_BloodImpact>(
                    "CDOTAUserMsg_UnitEvent_BloodImpact",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_UnitEvent_BloodImpact {
    fn clear(&mut self) {
        self.clear_scale();
        self.clear_x_normal();
        self.clear_y_normal();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_UnitEvent_BloodImpact {
    fn eq(&self, other: &CDOTAUserMsg_UnitEvent_BloodImpact) -> bool {
        self.scale == other.scale &&
        self.x_normal == other.x_normal &&
        self.y_normal == other.y_normal &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_UnitEvent_BloodImpact {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_UnitEvent_FadeGesture {
    // message fields
    activity: ::std::option::Option<super::ai_activity::Activity>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_UnitEvent_FadeGesture {}

impl CDOTAUserMsg_UnitEvent_FadeGesture {
    pub fn new() -> CDOTAUserMsg_UnitEvent_FadeGesture {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_UnitEvent_FadeGesture {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_UnitEvent_FadeGesture> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_UnitEvent_FadeGesture,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_UnitEvent_FadeGesture {
                    activity: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .Activity activity = 1;

    pub fn clear_activity(&mut self) {
        self.activity = ::std::option::Option::None;
    }

    pub fn has_activity(&self) -> bool {
        self.activity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_activity(&mut self, v: super::ai_activity::Activity) {
        self.activity = ::std::option::Option::Some(v);
    }

    pub fn get_activity(&self) -> super::ai_activity::Activity {
        self.activity.unwrap_or(super::ai_activity::Activity::ACT_INVALID)
    }
}

impl ::protobuf::Message for CDOTAUserMsg_UnitEvent_FadeGesture {
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
                    let tmp = try!(is.read_enum());
                    self.activity = ::std::option::Option::Some(tmp);
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
        for value in &self.activity {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.activity {
            try!(os.write_enum(1, v.value()));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_UnitEvent_FadeGesture>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_UnitEvent_FadeGesture {
    fn new() -> CDOTAUserMsg_UnitEvent_FadeGesture {
        CDOTAUserMsg_UnitEvent_FadeGesture::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_UnitEvent_FadeGesture>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "activity",
                    CDOTAUserMsg_UnitEvent_FadeGesture::has_activity,
                    CDOTAUserMsg_UnitEvent_FadeGesture::get_activity,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_UnitEvent_FadeGesture>(
                    "CDOTAUserMsg_UnitEvent_FadeGesture",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_UnitEvent_FadeGesture {
    fn clear(&mut self) {
        self.clear_activity();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_UnitEvent_FadeGesture {
    fn eq(&self, other: &CDOTAUserMsg_UnitEvent_FadeGesture) -> bool {
        self.activity == other.activity &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_UnitEvent_FadeGesture {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_ItemPurchased {
    // message fields
    item_index: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_ItemPurchased {}

impl CDOTAUserMsg_ItemPurchased {
    pub fn new() -> CDOTAUserMsg_ItemPurchased {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_ItemPurchased {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_ItemPurchased> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_ItemPurchased,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_ItemPurchased {
                    item_index: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 item_index = 1;

    pub fn clear_item_index(&mut self) {
        self.item_index = ::std::option::Option::None;
    }

    pub fn has_item_index(&self) -> bool {
        self.item_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_index(&mut self, v: i32) {
        self.item_index = ::std::option::Option::Some(v);
    }

    pub fn get_item_index(&self) -> i32 {
        self.item_index.unwrap_or(0)
    }
}

impl ::protobuf::Message for CDOTAUserMsg_ItemPurchased {
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
                    self.item_index = ::std::option::Option::Some(tmp);
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
        for value in &self.item_index {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_index {
            try!(os.write_int32(1, v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_ItemPurchased>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_ItemPurchased {
    fn new() -> CDOTAUserMsg_ItemPurchased {
        CDOTAUserMsg_ItemPurchased::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_ItemPurchased>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "item_index",
                    CDOTAUserMsg_ItemPurchased::has_item_index,
                    CDOTAUserMsg_ItemPurchased::get_item_index,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_ItemPurchased>(
                    "CDOTAUserMsg_ItemPurchased",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_ItemPurchased {
    fn clear(&mut self) {
        self.clear_item_index();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_ItemPurchased {
    fn eq(&self, other: &CDOTAUserMsg_ItemPurchased) -> bool {
        self.item_index == other.item_index &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_ItemPurchased {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_ItemFound {
    // message fields
    player: ::std::option::Option<i32>,
    quality: ::std::option::Option<i32>,
    rarity: ::std::option::Option<i32>,
    method: ::std::option::Option<i32>,
    itemdef: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_ItemFound {}

impl CDOTAUserMsg_ItemFound {
    pub fn new() -> CDOTAUserMsg_ItemFound {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_ItemFound {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_ItemFound> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_ItemFound,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_ItemFound {
                    player: ::std::option::Option::None,
                    quality: ::std::option::Option::None,
                    rarity: ::std::option::Option::None,
                    method: ::std::option::Option::None,
                    itemdef: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 player = 1;

    pub fn clear_player(&mut self) {
        self.player = ::std::option::Option::None;
    }

    pub fn has_player(&self) -> bool {
        self.player.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player(&mut self, v: i32) {
        self.player = ::std::option::Option::Some(v);
    }

    pub fn get_player(&self) -> i32 {
        self.player.unwrap_or(0)
    }

    // optional int32 quality = 2;

    pub fn clear_quality(&mut self) {
        self.quality = ::std::option::Option::None;
    }

    pub fn has_quality(&self) -> bool {
        self.quality.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality(&mut self, v: i32) {
        self.quality = ::std::option::Option::Some(v);
    }

    pub fn get_quality(&self) -> i32 {
        self.quality.unwrap_or(0)
    }

    // optional int32 rarity = 3;

    pub fn clear_rarity(&mut self) {
        self.rarity = ::std::option::Option::None;
    }

    pub fn has_rarity(&self) -> bool {
        self.rarity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rarity(&mut self, v: i32) {
        self.rarity = ::std::option::Option::Some(v);
    }

    pub fn get_rarity(&self) -> i32 {
        self.rarity.unwrap_or(0)
    }

    // optional int32 method = 4;

    pub fn clear_method(&mut self) {
        self.method = ::std::option::Option::None;
    }

    pub fn has_method(&self) -> bool {
        self.method.is_some()
    }

    // Param is passed by value, moved
    pub fn set_method(&mut self, v: i32) {
        self.method = ::std::option::Option::Some(v);
    }

    pub fn get_method(&self) -> i32 {
        self.method.unwrap_or(0)
    }

    // optional int32 itemdef = 5;

    pub fn clear_itemdef(&mut self) {
        self.itemdef = ::std::option::Option::None;
    }

    pub fn has_itemdef(&self) -> bool {
        self.itemdef.is_some()
    }

    // Param is passed by value, moved
    pub fn set_itemdef(&mut self, v: i32) {
        self.itemdef = ::std::option::Option::Some(v);
    }

    pub fn get_itemdef(&self) -> i32 {
        self.itemdef.unwrap_or(0)
    }
}

impl ::protobuf::Message for CDOTAUserMsg_ItemFound {
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
                    self.player = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.quality = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.rarity = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.method = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.itemdef = ::std::option::Option::Some(tmp);
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
        for value in &self.player {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.quality {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.rarity {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.method {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.itemdef {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.player {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.quality {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.rarity {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.method {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.itemdef {
            try!(os.write_int32(5, v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_ItemFound>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_ItemFound {
    fn new() -> CDOTAUserMsg_ItemFound {
        CDOTAUserMsg_ItemFound::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_ItemFound>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "player",
                    CDOTAUserMsg_ItemFound::has_player,
                    CDOTAUserMsg_ItemFound::get_player,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "quality",
                    CDOTAUserMsg_ItemFound::has_quality,
                    CDOTAUserMsg_ItemFound::get_quality,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "rarity",
                    CDOTAUserMsg_ItemFound::has_rarity,
                    CDOTAUserMsg_ItemFound::get_rarity,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "method",
                    CDOTAUserMsg_ItemFound::has_method,
                    CDOTAUserMsg_ItemFound::get_method,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "itemdef",
                    CDOTAUserMsg_ItemFound::has_itemdef,
                    CDOTAUserMsg_ItemFound::get_itemdef,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_ItemFound>(
                    "CDOTAUserMsg_ItemFound",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_ItemFound {
    fn clear(&mut self) {
        self.clear_player();
        self.clear_quality();
        self.clear_rarity();
        self.clear_method();
        self.clear_itemdef();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_ItemFound {
    fn eq(&self, other: &CDOTAUserMsg_ItemFound) -> bool {
        self.player == other.player &&
        self.quality == other.quality &&
        self.rarity == other.rarity &&
        self.method == other.method &&
        self.itemdef == other.itemdef &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_ItemFound {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_ParticleManager {
    // message fields
    field_type: ::std::option::Option<DOTA_PARTICLE_MESSAGE>,
    index: ::std::option::Option<u32>,
    release_particle_index: ::protobuf::SingularPtrField<CDOTAUserMsg_ParticleManager_ReleaseParticleIndex>,
    create_particle: ::protobuf::SingularPtrField<CDOTAUserMsg_ParticleManager_CreateParticle>,
    destroy_particle: ::protobuf::SingularPtrField<CDOTAUserMsg_ParticleManager_DestroyParticle>,
    destroy_particle_involving: ::protobuf::SingularPtrField<CDOTAUserMsg_ParticleManager_DestroyParticleInvolving>,
    update_particle: ::protobuf::SingularPtrField<CDOTAUserMsg_ParticleManager_UpdateParticle>,
    update_particle_fwd: ::protobuf::SingularPtrField<CDOTAUserMsg_ParticleManager_UpdateParticleFwd>,
    update_particle_orient: ::protobuf::SingularPtrField<CDOTAUserMsg_ParticleManager_UpdateParticleOrient>,
    update_particle_fallback: ::protobuf::SingularPtrField<CDOTAUserMsg_ParticleManager_UpdateParticleFallback>,
    update_particle_offset: ::protobuf::SingularPtrField<CDOTAUserMsg_ParticleManager_UpdateParticleOffset>,
    update_particle_ent: ::protobuf::SingularPtrField<CDOTAUserMsg_ParticleManager_UpdateParticleEnt>,
    update_particle_latency: ::protobuf::SingularPtrField<CDOTAUserMsg_ParticleManager_UpdateParticleLatency>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_ParticleManager {}

impl CDOTAUserMsg_ParticleManager {
    pub fn new() -> CDOTAUserMsg_ParticleManager {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_ParticleManager {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_ParticleManager> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_ParticleManager,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_ParticleManager {
                    field_type: ::std::option::Option::None,
                    index: ::std::option::Option::None,
                    release_particle_index: ::protobuf::SingularPtrField::none(),
                    create_particle: ::protobuf::SingularPtrField::none(),
                    destroy_particle: ::protobuf::SingularPtrField::none(),
                    destroy_particle_involving: ::protobuf::SingularPtrField::none(),
                    update_particle: ::protobuf::SingularPtrField::none(),
                    update_particle_fwd: ::protobuf::SingularPtrField::none(),
                    update_particle_orient: ::protobuf::SingularPtrField::none(),
                    update_particle_fallback: ::protobuf::SingularPtrField::none(),
                    update_particle_offset: ::protobuf::SingularPtrField::none(),
                    update_particle_ent: ::protobuf::SingularPtrField::none(),
                    update_particle_latency: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .DOTA_PARTICLE_MESSAGE type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: DOTA_PARTICLE_MESSAGE) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> DOTA_PARTICLE_MESSAGE {
        self.field_type.unwrap_or(DOTA_PARTICLE_MESSAGE::DOTA_PARTICLE_MANAGER_EVENT_CREATE)
    }

    // required uint32 index = 2;

    pub fn clear_index(&mut self) {
        self.index = ::std::option::Option::None;
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: u32) {
        self.index = ::std::option::Option::Some(v);
    }

    pub fn get_index(&self) -> u32 {
        self.index.unwrap_or(0)
    }

    // optional .CDOTAUserMsg_ParticleManager.ReleaseParticleIndex release_particle_index = 3;

    pub fn clear_release_particle_index(&mut self) {
        self.release_particle_index.clear();
    }

    pub fn has_release_particle_index(&self) -> bool {
        self.release_particle_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_release_particle_index(&mut self, v: CDOTAUserMsg_ParticleManager_ReleaseParticleIndex) {
        self.release_particle_index = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_release_particle_index(&mut self) -> &mut CDOTAUserMsg_ParticleManager_ReleaseParticleIndex {
        if self.release_particle_index.is_none() {
            self.release_particle_index.set_default();
        };
        self.release_particle_index.as_mut().unwrap()
    }

    // Take field
    pub fn take_release_particle_index(&mut self) -> CDOTAUserMsg_ParticleManager_ReleaseParticleIndex {
        self.release_particle_index.take().unwrap_or_else(|| CDOTAUserMsg_ParticleManager_ReleaseParticleIndex::new())
    }

    pub fn get_release_particle_index(&self) -> &CDOTAUserMsg_ParticleManager_ReleaseParticleIndex {
        self.release_particle_index.as_ref().unwrap_or_else(|| CDOTAUserMsg_ParticleManager_ReleaseParticleIndex::default_instance())
    }

    // optional .CDOTAUserMsg_ParticleManager.CreateParticle create_particle = 4;

    pub fn clear_create_particle(&mut self) {
        self.create_particle.clear();
    }

    pub fn has_create_particle(&self) -> bool {
        self.create_particle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_create_particle(&mut self, v: CDOTAUserMsg_ParticleManager_CreateParticle) {
        self.create_particle = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_create_particle(&mut self) -> &mut CDOTAUserMsg_ParticleManager_CreateParticle {
        if self.create_particle.is_none() {
            self.create_particle.set_default();
        };
        self.create_particle.as_mut().unwrap()
    }

    // Take field
    pub fn take_create_particle(&mut self) -> CDOTAUserMsg_ParticleManager_CreateParticle {
        self.create_particle.take().unwrap_or_else(|| CDOTAUserMsg_ParticleManager_CreateParticle::new())
    }

    pub fn get_create_particle(&self) -> &CDOTAUserMsg_ParticleManager_CreateParticle {
        self.create_particle.as_ref().unwrap_or_else(|| CDOTAUserMsg_ParticleManager_CreateParticle::default_instance())
    }

    // optional .CDOTAUserMsg_ParticleManager.DestroyParticle destroy_particle = 5;

    pub fn clear_destroy_particle(&mut self) {
        self.destroy_particle.clear();
    }

    pub fn has_destroy_particle(&self) -> bool {
        self.destroy_particle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_destroy_particle(&mut self, v: CDOTAUserMsg_ParticleManager_DestroyParticle) {
        self.destroy_particle = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_destroy_particle(&mut self) -> &mut CDOTAUserMsg_ParticleManager_DestroyParticle {
        if self.destroy_particle.is_none() {
            self.destroy_particle.set_default();
        };
        self.destroy_particle.as_mut().unwrap()
    }

    // Take field
    pub fn take_destroy_particle(&mut self) -> CDOTAUserMsg_ParticleManager_DestroyParticle {
        self.destroy_particle.take().unwrap_or_else(|| CDOTAUserMsg_ParticleManager_DestroyParticle::new())
    }

    pub fn get_destroy_particle(&self) -> &CDOTAUserMsg_ParticleManager_DestroyParticle {
        self.destroy_particle.as_ref().unwrap_or_else(|| CDOTAUserMsg_ParticleManager_DestroyParticle::default_instance())
    }

    // optional .CDOTAUserMsg_ParticleManager.DestroyParticleInvolving destroy_particle_involving = 6;

    pub fn clear_destroy_particle_involving(&mut self) {
        self.destroy_particle_involving.clear();
    }

    pub fn has_destroy_particle_involving(&self) -> bool {
        self.destroy_particle_involving.is_some()
    }

    // Param is passed by value, moved
    pub fn set_destroy_particle_involving(&mut self, v: CDOTAUserMsg_ParticleManager_DestroyParticleInvolving) {
        self.destroy_particle_involving = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_destroy_particle_involving(&mut self) -> &mut CDOTAUserMsg_ParticleManager_DestroyParticleInvolving {
        if self.destroy_particle_involving.is_none() {
            self.destroy_particle_involving.set_default();
        };
        self.destroy_particle_involving.as_mut().unwrap()
    }

    // Take field
    pub fn take_destroy_particle_involving(&mut self) -> CDOTAUserMsg_ParticleManager_DestroyParticleInvolving {
        self.destroy_particle_involving.take().unwrap_or_else(|| CDOTAUserMsg_ParticleManager_DestroyParticleInvolving::new())
    }

    pub fn get_destroy_particle_involving(&self) -> &CDOTAUserMsg_ParticleManager_DestroyParticleInvolving {
        self.destroy_particle_involving.as_ref().unwrap_or_else(|| CDOTAUserMsg_ParticleManager_DestroyParticleInvolving::default_instance())
    }

    // optional .CDOTAUserMsg_ParticleManager.UpdateParticle update_particle = 7;

    pub fn clear_update_particle(&mut self) {
        self.update_particle.clear();
    }

    pub fn has_update_particle(&self) -> bool {
        self.update_particle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_particle(&mut self, v: CDOTAUserMsg_ParticleManager_UpdateParticle) {
        self.update_particle = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_update_particle(&mut self) -> &mut CDOTAUserMsg_ParticleManager_UpdateParticle {
        if self.update_particle.is_none() {
            self.update_particle.set_default();
        };
        self.update_particle.as_mut().unwrap()
    }

    // Take field
    pub fn take_update_particle(&mut self) -> CDOTAUserMsg_ParticleManager_UpdateParticle {
        self.update_particle.take().unwrap_or_else(|| CDOTAUserMsg_ParticleManager_UpdateParticle::new())
    }

    pub fn get_update_particle(&self) -> &CDOTAUserMsg_ParticleManager_UpdateParticle {
        self.update_particle.as_ref().unwrap_or_else(|| CDOTAUserMsg_ParticleManager_UpdateParticle::default_instance())
    }

    // optional .CDOTAUserMsg_ParticleManager.UpdateParticleFwd update_particle_fwd = 8;

    pub fn clear_update_particle_fwd(&mut self) {
        self.update_particle_fwd.clear();
    }

    pub fn has_update_particle_fwd(&self) -> bool {
        self.update_particle_fwd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_particle_fwd(&mut self, v: CDOTAUserMsg_ParticleManager_UpdateParticleFwd) {
        self.update_particle_fwd = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_update_particle_fwd(&mut self) -> &mut CDOTAUserMsg_ParticleManager_UpdateParticleFwd {
        if self.update_particle_fwd.is_none() {
            self.update_particle_fwd.set_default();
        };
        self.update_particle_fwd.as_mut().unwrap()
    }

    // Take field
    pub fn take_update_particle_fwd(&mut self) -> CDOTAUserMsg_ParticleManager_UpdateParticleFwd {
        self.update_particle_fwd.take().unwrap_or_else(|| CDOTAUserMsg_ParticleManager_UpdateParticleFwd::new())
    }

    pub fn get_update_particle_fwd(&self) -> &CDOTAUserMsg_ParticleManager_UpdateParticleFwd {
        self.update_particle_fwd.as_ref().unwrap_or_else(|| CDOTAUserMsg_ParticleManager_UpdateParticleFwd::default_instance())
    }

    // optional .CDOTAUserMsg_ParticleManager.UpdateParticleOrient update_particle_orient = 9;

    pub fn clear_update_particle_orient(&mut self) {
        self.update_particle_orient.clear();
    }

    pub fn has_update_particle_orient(&self) -> bool {
        self.update_particle_orient.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_particle_orient(&mut self, v: CDOTAUserMsg_ParticleManager_UpdateParticleOrient) {
        self.update_particle_orient = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_update_particle_orient(&mut self) -> &mut CDOTAUserMsg_ParticleManager_UpdateParticleOrient {
        if self.update_particle_orient.is_none() {
            self.update_particle_orient.set_default();
        };
        self.update_particle_orient.as_mut().unwrap()
    }

    // Take field
    pub fn take_update_particle_orient(&mut self) -> CDOTAUserMsg_ParticleManager_UpdateParticleOrient {
        self.update_particle_orient.take().unwrap_or_else(|| CDOTAUserMsg_ParticleManager_UpdateParticleOrient::new())
    }

    pub fn get_update_particle_orient(&self) -> &CDOTAUserMsg_ParticleManager_UpdateParticleOrient {
        self.update_particle_orient.as_ref().unwrap_or_else(|| CDOTAUserMsg_ParticleManager_UpdateParticleOrient::default_instance())
    }

    // optional .CDOTAUserMsg_ParticleManager.UpdateParticleFallback update_particle_fallback = 10;

    pub fn clear_update_particle_fallback(&mut self) {
        self.update_particle_fallback.clear();
    }

    pub fn has_update_particle_fallback(&self) -> bool {
        self.update_particle_fallback.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_particle_fallback(&mut self, v: CDOTAUserMsg_ParticleManager_UpdateParticleFallback) {
        self.update_particle_fallback = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_update_particle_fallback(&mut self) -> &mut CDOTAUserMsg_ParticleManager_UpdateParticleFallback {
        if self.update_particle_fallback.is_none() {
            self.update_particle_fallback.set_default();
        };
        self.update_particle_fallback.as_mut().unwrap()
    }

    // Take field
    pub fn take_update_particle_fallback(&mut self) -> CDOTAUserMsg_ParticleManager_UpdateParticleFallback {
        self.update_particle_fallback.take().unwrap_or_else(|| CDOTAUserMsg_ParticleManager_UpdateParticleFallback::new())
    }

    pub fn get_update_particle_fallback(&self) -> &CDOTAUserMsg_ParticleManager_UpdateParticleFallback {
        self.update_particle_fallback.as_ref().unwrap_or_else(|| CDOTAUserMsg_ParticleManager_UpdateParticleFallback::default_instance())
    }

    // optional .CDOTAUserMsg_ParticleManager.UpdateParticleOffset update_particle_offset = 11;

    pub fn clear_update_particle_offset(&mut self) {
        self.update_particle_offset.clear();
    }

    pub fn has_update_particle_offset(&self) -> bool {
        self.update_particle_offset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_particle_offset(&mut self, v: CDOTAUserMsg_ParticleManager_UpdateParticleOffset) {
        self.update_particle_offset = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_update_particle_offset(&mut self) -> &mut CDOTAUserMsg_ParticleManager_UpdateParticleOffset {
        if self.update_particle_offset.is_none() {
            self.update_particle_offset.set_default();
        };
        self.update_particle_offset.as_mut().unwrap()
    }

    // Take field
    pub fn take_update_particle_offset(&mut self) -> CDOTAUserMsg_ParticleManager_UpdateParticleOffset {
        self.update_particle_offset.take().unwrap_or_else(|| CDOTAUserMsg_ParticleManager_UpdateParticleOffset::new())
    }

    pub fn get_update_particle_offset(&self) -> &CDOTAUserMsg_ParticleManager_UpdateParticleOffset {
        self.update_particle_offset.as_ref().unwrap_or_else(|| CDOTAUserMsg_ParticleManager_UpdateParticleOffset::default_instance())
    }

    // optional .CDOTAUserMsg_ParticleManager.UpdateParticleEnt update_particle_ent = 12;

    pub fn clear_update_particle_ent(&mut self) {
        self.update_particle_ent.clear();
    }

    pub fn has_update_particle_ent(&self) -> bool {
        self.update_particle_ent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_particle_ent(&mut self, v: CDOTAUserMsg_ParticleManager_UpdateParticleEnt) {
        self.update_particle_ent = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_update_particle_ent(&mut self) -> &mut CDOTAUserMsg_ParticleManager_UpdateParticleEnt {
        if self.update_particle_ent.is_none() {
            self.update_particle_ent.set_default();
        };
        self.update_particle_ent.as_mut().unwrap()
    }

    // Take field
    pub fn take_update_particle_ent(&mut self) -> CDOTAUserMsg_ParticleManager_UpdateParticleEnt {
        self.update_particle_ent.take().unwrap_or_else(|| CDOTAUserMsg_ParticleManager_UpdateParticleEnt::new())
    }

    pub fn get_update_particle_ent(&self) -> &CDOTAUserMsg_ParticleManager_UpdateParticleEnt {
        self.update_particle_ent.as_ref().unwrap_or_else(|| CDOTAUserMsg_ParticleManager_UpdateParticleEnt::default_instance())
    }

    // optional .CDOTAUserMsg_ParticleManager.UpdateParticleLatency update_particle_latency = 13;

    pub fn clear_update_particle_latency(&mut self) {
        self.update_particle_latency.clear();
    }

    pub fn has_update_particle_latency(&self) -> bool {
        self.update_particle_latency.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_particle_latency(&mut self, v: CDOTAUserMsg_ParticleManager_UpdateParticleLatency) {
        self.update_particle_latency = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_update_particle_latency(&mut self) -> &mut CDOTAUserMsg_ParticleManager_UpdateParticleLatency {
        if self.update_particle_latency.is_none() {
            self.update_particle_latency.set_default();
        };
        self.update_particle_latency.as_mut().unwrap()
    }

    // Take field
    pub fn take_update_particle_latency(&mut self) -> CDOTAUserMsg_ParticleManager_UpdateParticleLatency {
        self.update_particle_latency.take().unwrap_or_else(|| CDOTAUserMsg_ParticleManager_UpdateParticleLatency::new())
    }

    pub fn get_update_particle_latency(&self) -> &CDOTAUserMsg_ParticleManager_UpdateParticleLatency {
        self.update_particle_latency.as_ref().unwrap_or_else(|| CDOTAUserMsg_ParticleManager_UpdateParticleLatency::default_instance())
    }
}

impl ::protobuf::Message for CDOTAUserMsg_ParticleManager {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        };
        if self.index.is_none() {
            return false;
        };
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
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.index = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.release_particle_index));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.create_particle));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.destroy_particle));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.destroy_particle_involving));
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.update_particle));
                },
                8 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.update_particle_fwd));
                },
                9 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.update_particle_orient));
                },
                10 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.update_particle_fallback));
                },
                11 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.update_particle_offset));
                },
                12 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.update_particle_ent));
                },
                13 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.update_particle_latency));
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
        for value in &self.field_type {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.index {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.release_particle_index {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.create_particle {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.destroy_particle {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.destroy_particle_involving {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.update_particle {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.update_particle_fwd {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.update_particle_orient {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.update_particle_fallback {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.update_particle_offset {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.update_particle_ent {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.update_particle_latency {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.index {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.release_particle_index.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.create_particle.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.destroy_particle.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.destroy_particle_involving.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.update_particle.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.update_particle_fwd.as_ref() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.update_particle_orient.as_ref() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.update_particle_fallback.as_ref() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.update_particle_offset.as_ref() {
            try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.update_particle_ent.as_ref() {
            try!(os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.update_particle_latency.as_ref() {
            try!(os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_ParticleManager>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_ParticleManager {
    fn new() -> CDOTAUserMsg_ParticleManager {
        CDOTAUserMsg_ParticleManager::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_ParticleManager>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    CDOTAUserMsg_ParticleManager::has_field_type,
                    CDOTAUserMsg_ParticleManager::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "index",
                    CDOTAUserMsg_ParticleManager::has_index,
                    CDOTAUserMsg_ParticleManager::get_index,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "release_particle_index",
                    CDOTAUserMsg_ParticleManager::has_release_particle_index,
                    CDOTAUserMsg_ParticleManager::get_release_particle_index,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "create_particle",
                    CDOTAUserMsg_ParticleManager::has_create_particle,
                    CDOTAUserMsg_ParticleManager::get_create_particle,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "destroy_particle",
                    CDOTAUserMsg_ParticleManager::has_destroy_particle,
                    CDOTAUserMsg_ParticleManager::get_destroy_particle,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "destroy_particle_involving",
                    CDOTAUserMsg_ParticleManager::has_destroy_particle_involving,
                    CDOTAUserMsg_ParticleManager::get_destroy_particle_involving,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "update_particle",
                    CDOTAUserMsg_ParticleManager::has_update_particle,
                    CDOTAUserMsg_ParticleManager::get_update_particle,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "update_particle_fwd",
                    CDOTAUserMsg_ParticleManager::has_update_particle_fwd,
                    CDOTAUserMsg_ParticleManager::get_update_particle_fwd,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "update_particle_orient",
                    CDOTAUserMsg_ParticleManager::has_update_particle_orient,
                    CDOTAUserMsg_ParticleManager::get_update_particle_orient,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "update_particle_fallback",
                    CDOTAUserMsg_ParticleManager::has_update_particle_fallback,
                    CDOTAUserMsg_ParticleManager::get_update_particle_fallback,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "update_particle_offset",
                    CDOTAUserMsg_ParticleManager::has_update_particle_offset,
                    CDOTAUserMsg_ParticleManager::get_update_particle_offset,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "update_particle_ent",
                    CDOTAUserMsg_ParticleManager::has_update_particle_ent,
                    CDOTAUserMsg_ParticleManager::get_update_particle_ent,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "update_particle_latency",
                    CDOTAUserMsg_ParticleManager::has_update_particle_latency,
                    CDOTAUserMsg_ParticleManager::get_update_particle_latency,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_ParticleManager>(
                    "CDOTAUserMsg_ParticleManager",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_ParticleManager {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_index();
        self.clear_release_particle_index();
        self.clear_create_particle();
        self.clear_destroy_particle();
        self.clear_destroy_particle_involving();
        self.clear_update_particle();
        self.clear_update_particle_fwd();
        self.clear_update_particle_orient();
        self.clear_update_particle_fallback();
        self.clear_update_particle_offset();
        self.clear_update_particle_ent();
        self.clear_update_particle_latency();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_ParticleManager {
    fn eq(&self, other: &CDOTAUserMsg_ParticleManager) -> bool {
        self.field_type == other.field_type &&
        self.index == other.index &&
        self.release_particle_index == other.release_particle_index &&
        self.create_particle == other.create_particle &&
        self.destroy_particle == other.destroy_particle &&
        self.destroy_particle_involving == other.destroy_particle_involving &&
        self.update_particle == other.update_particle &&
        self.update_particle_fwd == other.update_particle_fwd &&
        self.update_particle_orient == other.update_particle_orient &&
        self.update_particle_fallback == other.update_particle_fallback &&
        self.update_particle_offset == other.update_particle_offset &&
        self.update_particle_ent == other.update_particle_ent &&
        self.update_particle_latency == other.update_particle_latency &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_ParticleManager {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_ParticleManager_ReleaseParticleIndex {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_ParticleManager_ReleaseParticleIndex {}

impl CDOTAUserMsg_ParticleManager_ReleaseParticleIndex {
    pub fn new() -> CDOTAUserMsg_ParticleManager_ReleaseParticleIndex {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_ParticleManager_ReleaseParticleIndex {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_ParticleManager_ReleaseParticleIndex> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_ParticleManager_ReleaseParticleIndex,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_ParticleManager_ReleaseParticleIndex {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for CDOTAUserMsg_ParticleManager_ReleaseParticleIndex {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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
        ::std::any::TypeId::of::<CDOTAUserMsg_ParticleManager_ReleaseParticleIndex>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_ParticleManager_ReleaseParticleIndex {
    fn new() -> CDOTAUserMsg_ParticleManager_ReleaseParticleIndex {
        CDOTAUserMsg_ParticleManager_ReleaseParticleIndex::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_ParticleManager_ReleaseParticleIndex>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_ParticleManager_ReleaseParticleIndex>(
                    "CDOTAUserMsg_ParticleManager_ReleaseParticleIndex",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_ParticleManager_ReleaseParticleIndex {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_ParticleManager_ReleaseParticleIndex {
    fn eq(&self, other: &CDOTAUserMsg_ParticleManager_ReleaseParticleIndex) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_ParticleManager_ReleaseParticleIndex {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_ParticleManager_CreateParticle {
    // message fields
    particle_name_index: ::std::option::Option<i32>,
    attach_type: ::std::option::Option<i32>,
    entity_handle: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_ParticleManager_CreateParticle {}

impl CDOTAUserMsg_ParticleManager_CreateParticle {
    pub fn new() -> CDOTAUserMsg_ParticleManager_CreateParticle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_ParticleManager_CreateParticle {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_ParticleManager_CreateParticle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_ParticleManager_CreateParticle,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_ParticleManager_CreateParticle {
                    particle_name_index: ::std::option::Option::None,
                    attach_type: ::std::option::Option::None,
                    entity_handle: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 particle_name_index = 1;

    pub fn clear_particle_name_index(&mut self) {
        self.particle_name_index = ::std::option::Option::None;
    }

    pub fn has_particle_name_index(&self) -> bool {
        self.particle_name_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_particle_name_index(&mut self, v: i32) {
        self.particle_name_index = ::std::option::Option::Some(v);
    }

    pub fn get_particle_name_index(&self) -> i32 {
        self.particle_name_index.unwrap_or(0)
    }

    // optional int32 attach_type = 2;

    pub fn clear_attach_type(&mut self) {
        self.attach_type = ::std::option::Option::None;
    }

    pub fn has_attach_type(&self) -> bool {
        self.attach_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attach_type(&mut self, v: i32) {
        self.attach_type = ::std::option::Option::Some(v);
    }

    pub fn get_attach_type(&self) -> i32 {
        self.attach_type.unwrap_or(0)
    }

    // optional int32 entity_handle = 3;

    pub fn clear_entity_handle(&mut self) {
        self.entity_handle = ::std::option::Option::None;
    }

    pub fn has_entity_handle(&self) -> bool {
        self.entity_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity_handle(&mut self, v: i32) {
        self.entity_handle = ::std::option::Option::Some(v);
    }

    pub fn get_entity_handle(&self) -> i32 {
        self.entity_handle.unwrap_or(0)
    }
}

impl ::protobuf::Message for CDOTAUserMsg_ParticleManager_CreateParticle {
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
                    self.particle_name_index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.attach_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.entity_handle = ::std::option::Option::Some(tmp);
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
        for value in &self.particle_name_index {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.attach_type {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.entity_handle {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.particle_name_index {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.attach_type {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.entity_handle {
            try!(os.write_int32(3, v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_ParticleManager_CreateParticle>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_ParticleManager_CreateParticle {
    fn new() -> CDOTAUserMsg_ParticleManager_CreateParticle {
        CDOTAUserMsg_ParticleManager_CreateParticle::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_ParticleManager_CreateParticle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "particle_name_index",
                    CDOTAUserMsg_ParticleManager_CreateParticle::has_particle_name_index,
                    CDOTAUserMsg_ParticleManager_CreateParticle::get_particle_name_index,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "attach_type",
                    CDOTAUserMsg_ParticleManager_CreateParticle::has_attach_type,
                    CDOTAUserMsg_ParticleManager_CreateParticle::get_attach_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "entity_handle",
                    CDOTAUserMsg_ParticleManager_CreateParticle::has_entity_handle,
                    CDOTAUserMsg_ParticleManager_CreateParticle::get_entity_handle,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_ParticleManager_CreateParticle>(
                    "CDOTAUserMsg_ParticleManager_CreateParticle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_ParticleManager_CreateParticle {
    fn clear(&mut self) {
        self.clear_particle_name_index();
        self.clear_attach_type();
        self.clear_entity_handle();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_ParticleManager_CreateParticle {
    fn eq(&self, other: &CDOTAUserMsg_ParticleManager_CreateParticle) -> bool {
        self.particle_name_index == other.particle_name_index &&
        self.attach_type == other.attach_type &&
        self.entity_handle == other.entity_handle &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_ParticleManager_CreateParticle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_ParticleManager_DestroyParticle {
    // message fields
    destroy_immediately: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_ParticleManager_DestroyParticle {}

impl CDOTAUserMsg_ParticleManager_DestroyParticle {
    pub fn new() -> CDOTAUserMsg_ParticleManager_DestroyParticle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_ParticleManager_DestroyParticle {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_ParticleManager_DestroyParticle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_ParticleManager_DestroyParticle,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_ParticleManager_DestroyParticle {
                    destroy_immediately: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool destroy_immediately = 1;

    pub fn clear_destroy_immediately(&mut self) {
        self.destroy_immediately = ::std::option::Option::None;
    }

    pub fn has_destroy_immediately(&self) -> bool {
        self.destroy_immediately.is_some()
    }

    // Param is passed by value, moved
    pub fn set_destroy_immediately(&mut self, v: bool) {
        self.destroy_immediately = ::std::option::Option::Some(v);
    }

    pub fn get_destroy_immediately(&self) -> bool {
        self.destroy_immediately.unwrap_or(false)
    }
}

impl ::protobuf::Message for CDOTAUserMsg_ParticleManager_DestroyParticle {
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
                    let tmp = try!(is.read_bool());
                    self.destroy_immediately = ::std::option::Option::Some(tmp);
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
        if self.destroy_immediately.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.destroy_immediately {
            try!(os.write_bool(1, v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_ParticleManager_DestroyParticle>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_ParticleManager_DestroyParticle {
    fn new() -> CDOTAUserMsg_ParticleManager_DestroyParticle {
        CDOTAUserMsg_ParticleManager_DestroyParticle::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_ParticleManager_DestroyParticle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "destroy_immediately",
                    CDOTAUserMsg_ParticleManager_DestroyParticle::has_destroy_immediately,
                    CDOTAUserMsg_ParticleManager_DestroyParticle::get_destroy_immediately,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_ParticleManager_DestroyParticle>(
                    "CDOTAUserMsg_ParticleManager_DestroyParticle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_ParticleManager_DestroyParticle {
    fn clear(&mut self) {
        self.clear_destroy_immediately();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_ParticleManager_DestroyParticle {
    fn eq(&self, other: &CDOTAUserMsg_ParticleManager_DestroyParticle) -> bool {
        self.destroy_immediately == other.destroy_immediately &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_ParticleManager_DestroyParticle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_ParticleManager_DestroyParticleInvolving {
    // message fields
    destroy_immediately: ::std::option::Option<bool>,
    entity_handle: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_ParticleManager_DestroyParticleInvolving {}

impl CDOTAUserMsg_ParticleManager_DestroyParticleInvolving {
    pub fn new() -> CDOTAUserMsg_ParticleManager_DestroyParticleInvolving {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_ParticleManager_DestroyParticleInvolving {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_ParticleManager_DestroyParticleInvolving> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_ParticleManager_DestroyParticleInvolving,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_ParticleManager_DestroyParticleInvolving {
                    destroy_immediately: ::std::option::Option::None,
                    entity_handle: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool destroy_immediately = 1;

    pub fn clear_destroy_immediately(&mut self) {
        self.destroy_immediately = ::std::option::Option::None;
    }

    pub fn has_destroy_immediately(&self) -> bool {
        self.destroy_immediately.is_some()
    }

    // Param is passed by value, moved
    pub fn set_destroy_immediately(&mut self, v: bool) {
        self.destroy_immediately = ::std::option::Option::Some(v);
    }

    pub fn get_destroy_immediately(&self) -> bool {
        self.destroy_immediately.unwrap_or(false)
    }

    // optional int32 entity_handle = 3;

    pub fn clear_entity_handle(&mut self) {
        self.entity_handle = ::std::option::Option::None;
    }

    pub fn has_entity_handle(&self) -> bool {
        self.entity_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity_handle(&mut self, v: i32) {
        self.entity_handle = ::std::option::Option::Some(v);
    }

    pub fn get_entity_handle(&self) -> i32 {
        self.entity_handle.unwrap_or(0)
    }
}

impl ::protobuf::Message for CDOTAUserMsg_ParticleManager_DestroyParticleInvolving {
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
                    let tmp = try!(is.read_bool());
                    self.destroy_immediately = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.entity_handle = ::std::option::Option::Some(tmp);
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
        if self.destroy_immediately.is_some() {
            my_size += 2;
        };
        for value in &self.entity_handle {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.destroy_immediately {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.entity_handle {
            try!(os.write_int32(3, v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_ParticleManager_DestroyParticleInvolving>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_ParticleManager_DestroyParticleInvolving {
    fn new() -> CDOTAUserMsg_ParticleManager_DestroyParticleInvolving {
        CDOTAUserMsg_ParticleManager_DestroyParticleInvolving::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_ParticleManager_DestroyParticleInvolving>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "destroy_immediately",
                    CDOTAUserMsg_ParticleManager_DestroyParticleInvolving::has_destroy_immediately,
                    CDOTAUserMsg_ParticleManager_DestroyParticleInvolving::get_destroy_immediately,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "entity_handle",
                    CDOTAUserMsg_ParticleManager_DestroyParticleInvolving::has_entity_handle,
                    CDOTAUserMsg_ParticleManager_DestroyParticleInvolving::get_entity_handle,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_ParticleManager_DestroyParticleInvolving>(
                    "CDOTAUserMsg_ParticleManager_DestroyParticleInvolving",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_ParticleManager_DestroyParticleInvolving {
    fn clear(&mut self) {
        self.clear_destroy_immediately();
        self.clear_entity_handle();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_ParticleManager_DestroyParticleInvolving {
    fn eq(&self, other: &CDOTAUserMsg_ParticleManager_DestroyParticleInvolving) -> bool {
        self.destroy_immediately == other.destroy_immediately &&
        self.entity_handle == other.entity_handle &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_ParticleManager_DestroyParticleInvolving {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_ParticleManager_UpdateParticle {
    // message fields
    control_point: ::std::option::Option<i32>,
    position: ::protobuf::SingularPtrField<super::netmessages::CMsgVector>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_ParticleManager_UpdateParticle {}

impl CDOTAUserMsg_ParticleManager_UpdateParticle {
    pub fn new() -> CDOTAUserMsg_ParticleManager_UpdateParticle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_ParticleManager_UpdateParticle {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_ParticleManager_UpdateParticle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_ParticleManager_UpdateParticle,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_ParticleManager_UpdateParticle {
                    control_point: ::std::option::Option::None,
                    position: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 control_point = 1;

    pub fn clear_control_point(&mut self) {
        self.control_point = ::std::option::Option::None;
    }

    pub fn has_control_point(&self) -> bool {
        self.control_point.is_some()
    }

    // Param is passed by value, moved
    pub fn set_control_point(&mut self, v: i32) {
        self.control_point = ::std::option::Option::Some(v);
    }

    pub fn get_control_point(&self) -> i32 {
        self.control_point.unwrap_or(0)
    }

    // optional .CMsgVector position = 2;

    pub fn clear_position(&mut self) {
        self.position.clear();
    }

    pub fn has_position(&self) -> bool {
        self.position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_position(&mut self, v: super::netmessages::CMsgVector) {
        self.position = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_position(&mut self) -> &mut super::netmessages::CMsgVector {
        if self.position.is_none() {
            self.position.set_default();
        };
        self.position.as_mut().unwrap()
    }

    // Take field
    pub fn take_position(&mut self) -> super::netmessages::CMsgVector {
        self.position.take().unwrap_or_else(|| super::netmessages::CMsgVector::new())
    }

    pub fn get_position(&self) -> &super::netmessages::CMsgVector {
        self.position.as_ref().unwrap_or_else(|| super::netmessages::CMsgVector::default_instance())
    }
}

impl ::protobuf::Message for CDOTAUserMsg_ParticleManager_UpdateParticle {
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
                    self.control_point = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.position));
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
        for value in &self.control_point {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.position {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.control_point {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.position.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_ParticleManager_UpdateParticle>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_ParticleManager_UpdateParticle {
    fn new() -> CDOTAUserMsg_ParticleManager_UpdateParticle {
        CDOTAUserMsg_ParticleManager_UpdateParticle::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_ParticleManager_UpdateParticle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "control_point",
                    CDOTAUserMsg_ParticleManager_UpdateParticle::has_control_point,
                    CDOTAUserMsg_ParticleManager_UpdateParticle::get_control_point,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "position",
                    CDOTAUserMsg_ParticleManager_UpdateParticle::has_position,
                    CDOTAUserMsg_ParticleManager_UpdateParticle::get_position,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_ParticleManager_UpdateParticle>(
                    "CDOTAUserMsg_ParticleManager_UpdateParticle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_ParticleManager_UpdateParticle {
    fn clear(&mut self) {
        self.clear_control_point();
        self.clear_position();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_ParticleManager_UpdateParticle {
    fn eq(&self, other: &CDOTAUserMsg_ParticleManager_UpdateParticle) -> bool {
        self.control_point == other.control_point &&
        self.position == other.position &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_ParticleManager_UpdateParticle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_ParticleManager_UpdateParticleFwd {
    // message fields
    control_point: ::std::option::Option<i32>,
    forward: ::protobuf::SingularPtrField<super::netmessages::CMsgVector>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_ParticleManager_UpdateParticleFwd {}

impl CDOTAUserMsg_ParticleManager_UpdateParticleFwd {
    pub fn new() -> CDOTAUserMsg_ParticleManager_UpdateParticleFwd {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_ParticleManager_UpdateParticleFwd {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_ParticleManager_UpdateParticleFwd> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_ParticleManager_UpdateParticleFwd,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_ParticleManager_UpdateParticleFwd {
                    control_point: ::std::option::Option::None,
                    forward: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 control_point = 1;

    pub fn clear_control_point(&mut self) {
        self.control_point = ::std::option::Option::None;
    }

    pub fn has_control_point(&self) -> bool {
        self.control_point.is_some()
    }

    // Param is passed by value, moved
    pub fn set_control_point(&mut self, v: i32) {
        self.control_point = ::std::option::Option::Some(v);
    }

    pub fn get_control_point(&self) -> i32 {
        self.control_point.unwrap_or(0)
    }

    // optional .CMsgVector forward = 2;

    pub fn clear_forward(&mut self) {
        self.forward.clear();
    }

    pub fn has_forward(&self) -> bool {
        self.forward.is_some()
    }

    // Param is passed by value, moved
    pub fn set_forward(&mut self, v: super::netmessages::CMsgVector) {
        self.forward = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_forward(&mut self) -> &mut super::netmessages::CMsgVector {
        if self.forward.is_none() {
            self.forward.set_default();
        };
        self.forward.as_mut().unwrap()
    }

    // Take field
    pub fn take_forward(&mut self) -> super::netmessages::CMsgVector {
        self.forward.take().unwrap_or_else(|| super::netmessages::CMsgVector::new())
    }

    pub fn get_forward(&self) -> &super::netmessages::CMsgVector {
        self.forward.as_ref().unwrap_or_else(|| super::netmessages::CMsgVector::default_instance())
    }
}

impl ::protobuf::Message for CDOTAUserMsg_ParticleManager_UpdateParticleFwd {
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
                    self.control_point = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.forward));
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
        for value in &self.control_point {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.forward {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.control_point {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.forward.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_ParticleManager_UpdateParticleFwd>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_ParticleManager_UpdateParticleFwd {
    fn new() -> CDOTAUserMsg_ParticleManager_UpdateParticleFwd {
        CDOTAUserMsg_ParticleManager_UpdateParticleFwd::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_ParticleManager_UpdateParticleFwd>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "control_point",
                    CDOTAUserMsg_ParticleManager_UpdateParticleFwd::has_control_point,
                    CDOTAUserMsg_ParticleManager_UpdateParticleFwd::get_control_point,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "forward",
                    CDOTAUserMsg_ParticleManager_UpdateParticleFwd::has_forward,
                    CDOTAUserMsg_ParticleManager_UpdateParticleFwd::get_forward,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_ParticleManager_UpdateParticleFwd>(
                    "CDOTAUserMsg_ParticleManager_UpdateParticleFwd",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_ParticleManager_UpdateParticleFwd {
    fn clear(&mut self) {
        self.clear_control_point();
        self.clear_forward();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_ParticleManager_UpdateParticleFwd {
    fn eq(&self, other: &CDOTAUserMsg_ParticleManager_UpdateParticleFwd) -> bool {
        self.control_point == other.control_point &&
        self.forward == other.forward &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_ParticleManager_UpdateParticleFwd {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_ParticleManager_UpdateParticleOrient {
    // message fields
    control_point: ::std::option::Option<i32>,
    forward: ::protobuf::SingularPtrField<super::netmessages::CMsgVector>,
    right: ::protobuf::SingularPtrField<super::netmessages::CMsgVector>,
    up: ::protobuf::SingularPtrField<super::netmessages::CMsgVector>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_ParticleManager_UpdateParticleOrient {}

impl CDOTAUserMsg_ParticleManager_UpdateParticleOrient {
    pub fn new() -> CDOTAUserMsg_ParticleManager_UpdateParticleOrient {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_ParticleManager_UpdateParticleOrient {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_ParticleManager_UpdateParticleOrient> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_ParticleManager_UpdateParticleOrient,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_ParticleManager_UpdateParticleOrient {
                    control_point: ::std::option::Option::None,
                    forward: ::protobuf::SingularPtrField::none(),
                    right: ::protobuf::SingularPtrField::none(),
                    up: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 control_point = 1;

    pub fn clear_control_point(&mut self) {
        self.control_point = ::std::option::Option::None;
    }

    pub fn has_control_point(&self) -> bool {
        self.control_point.is_some()
    }

    // Param is passed by value, moved
    pub fn set_control_point(&mut self, v: i32) {
        self.control_point = ::std::option::Option::Some(v);
    }

    pub fn get_control_point(&self) -> i32 {
        self.control_point.unwrap_or(0)
    }

    // optional .CMsgVector forward = 2;

    pub fn clear_forward(&mut self) {
        self.forward.clear();
    }

    pub fn has_forward(&self) -> bool {
        self.forward.is_some()
    }

    // Param is passed by value, moved
    pub fn set_forward(&mut self, v: super::netmessages::CMsgVector) {
        self.forward = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_forward(&mut self) -> &mut super::netmessages::CMsgVector {
        if self.forward.is_none() {
            self.forward.set_default();
        };
        self.forward.as_mut().unwrap()
    }

    // Take field
    pub fn take_forward(&mut self) -> super::netmessages::CMsgVector {
        self.forward.take().unwrap_or_else(|| super::netmessages::CMsgVector::new())
    }

    pub fn get_forward(&self) -> &super::netmessages::CMsgVector {
        self.forward.as_ref().unwrap_or_else(|| super::netmessages::CMsgVector::default_instance())
    }

    // optional .CMsgVector right = 3;

    pub fn clear_right(&mut self) {
        self.right.clear();
    }

    pub fn has_right(&self) -> bool {
        self.right.is_some()
    }

    // Param is passed by value, moved
    pub fn set_right(&mut self, v: super::netmessages::CMsgVector) {
        self.right = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_right(&mut self) -> &mut super::netmessages::CMsgVector {
        if self.right.is_none() {
            self.right.set_default();
        };
        self.right.as_mut().unwrap()
    }

    // Take field
    pub fn take_right(&mut self) -> super::netmessages::CMsgVector {
        self.right.take().unwrap_or_else(|| super::netmessages::CMsgVector::new())
    }

    pub fn get_right(&self) -> &super::netmessages::CMsgVector {
        self.right.as_ref().unwrap_or_else(|| super::netmessages::CMsgVector::default_instance())
    }

    // optional .CMsgVector up = 4;

    pub fn clear_up(&mut self) {
        self.up.clear();
    }

    pub fn has_up(&self) -> bool {
        self.up.is_some()
    }

    // Param is passed by value, moved
    pub fn set_up(&mut self, v: super::netmessages::CMsgVector) {
        self.up = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_up(&mut self) -> &mut super::netmessages::CMsgVector {
        if self.up.is_none() {
            self.up.set_default();
        };
        self.up.as_mut().unwrap()
    }

    // Take field
    pub fn take_up(&mut self) -> super::netmessages::CMsgVector {
        self.up.take().unwrap_or_else(|| super::netmessages::CMsgVector::new())
    }

    pub fn get_up(&self) -> &super::netmessages::CMsgVector {
        self.up.as_ref().unwrap_or_else(|| super::netmessages::CMsgVector::default_instance())
    }
}

impl ::protobuf::Message for CDOTAUserMsg_ParticleManager_UpdateParticleOrient {
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
                    self.control_point = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.forward));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.right));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.up));
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
        for value in &self.control_point {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.forward {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.right {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.up {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.control_point {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.forward.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.right.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.up.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_ParticleManager_UpdateParticleOrient>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_ParticleManager_UpdateParticleOrient {
    fn new() -> CDOTAUserMsg_ParticleManager_UpdateParticleOrient {
        CDOTAUserMsg_ParticleManager_UpdateParticleOrient::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_ParticleManager_UpdateParticleOrient>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "control_point",
                    CDOTAUserMsg_ParticleManager_UpdateParticleOrient::has_control_point,
                    CDOTAUserMsg_ParticleManager_UpdateParticleOrient::get_control_point,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "forward",
                    CDOTAUserMsg_ParticleManager_UpdateParticleOrient::has_forward,
                    CDOTAUserMsg_ParticleManager_UpdateParticleOrient::get_forward,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "right",
                    CDOTAUserMsg_ParticleManager_UpdateParticleOrient::has_right,
                    CDOTAUserMsg_ParticleManager_UpdateParticleOrient::get_right,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "up",
                    CDOTAUserMsg_ParticleManager_UpdateParticleOrient::has_up,
                    CDOTAUserMsg_ParticleManager_UpdateParticleOrient::get_up,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_ParticleManager_UpdateParticleOrient>(
                    "CDOTAUserMsg_ParticleManager_UpdateParticleOrient",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_ParticleManager_UpdateParticleOrient {
    fn clear(&mut self) {
        self.clear_control_point();
        self.clear_forward();
        self.clear_right();
        self.clear_up();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_ParticleManager_UpdateParticleOrient {
    fn eq(&self, other: &CDOTAUserMsg_ParticleManager_UpdateParticleOrient) -> bool {
        self.control_point == other.control_point &&
        self.forward == other.forward &&
        self.right == other.right &&
        self.up == other.up &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_ParticleManager_UpdateParticleOrient {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_ParticleManager_UpdateParticleFallback {
    // message fields
    control_point: ::std::option::Option<i32>,
    position: ::protobuf::SingularPtrField<super::netmessages::CMsgVector>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_ParticleManager_UpdateParticleFallback {}

impl CDOTAUserMsg_ParticleManager_UpdateParticleFallback {
    pub fn new() -> CDOTAUserMsg_ParticleManager_UpdateParticleFallback {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_ParticleManager_UpdateParticleFallback {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_ParticleManager_UpdateParticleFallback> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_ParticleManager_UpdateParticleFallback,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_ParticleManager_UpdateParticleFallback {
                    control_point: ::std::option::Option::None,
                    position: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 control_point = 1;

    pub fn clear_control_point(&mut self) {
        self.control_point = ::std::option::Option::None;
    }

    pub fn has_control_point(&self) -> bool {
        self.control_point.is_some()
    }

    // Param is passed by value, moved
    pub fn set_control_point(&mut self, v: i32) {
        self.control_point = ::std::option::Option::Some(v);
    }

    pub fn get_control_point(&self) -> i32 {
        self.control_point.unwrap_or(0)
    }

    // optional .CMsgVector position = 2;

    pub fn clear_position(&mut self) {
        self.position.clear();
    }

    pub fn has_position(&self) -> bool {
        self.position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_position(&mut self, v: super::netmessages::CMsgVector) {
        self.position = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_position(&mut self) -> &mut super::netmessages::CMsgVector {
        if self.position.is_none() {
            self.position.set_default();
        };
        self.position.as_mut().unwrap()
    }

    // Take field
    pub fn take_position(&mut self) -> super::netmessages::CMsgVector {
        self.position.take().unwrap_or_else(|| super::netmessages::CMsgVector::new())
    }

    pub fn get_position(&self) -> &super::netmessages::CMsgVector {
        self.position.as_ref().unwrap_or_else(|| super::netmessages::CMsgVector::default_instance())
    }
}

impl ::protobuf::Message for CDOTAUserMsg_ParticleManager_UpdateParticleFallback {
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
                    self.control_point = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.position));
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
        for value in &self.control_point {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.position {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.control_point {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.position.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_ParticleManager_UpdateParticleFallback>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_ParticleManager_UpdateParticleFallback {
    fn new() -> CDOTAUserMsg_ParticleManager_UpdateParticleFallback {
        CDOTAUserMsg_ParticleManager_UpdateParticleFallback::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_ParticleManager_UpdateParticleFallback>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "control_point",
                    CDOTAUserMsg_ParticleManager_UpdateParticleFallback::has_control_point,
                    CDOTAUserMsg_ParticleManager_UpdateParticleFallback::get_control_point,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "position",
                    CDOTAUserMsg_ParticleManager_UpdateParticleFallback::has_position,
                    CDOTAUserMsg_ParticleManager_UpdateParticleFallback::get_position,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_ParticleManager_UpdateParticleFallback>(
                    "CDOTAUserMsg_ParticleManager_UpdateParticleFallback",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_ParticleManager_UpdateParticleFallback {
    fn clear(&mut self) {
        self.clear_control_point();
        self.clear_position();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_ParticleManager_UpdateParticleFallback {
    fn eq(&self, other: &CDOTAUserMsg_ParticleManager_UpdateParticleFallback) -> bool {
        self.control_point == other.control_point &&
        self.position == other.position &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_ParticleManager_UpdateParticleFallback {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_ParticleManager_UpdateParticleOffset {
    // message fields
    control_point: ::std::option::Option<i32>,
    origin_offset: ::protobuf::SingularPtrField<super::netmessages::CMsgVector>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_ParticleManager_UpdateParticleOffset {}

impl CDOTAUserMsg_ParticleManager_UpdateParticleOffset {
    pub fn new() -> CDOTAUserMsg_ParticleManager_UpdateParticleOffset {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_ParticleManager_UpdateParticleOffset {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_ParticleManager_UpdateParticleOffset> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_ParticleManager_UpdateParticleOffset,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_ParticleManager_UpdateParticleOffset {
                    control_point: ::std::option::Option::None,
                    origin_offset: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 control_point = 1;

    pub fn clear_control_point(&mut self) {
        self.control_point = ::std::option::Option::None;
    }

    pub fn has_control_point(&self) -> bool {
        self.control_point.is_some()
    }

    // Param is passed by value, moved
    pub fn set_control_point(&mut self, v: i32) {
        self.control_point = ::std::option::Option::Some(v);
    }

    pub fn get_control_point(&self) -> i32 {
        self.control_point.unwrap_or(0)
    }

    // optional .CMsgVector origin_offset = 2;

    pub fn clear_origin_offset(&mut self) {
        self.origin_offset.clear();
    }

    pub fn has_origin_offset(&self) -> bool {
        self.origin_offset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_offset(&mut self, v: super::netmessages::CMsgVector) {
        self.origin_offset = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin_offset(&mut self) -> &mut super::netmessages::CMsgVector {
        if self.origin_offset.is_none() {
            self.origin_offset.set_default();
        };
        self.origin_offset.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin_offset(&mut self) -> super::netmessages::CMsgVector {
        self.origin_offset.take().unwrap_or_else(|| super::netmessages::CMsgVector::new())
    }

    pub fn get_origin_offset(&self) -> &super::netmessages::CMsgVector {
        self.origin_offset.as_ref().unwrap_or_else(|| super::netmessages::CMsgVector::default_instance())
    }
}

impl ::protobuf::Message for CDOTAUserMsg_ParticleManager_UpdateParticleOffset {
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
                    self.control_point = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.origin_offset));
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
        for value in &self.control_point {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.origin_offset {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.control_point {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.origin_offset.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_ParticleManager_UpdateParticleOffset>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_ParticleManager_UpdateParticleOffset {
    fn new() -> CDOTAUserMsg_ParticleManager_UpdateParticleOffset {
        CDOTAUserMsg_ParticleManager_UpdateParticleOffset::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_ParticleManager_UpdateParticleOffset>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "control_point",
                    CDOTAUserMsg_ParticleManager_UpdateParticleOffset::has_control_point,
                    CDOTAUserMsg_ParticleManager_UpdateParticleOffset::get_control_point,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "origin_offset",
                    CDOTAUserMsg_ParticleManager_UpdateParticleOffset::has_origin_offset,
                    CDOTAUserMsg_ParticleManager_UpdateParticleOffset::get_origin_offset,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_ParticleManager_UpdateParticleOffset>(
                    "CDOTAUserMsg_ParticleManager_UpdateParticleOffset",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_ParticleManager_UpdateParticleOffset {
    fn clear(&mut self) {
        self.clear_control_point();
        self.clear_origin_offset();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_ParticleManager_UpdateParticleOffset {
    fn eq(&self, other: &CDOTAUserMsg_ParticleManager_UpdateParticleOffset) -> bool {
        self.control_point == other.control_point &&
        self.origin_offset == other.origin_offset &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_ParticleManager_UpdateParticleOffset {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_ParticleManager_UpdateParticleEnt {
    // message fields
    control_point: ::std::option::Option<i32>,
    entity_handle: ::std::option::Option<i32>,
    attach_type: ::std::option::Option<i32>,
    attachment: ::std::option::Option<i32>,
    fallback_position: ::protobuf::SingularPtrField<super::netmessages::CMsgVector>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_ParticleManager_UpdateParticleEnt {}

impl CDOTAUserMsg_ParticleManager_UpdateParticleEnt {
    pub fn new() -> CDOTAUserMsg_ParticleManager_UpdateParticleEnt {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_ParticleManager_UpdateParticleEnt {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_ParticleManager_UpdateParticleEnt> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_ParticleManager_UpdateParticleEnt,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_ParticleManager_UpdateParticleEnt {
                    control_point: ::std::option::Option::None,
                    entity_handle: ::std::option::Option::None,
                    attach_type: ::std::option::Option::None,
                    attachment: ::std::option::Option::None,
                    fallback_position: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 control_point = 1;

    pub fn clear_control_point(&mut self) {
        self.control_point = ::std::option::Option::None;
    }

    pub fn has_control_point(&self) -> bool {
        self.control_point.is_some()
    }

    // Param is passed by value, moved
    pub fn set_control_point(&mut self, v: i32) {
        self.control_point = ::std::option::Option::Some(v);
    }

    pub fn get_control_point(&self) -> i32 {
        self.control_point.unwrap_or(0)
    }

    // optional int32 entity_handle = 2;

    pub fn clear_entity_handle(&mut self) {
        self.entity_handle = ::std::option::Option::None;
    }

    pub fn has_entity_handle(&self) -> bool {
        self.entity_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity_handle(&mut self, v: i32) {
        self.entity_handle = ::std::option::Option::Some(v);
    }

    pub fn get_entity_handle(&self) -> i32 {
        self.entity_handle.unwrap_or(0)
    }

    // optional int32 attach_type = 3;

    pub fn clear_attach_type(&mut self) {
        self.attach_type = ::std::option::Option::None;
    }

    pub fn has_attach_type(&self) -> bool {
        self.attach_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attach_type(&mut self, v: i32) {
        self.attach_type = ::std::option::Option::Some(v);
    }

    pub fn get_attach_type(&self) -> i32 {
        self.attach_type.unwrap_or(0)
    }

    // optional int32 attachment = 4;

    pub fn clear_attachment(&mut self) {
        self.attachment = ::std::option::Option::None;
    }

    pub fn has_attachment(&self) -> bool {
        self.attachment.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attachment(&mut self, v: i32) {
        self.attachment = ::std::option::Option::Some(v);
    }

    pub fn get_attachment(&self) -> i32 {
        self.attachment.unwrap_or(0)
    }

    // optional .CMsgVector fallback_position = 5;

    pub fn clear_fallback_position(&mut self) {
        self.fallback_position.clear();
    }

    pub fn has_fallback_position(&self) -> bool {
        self.fallback_position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fallback_position(&mut self, v: super::netmessages::CMsgVector) {
        self.fallback_position = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fallback_position(&mut self) -> &mut super::netmessages::CMsgVector {
        if self.fallback_position.is_none() {
            self.fallback_position.set_default();
        };
        self.fallback_position.as_mut().unwrap()
    }

    // Take field
    pub fn take_fallback_position(&mut self) -> super::netmessages::CMsgVector {
        self.fallback_position.take().unwrap_or_else(|| super::netmessages::CMsgVector::new())
    }

    pub fn get_fallback_position(&self) -> &super::netmessages::CMsgVector {
        self.fallback_position.as_ref().unwrap_or_else(|| super::netmessages::CMsgVector::default_instance())
    }
}

impl ::protobuf::Message for CDOTAUserMsg_ParticleManager_UpdateParticleEnt {
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
                    self.control_point = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.entity_handle = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.attach_type = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.attachment = ::std::option::Option::Some(tmp);
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fallback_position));
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
        for value in &self.control_point {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.entity_handle {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.attach_type {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.attachment {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.fallback_position {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.control_point {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.entity_handle {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.attach_type {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.attachment {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.fallback_position.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_ParticleManager_UpdateParticleEnt>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_ParticleManager_UpdateParticleEnt {
    fn new() -> CDOTAUserMsg_ParticleManager_UpdateParticleEnt {
        CDOTAUserMsg_ParticleManager_UpdateParticleEnt::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_ParticleManager_UpdateParticleEnt>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "control_point",
                    CDOTAUserMsg_ParticleManager_UpdateParticleEnt::has_control_point,
                    CDOTAUserMsg_ParticleManager_UpdateParticleEnt::get_control_point,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "entity_handle",
                    CDOTAUserMsg_ParticleManager_UpdateParticleEnt::has_entity_handle,
                    CDOTAUserMsg_ParticleManager_UpdateParticleEnt::get_entity_handle,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "attach_type",
                    CDOTAUserMsg_ParticleManager_UpdateParticleEnt::has_attach_type,
                    CDOTAUserMsg_ParticleManager_UpdateParticleEnt::get_attach_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "attachment",
                    CDOTAUserMsg_ParticleManager_UpdateParticleEnt::has_attachment,
                    CDOTAUserMsg_ParticleManager_UpdateParticleEnt::get_attachment,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "fallback_position",
                    CDOTAUserMsg_ParticleManager_UpdateParticleEnt::has_fallback_position,
                    CDOTAUserMsg_ParticleManager_UpdateParticleEnt::get_fallback_position,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_ParticleManager_UpdateParticleEnt>(
                    "CDOTAUserMsg_ParticleManager_UpdateParticleEnt",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_ParticleManager_UpdateParticleEnt {
    fn clear(&mut self) {
        self.clear_control_point();
        self.clear_entity_handle();
        self.clear_attach_type();
        self.clear_attachment();
        self.clear_fallback_position();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_ParticleManager_UpdateParticleEnt {
    fn eq(&self, other: &CDOTAUserMsg_ParticleManager_UpdateParticleEnt) -> bool {
        self.control_point == other.control_point &&
        self.entity_handle == other.entity_handle &&
        self.attach_type == other.attach_type &&
        self.attachment == other.attachment &&
        self.fallback_position == other.fallback_position &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_ParticleManager_UpdateParticleEnt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_ParticleManager_UpdateParticleLatency {
    // message fields
    player_latency: ::std::option::Option<i32>,
    tick: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_ParticleManager_UpdateParticleLatency {}

impl CDOTAUserMsg_ParticleManager_UpdateParticleLatency {
    pub fn new() -> CDOTAUserMsg_ParticleManager_UpdateParticleLatency {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_ParticleManager_UpdateParticleLatency {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_ParticleManager_UpdateParticleLatency> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_ParticleManager_UpdateParticleLatency,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_ParticleManager_UpdateParticleLatency {
                    player_latency: ::std::option::Option::None,
                    tick: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 player_latency = 1;

    pub fn clear_player_latency(&mut self) {
        self.player_latency = ::std::option::Option::None;
    }

    pub fn has_player_latency(&self) -> bool {
        self.player_latency.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_latency(&mut self, v: i32) {
        self.player_latency = ::std::option::Option::Some(v);
    }

    pub fn get_player_latency(&self) -> i32 {
        self.player_latency.unwrap_or(0)
    }

    // optional int32 tick = 2;

    pub fn clear_tick(&mut self) {
        self.tick = ::std::option::Option::None;
    }

    pub fn has_tick(&self) -> bool {
        self.tick.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tick(&mut self, v: i32) {
        self.tick = ::std::option::Option::Some(v);
    }

    pub fn get_tick(&self) -> i32 {
        self.tick.unwrap_or(0)
    }
}

impl ::protobuf::Message for CDOTAUserMsg_ParticleManager_UpdateParticleLatency {
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
                    self.player_latency = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.tick = ::std::option::Option::Some(tmp);
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
        for value in &self.player_latency {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.tick {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.player_latency {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.tick {
            try!(os.write_int32(2, v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_ParticleManager_UpdateParticleLatency>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_ParticleManager_UpdateParticleLatency {
    fn new() -> CDOTAUserMsg_ParticleManager_UpdateParticleLatency {
        CDOTAUserMsg_ParticleManager_UpdateParticleLatency::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_ParticleManager_UpdateParticleLatency>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "player_latency",
                    CDOTAUserMsg_ParticleManager_UpdateParticleLatency::has_player_latency,
                    CDOTAUserMsg_ParticleManager_UpdateParticleLatency::get_player_latency,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "tick",
                    CDOTAUserMsg_ParticleManager_UpdateParticleLatency::has_tick,
                    CDOTAUserMsg_ParticleManager_UpdateParticleLatency::get_tick,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_ParticleManager_UpdateParticleLatency>(
                    "CDOTAUserMsg_ParticleManager_UpdateParticleLatency",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_ParticleManager_UpdateParticleLatency {
    fn clear(&mut self) {
        self.clear_player_latency();
        self.clear_tick();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_ParticleManager_UpdateParticleLatency {
    fn eq(&self, other: &CDOTAUserMsg_ParticleManager_UpdateParticleLatency) -> bool {
        self.player_latency == other.player_latency &&
        self.tick == other.tick &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_ParticleManager_UpdateParticleLatency {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_OverheadEvent {
    // message fields
    message_type: ::std::option::Option<DOTA_OVERHEAD_ALERT>,
    value: ::std::option::Option<i32>,
    target_player_entindex: ::std::option::Option<i32>,
    target_entindex: ::std::option::Option<i32>,
    source_player_entindex: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_OverheadEvent {}

impl CDOTAUserMsg_OverheadEvent {
    pub fn new() -> CDOTAUserMsg_OverheadEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_OverheadEvent {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_OverheadEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_OverheadEvent,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_OverheadEvent {
                    message_type: ::std::option::Option::None,
                    value: ::std::option::Option::None,
                    target_player_entindex: ::std::option::Option::None,
                    target_entindex: ::std::option::Option::None,
                    source_player_entindex: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .DOTA_OVERHEAD_ALERT message_type = 1;

    pub fn clear_message_type(&mut self) {
        self.message_type = ::std::option::Option::None;
    }

    pub fn has_message_type(&self) -> bool {
        self.message_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message_type(&mut self, v: DOTA_OVERHEAD_ALERT) {
        self.message_type = ::std::option::Option::Some(v);
    }

    pub fn get_message_type(&self) -> DOTA_OVERHEAD_ALERT {
        self.message_type.unwrap_or(DOTA_OVERHEAD_ALERT::OVERHEAD_ALERT_GOLD)
    }

    // optional int32 value = 2;

    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: i32) {
        self.value = ::std::option::Option::Some(v);
    }

    pub fn get_value(&self) -> i32 {
        self.value.unwrap_or(0)
    }

    // optional int32 target_player_entindex = 3;

    pub fn clear_target_player_entindex(&mut self) {
        self.target_player_entindex = ::std::option::Option::None;
    }

    pub fn has_target_player_entindex(&self) -> bool {
        self.target_player_entindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target_player_entindex(&mut self, v: i32) {
        self.target_player_entindex = ::std::option::Option::Some(v);
    }

    pub fn get_target_player_entindex(&self) -> i32 {
        self.target_player_entindex.unwrap_or(0)
    }

    // optional int32 target_entindex = 4;

    pub fn clear_target_entindex(&mut self) {
        self.target_entindex = ::std::option::Option::None;
    }

    pub fn has_target_entindex(&self) -> bool {
        self.target_entindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target_entindex(&mut self, v: i32) {
        self.target_entindex = ::std::option::Option::Some(v);
    }

    pub fn get_target_entindex(&self) -> i32 {
        self.target_entindex.unwrap_or(0)
    }

    // optional int32 source_player_entindex = 5;

    pub fn clear_source_player_entindex(&mut self) {
        self.source_player_entindex = ::std::option::Option::None;
    }

    pub fn has_source_player_entindex(&self) -> bool {
        self.source_player_entindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source_player_entindex(&mut self, v: i32) {
        self.source_player_entindex = ::std::option::Option::Some(v);
    }

    pub fn get_source_player_entindex(&self) -> i32 {
        self.source_player_entindex.unwrap_or(0)
    }
}

impl ::protobuf::Message for CDOTAUserMsg_OverheadEvent {
    fn is_initialized(&self) -> bool {
        if self.message_type.is_none() {
            return false;
        };
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
                    let tmp = try!(is.read_enum());
                    self.message_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.value = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.target_player_entindex = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.target_entindex = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.source_player_entindex = ::std::option::Option::Some(tmp);
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
        for value in &self.message_type {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.value {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.target_player_entindex {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.target_entindex {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.source_player_entindex {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.message_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.value {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.target_player_entindex {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.target_entindex {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.source_player_entindex {
            try!(os.write_int32(5, v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_OverheadEvent>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_OverheadEvent {
    fn new() -> CDOTAUserMsg_OverheadEvent {
        CDOTAUserMsg_OverheadEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_OverheadEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "message_type",
                    CDOTAUserMsg_OverheadEvent::has_message_type,
                    CDOTAUserMsg_OverheadEvent::get_message_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "value",
                    CDOTAUserMsg_OverheadEvent::has_value,
                    CDOTAUserMsg_OverheadEvent::get_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "target_player_entindex",
                    CDOTAUserMsg_OverheadEvent::has_target_player_entindex,
                    CDOTAUserMsg_OverheadEvent::get_target_player_entindex,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "target_entindex",
                    CDOTAUserMsg_OverheadEvent::has_target_entindex,
                    CDOTAUserMsg_OverheadEvent::get_target_entindex,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "source_player_entindex",
                    CDOTAUserMsg_OverheadEvent::has_source_player_entindex,
                    CDOTAUserMsg_OverheadEvent::get_source_player_entindex,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_OverheadEvent>(
                    "CDOTAUserMsg_OverheadEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_OverheadEvent {
    fn clear(&mut self) {
        self.clear_message_type();
        self.clear_value();
        self.clear_target_player_entindex();
        self.clear_target_entindex();
        self.clear_source_player_entindex();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_OverheadEvent {
    fn eq(&self, other: &CDOTAUserMsg_OverheadEvent) -> bool {
        self.message_type == other.message_type &&
        self.value == other.value &&
        self.target_player_entindex == other.target_player_entindex &&
        self.target_entindex == other.target_entindex &&
        self.source_player_entindex == other.source_player_entindex &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_OverheadEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDOTAUserMsg_TutorialTipInfo {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    progress: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAUserMsg_TutorialTipInfo {}

impl CDOTAUserMsg_TutorialTipInfo {
    pub fn new() -> CDOTAUserMsg_TutorialTipInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAUserMsg_TutorialTipInfo {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAUserMsg_TutorialTipInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAUserMsg_TutorialTipInfo,
        };
        unsafe {
            instance.get(|| {
                CDOTAUserMsg_TutorialTipInfo {
                    name: ::protobuf::SingularField::none(),
                    progress: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional int32 progress = 2;

    pub fn clear_progress(&mut self) {
        self.progress = ::std::option::Option::None;
    }

    pub fn has_progress(&self) -> bool {
        self.progress.is_some()
    }

    // Param is passed by value, moved
    pub fn set_progress(&mut self, v: i32) {
        self.progress = ::std::option::Option::Some(v);
    }

    pub fn get_progress(&self) -> i32 {
        self.progress.unwrap_or(0)
    }
}

impl ::protobuf::Message for CDOTAUserMsg_TutorialTipInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.progress = ::std::option::Option::Some(tmp);
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
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.progress {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.progress {
            try!(os.write_int32(2, v));
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
        ::std::any::TypeId::of::<CDOTAUserMsg_TutorialTipInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAUserMsg_TutorialTipInfo {
    fn new() -> CDOTAUserMsg_TutorialTipInfo {
        CDOTAUserMsg_TutorialTipInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAUserMsg_TutorialTipInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    CDOTAUserMsg_TutorialTipInfo::has_name,
                    CDOTAUserMsg_TutorialTipInfo::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "progress",
                    CDOTAUserMsg_TutorialTipInfo::has_progress,
                    CDOTAUserMsg_TutorialTipInfo::get_progress,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAUserMsg_TutorialTipInfo>(
                    "CDOTAUserMsg_TutorialTipInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAUserMsg_TutorialTipInfo {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_progress();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAUserMsg_TutorialTipInfo {
    fn eq(&self, other: &CDOTAUserMsg_TutorialTipInfo) -> bool {
        self.name == other.name &&
        self.progress == other.progress &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAUserMsg_TutorialTipInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EDotaUserMessages {
    DOTA_UM_AddUnitToSelection = 64,
    DOTA_UM_AIDebugLine = 65,
    DOTA_UM_ChatEvent = 66,
    DOTA_UM_CombatHeroPositions = 67,
    DOTA_UM_CombatLogData = 68,
    DOTA_UM_CombatLogShowDeath = 70,
    DOTA_UM_CreateLinearProjectile = 71,
    DOTA_UM_DestroyLinearProjectile = 72,
    DOTA_UM_DodgeTrackingProjectiles = 73,
    DOTA_UM_GlobalLightColor = 74,
    DOTA_UM_GlobalLightDirection = 75,
    DOTA_UM_InvalidCommand = 76,
    DOTA_UM_LocationPing = 77,
    DOTA_UM_MapLine = 78,
    DOTA_UM_MiniKillCamInfo = 79,
    DOTA_UM_MinimapDebugPoint = 80,
    DOTA_UM_MinimapEvent = 81,
    DOTA_UM_NevermoreRequiem = 82,
    DOTA_UM_OverheadEvent = 83,
    DOTA_UM_SetNextAutobuyItem = 84,
    DOTA_UM_SharedCooldown = 85,
    DOTA_UM_SpectatorPlayerClick = 86,
    DOTA_UM_TutorialTipInfo = 87,
    DOTA_UM_UnitEvent = 88,
    DOTA_UM_ParticleManager = 89,
    DOTA_UM_BotChat = 90,
    DOTA_UM_HudError = 91,
    DOTA_UM_ItemPurchased = 92,
    DOTA_UM_Ping = 93,
    DOTA_UM_ItemFound = 94,
}

impl ::protobuf::ProtobufEnum for EDotaUserMessages {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EDotaUserMessages> {
        match value {
            64 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_AddUnitToSelection),
            65 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_AIDebugLine),
            66 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_ChatEvent),
            67 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_CombatHeroPositions),
            68 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_CombatLogData),
            70 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_CombatLogShowDeath),
            71 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_CreateLinearProjectile),
            72 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_DestroyLinearProjectile),
            73 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_DodgeTrackingProjectiles),
            74 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_GlobalLightColor),
            75 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_GlobalLightDirection),
            76 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_InvalidCommand),
            77 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_LocationPing),
            78 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_MapLine),
            79 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_MiniKillCamInfo),
            80 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_MinimapDebugPoint),
            81 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_MinimapEvent),
            82 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_NevermoreRequiem),
            83 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_OverheadEvent),
            84 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_SetNextAutobuyItem),
            85 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_SharedCooldown),
            86 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_SpectatorPlayerClick),
            87 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_TutorialTipInfo),
            88 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_UnitEvent),
            89 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_ParticleManager),
            90 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_BotChat),
            91 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_HudError),
            92 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_ItemPurchased),
            93 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_Ping),
            94 => ::std::option::Option::Some(EDotaUserMessages::DOTA_UM_ItemFound),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EDotaUserMessages] = &[
            EDotaUserMessages::DOTA_UM_AddUnitToSelection,
            EDotaUserMessages::DOTA_UM_AIDebugLine,
            EDotaUserMessages::DOTA_UM_ChatEvent,
            EDotaUserMessages::DOTA_UM_CombatHeroPositions,
            EDotaUserMessages::DOTA_UM_CombatLogData,
            EDotaUserMessages::DOTA_UM_CombatLogShowDeath,
            EDotaUserMessages::DOTA_UM_CreateLinearProjectile,
            EDotaUserMessages::DOTA_UM_DestroyLinearProjectile,
            EDotaUserMessages::DOTA_UM_DodgeTrackingProjectiles,
            EDotaUserMessages::DOTA_UM_GlobalLightColor,
            EDotaUserMessages::DOTA_UM_GlobalLightDirection,
            EDotaUserMessages::DOTA_UM_InvalidCommand,
            EDotaUserMessages::DOTA_UM_LocationPing,
            EDotaUserMessages::DOTA_UM_MapLine,
            EDotaUserMessages::DOTA_UM_MiniKillCamInfo,
            EDotaUserMessages::DOTA_UM_MinimapDebugPoint,
            EDotaUserMessages::DOTA_UM_MinimapEvent,
            EDotaUserMessages::DOTA_UM_NevermoreRequiem,
            EDotaUserMessages::DOTA_UM_OverheadEvent,
            EDotaUserMessages::DOTA_UM_SetNextAutobuyItem,
            EDotaUserMessages::DOTA_UM_SharedCooldown,
            EDotaUserMessages::DOTA_UM_SpectatorPlayerClick,
            EDotaUserMessages::DOTA_UM_TutorialTipInfo,
            EDotaUserMessages::DOTA_UM_UnitEvent,
            EDotaUserMessages::DOTA_UM_ParticleManager,
            EDotaUserMessages::DOTA_UM_BotChat,
            EDotaUserMessages::DOTA_UM_HudError,
            EDotaUserMessages::DOTA_UM_ItemPurchased,
            EDotaUserMessages::DOTA_UM_Ping,
            EDotaUserMessages::DOTA_UM_ItemFound,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<EDotaUserMessages>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EDotaUserMessages", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EDotaUserMessages {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DOTA_CHAT_MESSAGE {
    CHAT_MESSAGE_INVALID = -1,
    CHAT_MESSAGE_HERO_KILL = 0,
    CHAT_MESSAGE_HERO_DENY = 1,
    CHAT_MESSAGE_BARRACKS_KILL = 2,
    CHAT_MESSAGE_TOWER_KILL = 3,
    CHAT_MESSAGE_TOWER_DENY = 4,
    CHAT_MESSAGE_FIRSTBLOOD = 5,
    CHAT_MESSAGE_STREAK_KILL = 6,
    CHAT_MESSAGE_BUYBACK = 7,
    CHAT_MESSAGE_AEGIS = 8,
    CHAT_MESSAGE_ROSHAN_KILL = 9,
    CHAT_MESSAGE_COURIER_LOST = 10,
    CHAT_MESSAGE_COURIER_RESPAWNED = 11,
    CHAT_MESSAGE_GLYPH_USED = 12,
    CHAT_MESSAGE_ITEM_PURCHASE = 13,
    CHAT_MESSAGE_CONNECT = 14,
    CHAT_MESSAGE_DISCONNECT = 15,
    CHAT_MESSAGE_DISCONNECT_WAIT_FOR_RECONNECT = 16,
    CHAT_MESSAGE_DISCONNECT_TIME_REMAINING = 17,
    CHAT_MESSAGE_DISCONNECT_TIME_REMAINING_PLURAL = 18,
    CHAT_MESSAGE_RECONNECT = 19,
    CHAT_MESSAGE_ABANDON = 20,
    CHAT_MESSAGE_SAFE_TO_LEAVE = 21,
    CHAT_MESSAGE_RUNE_PICKUP = 22,
    CHAT_MESSAGE_RUNE_BOTTLE = 23,
    CHAT_MESSAGE_INTHEBAG = 24,
    CHAT_MESSAGE_SECRETSHOP = 25,
    CHAT_MESSAGE_ITEM_AUTOPURCHASED = 26,
    CHAT_MESSAGE_ITEMS_COMBINED = 27,
    CHAT_MESSAGE_SUPER_CREEPS = 28,
    CHAT_MESSAGE_CANT_USE_ACTION_ITEM = 29,
    CHAT_MESSAGE_CHARGES_EXHAUSTED = 30,
    CHAT_MESSAGE_CANTPAUSE = 31,
    CHAT_MESSAGE_NOPAUSESLEFT = 32,
    CHAT_MESSAGE_CANTPAUSEYET = 33,
    CHAT_MESSAGE_PAUSED = 34,
    CHAT_MESSAGE_UNPAUSE_COUNTDOWN = 35,
    CHAT_MESSAGE_UNPAUSED = 36,
    CHAT_MESSAGE_AUTO_UNPAUSED = 37,
    CHAT_MESSAGE_YOUPAUSED = 38,
    CHAT_MESSAGE_CANTUNPAUSETEAM = 39,
    CHAT_MESSAGE_SAFE_TO_LEAVE_ABANDONER = 40,
    CHAT_MESSAGE_VOICE_TEXT_BANNED = 41,
    CHAT_MESSAGE_SPECTATORS_WATCHING_THIS_GAME = 42,
    CHAT_MESSAGE_REPORT_REMINDER = 43,
    CHAT_MESSAGE_ECON_ITEM = 44,
    CHAT_MESSAGE_TAUNT = 45,
    CHAT_MESSAGE_RANDOM = 46,
    CHAT_MESSAGE_RD_TURN = 47,
}

impl ::protobuf::ProtobufEnum for DOTA_CHAT_MESSAGE {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DOTA_CHAT_MESSAGE> {
        match value {
            -1 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_INVALID),
            0 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_HERO_KILL),
            1 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_HERO_DENY),
            2 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_BARRACKS_KILL),
            3 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_TOWER_KILL),
            4 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_TOWER_DENY),
            5 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_FIRSTBLOOD),
            6 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_STREAK_KILL),
            7 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_BUYBACK),
            8 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_AEGIS),
            9 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_ROSHAN_KILL),
            10 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_COURIER_LOST),
            11 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_COURIER_RESPAWNED),
            12 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_GLYPH_USED),
            13 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_ITEM_PURCHASE),
            14 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_CONNECT),
            15 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_DISCONNECT),
            16 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_DISCONNECT_WAIT_FOR_RECONNECT),
            17 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_DISCONNECT_TIME_REMAINING),
            18 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_DISCONNECT_TIME_REMAINING_PLURAL),
            19 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_RECONNECT),
            20 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_ABANDON),
            21 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_SAFE_TO_LEAVE),
            22 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_RUNE_PICKUP),
            23 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_RUNE_BOTTLE),
            24 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_INTHEBAG),
            25 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_SECRETSHOP),
            26 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_ITEM_AUTOPURCHASED),
            27 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_ITEMS_COMBINED),
            28 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_SUPER_CREEPS),
            29 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_CANT_USE_ACTION_ITEM),
            30 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_CHARGES_EXHAUSTED),
            31 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_CANTPAUSE),
            32 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_NOPAUSESLEFT),
            33 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_CANTPAUSEYET),
            34 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_PAUSED),
            35 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_UNPAUSE_COUNTDOWN),
            36 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_UNPAUSED),
            37 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_AUTO_UNPAUSED),
            38 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_YOUPAUSED),
            39 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_CANTUNPAUSETEAM),
            40 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_SAFE_TO_LEAVE_ABANDONER),
            41 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_VOICE_TEXT_BANNED),
            42 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_SPECTATORS_WATCHING_THIS_GAME),
            43 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_REPORT_REMINDER),
            44 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_ECON_ITEM),
            45 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_TAUNT),
            46 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_RANDOM),
            47 => ::std::option::Option::Some(DOTA_CHAT_MESSAGE::CHAT_MESSAGE_RD_TURN),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DOTA_CHAT_MESSAGE] = &[
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_INVALID,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_HERO_KILL,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_HERO_DENY,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_BARRACKS_KILL,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_TOWER_KILL,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_TOWER_DENY,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_FIRSTBLOOD,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_STREAK_KILL,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_BUYBACK,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_AEGIS,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_ROSHAN_KILL,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_COURIER_LOST,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_COURIER_RESPAWNED,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_GLYPH_USED,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_ITEM_PURCHASE,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_CONNECT,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_DISCONNECT,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_DISCONNECT_WAIT_FOR_RECONNECT,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_DISCONNECT_TIME_REMAINING,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_DISCONNECT_TIME_REMAINING_PLURAL,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_RECONNECT,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_ABANDON,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_SAFE_TO_LEAVE,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_RUNE_PICKUP,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_RUNE_BOTTLE,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_INTHEBAG,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_SECRETSHOP,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_ITEM_AUTOPURCHASED,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_ITEMS_COMBINED,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_SUPER_CREEPS,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_CANT_USE_ACTION_ITEM,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_CHARGES_EXHAUSTED,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_CANTPAUSE,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_NOPAUSESLEFT,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_CANTPAUSEYET,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_PAUSED,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_UNPAUSE_COUNTDOWN,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_UNPAUSED,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_AUTO_UNPAUSED,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_YOUPAUSED,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_CANTUNPAUSETEAM,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_SAFE_TO_LEAVE_ABANDONER,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_VOICE_TEXT_BANNED,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_SPECTATORS_WATCHING_THIS_GAME,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_REPORT_REMINDER,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_ECON_ITEM,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_TAUNT,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_RANDOM,
            DOTA_CHAT_MESSAGE::CHAT_MESSAGE_RD_TURN,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<DOTA_CHAT_MESSAGE>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DOTA_CHAT_MESSAGE", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DOTA_CHAT_MESSAGE {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EDotaEntityMessages {
    DOTA_UNIT_SPEECH = 0,
    DOTA_UNIT_SPEECH_MUTE = 1,
    DOTA_UNIT_ADD_GESTURE = 2,
    DOTA_UNIT_REMOVE_GESTURE = 3,
    DOTA_UNIT_REMOVE_ALL_GESTURES = 4,
    DOTA_UNIT_FADE_GESTURE = 6,
}

impl ::protobuf::ProtobufEnum for EDotaEntityMessages {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EDotaEntityMessages> {
        match value {
            0 => ::std::option::Option::Some(EDotaEntityMessages::DOTA_UNIT_SPEECH),
            1 => ::std::option::Option::Some(EDotaEntityMessages::DOTA_UNIT_SPEECH_MUTE),
            2 => ::std::option::Option::Some(EDotaEntityMessages::DOTA_UNIT_ADD_GESTURE),
            3 => ::std::option::Option::Some(EDotaEntityMessages::DOTA_UNIT_REMOVE_GESTURE),
            4 => ::std::option::Option::Some(EDotaEntityMessages::DOTA_UNIT_REMOVE_ALL_GESTURES),
            6 => ::std::option::Option::Some(EDotaEntityMessages::DOTA_UNIT_FADE_GESTURE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EDotaEntityMessages] = &[
            EDotaEntityMessages::DOTA_UNIT_SPEECH,
            EDotaEntityMessages::DOTA_UNIT_SPEECH_MUTE,
            EDotaEntityMessages::DOTA_UNIT_ADD_GESTURE,
            EDotaEntityMessages::DOTA_UNIT_REMOVE_GESTURE,
            EDotaEntityMessages::DOTA_UNIT_REMOVE_ALL_GESTURES,
            EDotaEntityMessages::DOTA_UNIT_FADE_GESTURE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<EDotaEntityMessages>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EDotaEntityMessages", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EDotaEntityMessages {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DOTA_PARTICLE_MESSAGE {
    DOTA_PARTICLE_MANAGER_EVENT_CREATE = 0,
    DOTA_PARTICLE_MANAGER_EVENT_UPDATE = 1,
    DOTA_PARTICLE_MANAGER_EVENT_UPDATE_FORWARD = 2,
    DOTA_PARTICLE_MANAGER_EVENT_UPDATE_ORIENTATION = 3,
    DOTA_PARTICLE_MANAGER_EVENT_UPDATE_FALLBACK = 4,
    DOTA_PARTICLE_MANAGER_EVENT_UPDATE_ENT = 5,
    DOTA_PARTICLE_MANAGER_EVENT_UPDATE_OFFSET = 6,
    DOTA_PARTICLE_MANAGER_EVENT_DESTROY = 7,
    DOTA_PARTICLE_MANAGER_EVENT_DESTROY_INVOLVING = 8,
    DOTA_PARTICLE_MANAGER_EVENT_RELEASE = 9,
    DOTA_PARTICLE_MANAGER_EVENT_LATENCY = 10,
}

impl ::protobuf::ProtobufEnum for DOTA_PARTICLE_MESSAGE {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DOTA_PARTICLE_MESSAGE> {
        match value {
            0 => ::std::option::Option::Some(DOTA_PARTICLE_MESSAGE::DOTA_PARTICLE_MANAGER_EVENT_CREATE),
            1 => ::std::option::Option::Some(DOTA_PARTICLE_MESSAGE::DOTA_PARTICLE_MANAGER_EVENT_UPDATE),
            2 => ::std::option::Option::Some(DOTA_PARTICLE_MESSAGE::DOTA_PARTICLE_MANAGER_EVENT_UPDATE_FORWARD),
            3 => ::std::option::Option::Some(DOTA_PARTICLE_MESSAGE::DOTA_PARTICLE_MANAGER_EVENT_UPDATE_ORIENTATION),
            4 => ::std::option::Option::Some(DOTA_PARTICLE_MESSAGE::DOTA_PARTICLE_MANAGER_EVENT_UPDATE_FALLBACK),
            5 => ::std::option::Option::Some(DOTA_PARTICLE_MESSAGE::DOTA_PARTICLE_MANAGER_EVENT_UPDATE_ENT),
            6 => ::std::option::Option::Some(DOTA_PARTICLE_MESSAGE::DOTA_PARTICLE_MANAGER_EVENT_UPDATE_OFFSET),
            7 => ::std::option::Option::Some(DOTA_PARTICLE_MESSAGE::DOTA_PARTICLE_MANAGER_EVENT_DESTROY),
            8 => ::std::option::Option::Some(DOTA_PARTICLE_MESSAGE::DOTA_PARTICLE_MANAGER_EVENT_DESTROY_INVOLVING),
            9 => ::std::option::Option::Some(DOTA_PARTICLE_MESSAGE::DOTA_PARTICLE_MANAGER_EVENT_RELEASE),
            10 => ::std::option::Option::Some(DOTA_PARTICLE_MESSAGE::DOTA_PARTICLE_MANAGER_EVENT_LATENCY),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DOTA_PARTICLE_MESSAGE] = &[
            DOTA_PARTICLE_MESSAGE::DOTA_PARTICLE_MANAGER_EVENT_CREATE,
            DOTA_PARTICLE_MESSAGE::DOTA_PARTICLE_MANAGER_EVENT_UPDATE,
            DOTA_PARTICLE_MESSAGE::DOTA_PARTICLE_MANAGER_EVENT_UPDATE_FORWARD,
            DOTA_PARTICLE_MESSAGE::DOTA_PARTICLE_MANAGER_EVENT_UPDATE_ORIENTATION,
            DOTA_PARTICLE_MESSAGE::DOTA_PARTICLE_MANAGER_EVENT_UPDATE_FALLBACK,
            DOTA_PARTICLE_MESSAGE::DOTA_PARTICLE_MANAGER_EVENT_UPDATE_ENT,
            DOTA_PARTICLE_MESSAGE::DOTA_PARTICLE_MANAGER_EVENT_UPDATE_OFFSET,
            DOTA_PARTICLE_MESSAGE::DOTA_PARTICLE_MANAGER_EVENT_DESTROY,
            DOTA_PARTICLE_MESSAGE::DOTA_PARTICLE_MANAGER_EVENT_DESTROY_INVOLVING,
            DOTA_PARTICLE_MESSAGE::DOTA_PARTICLE_MANAGER_EVENT_RELEASE,
            DOTA_PARTICLE_MESSAGE::DOTA_PARTICLE_MANAGER_EVENT_LATENCY,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<DOTA_PARTICLE_MESSAGE>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DOTA_PARTICLE_MESSAGE", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DOTA_PARTICLE_MESSAGE {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DOTA_OVERHEAD_ALERT {
    OVERHEAD_ALERT_GOLD = 0,
    OVERHEAD_ALERT_DENY = 1,
    OVERHEAD_ALERT_CRITICAL = 2,
    OVERHEAD_ALERT_XP = 3,
    OVERHEAD_ALERT_BONUS_SPELL_DAMAGE = 4,
    OVERHEAD_ALERT_MISS = 5,
    OVERHEAD_ALERT_DAMAGE = 6,
    OVERHEAD_ALERT_EVADE = 7,
    OVERHEAD_ALERT_BLOCK = 8,
    OVERHEAD_ALERT_BONUS_POISON_DAMAGE = 9,
}

impl ::protobuf::ProtobufEnum for DOTA_OVERHEAD_ALERT {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DOTA_OVERHEAD_ALERT> {
        match value {
            0 => ::std::option::Option::Some(DOTA_OVERHEAD_ALERT::OVERHEAD_ALERT_GOLD),
            1 => ::std::option::Option::Some(DOTA_OVERHEAD_ALERT::OVERHEAD_ALERT_DENY),
            2 => ::std::option::Option::Some(DOTA_OVERHEAD_ALERT::OVERHEAD_ALERT_CRITICAL),
            3 => ::std::option::Option::Some(DOTA_OVERHEAD_ALERT::OVERHEAD_ALERT_XP),
            4 => ::std::option::Option::Some(DOTA_OVERHEAD_ALERT::OVERHEAD_ALERT_BONUS_SPELL_DAMAGE),
            5 => ::std::option::Option::Some(DOTA_OVERHEAD_ALERT::OVERHEAD_ALERT_MISS),
            6 => ::std::option::Option::Some(DOTA_OVERHEAD_ALERT::OVERHEAD_ALERT_DAMAGE),
            7 => ::std::option::Option::Some(DOTA_OVERHEAD_ALERT::OVERHEAD_ALERT_EVADE),
            8 => ::std::option::Option::Some(DOTA_OVERHEAD_ALERT::OVERHEAD_ALERT_BLOCK),
            9 => ::std::option::Option::Some(DOTA_OVERHEAD_ALERT::OVERHEAD_ALERT_BONUS_POISON_DAMAGE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DOTA_OVERHEAD_ALERT] = &[
            DOTA_OVERHEAD_ALERT::OVERHEAD_ALERT_GOLD,
            DOTA_OVERHEAD_ALERT::OVERHEAD_ALERT_DENY,
            DOTA_OVERHEAD_ALERT::OVERHEAD_ALERT_CRITICAL,
            DOTA_OVERHEAD_ALERT::OVERHEAD_ALERT_XP,
            DOTA_OVERHEAD_ALERT::OVERHEAD_ALERT_BONUS_SPELL_DAMAGE,
            DOTA_OVERHEAD_ALERT::OVERHEAD_ALERT_MISS,
            DOTA_OVERHEAD_ALERT::OVERHEAD_ALERT_DAMAGE,
            DOTA_OVERHEAD_ALERT::OVERHEAD_ALERT_EVADE,
            DOTA_OVERHEAD_ALERT::OVERHEAD_ALERT_BLOCK,
            DOTA_OVERHEAD_ALERT::OVERHEAD_ALERT_BONUS_POISON_DAMAGE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<DOTA_OVERHEAD_ALERT>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DOTA_OVERHEAD_ALERT", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DOTA_OVERHEAD_ALERT {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x17, 0x64, 0x6f, 0x74, 0x61, 0x5f, 0x75, 0x73, 0x65, 0x72, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x20, 0x67, 0x6f, 0x6f, 0x67, 0x6c,
    0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x64, 0x65, 0x73, 0x63, 0x72,
    0x69, 0x70, 0x74, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x11, 0x6e, 0x65, 0x74,
    0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x11,
    0x61, 0x69, 0x5f, 0x61, 0x63, 0x74, 0x69, 0x76, 0x69, 0x74, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x19, 0x64, 0x6f, 0x74, 0x61, 0x5f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x2b, 0x0a, 0x18,
    0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x41, 0x49, 0x44,
    0x65, 0x62, 0x75, 0x67, 0x4c, 0x69, 0x6e, 0x65, 0x12, 0x0f, 0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x22, 0x24, 0x0a, 0x11, 0x43, 0x44, 0x4f,
    0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x50, 0x69, 0x6e, 0x67, 0x12, 0x0f,
    0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x22,
    0xd9, 0x01, 0x0a, 0x16, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67,
    0x5f, 0x43, 0x68, 0x61, 0x74, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x20, 0x0a, 0x04, 0x74, 0x79,
    0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x12, 0x2e, 0x44, 0x4f, 0x54, 0x41, 0x5f,
    0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x12, 0x0d, 0x0a, 0x05,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x16, 0x0a, 0x0a, 0x70,
    0x6c, 0x61, 0x79, 0x65, 0x72, 0x69, 0x64, 0x5f, 0x31, 0x18, 0x03, 0x20, 0x01, 0x28, 0x11, 0x3a,
    0x02, 0x2d, 0x31, 0x12, 0x16, 0x0a, 0x0a, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x69, 0x64, 0x5f,
    0x32, 0x18, 0x04, 0x20, 0x01, 0x28, 0x11, 0x3a, 0x02, 0x2d, 0x31, 0x12, 0x16, 0x0a, 0x0a, 0x70,
    0x6c, 0x61, 0x79, 0x65, 0x72, 0x69, 0x64, 0x5f, 0x33, 0x18, 0x05, 0x20, 0x01, 0x28, 0x11, 0x3a,
    0x02, 0x2d, 0x31, 0x12, 0x16, 0x0a, 0x0a, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x69, 0x64, 0x5f,
    0x34, 0x18, 0x06, 0x20, 0x01, 0x28, 0x11, 0x3a, 0x02, 0x2d, 0x31, 0x12, 0x16, 0x0a, 0x0a, 0x70,
    0x6c, 0x61, 0x79, 0x65, 0x72, 0x69, 0x64, 0x5f, 0x35, 0x18, 0x07, 0x20, 0x01, 0x28, 0x11, 0x3a,
    0x02, 0x2d, 0x31, 0x12, 0x16, 0x0a, 0x0a, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x69, 0x64, 0x5f,
    0x36, 0x18, 0x08, 0x20, 0x01, 0x28, 0x11, 0x3a, 0x02, 0x2d, 0x31, 0x22, 0xcf, 0x01, 0x0a, 0x1a,
    0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x6f, 0x6d,
    0x62, 0x61, 0x74, 0x4c, 0x6f, 0x67, 0x44, 0x61, 0x74, 0x61, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x79,
    0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x13, 0x0a, 0x0b, 0x74, 0x61, 0x72, 0x67,
    0x65, 0x74, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x15, 0x0a,
    0x0d, 0x61, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x65, 0x72, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0d, 0x12, 0x19, 0x0a, 0x11, 0x61, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x65, 0x72,
    0x5f, 0x69, 0x6c, 0x6c, 0x75, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x12,
    0x17, 0x0a, 0x0f, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x5f, 0x69, 0x6c, 0x6c, 0x75, 0x73, 0x69,
    0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x08, 0x12, 0x16, 0x0a, 0x0e, 0x69, 0x6e, 0x66, 0x6c,
    0x69, 0x63, 0x74, 0x6f, 0x72, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d,
    0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x05, 0x12,
    0x0e, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x18, 0x08, 0x20, 0x01, 0x28, 0x05, 0x12,
    0x0c, 0x0a, 0x04, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x09, 0x20, 0x01, 0x28, 0x02, 0x22, 0x21, 0x0a,
    0x1f, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x6f,
    0x6d, 0x62, 0x61, 0x74, 0x4c, 0x6f, 0x67, 0x53, 0x68, 0x6f, 0x77, 0x44, 0x65, 0x61, 0x74, 0x68,
    0x22, 0x5a, 0x0a, 0x14, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67,
    0x5f, 0x42, 0x6f, 0x74, 0x43, 0x68, 0x61, 0x74, 0x12, 0x11, 0x0a, 0x09, 0x70, 0x6c, 0x61, 0x79,
    0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0e, 0x0a, 0x06, 0x66,
    0x6f, 0x72, 0x6d, 0x61, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x0f, 0x0a, 0x07, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x12, 0x0e, 0x0a, 0x06,
    0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x22, 0x71, 0x0a, 0x20,
    0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x6f, 0x6d,
    0x62, 0x61, 0x74, 0x48, 0x65, 0x72, 0x6f, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x12, 0x0d, 0x0a, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12,
    0x0c, 0x0a, 0x04, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12, 0x20, 0x0a,
    0x09, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x5f, 0x70, 0x6f, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x0d, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x32, 0x44, 0x12,
    0x0e, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05, 0x22,
    0xfd, 0x01, 0x0a, 0x1c, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67,
    0x5f, 0x4d, 0x69, 0x6e, 0x69, 0x4b, 0x69, 0x6c, 0x6c, 0x43, 0x61, 0x6d, 0x49, 0x6e, 0x66, 0x6f,
    0x12, 0x39, 0x0a, 0x09, 0x61, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x65, 0x72, 0x73, 0x18, 0x01, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x26, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d,
    0x73, 0x67, 0x5f, 0x4d, 0x69, 0x6e, 0x69, 0x4b, 0x69, 0x6c, 0x6c, 0x43, 0x61, 0x6d, 0x49, 0x6e,
    0x66, 0x6f, 0x2e, 0x41, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x65, 0x72, 0x1a, 0xa1, 0x01, 0x0a, 0x08,
    0x41, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x65, 0x72, 0x12, 0x10, 0x0a, 0x08, 0x61, 0x74, 0x74, 0x61,
    0x63, 0x6b, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x14, 0x0a, 0x0c, 0x74, 0x6f,
    0x74, 0x61, 0x6c, 0x5f, 0x64, 0x61, 0x6d, 0x61, 0x67, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05,
    0x12, 0x41, 0x0a, 0x09, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x69, 0x65, 0x73, 0x18, 0x03, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x2e, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d,
    0x73, 0x67, 0x5f, 0x4d, 0x69, 0x6e, 0x69, 0x4b, 0x69, 0x6c, 0x6c, 0x43, 0x61, 0x6d, 0x49, 0x6e,
    0x66, 0x6f, 0x2e, 0x41, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x65, 0x72, 0x2e, 0x41, 0x62, 0x69, 0x6c,
    0x69, 0x74, 0x79, 0x1a, 0x2a, 0x0a, 0x07, 0x41, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x12, 0x0f,
    0x0a, 0x07, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12,
    0x0e, 0x0a, 0x06, 0x64, 0x61, 0x6d, 0x61, 0x67, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x22,
    0x40, 0x0a, 0x1d, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f,
    0x47, 0x6c, 0x6f, 0x62, 0x61, 0x6c, 0x4c, 0x69, 0x67, 0x68, 0x74, 0x43, 0x6f, 0x6c, 0x6f, 0x72,
    0x12, 0x0d, 0x0a, 0x05, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12,
    0x10, 0x0a, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x02, 0x22, 0x55, 0x0a, 0x21, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73,
    0x67, 0x5f, 0x47, 0x6c, 0x6f, 0x62, 0x61, 0x6c, 0x4c, 0x69, 0x67, 0x68, 0x74, 0x44, 0x69, 0x72,
    0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1e, 0x0a, 0x09, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x43, 0x4d, 0x73, 0x67,
    0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x12, 0x10, 0x0a, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x22, 0x5d, 0x0a, 0x19, 0x43, 0x44, 0x4f, 0x54,
    0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x50, 0x69, 0x6e, 0x67, 0x12, 0x11, 0x0a, 0x09, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x2d, 0x0a, 0x0d, 0x6c, 0x6f, 0x63, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x70, 0x69, 0x6e, 0x67, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x16, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x4d, 0x73, 0x67, 0x5f, 0x4c, 0x6f, 0x63, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x50, 0x69, 0x6e, 0x67, 0x22, 0x6e, 0x0a, 0x19, 0x43, 0x44, 0x4f, 0x54, 0x41,
    0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x4d, 0x69, 0x6e, 0x69, 0x6d, 0x61, 0x70, 0x45,
    0x76, 0x65, 0x6e, 0x74, 0x12, 0x12, 0x0a, 0x0a, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x79,
    0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x15, 0x0a, 0x0d, 0x65, 0x6e, 0x74, 0x69,
    0x74, 0x79, 0x5f, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12,
    0x09, 0x0a, 0x01, 0x78, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x12, 0x09, 0x0a, 0x01, 0x79, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x05, 0x12, 0x10, 0x0a, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05, 0x22, 0x4d, 0x0a, 0x14, 0x43, 0x44, 0x4f, 0x54, 0x41,
    0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x4d, 0x61, 0x70, 0x4c, 0x69, 0x6e, 0x65, 0x12,
    0x11, 0x0a, 0x09, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x05, 0x12, 0x22, 0x0a, 0x07, 0x6d, 0x61, 0x70, 0x6c, 0x69, 0x6e, 0x65, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x4d, 0x73, 0x67, 0x5f, 0x4d,
    0x61, 0x70, 0x4c, 0x69, 0x6e, 0x65, 0x22, 0x6e, 0x0a, 0x1e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55,
    0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x4d, 0x69, 0x6e, 0x69, 0x6d, 0x61, 0x70, 0x44, 0x65,
    0x62, 0x75, 0x67, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x12, 0x1d, 0x0a, 0x08, 0x6c, 0x6f, 0x63, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x43, 0x4d, 0x73,
    0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x12, 0x0d, 0x0a, 0x05, 0x63, 0x6f, 0x6c, 0x6f, 0x72,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0c, 0x0a, 0x04, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x05, 0x12, 0x10, 0x0a, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x02, 0x22, 0xae, 0x01, 0x0a, 0x23, 0x43, 0x44, 0x4f, 0x54, 0x41,
    0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x4c, 0x69,
    0x6e, 0x65, 0x61, 0x72, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x69, 0x6c, 0x65, 0x12, 0x1b,
    0x0a, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b,
    0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x12, 0x1f, 0x0a, 0x08, 0x76,
    0x65, 0x6c, 0x6f, 0x63, 0x69, 0x74, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e,
    0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x32, 0x44, 0x12, 0x0f, 0x0a, 0x07,
    0x6c, 0x61, 0x74, 0x65, 0x6e, 0x63, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x12, 0x10, 0x0a,
    0x08, 0x65, 0x6e, 0x74, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05, 0x12,
    0x16, 0x0a, 0x0e, 0x70, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x5f, 0x69, 0x6e, 0x64, 0x65,
    0x78, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0e, 0x0a, 0x06, 0x68, 0x61, 0x6e, 0x64, 0x6c,
    0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x05, 0x22, 0x36, 0x0a, 0x24, 0x43, 0x44, 0x4f, 0x54, 0x41,
    0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x44, 0x65, 0x73, 0x74, 0x72, 0x6f, 0x79, 0x4c,
    0x69, 0x6e, 0x65, 0x61, 0x72, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x69, 0x6c, 0x65, 0x12,
    0x0e, 0x0a, 0x06, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x22,
    0x39, 0x0a, 0x25, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f,
    0x44, 0x6f, 0x64, 0x67, 0x65, 0x54, 0x72, 0x61, 0x63, 0x6b, 0x69, 0x6e, 0x67, 0x50, 0x72, 0x6f,
    0x6a, 0x65, 0x63, 0x74, 0x69, 0x6c, 0x65, 0x73, 0x12, 0x10, 0x0a, 0x08, 0x65, 0x6e, 0x74, 0x69,
    0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x02, 0x28, 0x05, 0x22, 0x5f, 0x0a, 0x21, 0x43, 0x44,
    0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x70, 0x65, 0x63, 0x74,
    0x61, 0x74, 0x6f, 0x72, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x43, 0x6c, 0x69, 0x63, 0x6b, 0x12,
    0x10, 0x0a, 0x08, 0x65, 0x6e, 0x74, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x05, 0x12, 0x12, 0x0a, 0x0a, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x05, 0x12, 0x14, 0x0a, 0x0c, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x5f,
    0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x22, 0x62, 0x0a, 0x1d, 0x43,
    0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x4e, 0x65, 0x76, 0x65,
    0x72, 0x6d, 0x6f, 0x72, 0x65, 0x52, 0x65, 0x71, 0x75, 0x69, 0x65, 0x6d, 0x12, 0x15, 0x0a, 0x0d,
    0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x5f, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x05, 0x12, 0x0d, 0x0a, 0x05, 0x6c, 0x69, 0x6e, 0x65, 0x73, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x05, 0x12, 0x1b, 0x0a, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x22,
    0x2e, 0x0a, 0x1b, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f,
    0x49, 0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x12, 0x0f,
    0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x22,
    0x29, 0x0a, 0x15, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f,
    0x48, 0x75, 0x64, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x10, 0x0a, 0x08, 0x6f, 0x72, 0x64, 0x65,
    0x72, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x22, 0x63, 0x0a, 0x1b, 0x43, 0x44,
    0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x68, 0x61, 0x72, 0x65,
    0x64, 0x43, 0x6f, 0x6f, 0x6c, 0x64, 0x6f, 0x77, 0x6e, 0x12, 0x10, 0x0a, 0x08, 0x65, 0x6e, 0x74,
    0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0c, 0x0a, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x10, 0x0a, 0x08, 0x63, 0x6f, 0x6f,
    0x6c, 0x64, 0x6f, 0x77, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x02, 0x12, 0x12, 0x0a, 0x0a, 0x6e,
    0x61, 0x6d, 0x65, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05, 0x22,
    0x2f, 0x0a, 0x1f, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f,
    0x53, 0x65, 0x74, 0x4e, 0x65, 0x78, 0x74, 0x41, 0x75, 0x74, 0x6f, 0x62, 0x75, 0x79, 0x49, 0x74,
    0x65, 0x6d, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x22, 0xa3, 0x06, 0x0a, 0x16, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73,
    0x67, 0x5f, 0x55, 0x6e, 0x69, 0x74, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x26, 0x0a, 0x08, 0x6d,
    0x73, 0x67, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x14, 0x2e,
    0x45, 0x44, 0x6f, 0x74, 0x61, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x73, 0x12, 0x14, 0x0a, 0x0c, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x5f, 0x69, 0x6e,
    0x64, 0x65, 0x78, 0x18, 0x02, 0x20, 0x02, 0x28, 0x05, 0x12, 0x2e, 0x0a, 0x06, 0x73, 0x70, 0x65,
    0x65, 0x63, 0x68, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x43, 0x44, 0x4f, 0x54,
    0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x55, 0x6e, 0x69, 0x74, 0x45, 0x76, 0x65,
    0x6e, 0x74, 0x2e, 0x53, 0x70, 0x65, 0x65, 0x63, 0x68, 0x12, 0x37, 0x0a, 0x0b, 0x73, 0x70, 0x65,
    0x65, 0x63, 0x68, 0x5f, 0x6d, 0x75, 0x74, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x22,
    0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x55, 0x6e,
    0x69, 0x74, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x2e, 0x53, 0x70, 0x65, 0x65, 0x63, 0x68, 0x4d, 0x75,
    0x74, 0x65, 0x12, 0x37, 0x0a, 0x0b, 0x61, 0x64, 0x64, 0x5f, 0x67, 0x65, 0x73, 0x74, 0x75, 0x72,
    0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55,
    0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x55, 0x6e, 0x69, 0x74, 0x45, 0x76, 0x65, 0x6e, 0x74,
    0x2e, 0x41, 0x64, 0x64, 0x47, 0x65, 0x73, 0x74, 0x75, 0x72, 0x65, 0x12, 0x3d, 0x0a, 0x0e, 0x72,
    0x65, 0x6d, 0x6f, 0x76, 0x65, 0x5f, 0x67, 0x65, 0x73, 0x74, 0x75, 0x72, 0x65, 0x18, 0x06, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d,
    0x73, 0x67, 0x5f, 0x55, 0x6e, 0x69, 0x74, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x2e, 0x52, 0x65, 0x6d,
    0x6f, 0x76, 0x65, 0x47, 0x65, 0x73, 0x74, 0x75, 0x72, 0x65, 0x12, 0x39, 0x0a, 0x0c, 0x62, 0x6c,
    0x6f, 0x6f, 0x64, 0x5f, 0x69, 0x6d, 0x70, 0x61, 0x63, 0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x23, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f,
    0x55, 0x6e, 0x69, 0x74, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x2e, 0x42, 0x6c, 0x6f, 0x6f, 0x64, 0x49,
    0x6d, 0x70, 0x61, 0x63, 0x74, 0x12, 0x39, 0x0a, 0x0c, 0x66, 0x61, 0x64, 0x65, 0x5f, 0x67, 0x65,
    0x73, 0x74, 0x75, 0x72, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x43, 0x44,
    0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x55, 0x6e, 0x69, 0x74, 0x45,
    0x76, 0x65, 0x6e, 0x74, 0x2e, 0x46, 0x61, 0x64, 0x65, 0x47, 0x65, 0x73, 0x74, 0x75, 0x72, 0x65,
    0x1a, 0x52, 0x0a, 0x06, 0x53, 0x70, 0x65, 0x65, 0x63, 0x68, 0x12, 0x0f, 0x0a, 0x07, 0x63, 0x6f,
    0x6e, 0x63, 0x65, 0x70, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x10, 0x0a, 0x08, 0x72,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x16, 0x0a,
    0x0e, 0x72, 0x65, 0x63, 0x69, 0x70, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0d, 0x0a, 0x05, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x05, 0x1a, 0x20, 0x0a, 0x0a, 0x53, 0x70, 0x65, 0x65, 0x63, 0x68, 0x4d, 0x75,
    0x74, 0x65, 0x12, 0x12, 0x0a, 0x05, 0x64, 0x65, 0x6c, 0x61, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x02, 0x3a, 0x03, 0x30, 0x2e, 0x35, 0x1a, 0x62, 0x0a, 0x0a, 0x41, 0x64, 0x64, 0x47, 0x65, 0x73,
    0x74, 0x75, 0x72, 0x65, 0x12, 0x1b, 0x0a, 0x08, 0x61, 0x63, 0x74, 0x69, 0x76, 0x69, 0x74, 0x79,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x09, 0x2e, 0x41, 0x63, 0x74, 0x69, 0x76, 0x69, 0x74,
    0x79, 0x12, 0x0c, 0x0a, 0x04, 0x73, 0x6c, 0x6f, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12,
    0x12, 0x0a, 0x07, 0x66, 0x61, 0x64, 0x65, 0x5f, 0x69, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x02,
    0x3a, 0x01, 0x30, 0x12, 0x15, 0x0a, 0x08, 0x66, 0x61, 0x64, 0x65, 0x5f, 0x6f, 0x75, 0x74, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x02, 0x3a, 0x03, 0x30, 0x2e, 0x31, 0x1a, 0x2c, 0x0a, 0x0d, 0x52, 0x65,
    0x6d, 0x6f, 0x76, 0x65, 0x47, 0x65, 0x73, 0x74, 0x75, 0x72, 0x65, 0x12, 0x1b, 0x0a, 0x08, 0x61,
    0x63, 0x74, 0x69, 0x76, 0x69, 0x74, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x09, 0x2e,
    0x41, 0x63, 0x74, 0x69, 0x76, 0x69, 0x74, 0x79, 0x1a, 0x40, 0x0a, 0x0b, 0x42, 0x6c, 0x6f, 0x6f,
    0x64, 0x49, 0x6d, 0x70, 0x61, 0x63, 0x74, 0x12, 0x0d, 0x0a, 0x05, 0x73, 0x63, 0x61, 0x6c, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x10, 0x0a, 0x08, 0x78, 0x5f, 0x6e, 0x6f, 0x72, 0x6d,
    0x61, 0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12, 0x10, 0x0a, 0x08, 0x79, 0x5f, 0x6e, 0x6f,
    0x72, 0x6d, 0x61, 0x6c, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x1a, 0x2a, 0x0a, 0x0b, 0x46, 0x61,
    0x64, 0x65, 0x47, 0x65, 0x73, 0x74, 0x75, 0x72, 0x65, 0x12, 0x1b, 0x0a, 0x08, 0x61, 0x63, 0x74,
    0x69, 0x76, 0x69, 0x74, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x09, 0x2e, 0x41, 0x63,
    0x74, 0x69, 0x76, 0x69, 0x74, 0x79, 0x22, 0x30, 0x0a, 0x1a, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55,
    0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x49, 0x74, 0x65, 0x6d, 0x50, 0x75, 0x72, 0x63, 0x68,
    0x61, 0x73, 0x65, 0x64, 0x12, 0x12, 0x0a, 0x0a, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x69, 0x6e, 0x64,
    0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x22, 0x6a, 0x0a, 0x16, 0x43, 0x44, 0x4f, 0x54,
    0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x49, 0x74, 0x65, 0x6d, 0x46, 0x6f, 0x75,
    0x6e, 0x64, 0x12, 0x0e, 0x0a, 0x06, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x05, 0x12, 0x0f, 0x0a, 0x07, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x05, 0x12, 0x0e, 0x0a, 0x06, 0x72, 0x61, 0x72, 0x69, 0x74, 0x79, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x05, 0x12, 0x0e, 0x0a, 0x06, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x05, 0x12, 0x0f, 0x0a, 0x07, 0x69, 0x74, 0x65, 0x6d, 0x64, 0x65, 0x66, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x05, 0x22, 0xcb, 0x0e, 0x0a, 0x1c, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73,
    0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4d, 0x61,
    0x6e, 0x61, 0x67, 0x65, 0x72, 0x12, 0x24, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x0e, 0x32, 0x16, 0x2e, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x50, 0x41, 0x52, 0x54, 0x49,
    0x43, 0x4c, 0x45, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x12, 0x0d, 0x0a, 0x05, 0x69,
    0x6e, 0x64, 0x65, 0x78, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x52, 0x0a, 0x16, 0x72, 0x65,
    0x6c, 0x65, 0x61, 0x73, 0x65, 0x5f, 0x70, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x5f, 0x69,
    0x6e, 0x64, 0x65, 0x78, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x32, 0x2e, 0x43, 0x44, 0x4f,
    0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63,
    0x6c, 0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e, 0x52, 0x65, 0x6c, 0x65, 0x61, 0x73,
    0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x45,
    0x0a, 0x0f, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x5f, 0x70, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c,
    0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2c, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55,
    0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4d,
    0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72,
    0x74, 0x69, 0x63, 0x6c, 0x65, 0x12, 0x47, 0x0a, 0x10, 0x64, 0x65, 0x73, 0x74, 0x72, 0x6f, 0x79,
    0x5f, 0x70, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x2d, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x50,
    0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e, 0x44,
    0x65, 0x73, 0x74, 0x72, 0x6f, 0x79, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x12, 0x5a,
    0x0a, 0x1a, 0x64, 0x65, 0x73, 0x74, 0x72, 0x6f, 0x79, 0x5f, 0x70, 0x61, 0x72, 0x74, 0x69, 0x63,
    0x6c, 0x65, 0x5f, 0x69, 0x6e, 0x76, 0x6f, 0x6c, 0x76, 0x69, 0x6e, 0x67, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x36, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73,
    0x67, 0x5f, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65,
    0x72, 0x2e, 0x44, 0x65, 0x73, 0x74, 0x72, 0x6f, 0x79, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c,
    0x65, 0x49, 0x6e, 0x76, 0x6f, 0x6c, 0x76, 0x69, 0x6e, 0x67, 0x12, 0x45, 0x0a, 0x0f, 0x75, 0x70,
    0x64, 0x61, 0x74, 0x65, 0x5f, 0x70, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x18, 0x07, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x2c, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d,
    0x73, 0x67, 0x5f, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67,
    0x65, 0x72, 0x2e, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c,
    0x65, 0x12, 0x4c, 0x0a, 0x13, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x70, 0x61, 0x72, 0x74,
    0x69, 0x63, 0x6c, 0x65, 0x5f, 0x66, 0x77, 0x64, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2f,
    0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x50, 0x61,
    0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e, 0x55, 0x70,
    0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x46, 0x77, 0x64, 0x12,
    0x52, 0x0a, 0x16, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x70, 0x61, 0x72, 0x74, 0x69, 0x63,
    0x6c, 0x65, 0x5f, 0x6f, 0x72, 0x69, 0x65, 0x6e, 0x74, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x32, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x50,
    0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e, 0x55,
    0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4f, 0x72, 0x69,
    0x65, 0x6e, 0x74, 0x12, 0x56, 0x0a, 0x18, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x70, 0x61,
    0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x5f, 0x66, 0x61, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x18,
    0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x34, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65,
    0x72, 0x4d, 0x73, 0x67, 0x5f, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4d, 0x61, 0x6e,
    0x61, 0x67, 0x65, 0x72, 0x2e, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69,
    0x63, 0x6c, 0x65, 0x46, 0x61, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x12, 0x52, 0x0a, 0x16, 0x75,
    0x70, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x70, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x5f, 0x6f,
    0x66, 0x66, 0x73, 0x65, 0x74, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x32, 0x2e, 0x43, 0x44,
    0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x50, 0x61, 0x72, 0x74, 0x69,
    0x63, 0x6c, 0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e, 0x55, 0x70, 0x64, 0x61, 0x74,
    0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x12,
    0x4c, 0x0a, 0x13, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x70, 0x61, 0x72, 0x74, 0x69, 0x63,
    0x6c, 0x65, 0x5f, 0x65, 0x6e, 0x74, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2f, 0x2e, 0x43,
    0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x50, 0x61, 0x72, 0x74,
    0x69, 0x63, 0x6c, 0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e, 0x55, 0x70, 0x64, 0x61,
    0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x45, 0x6e, 0x74, 0x12, 0x54, 0x0a,
    0x17, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x70, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65,
    0x5f, 0x6c, 0x61, 0x74, 0x65, 0x6e, 0x63, 0x79, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x33,
    0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x50, 0x61,
    0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e, 0x55, 0x70,
    0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4c, 0x61, 0x74, 0x65,
    0x6e, 0x63, 0x79, 0x1a, 0x16, 0x0a, 0x14, 0x52, 0x65, 0x6c, 0x65, 0x61, 0x73, 0x65, 0x50, 0x61,
    0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x1a, 0x59, 0x0a, 0x0e, 0x43,
    0x72, 0x65, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x12, 0x1b, 0x0a,
    0x13, 0x70, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x5f, 0x69,
    0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x13, 0x0a, 0x0b, 0x61, 0x74,
    0x74, 0x61, 0x63, 0x68, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12,
    0x15, 0x0a, 0x0d, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x5f, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x1a, 0x2e, 0x0a, 0x0f, 0x44, 0x65, 0x73, 0x74, 0x72, 0x6f,
    0x79, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x12, 0x1b, 0x0a, 0x13, 0x64, 0x65, 0x73,
    0x74, 0x72, 0x6f, 0x79, 0x5f, 0x69, 0x6d, 0x6d, 0x65, 0x64, 0x69, 0x61, 0x74, 0x65, 0x6c, 0x79,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x1a, 0x4e, 0x0a, 0x18, 0x44, 0x65, 0x73, 0x74, 0x72, 0x6f,
    0x79, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x49, 0x6e, 0x76, 0x6f, 0x6c, 0x76, 0x69,
    0x6e, 0x67, 0x12, 0x1b, 0x0a, 0x13, 0x64, 0x65, 0x73, 0x74, 0x72, 0x6f, 0x79, 0x5f, 0x69, 0x6d,
    0x6d, 0x65, 0x64, 0x69, 0x61, 0x74, 0x65, 0x6c, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x12,
    0x15, 0x0a, 0x0d, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x5f, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x1a, 0x46, 0x0a, 0x0e, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65,
    0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x12, 0x15, 0x0a, 0x0d, 0x63, 0x6f, 0x6e, 0x74,
    0x72, 0x6f, 0x6c, 0x5f, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12,
    0x1d, 0x0a, 0x08, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x0b, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x1a, 0x48,
    0x0a, 0x11, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65,
    0x46, 0x77, 0x64, 0x12, 0x15, 0x0a, 0x0d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x5f, 0x70,
    0x6f, 0x69, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x1c, 0x0a, 0x07, 0x66, 0x6f,
    0x72, 0x77, 0x61, 0x72, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x43, 0x4d,
    0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x1a, 0x80, 0x01, 0x0a, 0x14, 0x55, 0x70, 0x64,
    0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4f, 0x72, 0x69, 0x65, 0x6e,
    0x74, 0x12, 0x15, 0x0a, 0x0d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x5f, 0x70, 0x6f, 0x69,
    0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x1c, 0x0a, 0x07, 0x66, 0x6f, 0x72, 0x77,
    0x61, 0x72, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x43, 0x4d, 0x73, 0x67,
    0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x12, 0x1a, 0x0a, 0x05, 0x72, 0x69, 0x67, 0x68, 0x74, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74,
    0x6f, 0x72, 0x12, 0x17, 0x0a, 0x02, 0x75, 0x70, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b,
    0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x1a, 0x4e, 0x0a, 0x16, 0x55,
    0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x46, 0x61, 0x6c,
    0x6c, 0x62, 0x61, 0x63, 0x6b, 0x12, 0x15, 0x0a, 0x0d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c,
    0x5f, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x1d, 0x0a, 0x08,
    0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b,
    0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x1a, 0x51, 0x0a, 0x14, 0x55,
    0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4f, 0x66, 0x66,
    0x73, 0x65, 0x74, 0x12, 0x15, 0x0a, 0x0d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x5f, 0x70,
    0x6f, 0x69, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x22, 0x0a, 0x0d, 0x6f, 0x72,
    0x69, 0x67, 0x69, 0x6e, 0x5f, 0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x0b, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x1a, 0x92,
    0x01, 0x0a, 0x11, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c,
    0x65, 0x45, 0x6e, 0x74, 0x12, 0x15, 0x0a, 0x0d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x5f,
    0x70, 0x6f, 0x69, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x15, 0x0a, 0x0d, 0x65,
    0x6e, 0x74, 0x69, 0x74, 0x79, 0x5f, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x05, 0x12, 0x13, 0x0a, 0x0b, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x5f, 0x74, 0x79, 0x70,
    0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x12, 0x12, 0x0a, 0x0a, 0x61, 0x74, 0x74, 0x61, 0x63,
    0x68, 0x6d, 0x65, 0x6e, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05, 0x12, 0x26, 0x0a, 0x11, 0x66,
    0x61, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x5f, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63,
    0x74, 0x6f, 0x72, 0x1a, 0x3d, 0x0a, 0x15, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72,
    0x74, 0x69, 0x63, 0x6c, 0x65, 0x4c, 0x61, 0x74, 0x65, 0x6e, 0x63, 0x79, 0x12, 0x16, 0x0a, 0x0e,
    0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x6c, 0x61, 0x74, 0x65, 0x6e, 0x63, 0x79, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x05, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x69, 0x63, 0x6b, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x05, 0x22, 0xb0, 0x01, 0x0a, 0x1a, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73, 0x65, 0x72,
    0x4d, 0x73, 0x67, 0x5f, 0x4f, 0x76, 0x65, 0x72, 0x68, 0x65, 0x61, 0x64, 0x45, 0x76, 0x65, 0x6e,
    0x74, 0x12, 0x2a, 0x0a, 0x0c, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x74, 0x79, 0x70,
    0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x14, 0x2e, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x4f,
    0x56, 0x45, 0x52, 0x48, 0x45, 0x41, 0x44, 0x5f, 0x41, 0x4c, 0x45, 0x52, 0x54, 0x12, 0x0d, 0x0a,
    0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12, 0x1e, 0x0a, 0x16,
    0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x5f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x65, 0x6e,
    0x74, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x12, 0x17, 0x0a, 0x0f,
    0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x5f, 0x65, 0x6e, 0x74, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x05, 0x12, 0x1e, 0x0a, 0x16, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f,
    0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x65, 0x6e, 0x74, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x05, 0x22, 0x3e, 0x0a, 0x1c, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x55, 0x73,
    0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x54, 0x75, 0x74, 0x6f, 0x72, 0x69, 0x61, 0x6c, 0x54, 0x69,
    0x70, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x09, 0x12, 0x10, 0x0a, 0x08, 0x70, 0x72, 0x6f, 0x67, 0x72, 0x65, 0x73, 0x73, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x05, 0x2a, 0xe6, 0x06, 0x0a, 0x11, 0x45, 0x44, 0x6f, 0x74, 0x61, 0x55,
    0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x12, 0x1e, 0x0a, 0x1a, 0x44,
    0x4f, 0x54, 0x41, 0x5f, 0x55, 0x4d, 0x5f, 0x41, 0x64, 0x64, 0x55, 0x6e, 0x69, 0x74, 0x54, 0x6f,
    0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x10, 0x40, 0x12, 0x17, 0x0a, 0x13, 0x44,
    0x4f, 0x54, 0x41, 0x5f, 0x55, 0x4d, 0x5f, 0x41, 0x49, 0x44, 0x65, 0x62, 0x75, 0x67, 0x4c, 0x69,
    0x6e, 0x65, 0x10, 0x41, 0x12, 0x15, 0x0a, 0x11, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x55, 0x4d, 0x5f,
    0x43, 0x68, 0x61, 0x74, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x10, 0x42, 0x12, 0x1f, 0x0a, 0x1b, 0x44,
    0x4f, 0x54, 0x41, 0x5f, 0x55, 0x4d, 0x5f, 0x43, 0x6f, 0x6d, 0x62, 0x61, 0x74, 0x48, 0x65, 0x72,
    0x6f, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x10, 0x43, 0x12, 0x19, 0x0a, 0x15,
    0x44, 0x4f, 0x54, 0x41, 0x5f, 0x55, 0x4d, 0x5f, 0x43, 0x6f, 0x6d, 0x62, 0x61, 0x74, 0x4c, 0x6f,
    0x67, 0x44, 0x61, 0x74, 0x61, 0x10, 0x44, 0x12, 0x1e, 0x0a, 0x1a, 0x44, 0x4f, 0x54, 0x41, 0x5f,
    0x55, 0x4d, 0x5f, 0x43, 0x6f, 0x6d, 0x62, 0x61, 0x74, 0x4c, 0x6f, 0x67, 0x53, 0x68, 0x6f, 0x77,
    0x44, 0x65, 0x61, 0x74, 0x68, 0x10, 0x46, 0x12, 0x22, 0x0a, 0x1e, 0x44, 0x4f, 0x54, 0x41, 0x5f,
    0x55, 0x4d, 0x5f, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x4c, 0x69, 0x6e, 0x65, 0x61, 0x72, 0x50,
    0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x69, 0x6c, 0x65, 0x10, 0x47, 0x12, 0x23, 0x0a, 0x1f, 0x44,
    0x4f, 0x54, 0x41, 0x5f, 0x55, 0x4d, 0x5f, 0x44, 0x65, 0x73, 0x74, 0x72, 0x6f, 0x79, 0x4c, 0x69,
    0x6e, 0x65, 0x61, 0x72, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x69, 0x6c, 0x65, 0x10, 0x48,
    0x12, 0x24, 0x0a, 0x20, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x55, 0x4d, 0x5f, 0x44, 0x6f, 0x64, 0x67,
    0x65, 0x54, 0x72, 0x61, 0x63, 0x6b, 0x69, 0x6e, 0x67, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74,
    0x69, 0x6c, 0x65, 0x73, 0x10, 0x49, 0x12, 0x1c, 0x0a, 0x18, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x55,
    0x4d, 0x5f, 0x47, 0x6c, 0x6f, 0x62, 0x61, 0x6c, 0x4c, 0x69, 0x67, 0x68, 0x74, 0x43, 0x6f, 0x6c,
    0x6f, 0x72, 0x10, 0x4a, 0x12, 0x20, 0x0a, 0x1c, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x55, 0x4d, 0x5f,
    0x47, 0x6c, 0x6f, 0x62, 0x61, 0x6c, 0x4c, 0x69, 0x67, 0x68, 0x74, 0x44, 0x69, 0x72, 0x65, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x10, 0x4b, 0x12, 0x1a, 0x0a, 0x16, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x55,
    0x4d, 0x5f, 0x49, 0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64,
    0x10, 0x4c, 0x12, 0x18, 0x0a, 0x14, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x55, 0x4d, 0x5f, 0x4c, 0x6f,
    0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x69, 0x6e, 0x67, 0x10, 0x4d, 0x12, 0x13, 0x0a, 0x0f,
    0x44, 0x4f, 0x54, 0x41, 0x5f, 0x55, 0x4d, 0x5f, 0x4d, 0x61, 0x70, 0x4c, 0x69, 0x6e, 0x65, 0x10,
    0x4e, 0x12, 0x1b, 0x0a, 0x17, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x55, 0x4d, 0x5f, 0x4d, 0x69, 0x6e,
    0x69, 0x4b, 0x69, 0x6c, 0x6c, 0x43, 0x61, 0x6d, 0x49, 0x6e, 0x66, 0x6f, 0x10, 0x4f, 0x12, 0x1d,
    0x0a, 0x19, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x55, 0x4d, 0x5f, 0x4d, 0x69, 0x6e, 0x69, 0x6d, 0x61,
    0x70, 0x44, 0x65, 0x62, 0x75, 0x67, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x10, 0x50, 0x12, 0x18, 0x0a,
    0x14, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x55, 0x4d, 0x5f, 0x4d, 0x69, 0x6e, 0x69, 0x6d, 0x61, 0x70,
    0x45, 0x76, 0x65, 0x6e, 0x74, 0x10, 0x51, 0x12, 0x1c, 0x0a, 0x18, 0x44, 0x4f, 0x54, 0x41, 0x5f,
    0x55, 0x4d, 0x5f, 0x4e, 0x65, 0x76, 0x65, 0x72, 0x6d, 0x6f, 0x72, 0x65, 0x52, 0x65, 0x71, 0x75,
    0x69, 0x65, 0x6d, 0x10, 0x52, 0x12, 0x19, 0x0a, 0x15, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x55, 0x4d,
    0x5f, 0x4f, 0x76, 0x65, 0x72, 0x68, 0x65, 0x61, 0x64, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x10, 0x53,
    0x12, 0x1e, 0x0a, 0x1a, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x55, 0x4d, 0x5f, 0x53, 0x65, 0x74, 0x4e,
    0x65, 0x78, 0x74, 0x41, 0x75, 0x74, 0x6f, 0x62, 0x75, 0x79, 0x49, 0x74, 0x65, 0x6d, 0x10, 0x54,
    0x12, 0x1a, 0x0a, 0x16, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x55, 0x4d, 0x5f, 0x53, 0x68, 0x61, 0x72,
    0x65, 0x64, 0x43, 0x6f, 0x6f, 0x6c, 0x64, 0x6f, 0x77, 0x6e, 0x10, 0x55, 0x12, 0x20, 0x0a, 0x1c,
    0x44, 0x4f, 0x54, 0x41, 0x5f, 0x55, 0x4d, 0x5f, 0x53, 0x70, 0x65, 0x63, 0x74, 0x61, 0x74, 0x6f,
    0x72, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x43, 0x6c, 0x69, 0x63, 0x6b, 0x10, 0x56, 0x12, 0x1b,
    0x0a, 0x17, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x55, 0x4d, 0x5f, 0x54, 0x75, 0x74, 0x6f, 0x72, 0x69,
    0x61, 0x6c, 0x54, 0x69, 0x70, 0x49, 0x6e, 0x66, 0x6f, 0x10, 0x57, 0x12, 0x15, 0x0a, 0x11, 0x44,
    0x4f, 0x54, 0x41, 0x5f, 0x55, 0x4d, 0x5f, 0x55, 0x6e, 0x69, 0x74, 0x45, 0x76, 0x65, 0x6e, 0x74,
    0x10, 0x58, 0x12, 0x1b, 0x0a, 0x17, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x55, 0x4d, 0x5f, 0x50, 0x61,
    0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x10, 0x59, 0x12,
    0x13, 0x0a, 0x0f, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x55, 0x4d, 0x5f, 0x42, 0x6f, 0x74, 0x43, 0x68,
    0x61, 0x74, 0x10, 0x5a, 0x12, 0x14, 0x0a, 0x10, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x55, 0x4d, 0x5f,
    0x48, 0x75, 0x64, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x10, 0x5b, 0x12, 0x19, 0x0a, 0x15, 0x44, 0x4f,
    0x54, 0x41, 0x5f, 0x55, 0x4d, 0x5f, 0x49, 0x74, 0x65, 0x6d, 0x50, 0x75, 0x72, 0x63, 0x68, 0x61,
    0x73, 0x65, 0x64, 0x10, 0x5c, 0x12, 0x10, 0x0a, 0x0c, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x55, 0x4d,
    0x5f, 0x50, 0x69, 0x6e, 0x67, 0x10, 0x5d, 0x12, 0x15, 0x0a, 0x11, 0x44, 0x4f, 0x54, 0x41, 0x5f,
    0x55, 0x4d, 0x5f, 0x49, 0x74, 0x65, 0x6d, 0x46, 0x6f, 0x75, 0x6e, 0x64, 0x10, 0x5e, 0x2a, 0xaa,
    0x0c, 0x0a, 0x11, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53,
    0x53, 0x41, 0x47, 0x45, 0x12, 0x21, 0x0a, 0x14, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53,
    0x53, 0x41, 0x47, 0x45, 0x5f, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x10, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01, 0x12, 0x1a, 0x0a, 0x16, 0x43, 0x48, 0x41, 0x54, 0x5f,
    0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f, 0x48, 0x45, 0x52, 0x4f, 0x5f, 0x4b, 0x49, 0x4c,
    0x4c, 0x10, 0x00, 0x12, 0x1a, 0x0a, 0x16, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53,
    0x41, 0x47, 0x45, 0x5f, 0x48, 0x45, 0x52, 0x4f, 0x5f, 0x44, 0x45, 0x4e, 0x59, 0x10, 0x01, 0x12,
    0x1e, 0x0a, 0x1a, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f,
    0x42, 0x41, 0x52, 0x52, 0x41, 0x43, 0x4b, 0x53, 0x5f, 0x4b, 0x49, 0x4c, 0x4c, 0x10, 0x02, 0x12,
    0x1b, 0x0a, 0x17, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f,
    0x54, 0x4f, 0x57, 0x45, 0x52, 0x5f, 0x4b, 0x49, 0x4c, 0x4c, 0x10, 0x03, 0x12, 0x1b, 0x0a, 0x17,
    0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f, 0x54, 0x4f, 0x57,
    0x45, 0x52, 0x5f, 0x44, 0x45, 0x4e, 0x59, 0x10, 0x04, 0x12, 0x1b, 0x0a, 0x17, 0x43, 0x48, 0x41,
    0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f, 0x46, 0x49, 0x52, 0x53, 0x54, 0x42,
    0x4c, 0x4f, 0x4f, 0x44, 0x10, 0x05, 0x12, 0x1c, 0x0a, 0x18, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d,
    0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f, 0x53, 0x54, 0x52, 0x45, 0x41, 0x4b, 0x5f, 0x4b, 0x49,
    0x4c, 0x4c, 0x10, 0x06, 0x12, 0x18, 0x0a, 0x14, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53,
    0x53, 0x41, 0x47, 0x45, 0x5f, 0x42, 0x55, 0x59, 0x42, 0x41, 0x43, 0x4b, 0x10, 0x07, 0x12, 0x16,
    0x0a, 0x12, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f, 0x41,
    0x45, 0x47, 0x49, 0x53, 0x10, 0x08, 0x12, 0x1c, 0x0a, 0x18, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d,
    0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f, 0x52, 0x4f, 0x53, 0x48, 0x41, 0x4e, 0x5f, 0x4b, 0x49,
    0x4c, 0x4c, 0x10, 0x09, 0x12, 0x1d, 0x0a, 0x19, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53,
    0x53, 0x41, 0x47, 0x45, 0x5f, 0x43, 0x4f, 0x55, 0x52, 0x49, 0x45, 0x52, 0x5f, 0x4c, 0x4f, 0x53,
    0x54, 0x10, 0x0a, 0x12, 0x22, 0x0a, 0x1e, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53,
    0x41, 0x47, 0x45, 0x5f, 0x43, 0x4f, 0x55, 0x52, 0x49, 0x45, 0x52, 0x5f, 0x52, 0x45, 0x53, 0x50,
    0x41, 0x57, 0x4e, 0x45, 0x44, 0x10, 0x0b, 0x12, 0x1b, 0x0a, 0x17, 0x43, 0x48, 0x41, 0x54, 0x5f,
    0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f, 0x47, 0x4c, 0x59, 0x50, 0x48, 0x5f, 0x55, 0x53,
    0x45, 0x44, 0x10, 0x0c, 0x12, 0x1e, 0x0a, 0x1a, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53,
    0x53, 0x41, 0x47, 0x45, 0x5f, 0x49, 0x54, 0x45, 0x4d, 0x5f, 0x50, 0x55, 0x52, 0x43, 0x48, 0x41,
    0x53, 0x45, 0x10, 0x0d, 0x12, 0x18, 0x0a, 0x14, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53,
    0x53, 0x41, 0x47, 0x45, 0x5f, 0x43, 0x4f, 0x4e, 0x4e, 0x45, 0x43, 0x54, 0x10, 0x0e, 0x12, 0x1b,
    0x0a, 0x17, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f, 0x44,
    0x49, 0x53, 0x43, 0x4f, 0x4e, 0x4e, 0x45, 0x43, 0x54, 0x10, 0x0f, 0x12, 0x2e, 0x0a, 0x2a, 0x43,
    0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f, 0x44, 0x49, 0x53, 0x43,
    0x4f, 0x4e, 0x4e, 0x45, 0x43, 0x54, 0x5f, 0x57, 0x41, 0x49, 0x54, 0x5f, 0x46, 0x4f, 0x52, 0x5f,
    0x52, 0x45, 0x43, 0x4f, 0x4e, 0x4e, 0x45, 0x43, 0x54, 0x10, 0x10, 0x12, 0x2a, 0x0a, 0x26, 0x43,
    0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f, 0x44, 0x49, 0x53, 0x43,
    0x4f, 0x4e, 0x4e, 0x45, 0x43, 0x54, 0x5f, 0x54, 0x49, 0x4d, 0x45, 0x5f, 0x52, 0x45, 0x4d, 0x41,
    0x49, 0x4e, 0x49, 0x4e, 0x47, 0x10, 0x11, 0x12, 0x31, 0x0a, 0x2d, 0x43, 0x48, 0x41, 0x54, 0x5f,
    0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f, 0x44, 0x49, 0x53, 0x43, 0x4f, 0x4e, 0x4e, 0x45,
    0x43, 0x54, 0x5f, 0x54, 0x49, 0x4d, 0x45, 0x5f, 0x52, 0x45, 0x4d, 0x41, 0x49, 0x4e, 0x49, 0x4e,
    0x47, 0x5f, 0x50, 0x4c, 0x55, 0x52, 0x41, 0x4c, 0x10, 0x12, 0x12, 0x1a, 0x0a, 0x16, 0x43, 0x48,
    0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f, 0x52, 0x45, 0x43, 0x4f, 0x4e,
    0x4e, 0x45, 0x43, 0x54, 0x10, 0x13, 0x12, 0x18, 0x0a, 0x14, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d,
    0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f, 0x41, 0x42, 0x41, 0x4e, 0x44, 0x4f, 0x4e, 0x10, 0x14,
    0x12, 0x1e, 0x0a, 0x1a, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45,
    0x5f, 0x53, 0x41, 0x46, 0x45, 0x5f, 0x54, 0x4f, 0x5f, 0x4c, 0x45, 0x41, 0x56, 0x45, 0x10, 0x15,
    0x12, 0x1c, 0x0a, 0x18, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45,
    0x5f, 0x52, 0x55, 0x4e, 0x45, 0x5f, 0x50, 0x49, 0x43, 0x4b, 0x55, 0x50, 0x10, 0x16, 0x12, 0x1c,
    0x0a, 0x18, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f, 0x52,
    0x55, 0x4e, 0x45, 0x5f, 0x42, 0x4f, 0x54, 0x54, 0x4c, 0x45, 0x10, 0x17, 0x12, 0x19, 0x0a, 0x15,
    0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f, 0x49, 0x4e, 0x54,
    0x48, 0x45, 0x42, 0x41, 0x47, 0x10, 0x18, 0x12, 0x1b, 0x0a, 0x17, 0x43, 0x48, 0x41, 0x54, 0x5f,
    0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f, 0x53, 0x45, 0x43, 0x52, 0x45, 0x54, 0x53, 0x48,
    0x4f, 0x50, 0x10, 0x19, 0x12, 0x23, 0x0a, 0x1f, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53,
    0x53, 0x41, 0x47, 0x45, 0x5f, 0x49, 0x54, 0x45, 0x4d, 0x5f, 0x41, 0x55, 0x54, 0x4f, 0x50, 0x55,
    0x52, 0x43, 0x48, 0x41, 0x53, 0x45, 0x44, 0x10, 0x1a, 0x12, 0x1f, 0x0a, 0x1b, 0x43, 0x48, 0x41,
    0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f, 0x49, 0x54, 0x45, 0x4d, 0x53, 0x5f,
    0x43, 0x4f, 0x4d, 0x42, 0x49, 0x4e, 0x45, 0x44, 0x10, 0x1b, 0x12, 0x1d, 0x0a, 0x19, 0x43, 0x48,
    0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f, 0x53, 0x55, 0x50, 0x45, 0x52,
    0x5f, 0x43, 0x52, 0x45, 0x45, 0x50, 0x53, 0x10, 0x1c, 0x12, 0x25, 0x0a, 0x21, 0x43, 0x48, 0x41,
    0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f, 0x43, 0x41, 0x4e, 0x54, 0x5f, 0x55,
    0x53, 0x45, 0x5f, 0x41, 0x43, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x49, 0x54, 0x45, 0x4d, 0x10, 0x1d,
    0x12, 0x22, 0x0a, 0x1e, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45,
    0x5f, 0x43, 0x48, 0x41, 0x52, 0x47, 0x45, 0x53, 0x5f, 0x45, 0x58, 0x48, 0x41, 0x55, 0x53, 0x54,
    0x45, 0x44, 0x10, 0x1e, 0x12, 0x1a, 0x0a, 0x16, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53,
    0x53, 0x41, 0x47, 0x45, 0x5f, 0x43, 0x41, 0x4e, 0x54, 0x50, 0x41, 0x55, 0x53, 0x45, 0x10, 0x1f,
    0x12, 0x1d, 0x0a, 0x19, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45,
    0x5f, 0x4e, 0x4f, 0x50, 0x41, 0x55, 0x53, 0x45, 0x53, 0x4c, 0x45, 0x46, 0x54, 0x10, 0x20, 0x12,
    0x1d, 0x0a, 0x19, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f,
    0x43, 0x41, 0x4e, 0x54, 0x50, 0x41, 0x55, 0x53, 0x45, 0x59, 0x45, 0x54, 0x10, 0x21, 0x12, 0x17,
    0x0a, 0x13, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f, 0x50,
    0x41, 0x55, 0x53, 0x45, 0x44, 0x10, 0x22, 0x12, 0x22, 0x0a, 0x1e, 0x43, 0x48, 0x41, 0x54, 0x5f,
    0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f, 0x55, 0x4e, 0x50, 0x41, 0x55, 0x53, 0x45, 0x5f,
    0x43, 0x4f, 0x55, 0x4e, 0x54, 0x44, 0x4f, 0x57, 0x4e, 0x10, 0x23, 0x12, 0x19, 0x0a, 0x15, 0x43,
    0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f, 0x55, 0x4e, 0x50, 0x41,
    0x55, 0x53, 0x45, 0x44, 0x10, 0x24, 0x12, 0x1e, 0x0a, 0x1a, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d,
    0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f, 0x41, 0x55, 0x54, 0x4f, 0x5f, 0x55, 0x4e, 0x50, 0x41,
    0x55, 0x53, 0x45, 0x44, 0x10, 0x25, 0x12, 0x1a, 0x0a, 0x16, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d,
    0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f, 0x59, 0x4f, 0x55, 0x50, 0x41, 0x55, 0x53, 0x45, 0x44,
    0x10, 0x26, 0x12, 0x20, 0x0a, 0x1c, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41,
    0x47, 0x45, 0x5f, 0x43, 0x41, 0x4e, 0x54, 0x55, 0x4e, 0x50, 0x41, 0x55, 0x53, 0x45, 0x54, 0x45,
    0x41, 0x4d, 0x10, 0x27, 0x12, 0x28, 0x0a, 0x24, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53,
    0x53, 0x41, 0x47, 0x45, 0x5f, 0x53, 0x41, 0x46, 0x45, 0x5f, 0x54, 0x4f, 0x5f, 0x4c, 0x45, 0x41,
    0x56, 0x45, 0x5f, 0x41, 0x42, 0x41, 0x4e, 0x44, 0x4f, 0x4e, 0x45, 0x52, 0x10, 0x28, 0x12, 0x22,
    0x0a, 0x1e, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f, 0x56,
    0x4f, 0x49, 0x43, 0x45, 0x5f, 0x54, 0x45, 0x58, 0x54, 0x5f, 0x42, 0x41, 0x4e, 0x4e, 0x45, 0x44,
    0x10, 0x29, 0x12, 0x2e, 0x0a, 0x2a, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41,
    0x47, 0x45, 0x5f, 0x53, 0x50, 0x45, 0x43, 0x54, 0x41, 0x54, 0x4f, 0x52, 0x53, 0x5f, 0x57, 0x41,
    0x54, 0x43, 0x48, 0x49, 0x4e, 0x47, 0x5f, 0x54, 0x48, 0x49, 0x53, 0x5f, 0x47, 0x41, 0x4d, 0x45,
    0x10, 0x2a, 0x12, 0x20, 0x0a, 0x1c, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41,
    0x47, 0x45, 0x5f, 0x52, 0x45, 0x50, 0x4f, 0x52, 0x54, 0x5f, 0x52, 0x45, 0x4d, 0x49, 0x4e, 0x44,
    0x45, 0x52, 0x10, 0x2b, 0x12, 0x1a, 0x0a, 0x16, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53,
    0x53, 0x41, 0x47, 0x45, 0x5f, 0x45, 0x43, 0x4f, 0x4e, 0x5f, 0x49, 0x54, 0x45, 0x4d, 0x10, 0x2c,
    0x12, 0x16, 0x0a, 0x12, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45,
    0x5f, 0x54, 0x41, 0x55, 0x4e, 0x54, 0x10, 0x2d, 0x12, 0x17, 0x0a, 0x13, 0x43, 0x48, 0x41, 0x54,
    0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x5f, 0x52, 0x41, 0x4e, 0x44, 0x4f, 0x4d, 0x10,
    0x2e, 0x12, 0x18, 0x0a, 0x14, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47,
    0x45, 0x5f, 0x52, 0x44, 0x5f, 0x54, 0x55, 0x52, 0x4e, 0x10, 0x2f, 0x2a, 0xbe, 0x01, 0x0a, 0x13,
    0x45, 0x44, 0x6f, 0x74, 0x61, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x73, 0x12, 0x14, 0x0a, 0x10, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x55, 0x4e, 0x49, 0x54,
    0x5f, 0x53, 0x50, 0x45, 0x45, 0x43, 0x48, 0x10, 0x00, 0x12, 0x19, 0x0a, 0x15, 0x44, 0x4f, 0x54,
    0x41, 0x5f, 0x55, 0x4e, 0x49, 0x54, 0x5f, 0x53, 0x50, 0x45, 0x45, 0x43, 0x48, 0x5f, 0x4d, 0x55,
    0x54, 0x45, 0x10, 0x01, 0x12, 0x19, 0x0a, 0x15, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x55, 0x4e, 0x49,
    0x54, 0x5f, 0x41, 0x44, 0x44, 0x5f, 0x47, 0x45, 0x53, 0x54, 0x55, 0x52, 0x45, 0x10, 0x02, 0x12,
    0x1c, 0x0a, 0x18, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x55, 0x4e, 0x49, 0x54, 0x5f, 0x52, 0x45, 0x4d,
    0x4f, 0x56, 0x45, 0x5f, 0x47, 0x45, 0x53, 0x54, 0x55, 0x52, 0x45, 0x10, 0x03, 0x12, 0x21, 0x0a,
    0x1d, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x55, 0x4e, 0x49, 0x54, 0x5f, 0x52, 0x45, 0x4d, 0x4f, 0x56,
    0x45, 0x5f, 0x41, 0x4c, 0x4c, 0x5f, 0x47, 0x45, 0x53, 0x54, 0x55, 0x52, 0x45, 0x53, 0x10, 0x04,
    0x12, 0x1a, 0x0a, 0x16, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x55, 0x4e, 0x49, 0x54, 0x5f, 0x46, 0x41,
    0x44, 0x45, 0x5f, 0x47, 0x45, 0x53, 0x54, 0x55, 0x52, 0x45, 0x10, 0x06, 0x2a, 0x85, 0x04, 0x0a,
    0x15, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x50, 0x41, 0x52, 0x54, 0x49, 0x43, 0x4c, 0x45, 0x5f, 0x4d,
    0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x12, 0x26, 0x0a, 0x22, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x50,
    0x41, 0x52, 0x54, 0x49, 0x43, 0x4c, 0x45, 0x5f, 0x4d, 0x41, 0x4e, 0x41, 0x47, 0x45, 0x52, 0x5f,
    0x45, 0x56, 0x45, 0x4e, 0x54, 0x5f, 0x43, 0x52, 0x45, 0x41, 0x54, 0x45, 0x10, 0x00, 0x12, 0x26,
    0x0a, 0x22, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x50, 0x41, 0x52, 0x54, 0x49, 0x43, 0x4c, 0x45, 0x5f,
    0x4d, 0x41, 0x4e, 0x41, 0x47, 0x45, 0x52, 0x5f, 0x45, 0x56, 0x45, 0x4e, 0x54, 0x5f, 0x55, 0x50,
    0x44, 0x41, 0x54, 0x45, 0x10, 0x01, 0x12, 0x2e, 0x0a, 0x2a, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x50,
    0x41, 0x52, 0x54, 0x49, 0x43, 0x4c, 0x45, 0x5f, 0x4d, 0x41, 0x4e, 0x41, 0x47, 0x45, 0x52, 0x5f,
    0x45, 0x56, 0x45, 0x4e, 0x54, 0x5f, 0x55, 0x50, 0x44, 0x41, 0x54, 0x45, 0x5f, 0x46, 0x4f, 0x52,
    0x57, 0x41, 0x52, 0x44, 0x10, 0x02, 0x12, 0x32, 0x0a, 0x2e, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x50,
    0x41, 0x52, 0x54, 0x49, 0x43, 0x4c, 0x45, 0x5f, 0x4d, 0x41, 0x4e, 0x41, 0x47, 0x45, 0x52, 0x5f,
    0x45, 0x56, 0x45, 0x4e, 0x54, 0x5f, 0x55, 0x50, 0x44, 0x41, 0x54, 0x45, 0x5f, 0x4f, 0x52, 0x49,
    0x45, 0x4e, 0x54, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x10, 0x03, 0x12, 0x2f, 0x0a, 0x2b, 0x44, 0x4f,
    0x54, 0x41, 0x5f, 0x50, 0x41, 0x52, 0x54, 0x49, 0x43, 0x4c, 0x45, 0x5f, 0x4d, 0x41, 0x4e, 0x41,
    0x47, 0x45, 0x52, 0x5f, 0x45, 0x56, 0x45, 0x4e, 0x54, 0x5f, 0x55, 0x50, 0x44, 0x41, 0x54, 0x45,
    0x5f, 0x46, 0x41, 0x4c, 0x4c, 0x42, 0x41, 0x43, 0x4b, 0x10, 0x04, 0x12, 0x2a, 0x0a, 0x26, 0x44,
    0x4f, 0x54, 0x41, 0x5f, 0x50, 0x41, 0x52, 0x54, 0x49, 0x43, 0x4c, 0x45, 0x5f, 0x4d, 0x41, 0x4e,
    0x41, 0x47, 0x45, 0x52, 0x5f, 0x45, 0x56, 0x45, 0x4e, 0x54, 0x5f, 0x55, 0x50, 0x44, 0x41, 0x54,
    0x45, 0x5f, 0x45, 0x4e, 0x54, 0x10, 0x05, 0x12, 0x2d, 0x0a, 0x29, 0x44, 0x4f, 0x54, 0x41, 0x5f,
    0x50, 0x41, 0x52, 0x54, 0x49, 0x43, 0x4c, 0x45, 0x5f, 0x4d, 0x41, 0x4e, 0x41, 0x47, 0x45, 0x52,
    0x5f, 0x45, 0x56, 0x45, 0x4e, 0x54, 0x5f, 0x55, 0x50, 0x44, 0x41, 0x54, 0x45, 0x5f, 0x4f, 0x46,
    0x46, 0x53, 0x45, 0x54, 0x10, 0x06, 0x12, 0x27, 0x0a, 0x23, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x50,
    0x41, 0x52, 0x54, 0x49, 0x43, 0x4c, 0x45, 0x5f, 0x4d, 0x41, 0x4e, 0x41, 0x47, 0x45, 0x52, 0x5f,
    0x45, 0x56, 0x45, 0x4e, 0x54, 0x5f, 0x44, 0x45, 0x53, 0x54, 0x52, 0x4f, 0x59, 0x10, 0x07, 0x12,
    0x31, 0x0a, 0x2d, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x50, 0x41, 0x52, 0x54, 0x49, 0x43, 0x4c, 0x45,
    0x5f, 0x4d, 0x41, 0x4e, 0x41, 0x47, 0x45, 0x52, 0x5f, 0x45, 0x56, 0x45, 0x4e, 0x54, 0x5f, 0x44,
    0x45, 0x53, 0x54, 0x52, 0x4f, 0x59, 0x5f, 0x49, 0x4e, 0x56, 0x4f, 0x4c, 0x56, 0x49, 0x4e, 0x47,
    0x10, 0x08, 0x12, 0x27, 0x0a, 0x23, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x50, 0x41, 0x52, 0x54, 0x49,
    0x43, 0x4c, 0x45, 0x5f, 0x4d, 0x41, 0x4e, 0x41, 0x47, 0x45, 0x52, 0x5f, 0x45, 0x56, 0x45, 0x4e,
    0x54, 0x5f, 0x52, 0x45, 0x4c, 0x45, 0x41, 0x53, 0x45, 0x10, 0x09, 0x12, 0x27, 0x0a, 0x23, 0x44,
    0x4f, 0x54, 0x41, 0x5f, 0x50, 0x41, 0x52, 0x54, 0x49, 0x43, 0x4c, 0x45, 0x5f, 0x4d, 0x41, 0x4e,
    0x41, 0x47, 0x45, 0x52, 0x5f, 0x45, 0x56, 0x45, 0x4e, 0x54, 0x5f, 0x4c, 0x41, 0x54, 0x45, 0x4e,
    0x43, 0x59, 0x10, 0x0a, 0x2a, 0xb2, 0x02, 0x0a, 0x13, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x4f, 0x56,
    0x45, 0x52, 0x48, 0x45, 0x41, 0x44, 0x5f, 0x41, 0x4c, 0x45, 0x52, 0x54, 0x12, 0x17, 0x0a, 0x13,
    0x4f, 0x56, 0x45, 0x52, 0x48, 0x45, 0x41, 0x44, 0x5f, 0x41, 0x4c, 0x45, 0x52, 0x54, 0x5f, 0x47,
    0x4f, 0x4c, 0x44, 0x10, 0x00, 0x12, 0x17, 0x0a, 0x13, 0x4f, 0x56, 0x45, 0x52, 0x48, 0x45, 0x41,
    0x44, 0x5f, 0x41, 0x4c, 0x45, 0x52, 0x54, 0x5f, 0x44, 0x45, 0x4e, 0x59, 0x10, 0x01, 0x12, 0x1b,
    0x0a, 0x17, 0x4f, 0x56, 0x45, 0x52, 0x48, 0x45, 0x41, 0x44, 0x5f, 0x41, 0x4c, 0x45, 0x52, 0x54,
    0x5f, 0x43, 0x52, 0x49, 0x54, 0x49, 0x43, 0x41, 0x4c, 0x10, 0x02, 0x12, 0x15, 0x0a, 0x11, 0x4f,
    0x56, 0x45, 0x52, 0x48, 0x45, 0x41, 0x44, 0x5f, 0x41, 0x4c, 0x45, 0x52, 0x54, 0x5f, 0x58, 0x50,
    0x10, 0x03, 0x12, 0x25, 0x0a, 0x21, 0x4f, 0x56, 0x45, 0x52, 0x48, 0x45, 0x41, 0x44, 0x5f, 0x41,
    0x4c, 0x45, 0x52, 0x54, 0x5f, 0x42, 0x4f, 0x4e, 0x55, 0x53, 0x5f, 0x53, 0x50, 0x45, 0x4c, 0x4c,
    0x5f, 0x44, 0x41, 0x4d, 0x41, 0x47, 0x45, 0x10, 0x04, 0x12, 0x17, 0x0a, 0x13, 0x4f, 0x56, 0x45,
    0x52, 0x48, 0x45, 0x41, 0x44, 0x5f, 0x41, 0x4c, 0x45, 0x52, 0x54, 0x5f, 0x4d, 0x49, 0x53, 0x53,
    0x10, 0x05, 0x12, 0x19, 0x0a, 0x15, 0x4f, 0x56, 0x45, 0x52, 0x48, 0x45, 0x41, 0x44, 0x5f, 0x41,
    0x4c, 0x45, 0x52, 0x54, 0x5f, 0x44, 0x41, 0x4d, 0x41, 0x47, 0x45, 0x10, 0x06, 0x12, 0x18, 0x0a,
    0x14, 0x4f, 0x56, 0x45, 0x52, 0x48, 0x45, 0x41, 0x44, 0x5f, 0x41, 0x4c, 0x45, 0x52, 0x54, 0x5f,
    0x45, 0x56, 0x41, 0x44, 0x45, 0x10, 0x07, 0x12, 0x18, 0x0a, 0x14, 0x4f, 0x56, 0x45, 0x52, 0x48,
    0x45, 0x41, 0x44, 0x5f, 0x41, 0x4c, 0x45, 0x52, 0x54, 0x5f, 0x42, 0x4c, 0x4f, 0x43, 0x4b, 0x10,
    0x08, 0x12, 0x26, 0x0a, 0x22, 0x4f, 0x56, 0x45, 0x52, 0x48, 0x45, 0x41, 0x44, 0x5f, 0x41, 0x4c,
    0x45, 0x52, 0x54, 0x5f, 0x42, 0x4f, 0x4e, 0x55, 0x53, 0x5f, 0x50, 0x4f, 0x49, 0x53, 0x4f, 0x4e,
    0x5f, 0x44, 0x41, 0x4d, 0x41, 0x47, 0x45, 0x10, 0x09, 0x42, 0x05, 0x48, 0x01, 0x80, 0x01, 0x00,
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
