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
pub struct CDemoFileHeader {
    // message fields
    demo_file_stamp: ::protobuf::SingularField<::std::string::String>,
    network_protocol: ::std::option::Option<i32>,
    server_name: ::protobuf::SingularField<::std::string::String>,
    client_name: ::protobuf::SingularField<::std::string::String>,
    map_name: ::protobuf::SingularField<::std::string::String>,
    game_directory: ::protobuf::SingularField<::std::string::String>,
    fullpackets_version: ::std::option::Option<i32>,
    allow_clientside_entities: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoFileHeader {}

impl CDemoFileHeader {
    pub fn new() -> CDemoFileHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoFileHeader {
        static mut instance: ::protobuf::lazy::Lazy<CDemoFileHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoFileHeader,
        };
        unsafe {
            instance.get(|| {
                CDemoFileHeader {
                    demo_file_stamp: ::protobuf::SingularField::none(),
                    network_protocol: ::std::option::Option::None,
                    server_name: ::protobuf::SingularField::none(),
                    client_name: ::protobuf::SingularField::none(),
                    map_name: ::protobuf::SingularField::none(),
                    game_directory: ::protobuf::SingularField::none(),
                    fullpackets_version: ::std::option::Option::None,
                    allow_clientside_entities: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string demo_file_stamp = 1;

    pub fn clear_demo_file_stamp(&mut self) {
        self.demo_file_stamp.clear();
    }

    pub fn has_demo_file_stamp(&self) -> bool {
        self.demo_file_stamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_demo_file_stamp(&mut self, v: ::std::string::String) {
        self.demo_file_stamp = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_demo_file_stamp(&mut self) -> &mut ::std::string::String {
        if self.demo_file_stamp.is_none() {
            self.demo_file_stamp.set_default();
        };
        self.demo_file_stamp.as_mut().unwrap()
    }

    // Take field
    pub fn take_demo_file_stamp(&mut self) -> ::std::string::String {
        self.demo_file_stamp.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_demo_file_stamp(&self) -> &str {
        match self.demo_file_stamp.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional int32 network_protocol = 2;

    pub fn clear_network_protocol(&mut self) {
        self.network_protocol = ::std::option::Option::None;
    }

    pub fn has_network_protocol(&self) -> bool {
        self.network_protocol.is_some()
    }

    // Param is passed by value, moved
    pub fn set_network_protocol(&mut self, v: i32) {
        self.network_protocol = ::std::option::Option::Some(v);
    }

    pub fn get_network_protocol(&self) -> i32 {
        self.network_protocol.unwrap_or(0)
    }

    // optional string server_name = 3;

    pub fn clear_server_name(&mut self) {
        self.server_name.clear();
    }

    pub fn has_server_name(&self) -> bool {
        self.server_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_name(&mut self, v: ::std::string::String) {
        self.server_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_server_name(&mut self) -> &mut ::std::string::String {
        if self.server_name.is_none() {
            self.server_name.set_default();
        };
        self.server_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_server_name(&mut self) -> ::std::string::String {
        self.server_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_server_name(&self) -> &str {
        match self.server_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string client_name = 4;

    pub fn clear_client_name(&mut self) {
        self.client_name.clear();
    }

    pub fn has_client_name(&self) -> bool {
        self.client_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_name(&mut self, v: ::std::string::String) {
        self.client_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_client_name(&mut self) -> &mut ::std::string::String {
        if self.client_name.is_none() {
            self.client_name.set_default();
        };
        self.client_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_client_name(&mut self) -> ::std::string::String {
        self.client_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_client_name(&self) -> &str {
        match self.client_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string map_name = 5;

    pub fn clear_map_name(&mut self) {
        self.map_name.clear();
    }

    pub fn has_map_name(&self) -> bool {
        self.map_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_map_name(&mut self, v: ::std::string::String) {
        self.map_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_map_name(&mut self) -> &mut ::std::string::String {
        if self.map_name.is_none() {
            self.map_name.set_default();
        };
        self.map_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_map_name(&mut self) -> ::std::string::String {
        self.map_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_map_name(&self) -> &str {
        match self.map_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string game_directory = 6;

    pub fn clear_game_directory(&mut self) {
        self.game_directory.clear();
    }

    pub fn has_game_directory(&self) -> bool {
        self.game_directory.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_directory(&mut self, v: ::std::string::String) {
        self.game_directory = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_game_directory(&mut self) -> &mut ::std::string::String {
        if self.game_directory.is_none() {
            self.game_directory.set_default();
        };
        self.game_directory.as_mut().unwrap()
    }

    // Take field
    pub fn take_game_directory(&mut self) -> ::std::string::String {
        self.game_directory.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_game_directory(&self) -> &str {
        match self.game_directory.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional int32 fullpackets_version = 7;

    pub fn clear_fullpackets_version(&mut self) {
        self.fullpackets_version = ::std::option::Option::None;
    }

    pub fn has_fullpackets_version(&self) -> bool {
        self.fullpackets_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fullpackets_version(&mut self, v: i32) {
        self.fullpackets_version = ::std::option::Option::Some(v);
    }

    pub fn get_fullpackets_version(&self) -> i32 {
        self.fullpackets_version.unwrap_or(0)
    }

    // optional bool allow_clientside_entities = 8;

    pub fn clear_allow_clientside_entities(&mut self) {
        self.allow_clientside_entities = ::std::option::Option::None;
    }

    pub fn has_allow_clientside_entities(&self) -> bool {
        self.allow_clientside_entities.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allow_clientside_entities(&mut self, v: bool) {
        self.allow_clientside_entities = ::std::option::Option::Some(v);
    }

    pub fn get_allow_clientside_entities(&self) -> bool {
        self.allow_clientside_entities.unwrap_or(false)
    }
}

impl ::protobuf::Message for CDemoFileHeader {
    fn is_initialized(&self) -> bool {
        if self.demo_file_stamp.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.demo_file_stamp));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.network_protocol = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.server_name));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.client_name));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.map_name));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.game_directory));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.fullpackets_version = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.allow_clientside_entities = ::std::option::Option::Some(tmp);
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
        for value in &self.demo_file_stamp {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.network_protocol {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.server_name {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.client_name {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in &self.map_name {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        for value in &self.game_directory {
            my_size += ::protobuf::rt::string_size(6, &value);
        };
        for value in &self.fullpackets_version {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.allow_clientside_entities.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.demo_file_stamp.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.network_protocol {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.server_name.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.client_name.as_ref() {
            try!(os.write_string(4, &v));
        };
        if let Some(v) = self.map_name.as_ref() {
            try!(os.write_string(5, &v));
        };
        if let Some(v) = self.game_directory.as_ref() {
            try!(os.write_string(6, &v));
        };
        if let Some(v) = self.fullpackets_version {
            try!(os.write_int32(7, v));
        };
        if let Some(v) = self.allow_clientside_entities {
            try!(os.write_bool(8, v));
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
        ::std::any::TypeId::of::<CDemoFileHeader>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoFileHeader {
    fn new() -> CDemoFileHeader {
        CDemoFileHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoFileHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "demo_file_stamp",
                    CDemoFileHeader::has_demo_file_stamp,
                    CDemoFileHeader::get_demo_file_stamp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "network_protocol",
                    CDemoFileHeader::has_network_protocol,
                    CDemoFileHeader::get_network_protocol,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "server_name",
                    CDemoFileHeader::has_server_name,
                    CDemoFileHeader::get_server_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "client_name",
                    CDemoFileHeader::has_client_name,
                    CDemoFileHeader::get_client_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "map_name",
                    CDemoFileHeader::has_map_name,
                    CDemoFileHeader::get_map_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "game_directory",
                    CDemoFileHeader::has_game_directory,
                    CDemoFileHeader::get_game_directory,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "fullpackets_version",
                    CDemoFileHeader::has_fullpackets_version,
                    CDemoFileHeader::get_fullpackets_version,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "allow_clientside_entities",
                    CDemoFileHeader::has_allow_clientside_entities,
                    CDemoFileHeader::get_allow_clientside_entities,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoFileHeader>(
                    "CDemoFileHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoFileHeader {
    fn clear(&mut self) {
        self.clear_demo_file_stamp();
        self.clear_network_protocol();
        self.clear_server_name();
        self.clear_client_name();
        self.clear_map_name();
        self.clear_game_directory();
        self.clear_fullpackets_version();
        self.clear_allow_clientside_entities();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDemoFileHeader {
    fn eq(&self, other: &CDemoFileHeader) -> bool {
        self.demo_file_stamp == other.demo_file_stamp &&
        self.network_protocol == other.network_protocol &&
        self.server_name == other.server_name &&
        self.client_name == other.client_name &&
        self.map_name == other.map_name &&
        self.game_directory == other.game_directory &&
        self.fullpackets_version == other.fullpackets_version &&
        self.allow_clientside_entities == other.allow_clientside_entities &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDemoFileHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CGameInfo {
    // message fields
    dota: ::protobuf::SingularPtrField<CGameInfo_CDotaGameInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGameInfo {}

impl CGameInfo {
    pub fn new() -> CGameInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGameInfo {
        static mut instance: ::protobuf::lazy::Lazy<CGameInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGameInfo,
        };
        unsafe {
            instance.get(|| {
                CGameInfo {
                    dota: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .CGameInfo.CDotaGameInfo dota = 4;

    pub fn clear_dota(&mut self) {
        self.dota.clear();
    }

    pub fn has_dota(&self) -> bool {
        self.dota.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dota(&mut self, v: CGameInfo_CDotaGameInfo) {
        self.dota = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dota(&mut self) -> &mut CGameInfo_CDotaGameInfo {
        if self.dota.is_none() {
            self.dota.set_default();
        };
        self.dota.as_mut().unwrap()
    }

    // Take field
    pub fn take_dota(&mut self) -> CGameInfo_CDotaGameInfo {
        self.dota.take().unwrap_or_else(|| CGameInfo_CDotaGameInfo::new())
    }

    pub fn get_dota(&self) -> &CGameInfo_CDotaGameInfo {
        self.dota.as_ref().unwrap_or_else(|| CGameInfo_CDotaGameInfo::default_instance())
    }
}

impl ::protobuf::Message for CGameInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.dota));
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
        for value in &self.dota {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dota.as_ref() {
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
        ::std::any::TypeId::of::<CGameInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGameInfo {
    fn new() -> CGameInfo {
        CGameInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGameInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "dota",
                    CGameInfo::has_dota,
                    CGameInfo::get_dota,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGameInfo>(
                    "CGameInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGameInfo {
    fn clear(&mut self) {
        self.clear_dota();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CGameInfo {
    fn eq(&self, other: &CGameInfo) -> bool {
        self.dota == other.dota &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CGameInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CGameInfo_CDotaGameInfo {
    // message fields
    match_id: ::std::option::Option<u32>,
    game_mode: ::std::option::Option<i32>,
    game_winner: ::std::option::Option<i32>,
    player_info: ::protobuf::RepeatedField<CGameInfo_CDotaGameInfo_CPlayerInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGameInfo_CDotaGameInfo {}

impl CGameInfo_CDotaGameInfo {
    pub fn new() -> CGameInfo_CDotaGameInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGameInfo_CDotaGameInfo {
        static mut instance: ::protobuf::lazy::Lazy<CGameInfo_CDotaGameInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGameInfo_CDotaGameInfo,
        };
        unsafe {
            instance.get(|| {
                CGameInfo_CDotaGameInfo {
                    match_id: ::std::option::Option::None,
                    game_mode: ::std::option::Option::None,
                    game_winner: ::std::option::Option::None,
                    player_info: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 match_id = 1;

    pub fn clear_match_id(&mut self) {
        self.match_id = ::std::option::Option::None;
    }

    pub fn has_match_id(&self) -> bool {
        self.match_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_match_id(&mut self, v: u32) {
        self.match_id = ::std::option::Option::Some(v);
    }

    pub fn get_match_id(&self) -> u32 {
        self.match_id.unwrap_or(0)
    }

    // optional int32 game_mode = 2;

    pub fn clear_game_mode(&mut self) {
        self.game_mode = ::std::option::Option::None;
    }

    pub fn has_game_mode(&self) -> bool {
        self.game_mode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_mode(&mut self, v: i32) {
        self.game_mode = ::std::option::Option::Some(v);
    }

    pub fn get_game_mode(&self) -> i32 {
        self.game_mode.unwrap_or(0)
    }

    // optional int32 game_winner = 3;

    pub fn clear_game_winner(&mut self) {
        self.game_winner = ::std::option::Option::None;
    }

    pub fn has_game_winner(&self) -> bool {
        self.game_winner.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_winner(&mut self, v: i32) {
        self.game_winner = ::std::option::Option::Some(v);
    }

    pub fn get_game_winner(&self) -> i32 {
        self.game_winner.unwrap_or(0)
    }

    // repeated .CGameInfo.CDotaGameInfo.CPlayerInfo player_info = 4;

    pub fn clear_player_info(&mut self) {
        self.player_info.clear();
    }

    // Param is passed by value, moved
    pub fn set_player_info(&mut self, v: ::protobuf::RepeatedField<CGameInfo_CDotaGameInfo_CPlayerInfo>) {
        self.player_info = v;
    }

    // Mutable pointer to the field.
    pub fn mut_player_info(&mut self) -> &mut ::protobuf::RepeatedField<CGameInfo_CDotaGameInfo_CPlayerInfo> {
        &mut self.player_info
    }

    // Take field
    pub fn take_player_info(&mut self) -> ::protobuf::RepeatedField<CGameInfo_CDotaGameInfo_CPlayerInfo> {
        ::std::mem::replace(&mut self.player_info, ::protobuf::RepeatedField::new())
    }

    pub fn get_player_info(&self) -> &[CGameInfo_CDotaGameInfo_CPlayerInfo] {
        &self.player_info
    }
}

impl ::protobuf::Message for CGameInfo_CDotaGameInfo {
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
                    self.match_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.game_mode = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.game_winner = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.player_info));
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
        for value in &self.match_id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.game_mode {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.game_winner {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.player_info {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.match_id {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.game_mode {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.game_winner {
            try!(os.write_int32(3, v));
        };
        for v in &self.player_info {
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
        ::std::any::TypeId::of::<CGameInfo_CDotaGameInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGameInfo_CDotaGameInfo {
    fn new() -> CGameInfo_CDotaGameInfo {
        CGameInfo_CDotaGameInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGameInfo_CDotaGameInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "match_id",
                    CGameInfo_CDotaGameInfo::has_match_id,
                    CGameInfo_CDotaGameInfo::get_match_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "game_mode",
                    CGameInfo_CDotaGameInfo::has_game_mode,
                    CGameInfo_CDotaGameInfo::get_game_mode,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "game_winner",
                    CGameInfo_CDotaGameInfo::has_game_winner,
                    CGameInfo_CDotaGameInfo::get_game_winner,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "player_info",
                    CGameInfo_CDotaGameInfo::get_player_info,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGameInfo_CDotaGameInfo>(
                    "CGameInfo_CDotaGameInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGameInfo_CDotaGameInfo {
    fn clear(&mut self) {
        self.clear_match_id();
        self.clear_game_mode();
        self.clear_game_winner();
        self.clear_player_info();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CGameInfo_CDotaGameInfo {
    fn eq(&self, other: &CGameInfo_CDotaGameInfo) -> bool {
        self.match_id == other.match_id &&
        self.game_mode == other.game_mode &&
        self.game_winner == other.game_winner &&
        self.player_info == other.player_info &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CGameInfo_CDotaGameInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CGameInfo_CDotaGameInfo_CPlayerInfo {
    // message fields
    hero_name: ::protobuf::SingularField<::std::string::String>,
    player_name: ::protobuf::SingularField<::std::string::String>,
    is_fake_client: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGameInfo_CDotaGameInfo_CPlayerInfo {}

impl CGameInfo_CDotaGameInfo_CPlayerInfo {
    pub fn new() -> CGameInfo_CDotaGameInfo_CPlayerInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGameInfo_CDotaGameInfo_CPlayerInfo {
        static mut instance: ::protobuf::lazy::Lazy<CGameInfo_CDotaGameInfo_CPlayerInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGameInfo_CDotaGameInfo_CPlayerInfo,
        };
        unsafe {
            instance.get(|| {
                CGameInfo_CDotaGameInfo_CPlayerInfo {
                    hero_name: ::protobuf::SingularField::none(),
                    player_name: ::protobuf::SingularField::none(),
                    is_fake_client: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string hero_name = 1;

    pub fn clear_hero_name(&mut self) {
        self.hero_name.clear();
    }

    pub fn has_hero_name(&self) -> bool {
        self.hero_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hero_name(&mut self, v: ::std::string::String) {
        self.hero_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hero_name(&mut self) -> &mut ::std::string::String {
        if self.hero_name.is_none() {
            self.hero_name.set_default();
        };
        self.hero_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_hero_name(&mut self) -> ::std::string::String {
        self.hero_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_hero_name(&self) -> &str {
        match self.hero_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string player_name = 2;

    pub fn clear_player_name(&mut self) {
        self.player_name.clear();
    }

    pub fn has_player_name(&self) -> bool {
        self.player_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_name(&mut self, v: ::std::string::String) {
        self.player_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_player_name(&mut self) -> &mut ::std::string::String {
        if self.player_name.is_none() {
            self.player_name.set_default();
        };
        self.player_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_player_name(&mut self) -> ::std::string::String {
        self.player_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_player_name(&self) -> &str {
        match self.player_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bool is_fake_client = 3;

    pub fn clear_is_fake_client(&mut self) {
        self.is_fake_client = ::std::option::Option::None;
    }

    pub fn has_is_fake_client(&self) -> bool {
        self.is_fake_client.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_fake_client(&mut self, v: bool) {
        self.is_fake_client = ::std::option::Option::Some(v);
    }

    pub fn get_is_fake_client(&self) -> bool {
        self.is_fake_client.unwrap_or(false)
    }
}

impl ::protobuf::Message for CGameInfo_CDotaGameInfo_CPlayerInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.hero_name));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.player_name));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.is_fake_client = ::std::option::Option::Some(tmp);
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
        for value in &self.hero_name {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.player_name {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if self.is_fake_client.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hero_name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.player_name.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.is_fake_client {
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
        ::std::any::TypeId::of::<CGameInfo_CDotaGameInfo_CPlayerInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGameInfo_CDotaGameInfo_CPlayerInfo {
    fn new() -> CGameInfo_CDotaGameInfo_CPlayerInfo {
        CGameInfo_CDotaGameInfo_CPlayerInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGameInfo_CDotaGameInfo_CPlayerInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "hero_name",
                    CGameInfo_CDotaGameInfo_CPlayerInfo::has_hero_name,
                    CGameInfo_CDotaGameInfo_CPlayerInfo::get_hero_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "player_name",
                    CGameInfo_CDotaGameInfo_CPlayerInfo::has_player_name,
                    CGameInfo_CDotaGameInfo_CPlayerInfo::get_player_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "is_fake_client",
                    CGameInfo_CDotaGameInfo_CPlayerInfo::has_is_fake_client,
                    CGameInfo_CDotaGameInfo_CPlayerInfo::get_is_fake_client,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGameInfo_CDotaGameInfo_CPlayerInfo>(
                    "CGameInfo_CDotaGameInfo_CPlayerInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGameInfo_CDotaGameInfo_CPlayerInfo {
    fn clear(&mut self) {
        self.clear_hero_name();
        self.clear_player_name();
        self.clear_is_fake_client();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CGameInfo_CDotaGameInfo_CPlayerInfo {
    fn eq(&self, other: &CGameInfo_CDotaGameInfo_CPlayerInfo) -> bool {
        self.hero_name == other.hero_name &&
        self.player_name == other.player_name &&
        self.is_fake_client == other.is_fake_client &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CGameInfo_CDotaGameInfo_CPlayerInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDemoFileInfo {
    // message fields
    playback_time: ::std::option::Option<f32>,
    playback_ticks: ::std::option::Option<i32>,
    playback_frames: ::std::option::Option<i32>,
    game_info: ::protobuf::SingularPtrField<CGameInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoFileInfo {}

impl CDemoFileInfo {
    pub fn new() -> CDemoFileInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoFileInfo {
        static mut instance: ::protobuf::lazy::Lazy<CDemoFileInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoFileInfo,
        };
        unsafe {
            instance.get(|| {
                CDemoFileInfo {
                    playback_time: ::std::option::Option::None,
                    playback_ticks: ::std::option::Option::None,
                    playback_frames: ::std::option::Option::None,
                    game_info: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional float playback_time = 1;

    pub fn clear_playback_time(&mut self) {
        self.playback_time = ::std::option::Option::None;
    }

    pub fn has_playback_time(&self) -> bool {
        self.playback_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_playback_time(&mut self, v: f32) {
        self.playback_time = ::std::option::Option::Some(v);
    }

    pub fn get_playback_time(&self) -> f32 {
        self.playback_time.unwrap_or(0.)
    }

    // optional int32 playback_ticks = 2;

    pub fn clear_playback_ticks(&mut self) {
        self.playback_ticks = ::std::option::Option::None;
    }

    pub fn has_playback_ticks(&self) -> bool {
        self.playback_ticks.is_some()
    }

    // Param is passed by value, moved
    pub fn set_playback_ticks(&mut self, v: i32) {
        self.playback_ticks = ::std::option::Option::Some(v);
    }

    pub fn get_playback_ticks(&self) -> i32 {
        self.playback_ticks.unwrap_or(0)
    }

    // optional int32 playback_frames = 3;

    pub fn clear_playback_frames(&mut self) {
        self.playback_frames = ::std::option::Option::None;
    }

    pub fn has_playback_frames(&self) -> bool {
        self.playback_frames.is_some()
    }

    // Param is passed by value, moved
    pub fn set_playback_frames(&mut self, v: i32) {
        self.playback_frames = ::std::option::Option::Some(v);
    }

    pub fn get_playback_frames(&self) -> i32 {
        self.playback_frames.unwrap_or(0)
    }

    // optional .CGameInfo game_info = 4;

    pub fn clear_game_info(&mut self) {
        self.game_info.clear();
    }

    pub fn has_game_info(&self) -> bool {
        self.game_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_info(&mut self, v: CGameInfo) {
        self.game_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_game_info(&mut self) -> &mut CGameInfo {
        if self.game_info.is_none() {
            self.game_info.set_default();
        };
        self.game_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_game_info(&mut self) -> CGameInfo {
        self.game_info.take().unwrap_or_else(|| CGameInfo::new())
    }

    pub fn get_game_info(&self) -> &CGameInfo {
        self.game_info.as_ref().unwrap_or_else(|| CGameInfo::default_instance())
    }
}

impl ::protobuf::Message for CDemoFileInfo {
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
                    self.playback_time = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.playback_ticks = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.playback_frames = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.game_info));
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
        if self.playback_time.is_some() {
            my_size += 5;
        };
        for value in &self.playback_ticks {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.playback_frames {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.game_info {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.playback_time {
            try!(os.write_float(1, v));
        };
        if let Some(v) = self.playback_ticks {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.playback_frames {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.game_info.as_ref() {
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
        ::std::any::TypeId::of::<CDemoFileInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoFileInfo {
    fn new() -> CDemoFileInfo {
        CDemoFileInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoFileInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "playback_time",
                    CDemoFileInfo::has_playback_time,
                    CDemoFileInfo::get_playback_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "playback_ticks",
                    CDemoFileInfo::has_playback_ticks,
                    CDemoFileInfo::get_playback_ticks,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "playback_frames",
                    CDemoFileInfo::has_playback_frames,
                    CDemoFileInfo::get_playback_frames,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "game_info",
                    CDemoFileInfo::has_game_info,
                    CDemoFileInfo::get_game_info,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoFileInfo>(
                    "CDemoFileInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoFileInfo {
    fn clear(&mut self) {
        self.clear_playback_time();
        self.clear_playback_ticks();
        self.clear_playback_frames();
        self.clear_game_info();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDemoFileInfo {
    fn eq(&self, other: &CDemoFileInfo) -> bool {
        self.playback_time == other.playback_time &&
        self.playback_ticks == other.playback_ticks &&
        self.playback_frames == other.playback_frames &&
        self.game_info == other.game_info &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDemoFileInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDemoPacket {
    // message fields
    sequence_in: ::std::option::Option<i32>,
    sequence_out_ack: ::std::option::Option<i32>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoPacket {}

impl CDemoPacket {
    pub fn new() -> CDemoPacket {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoPacket {
        static mut instance: ::protobuf::lazy::Lazy<CDemoPacket> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoPacket,
        };
        unsafe {
            instance.get(|| {
                CDemoPacket {
                    sequence_in: ::std::option::Option::None,
                    sequence_out_ack: ::std::option::Option::None,
                    data: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 sequence_in = 1;

    pub fn clear_sequence_in(&mut self) {
        self.sequence_in = ::std::option::Option::None;
    }

    pub fn has_sequence_in(&self) -> bool {
        self.sequence_in.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sequence_in(&mut self, v: i32) {
        self.sequence_in = ::std::option::Option::Some(v);
    }

    pub fn get_sequence_in(&self) -> i32 {
        self.sequence_in.unwrap_or(0)
    }

    // optional int32 sequence_out_ack = 2;

    pub fn clear_sequence_out_ack(&mut self) {
        self.sequence_out_ack = ::std::option::Option::None;
    }

    pub fn has_sequence_out_ack(&self) -> bool {
        self.sequence_out_ack.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sequence_out_ack(&mut self, v: i32) {
        self.sequence_out_ack = ::std::option::Option::Some(v);
    }

    pub fn get_sequence_out_ack(&self) -> i32 {
        self.sequence_out_ack.unwrap_or(0)
    }

    // optional bytes data = 3;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        };
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for CDemoPacket {
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
                    self.sequence_in = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.sequence_out_ack = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data));
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
        for value in &self.sequence_in {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.sequence_out_ack {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.data {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sequence_in {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.sequence_out_ack {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.data.as_ref() {
            try!(os.write_bytes(3, &v));
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
        ::std::any::TypeId::of::<CDemoPacket>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoPacket {
    fn new() -> CDemoPacket {
        CDemoPacket::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoPacket>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "sequence_in",
                    CDemoPacket::has_sequence_in,
                    CDemoPacket::get_sequence_in,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "sequence_out_ack",
                    CDemoPacket::has_sequence_out_ack,
                    CDemoPacket::get_sequence_out_ack,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "data",
                    CDemoPacket::has_data,
                    CDemoPacket::get_data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoPacket>(
                    "CDemoPacket",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoPacket {
    fn clear(&mut self) {
        self.clear_sequence_in();
        self.clear_sequence_out_ack();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDemoPacket {
    fn eq(&self, other: &CDemoPacket) -> bool {
        self.sequence_in == other.sequence_in &&
        self.sequence_out_ack == other.sequence_out_ack &&
        self.data == other.data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDemoPacket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDemoFullPacket {
    // message fields
    string_table: ::protobuf::SingularPtrField<CDemoStringTables>,
    packet: ::protobuf::SingularPtrField<CDemoPacket>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoFullPacket {}

impl CDemoFullPacket {
    pub fn new() -> CDemoFullPacket {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoFullPacket {
        static mut instance: ::protobuf::lazy::Lazy<CDemoFullPacket> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoFullPacket,
        };
        unsafe {
            instance.get(|| {
                CDemoFullPacket {
                    string_table: ::protobuf::SingularPtrField::none(),
                    packet: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .CDemoStringTables string_table = 1;

    pub fn clear_string_table(&mut self) {
        self.string_table.clear();
    }

    pub fn has_string_table(&self) -> bool {
        self.string_table.is_some()
    }

    // Param is passed by value, moved
    pub fn set_string_table(&mut self, v: CDemoStringTables) {
        self.string_table = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_string_table(&mut self) -> &mut CDemoStringTables {
        if self.string_table.is_none() {
            self.string_table.set_default();
        };
        self.string_table.as_mut().unwrap()
    }

    // Take field
    pub fn take_string_table(&mut self) -> CDemoStringTables {
        self.string_table.take().unwrap_or_else(|| CDemoStringTables::new())
    }

    pub fn get_string_table(&self) -> &CDemoStringTables {
        self.string_table.as_ref().unwrap_or_else(|| CDemoStringTables::default_instance())
    }

    // optional .CDemoPacket packet = 2;

    pub fn clear_packet(&mut self) {
        self.packet.clear();
    }

    pub fn has_packet(&self) -> bool {
        self.packet.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packet(&mut self, v: CDemoPacket) {
        self.packet = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_packet(&mut self) -> &mut CDemoPacket {
        if self.packet.is_none() {
            self.packet.set_default();
        };
        self.packet.as_mut().unwrap()
    }

    // Take field
    pub fn take_packet(&mut self) -> CDemoPacket {
        self.packet.take().unwrap_or_else(|| CDemoPacket::new())
    }

    pub fn get_packet(&self) -> &CDemoPacket {
        self.packet.as_ref().unwrap_or_else(|| CDemoPacket::default_instance())
    }
}

impl ::protobuf::Message for CDemoFullPacket {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.string_table));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.packet));
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
        for value in &self.string_table {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.packet {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.string_table.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.packet.as_ref() {
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
        ::std::any::TypeId::of::<CDemoFullPacket>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoFullPacket {
    fn new() -> CDemoFullPacket {
        CDemoFullPacket::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoFullPacket>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "string_table",
                    CDemoFullPacket::has_string_table,
                    CDemoFullPacket::get_string_table,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "packet",
                    CDemoFullPacket::has_packet,
                    CDemoFullPacket::get_packet,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoFullPacket>(
                    "CDemoFullPacket",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoFullPacket {
    fn clear(&mut self) {
        self.clear_string_table();
        self.clear_packet();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDemoFullPacket {
    fn eq(&self, other: &CDemoFullPacket) -> bool {
        self.string_table == other.string_table &&
        self.packet == other.packet &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDemoFullPacket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDemoSyncTick {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoSyncTick {}

impl CDemoSyncTick {
    pub fn new() -> CDemoSyncTick {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoSyncTick {
        static mut instance: ::protobuf::lazy::Lazy<CDemoSyncTick> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoSyncTick,
        };
        unsafe {
            instance.get(|| {
                CDemoSyncTick {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for CDemoSyncTick {
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
        ::std::any::TypeId::of::<CDemoSyncTick>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoSyncTick {
    fn new() -> CDemoSyncTick {
        CDemoSyncTick::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoSyncTick>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CDemoSyncTick>(
                    "CDemoSyncTick",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoSyncTick {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDemoSyncTick {
    fn eq(&self, other: &CDemoSyncTick) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDemoSyncTick {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDemoConsoleCmd {
    // message fields
    cmdstring: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoConsoleCmd {}

impl CDemoConsoleCmd {
    pub fn new() -> CDemoConsoleCmd {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoConsoleCmd {
        static mut instance: ::protobuf::lazy::Lazy<CDemoConsoleCmd> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoConsoleCmd,
        };
        unsafe {
            instance.get(|| {
                CDemoConsoleCmd {
                    cmdstring: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string cmdstring = 1;

    pub fn clear_cmdstring(&mut self) {
        self.cmdstring.clear();
    }

    pub fn has_cmdstring(&self) -> bool {
        self.cmdstring.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmdstring(&mut self, v: ::std::string::String) {
        self.cmdstring = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmdstring(&mut self) -> &mut ::std::string::String {
        if self.cmdstring.is_none() {
            self.cmdstring.set_default();
        };
        self.cmdstring.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmdstring(&mut self) -> ::std::string::String {
        self.cmdstring.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_cmdstring(&self) -> &str {
        match self.cmdstring.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for CDemoConsoleCmd {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.cmdstring));
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
        for value in &self.cmdstring {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cmdstring.as_ref() {
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
        ::std::any::TypeId::of::<CDemoConsoleCmd>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoConsoleCmd {
    fn new() -> CDemoConsoleCmd {
        CDemoConsoleCmd::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoConsoleCmd>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "cmdstring",
                    CDemoConsoleCmd::has_cmdstring,
                    CDemoConsoleCmd::get_cmdstring,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoConsoleCmd>(
                    "CDemoConsoleCmd",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoConsoleCmd {
    fn clear(&mut self) {
        self.clear_cmdstring();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDemoConsoleCmd {
    fn eq(&self, other: &CDemoConsoleCmd) -> bool {
        self.cmdstring == other.cmdstring &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDemoConsoleCmd {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDemoSendTables {
    // message fields
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoSendTables {}

impl CDemoSendTables {
    pub fn new() -> CDemoSendTables {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoSendTables {
        static mut instance: ::protobuf::lazy::Lazy<CDemoSendTables> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoSendTables,
        };
        unsafe {
            instance.get(|| {
                CDemoSendTables {
                    data: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes data = 1;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        };
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for CDemoSendTables {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data));
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
        for value in &self.data {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.data.as_ref() {
            try!(os.write_bytes(1, &v));
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
        ::std::any::TypeId::of::<CDemoSendTables>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoSendTables {
    fn new() -> CDemoSendTables {
        CDemoSendTables::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoSendTables>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "data",
                    CDemoSendTables::has_data,
                    CDemoSendTables::get_data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoSendTables>(
                    "CDemoSendTables",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoSendTables {
    fn clear(&mut self) {
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDemoSendTables {
    fn eq(&self, other: &CDemoSendTables) -> bool {
        self.data == other.data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDemoSendTables {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDemoClassInfo {
    // message fields
    classes: ::protobuf::RepeatedField<CDemoClassInfo_class_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoClassInfo {}

impl CDemoClassInfo {
    pub fn new() -> CDemoClassInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoClassInfo {
        static mut instance: ::protobuf::lazy::Lazy<CDemoClassInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoClassInfo,
        };
        unsafe {
            instance.get(|| {
                CDemoClassInfo {
                    classes: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .CDemoClassInfo.class_t classes = 1;

    pub fn clear_classes(&mut self) {
        self.classes.clear();
    }

    // Param is passed by value, moved
    pub fn set_classes(&mut self, v: ::protobuf::RepeatedField<CDemoClassInfo_class_t>) {
        self.classes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_classes(&mut self) -> &mut ::protobuf::RepeatedField<CDemoClassInfo_class_t> {
        &mut self.classes
    }

    // Take field
    pub fn take_classes(&mut self) -> ::protobuf::RepeatedField<CDemoClassInfo_class_t> {
        ::std::mem::replace(&mut self.classes, ::protobuf::RepeatedField::new())
    }

    pub fn get_classes(&self) -> &[CDemoClassInfo_class_t] {
        &self.classes
    }
}

impl ::protobuf::Message for CDemoClassInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.classes));
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
        for value in &self.classes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.classes {
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
        ::std::any::TypeId::of::<CDemoClassInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoClassInfo {
    fn new() -> CDemoClassInfo {
        CDemoClassInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoClassInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "classes",
                    CDemoClassInfo::get_classes,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoClassInfo>(
                    "CDemoClassInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoClassInfo {
    fn clear(&mut self) {
        self.clear_classes();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDemoClassInfo {
    fn eq(&self, other: &CDemoClassInfo) -> bool {
        self.classes == other.classes &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDemoClassInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDemoClassInfo_class_t {
    // message fields
    class_id: ::std::option::Option<i32>,
    network_name: ::protobuf::SingularField<::std::string::String>,
    table_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoClassInfo_class_t {}

impl CDemoClassInfo_class_t {
    pub fn new() -> CDemoClassInfo_class_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoClassInfo_class_t {
        static mut instance: ::protobuf::lazy::Lazy<CDemoClassInfo_class_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoClassInfo_class_t,
        };
        unsafe {
            instance.get(|| {
                CDemoClassInfo_class_t {
                    class_id: ::std::option::Option::None,
                    network_name: ::protobuf::SingularField::none(),
                    table_name: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 class_id = 1;

    pub fn clear_class_id(&mut self) {
        self.class_id = ::std::option::Option::None;
    }

    pub fn has_class_id(&self) -> bool {
        self.class_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_class_id(&mut self, v: i32) {
        self.class_id = ::std::option::Option::Some(v);
    }

    pub fn get_class_id(&self) -> i32 {
        self.class_id.unwrap_or(0)
    }

    // optional string network_name = 2;

    pub fn clear_network_name(&mut self) {
        self.network_name.clear();
    }

    pub fn has_network_name(&self) -> bool {
        self.network_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_network_name(&mut self, v: ::std::string::String) {
        self.network_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_network_name(&mut self) -> &mut ::std::string::String {
        if self.network_name.is_none() {
            self.network_name.set_default();
        };
        self.network_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_network_name(&mut self) -> ::std::string::String {
        self.network_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_network_name(&self) -> &str {
        match self.network_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string table_name = 3;

    pub fn clear_table_name(&mut self) {
        self.table_name.clear();
    }

    pub fn has_table_name(&self) -> bool {
        self.table_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table_name(&mut self, v: ::std::string::String) {
        self.table_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_table_name(&mut self) -> &mut ::std::string::String {
        if self.table_name.is_none() {
            self.table_name.set_default();
        };
        self.table_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_table_name(&mut self) -> ::std::string::String {
        self.table_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_table_name(&self) -> &str {
        match self.table_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for CDemoClassInfo_class_t {
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
                    self.class_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.network_name));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.table_name));
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
        for value in &self.class_id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.network_name {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.table_name {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.class_id {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.network_name.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.table_name.as_ref() {
            try!(os.write_string(3, &v));
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
        ::std::any::TypeId::of::<CDemoClassInfo_class_t>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoClassInfo_class_t {
    fn new() -> CDemoClassInfo_class_t {
        CDemoClassInfo_class_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoClassInfo_class_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "class_id",
                    CDemoClassInfo_class_t::has_class_id,
                    CDemoClassInfo_class_t::get_class_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "network_name",
                    CDemoClassInfo_class_t::has_network_name,
                    CDemoClassInfo_class_t::get_network_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "table_name",
                    CDemoClassInfo_class_t::has_table_name,
                    CDemoClassInfo_class_t::get_table_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoClassInfo_class_t>(
                    "CDemoClassInfo_class_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoClassInfo_class_t {
    fn clear(&mut self) {
        self.clear_class_id();
        self.clear_network_name();
        self.clear_table_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDemoClassInfo_class_t {
    fn eq(&self, other: &CDemoClassInfo_class_t) -> bool {
        self.class_id == other.class_id &&
        self.network_name == other.network_name &&
        self.table_name == other.table_name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDemoClassInfo_class_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDemoCustomData {
    // message fields
    callback_index: ::std::option::Option<i32>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoCustomData {}

impl CDemoCustomData {
    pub fn new() -> CDemoCustomData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoCustomData {
        static mut instance: ::protobuf::lazy::Lazy<CDemoCustomData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoCustomData,
        };
        unsafe {
            instance.get(|| {
                CDemoCustomData {
                    callback_index: ::std::option::Option::None,
                    data: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 callback_index = 1;

    pub fn clear_callback_index(&mut self) {
        self.callback_index = ::std::option::Option::None;
    }

    pub fn has_callback_index(&self) -> bool {
        self.callback_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_callback_index(&mut self, v: i32) {
        self.callback_index = ::std::option::Option::Some(v);
    }

    pub fn get_callback_index(&self) -> i32 {
        self.callback_index.unwrap_or(0)
    }

    // optional bytes data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        };
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for CDemoCustomData {
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
                    self.callback_index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data));
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
        for value in &self.callback_index {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.data {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.callback_index {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.data.as_ref() {
            try!(os.write_bytes(2, &v));
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
        ::std::any::TypeId::of::<CDemoCustomData>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoCustomData {
    fn new() -> CDemoCustomData {
        CDemoCustomData::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoCustomData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "callback_index",
                    CDemoCustomData::has_callback_index,
                    CDemoCustomData::get_callback_index,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "data",
                    CDemoCustomData::has_data,
                    CDemoCustomData::get_data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoCustomData>(
                    "CDemoCustomData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoCustomData {
    fn clear(&mut self) {
        self.clear_callback_index();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDemoCustomData {
    fn eq(&self, other: &CDemoCustomData) -> bool {
        self.callback_index == other.callback_index &&
        self.data == other.data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDemoCustomData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDemoCustomDataCallbacks {
    // message fields
    save_id: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoCustomDataCallbacks {}

impl CDemoCustomDataCallbacks {
    pub fn new() -> CDemoCustomDataCallbacks {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoCustomDataCallbacks {
        static mut instance: ::protobuf::lazy::Lazy<CDemoCustomDataCallbacks> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoCustomDataCallbacks,
        };
        unsafe {
            instance.get(|| {
                CDemoCustomDataCallbacks {
                    save_id: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated string save_id = 1;

    pub fn clear_save_id(&mut self) {
        self.save_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_save_id(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.save_id = v;
    }

    // Mutable pointer to the field.
    pub fn mut_save_id(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.save_id
    }

    // Take field
    pub fn take_save_id(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.save_id, ::protobuf::RepeatedField::new())
    }

    pub fn get_save_id(&self) -> &[::std::string::String] {
        &self.save_id
    }
}

impl ::protobuf::Message for CDemoCustomDataCallbacks {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.save_id));
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
        for value in &self.save_id {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.save_id {
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
        ::std::any::TypeId::of::<CDemoCustomDataCallbacks>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoCustomDataCallbacks {
    fn new() -> CDemoCustomDataCallbacks {
        CDemoCustomDataCallbacks::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoCustomDataCallbacks>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "save_id",
                    CDemoCustomDataCallbacks::get_save_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoCustomDataCallbacks>(
                    "CDemoCustomDataCallbacks",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoCustomDataCallbacks {
    fn clear(&mut self) {
        self.clear_save_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDemoCustomDataCallbacks {
    fn eq(&self, other: &CDemoCustomDataCallbacks) -> bool {
        self.save_id == other.save_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDemoCustomDataCallbacks {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDemoStringTables {
    // message fields
    tables: ::protobuf::RepeatedField<CDemoStringTables_table_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoStringTables {}

impl CDemoStringTables {
    pub fn new() -> CDemoStringTables {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoStringTables {
        static mut instance: ::protobuf::lazy::Lazy<CDemoStringTables> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoStringTables,
        };
        unsafe {
            instance.get(|| {
                CDemoStringTables {
                    tables: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .CDemoStringTables.table_t tables = 1;

    pub fn clear_tables(&mut self) {
        self.tables.clear();
    }

    // Param is passed by value, moved
    pub fn set_tables(&mut self, v: ::protobuf::RepeatedField<CDemoStringTables_table_t>) {
        self.tables = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tables(&mut self) -> &mut ::protobuf::RepeatedField<CDemoStringTables_table_t> {
        &mut self.tables
    }

    // Take field
    pub fn take_tables(&mut self) -> ::protobuf::RepeatedField<CDemoStringTables_table_t> {
        ::std::mem::replace(&mut self.tables, ::protobuf::RepeatedField::new())
    }

    pub fn get_tables(&self) -> &[CDemoStringTables_table_t] {
        &self.tables
    }
}

impl ::protobuf::Message for CDemoStringTables {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tables));
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
        for value in &self.tables {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.tables {
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
        ::std::any::TypeId::of::<CDemoStringTables>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoStringTables {
    fn new() -> CDemoStringTables {
        CDemoStringTables::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoStringTables>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "tables",
                    CDemoStringTables::get_tables,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoStringTables>(
                    "CDemoStringTables",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoStringTables {
    fn clear(&mut self) {
        self.clear_tables();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDemoStringTables {
    fn eq(&self, other: &CDemoStringTables) -> bool {
        self.tables == other.tables &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDemoStringTables {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDemoStringTables_items_t {
    // message fields
    str: ::protobuf::SingularField<::std::string::String>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoStringTables_items_t {}

impl CDemoStringTables_items_t {
    pub fn new() -> CDemoStringTables_items_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoStringTables_items_t {
        static mut instance: ::protobuf::lazy::Lazy<CDemoStringTables_items_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoStringTables_items_t,
        };
        unsafe {
            instance.get(|| {
                CDemoStringTables_items_t {
                    str: ::protobuf::SingularField::none(),
                    data: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string str = 1;

    pub fn clear_str(&mut self) {
        self.str.clear();
    }

    pub fn has_str(&self) -> bool {
        self.str.is_some()
    }

    // Param is passed by value, moved
    pub fn set_str(&mut self, v: ::std::string::String) {
        self.str = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_str(&mut self) -> &mut ::std::string::String {
        if self.str.is_none() {
            self.str.set_default();
        };
        self.str.as_mut().unwrap()
    }

    // Take field
    pub fn take_str(&mut self) -> ::std::string::String {
        self.str.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_str(&self) -> &str {
        match self.str.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bytes data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        };
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for CDemoStringTables_items_t {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.str));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data));
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
        for value in &self.str {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.data {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.str.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.data.as_ref() {
            try!(os.write_bytes(2, &v));
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
        ::std::any::TypeId::of::<CDemoStringTables_items_t>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoStringTables_items_t {
    fn new() -> CDemoStringTables_items_t {
        CDemoStringTables_items_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoStringTables_items_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "str",
                    CDemoStringTables_items_t::has_str,
                    CDemoStringTables_items_t::get_str,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "data",
                    CDemoStringTables_items_t::has_data,
                    CDemoStringTables_items_t::get_data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoStringTables_items_t>(
                    "CDemoStringTables_items_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoStringTables_items_t {
    fn clear(&mut self) {
        self.clear_str();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDemoStringTables_items_t {
    fn eq(&self, other: &CDemoStringTables_items_t) -> bool {
        self.str == other.str &&
        self.data == other.data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDemoStringTables_items_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDemoStringTables_table_t {
    // message fields
    table_name: ::protobuf::SingularField<::std::string::String>,
    items: ::protobuf::RepeatedField<CDemoStringTables_items_t>,
    items_clientside: ::protobuf::RepeatedField<CDemoStringTables_items_t>,
    table_flags: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoStringTables_table_t {}

impl CDemoStringTables_table_t {
    pub fn new() -> CDemoStringTables_table_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoStringTables_table_t {
        static mut instance: ::protobuf::lazy::Lazy<CDemoStringTables_table_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoStringTables_table_t,
        };
        unsafe {
            instance.get(|| {
                CDemoStringTables_table_t {
                    table_name: ::protobuf::SingularField::none(),
                    items: ::protobuf::RepeatedField::new(),
                    items_clientside: ::protobuf::RepeatedField::new(),
                    table_flags: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string table_name = 1;

    pub fn clear_table_name(&mut self) {
        self.table_name.clear();
    }

    pub fn has_table_name(&self) -> bool {
        self.table_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table_name(&mut self, v: ::std::string::String) {
        self.table_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_table_name(&mut self) -> &mut ::std::string::String {
        if self.table_name.is_none() {
            self.table_name.set_default();
        };
        self.table_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_table_name(&mut self) -> ::std::string::String {
        self.table_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_table_name(&self) -> &str {
        match self.table_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated .CDemoStringTables.items_t items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<CDemoStringTables_items_t>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<CDemoStringTables_items_t> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<CDemoStringTables_items_t> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[CDemoStringTables_items_t] {
        &self.items
    }

    // repeated .CDemoStringTables.items_t items_clientside = 3;

    pub fn clear_items_clientside(&mut self) {
        self.items_clientside.clear();
    }

    // Param is passed by value, moved
    pub fn set_items_clientside(&mut self, v: ::protobuf::RepeatedField<CDemoStringTables_items_t>) {
        self.items_clientside = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items_clientside(&mut self) -> &mut ::protobuf::RepeatedField<CDemoStringTables_items_t> {
        &mut self.items_clientside
    }

    // Take field
    pub fn take_items_clientside(&mut self) -> ::protobuf::RepeatedField<CDemoStringTables_items_t> {
        ::std::mem::replace(&mut self.items_clientside, ::protobuf::RepeatedField::new())
    }

    pub fn get_items_clientside(&self) -> &[CDemoStringTables_items_t] {
        &self.items_clientside
    }

    // optional int32 table_flags = 4;

    pub fn clear_table_flags(&mut self) {
        self.table_flags = ::std::option::Option::None;
    }

    pub fn has_table_flags(&self) -> bool {
        self.table_flags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table_flags(&mut self, v: i32) {
        self.table_flags = ::std::option::Option::Some(v);
    }

    pub fn get_table_flags(&self) -> i32 {
        self.table_flags.unwrap_or(0)
    }
}

impl ::protobuf::Message for CDemoStringTables_table_t {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.table_name));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.items));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.items_clientside));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.table_flags = ::std::option::Option::Some(tmp);
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
        for value in &self.table_name {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.items_clientside {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.table_flags {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.table_name.as_ref() {
            try!(os.write_string(1, &v));
        };
        for v in &self.items {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.items_clientside {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.table_flags {
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
        ::std::any::TypeId::of::<CDemoStringTables_table_t>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoStringTables_table_t {
    fn new() -> CDemoStringTables_table_t {
        CDemoStringTables_table_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoStringTables_table_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "table_name",
                    CDemoStringTables_table_t::has_table_name,
                    CDemoStringTables_table_t::get_table_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "items",
                    CDemoStringTables_table_t::get_items,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "items_clientside",
                    CDemoStringTables_table_t::get_items_clientside,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "table_flags",
                    CDemoStringTables_table_t::has_table_flags,
                    CDemoStringTables_table_t::get_table_flags,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoStringTables_table_t>(
                    "CDemoStringTables_table_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoStringTables_table_t {
    fn clear(&mut self) {
        self.clear_table_name();
        self.clear_items();
        self.clear_items_clientside();
        self.clear_table_flags();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDemoStringTables_table_t {
    fn eq(&self, other: &CDemoStringTables_table_t) -> bool {
        self.table_name == other.table_name &&
        self.items == other.items &&
        self.items_clientside == other.items_clientside &&
        self.table_flags == other.table_flags &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDemoStringTables_table_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDemoStop {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoStop {}

impl CDemoStop {
    pub fn new() -> CDemoStop {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoStop {
        static mut instance: ::protobuf::lazy::Lazy<CDemoStop> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoStop,
        };
        unsafe {
            instance.get(|| {
                CDemoStop {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for CDemoStop {
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
        ::std::any::TypeId::of::<CDemoStop>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoStop {
    fn new() -> CDemoStop {
        CDemoStop::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoStop>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CDemoStop>(
                    "CDemoStop",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoStop {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDemoStop {
    fn eq(&self, other: &CDemoStop) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDemoStop {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CDemoUserCmd {
    // message fields
    cmd_number: ::std::option::Option<i32>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoUserCmd {}

impl CDemoUserCmd {
    pub fn new() -> CDemoUserCmd {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoUserCmd {
        static mut instance: ::protobuf::lazy::Lazy<CDemoUserCmd> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoUserCmd,
        };
        unsafe {
            instance.get(|| {
                CDemoUserCmd {
                    cmd_number: ::std::option::Option::None,
                    data: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 cmd_number = 1;

    pub fn clear_cmd_number(&mut self) {
        self.cmd_number = ::std::option::Option::None;
    }

    pub fn has_cmd_number(&self) -> bool {
        self.cmd_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_number(&mut self, v: i32) {
        self.cmd_number = ::std::option::Option::Some(v);
    }

    pub fn get_cmd_number(&self) -> i32 {
        self.cmd_number.unwrap_or(0)
    }

    // optional bytes data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        };
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for CDemoUserCmd {
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
                    self.cmd_number = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data));
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
        for value in &self.cmd_number {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.data {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cmd_number {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.data.as_ref() {
            try!(os.write_bytes(2, &v));
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
        ::std::any::TypeId::of::<CDemoUserCmd>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoUserCmd {
    fn new() -> CDemoUserCmd {
        CDemoUserCmd::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoUserCmd>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "cmd_number",
                    CDemoUserCmd::has_cmd_number,
                    CDemoUserCmd::get_cmd_number,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "data",
                    CDemoUserCmd::has_data,
                    CDemoUserCmd::get_data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoUserCmd>(
                    "CDemoUserCmd",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoUserCmd {
    fn clear(&mut self) {
        self.clear_cmd_number();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDemoUserCmd {
    fn eq(&self, other: &CDemoUserCmd) -> bool {
        self.cmd_number == other.cmd_number &&
        self.data == other.data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDemoUserCmd {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EDemoCommands {
    DEM_Error = -1,
    DEM_Stop = 0,
    DEM_FileHeader = 1,
    DEM_FileInfo = 2,
    DEM_SyncTick = 3,
    DEM_SendTables = 4,
    DEM_ClassInfo = 5,
    DEM_StringTables = 6,
    DEM_Packet = 7,
    DEM_SignonPacket = 8,
    DEM_ConsoleCmd = 9,
    DEM_CustomData = 10,
    DEM_CustomDataCallbacks = 11,
    DEM_UserCmd = 12,
    DEM_FullPacket = 13,
    DEM_Max = 14,
    DEM_IsCompressed = 112,
}

impl ::protobuf::ProtobufEnum for EDemoCommands {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EDemoCommands> {
        match value {
            -1 => ::std::option::Option::Some(EDemoCommands::DEM_Error),
            0 => ::std::option::Option::Some(EDemoCommands::DEM_Stop),
            1 => ::std::option::Option::Some(EDemoCommands::DEM_FileHeader),
            2 => ::std::option::Option::Some(EDemoCommands::DEM_FileInfo),
            3 => ::std::option::Option::Some(EDemoCommands::DEM_SyncTick),
            4 => ::std::option::Option::Some(EDemoCommands::DEM_SendTables),
            5 => ::std::option::Option::Some(EDemoCommands::DEM_ClassInfo),
            6 => ::std::option::Option::Some(EDemoCommands::DEM_StringTables),
            7 => ::std::option::Option::Some(EDemoCommands::DEM_Packet),
            8 => ::std::option::Option::Some(EDemoCommands::DEM_SignonPacket),
            9 => ::std::option::Option::Some(EDemoCommands::DEM_ConsoleCmd),
            10 => ::std::option::Option::Some(EDemoCommands::DEM_CustomData),
            11 => ::std::option::Option::Some(EDemoCommands::DEM_CustomDataCallbacks),
            12 => ::std::option::Option::Some(EDemoCommands::DEM_UserCmd),
            13 => ::std::option::Option::Some(EDemoCommands::DEM_FullPacket),
            14 => ::std::option::Option::Some(EDemoCommands::DEM_Max),
            112 => ::std::option::Option::Some(EDemoCommands::DEM_IsCompressed),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EDemoCommands] = &[
            EDemoCommands::DEM_Error,
            EDemoCommands::DEM_Stop,
            EDemoCommands::DEM_FileHeader,
            EDemoCommands::DEM_FileInfo,
            EDemoCommands::DEM_SyncTick,
            EDemoCommands::DEM_SendTables,
            EDemoCommands::DEM_ClassInfo,
            EDemoCommands::DEM_StringTables,
            EDemoCommands::DEM_Packet,
            EDemoCommands::DEM_SignonPacket,
            EDemoCommands::DEM_ConsoleCmd,
            EDemoCommands::DEM_CustomData,
            EDemoCommands::DEM_CustomDataCallbacks,
            EDemoCommands::DEM_UserCmd,
            EDemoCommands::DEM_FullPacket,
            EDemoCommands::DEM_Max,
            EDemoCommands::DEM_IsCompressed,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<EDemoCommands>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EDemoCommands", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EDemoCommands {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0a, 0x64, 0x65, 0x6d, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x20, 0x67, 0x6f,
    0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x64, 0x65,
    0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xd8,
    0x01, 0x0a, 0x0f, 0x43, 0x44, 0x65, 0x6d, 0x6f, 0x46, 0x69, 0x6c, 0x65, 0x48, 0x65, 0x61, 0x64,
    0x65, 0x72, 0x12, 0x17, 0x0a, 0x0f, 0x64, 0x65, 0x6d, 0x6f, 0x5f, 0x66, 0x69, 0x6c, 0x65, 0x5f,
    0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x18, 0x0a, 0x10, 0x6e,
    0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x05, 0x12, 0x13, 0x0a, 0x0b, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f,
    0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x12, 0x13, 0x0a, 0x0b, 0x63, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x12,
    0x10, 0x0a, 0x08, 0x6d, 0x61, 0x70, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x09, 0x12, 0x16, 0x0a, 0x0e, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74,
    0x6f, 0x72, 0x79, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x12, 0x1b, 0x0a, 0x13, 0x66, 0x75, 0x6c,
    0x6c, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x18, 0x07, 0x20, 0x01, 0x28, 0x05, 0x12, 0x21, 0x0a, 0x19, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x5f,
    0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x69, 0x64, 0x65, 0x5f, 0x65, 0x6e, 0x74, 0x69, 0x74,
    0x69, 0x65, 0x73, 0x18, 0x08, 0x20, 0x01, 0x28, 0x08, 0x22, 0x89, 0x02, 0x0a, 0x09, 0x43, 0x47,
    0x61, 0x6d, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x26, 0x0a, 0x04, 0x64, 0x6f, 0x74, 0x61, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x43, 0x47, 0x61, 0x6d, 0x65, 0x49, 0x6e, 0x66,
    0x6f, 0x2e, 0x43, 0x44, 0x6f, 0x74, 0x61, 0x47, 0x61, 0x6d, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x1a,
    0xd3, 0x01, 0x0a, 0x0d, 0x43, 0x44, 0x6f, 0x74, 0x61, 0x47, 0x61, 0x6d, 0x65, 0x49, 0x6e, 0x66,
    0x6f, 0x12, 0x10, 0x0a, 0x08, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0d, 0x12, 0x11, 0x0a, 0x09, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x6d, 0x6f, 0x64, 0x65,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12, 0x13, 0x0a, 0x0b, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x77,
    0x69, 0x6e, 0x6e, 0x65, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x12, 0x39, 0x0a, 0x0b, 0x70,
    0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x24, 0x2e, 0x43, 0x47, 0x61, 0x6d, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x2e, 0x43, 0x44, 0x6f,
    0x74, 0x61, 0x47, 0x61, 0x6d, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x2e, 0x43, 0x50, 0x6c, 0x61, 0x79,
    0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x1a, 0x4d, 0x0a, 0x0b, 0x43, 0x50, 0x6c, 0x61, 0x79, 0x65,
    0x72, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x11, 0x0a, 0x09, 0x68, 0x65, 0x72, 0x6f, 0x5f, 0x6e, 0x61,
    0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x12, 0x13, 0x0a, 0x0b, 0x70, 0x6c, 0x61, 0x79,
    0x65, 0x72, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x16, 0x0a,
    0x0e, 0x69, 0x73, 0x5f, 0x66, 0x61, 0x6b, 0x65, 0x5f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x08, 0x22, 0x76, 0x0a, 0x0d, 0x43, 0x44, 0x65, 0x6d, 0x6f, 0x46, 0x69,
    0x6c, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x15, 0x0a, 0x0d, 0x70, 0x6c, 0x61, 0x79, 0x62, 0x61,
    0x63, 0x6b, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x02, 0x12, 0x16, 0x0a,
    0x0e, 0x70, 0x6c, 0x61, 0x79, 0x62, 0x61, 0x63, 0x6b, 0x5f, 0x74, 0x69, 0x63, 0x6b, 0x73, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x05, 0x12, 0x17, 0x0a, 0x0f, 0x70, 0x6c, 0x61, 0x79, 0x62, 0x61, 0x63,
    0x6b, 0x5f, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x12, 0x1d,
    0x0a, 0x09, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x0a, 0x2e, 0x43, 0x47, 0x61, 0x6d, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x22, 0x4a, 0x0a,
    0x0b, 0x43, 0x44, 0x65, 0x6d, 0x6f, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x12, 0x13, 0x0a, 0x0b,
    0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x5f, 0x69, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x05, 0x12, 0x18, 0x0a, 0x10, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x5f, 0x6f, 0x75,
    0x74, 0x5f, 0x61, 0x63, 0x6b, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0c, 0x0a, 0x04, 0x64,
    0x61, 0x74, 0x61, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x59, 0x0a, 0x0f, 0x43, 0x44, 0x65,
    0x6d, 0x6f, 0x46, 0x75, 0x6c, 0x6c, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x12, 0x28, 0x0a, 0x0c,
    0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x12, 0x2e, 0x43, 0x44, 0x65, 0x6d, 0x6f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67,
    0x54, 0x61, 0x62, 0x6c, 0x65, 0x73, 0x12, 0x1c, 0x0a, 0x06, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x43, 0x44, 0x65, 0x6d, 0x6f, 0x50, 0x61,
    0x63, 0x6b, 0x65, 0x74, 0x22, 0x0f, 0x0a, 0x0d, 0x43, 0x44, 0x65, 0x6d, 0x6f, 0x53, 0x79, 0x6e,
    0x63, 0x54, 0x69, 0x63, 0x6b, 0x22, 0x24, 0x0a, 0x0f, 0x43, 0x44, 0x65, 0x6d, 0x6f, 0x43, 0x6f,
    0x6e, 0x73, 0x6f, 0x6c, 0x65, 0x43, 0x6d, 0x64, 0x12, 0x11, 0x0a, 0x09, 0x63, 0x6d, 0x64, 0x73,
    0x74, 0x72, 0x69, 0x6e, 0x67, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x22, 0x1f, 0x0a, 0x0f, 0x43,
    0x44, 0x65, 0x6d, 0x6f, 0x53, 0x65, 0x6e, 0x64, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x73, 0x12, 0x0c,
    0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x81, 0x01, 0x0a,
    0x0e, 0x43, 0x44, 0x65, 0x6d, 0x6f, 0x43, 0x6c, 0x61, 0x73, 0x73, 0x49, 0x6e, 0x66, 0x6f, 0x12,
    0x28, 0x0a, 0x07, 0x63, 0x6c, 0x61, 0x73, 0x73, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x17, 0x2e, 0x43, 0x44, 0x65, 0x6d, 0x6f, 0x43, 0x6c, 0x61, 0x73, 0x73, 0x49, 0x6e, 0x66,
    0x6f, 0x2e, 0x63, 0x6c, 0x61, 0x73, 0x73, 0x5f, 0x74, 0x1a, 0x45, 0x0a, 0x07, 0x63, 0x6c, 0x61,
    0x73, 0x73, 0x5f, 0x74, 0x12, 0x10, 0x0a, 0x08, 0x63, 0x6c, 0x61, 0x73, 0x73, 0x5f, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x14, 0x0a, 0x0c, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72,
    0x6b, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x12, 0x0a, 0x0a,
    0x74, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09,
    0x22, 0x37, 0x0a, 0x0f, 0x43, 0x44, 0x65, 0x6d, 0x6f, 0x43, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x44,
    0x61, 0x74, 0x61, 0x12, 0x16, 0x0a, 0x0e, 0x63, 0x61, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x5f,
    0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0c, 0x0a, 0x04, 0x64,
    0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x2b, 0x0a, 0x18, 0x43, 0x44, 0x65,
    0x6d, 0x6f, 0x43, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x43, 0x61, 0x6c, 0x6c,
    0x62, 0x61, 0x63, 0x6b, 0x73, 0x12, 0x0f, 0x0a, 0x07, 0x73, 0x61, 0x76, 0x65, 0x5f, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x03, 0x28, 0x09, 0x22, 0xfb, 0x01, 0x0a, 0x11, 0x43, 0x44, 0x65, 0x6d, 0x6f,
    0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x73, 0x12, 0x2a, 0x0a, 0x06,
    0x74, 0x61, 0x62, 0x6c, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x43,
    0x44, 0x65, 0x6d, 0x6f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x73,
    0x2e, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x74, 0x1a, 0x24, 0x0a, 0x07, 0x69, 0x74, 0x65, 0x6d,
    0x73, 0x5f, 0x74, 0x12, 0x0b, 0x0a, 0x03, 0x73, 0x74, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x12, 0x0c, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x1a, 0x93,
    0x01, 0x0a, 0x07, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x74, 0x12, 0x12, 0x0a, 0x0a, 0x74, 0x61,
    0x62, 0x6c, 0x65, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x12, 0x29,
    0x0a, 0x05, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1a, 0x2e,
    0x43, 0x44, 0x65, 0x6d, 0x6f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x54, 0x61, 0x62, 0x6c, 0x65,
    0x73, 0x2e, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x5f, 0x74, 0x12, 0x34, 0x0a, 0x10, 0x69, 0x74, 0x65,
    0x6d, 0x73, 0x5f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x69, 0x64, 0x65, 0x18, 0x03, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x43, 0x44, 0x65, 0x6d, 0x6f, 0x53, 0x74, 0x72, 0x69, 0x6e,
    0x67, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x73, 0x2e, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x5f, 0x74, 0x12,
    0x13, 0x0a, 0x0b, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x05, 0x22, 0x0b, 0x0a, 0x09, 0x43, 0x44, 0x65, 0x6d, 0x6f, 0x53, 0x74, 0x6f,
    0x70, 0x22, 0x30, 0x0a, 0x0c, 0x43, 0x44, 0x65, 0x6d, 0x6f, 0x55, 0x73, 0x65, 0x72, 0x43, 0x6d,
    0x64, 0x12, 0x12, 0x0a, 0x0a, 0x63, 0x6d, 0x64, 0x5f, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0c, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0c, 0x2a, 0xdd, 0x02, 0x0a, 0x0d, 0x45, 0x44, 0x65, 0x6d, 0x6f, 0x43, 0x6f, 0x6d,
    0x6d, 0x61, 0x6e, 0x64, 0x73, 0x12, 0x16, 0x0a, 0x09, 0x44, 0x45, 0x4d, 0x5f, 0x45, 0x72, 0x72,
    0x6f, 0x72, 0x10, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01, 0x12, 0x0c, 0x0a,
    0x08, 0x44, 0x45, 0x4d, 0x5f, 0x53, 0x74, 0x6f, 0x70, 0x10, 0x00, 0x12, 0x12, 0x0a, 0x0e, 0x44,
    0x45, 0x4d, 0x5f, 0x46, 0x69, 0x6c, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x10, 0x01, 0x12,
    0x10, 0x0a, 0x0c, 0x44, 0x45, 0x4d, 0x5f, 0x46, 0x69, 0x6c, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x10,
    0x02, 0x12, 0x10, 0x0a, 0x0c, 0x44, 0x45, 0x4d, 0x5f, 0x53, 0x79, 0x6e, 0x63, 0x54, 0x69, 0x63,
    0x6b, 0x10, 0x03, 0x12, 0x12, 0x0a, 0x0e, 0x44, 0x45, 0x4d, 0x5f, 0x53, 0x65, 0x6e, 0x64, 0x54,
    0x61, 0x62, 0x6c, 0x65, 0x73, 0x10, 0x04, 0x12, 0x11, 0x0a, 0x0d, 0x44, 0x45, 0x4d, 0x5f, 0x43,
    0x6c, 0x61, 0x73, 0x73, 0x49, 0x6e, 0x66, 0x6f, 0x10, 0x05, 0x12, 0x14, 0x0a, 0x10, 0x44, 0x45,
    0x4d, 0x5f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x73, 0x10, 0x06,
    0x12, 0x0e, 0x0a, 0x0a, 0x44, 0x45, 0x4d, 0x5f, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x10, 0x07,
    0x12, 0x14, 0x0a, 0x10, 0x44, 0x45, 0x4d, 0x5f, 0x53, 0x69, 0x67, 0x6e, 0x6f, 0x6e, 0x50, 0x61,
    0x63, 0x6b, 0x65, 0x74, 0x10, 0x08, 0x12, 0x12, 0x0a, 0x0e, 0x44, 0x45, 0x4d, 0x5f, 0x43, 0x6f,
    0x6e, 0x73, 0x6f, 0x6c, 0x65, 0x43, 0x6d, 0x64, 0x10, 0x09, 0x12, 0x12, 0x0a, 0x0e, 0x44, 0x45,
    0x4d, 0x5f, 0x43, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x10, 0x0a, 0x12, 0x1b,
    0x0a, 0x17, 0x44, 0x45, 0x4d, 0x5f, 0x43, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x44, 0x61, 0x74, 0x61,
    0x43, 0x61, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x73, 0x10, 0x0b, 0x12, 0x0f, 0x0a, 0x0b, 0x44,
    0x45, 0x4d, 0x5f, 0x55, 0x73, 0x65, 0x72, 0x43, 0x6d, 0x64, 0x10, 0x0c, 0x12, 0x12, 0x0a, 0x0e,
    0x44, 0x45, 0x4d, 0x5f, 0x46, 0x75, 0x6c, 0x6c, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x10, 0x0d,
    0x12, 0x0b, 0x0a, 0x07, 0x44, 0x45, 0x4d, 0x5f, 0x4d, 0x61, 0x78, 0x10, 0x0e, 0x12, 0x14, 0x0a,
    0x10, 0x44, 0x45, 0x4d, 0x5f, 0x49, 0x73, 0x43, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65,
    0x64, 0x10, 0x70, 0x42, 0x03, 0x80, 0x01, 0x00,
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
