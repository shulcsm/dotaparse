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
pub struct CMsgVector {
    // message fields
    x: ::std::option::Option<f32>,
    y: ::std::option::Option<f32>,
    z: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgVector {}

impl CMsgVector {
    pub fn new() -> CMsgVector {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgVector {
        static mut instance: ::protobuf::lazy::Lazy<CMsgVector> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgVector,
        };
        unsafe {
            instance.get(|| {
                CMsgVector {
                    x: ::std::option::Option::None,
                    y: ::std::option::Option::None,
                    z: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional float x = 1;

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: f32) {
        self.x = ::std::option::Option::Some(v);
    }

    pub fn get_x(&self) -> f32 {
        self.x.unwrap_or(0.)
    }

    // optional float y = 2;

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: f32) {
        self.y = ::std::option::Option::Some(v);
    }

    pub fn get_y(&self) -> f32 {
        self.y.unwrap_or(0.)
    }

    // optional float z = 3;

    pub fn clear_z(&mut self) {
        self.z = ::std::option::Option::None;
    }

    pub fn has_z(&self) -> bool {
        self.z.is_some()
    }

    // Param is passed by value, moved
    pub fn set_z(&mut self, v: f32) {
        self.z = ::std::option::Option::Some(v);
    }

    pub fn get_z(&self) -> f32 {
        self.z.unwrap_or(0.)
    }
}

impl ::protobuf::Message for CMsgVector {
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
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.y = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.z = ::std::option::Option::Some(tmp);
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
        if self.x.is_some() {
            my_size += 5;
        };
        if self.y.is_some() {
            my_size += 5;
        };
        if self.z.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            try!(os.write_float(1, v));
        };
        if let Some(v) = self.y {
            try!(os.write_float(2, v));
        };
        if let Some(v) = self.z {
            try!(os.write_float(3, v));
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
        ::std::any::TypeId::of::<CMsgVector>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgVector {
    fn new() -> CMsgVector {
        CMsgVector::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgVector>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "x",
                    CMsgVector::has_x,
                    CMsgVector::get_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "y",
                    CMsgVector::has_y,
                    CMsgVector::get_y,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "z",
                    CMsgVector::has_z,
                    CMsgVector::get_z,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgVector>(
                    "CMsgVector",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgVector {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.clear_z();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CMsgVector {
    fn eq(&self, other: &CMsgVector) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CMsgVector {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CMsgVector2D {
    // message fields
    x: ::std::option::Option<f32>,
    y: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgVector2D {}

impl CMsgVector2D {
    pub fn new() -> CMsgVector2D {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgVector2D {
        static mut instance: ::protobuf::lazy::Lazy<CMsgVector2D> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgVector2D,
        };
        unsafe {
            instance.get(|| {
                CMsgVector2D {
                    x: ::std::option::Option::None,
                    y: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional float x = 1;

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: f32) {
        self.x = ::std::option::Option::Some(v);
    }

    pub fn get_x(&self) -> f32 {
        self.x.unwrap_or(0.)
    }

    // optional float y = 2;

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: f32) {
        self.y = ::std::option::Option::Some(v);
    }

    pub fn get_y(&self) -> f32 {
        self.y.unwrap_or(0.)
    }
}

impl ::protobuf::Message for CMsgVector2D {
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
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.y = ::std::option::Option::Some(tmp);
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
        if self.x.is_some() {
            my_size += 5;
        };
        if self.y.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            try!(os.write_float(1, v));
        };
        if let Some(v) = self.y {
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
        ::std::any::TypeId::of::<CMsgVector2D>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgVector2D {
    fn new() -> CMsgVector2D {
        CMsgVector2D::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgVector2D>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "x",
                    CMsgVector2D::has_x,
                    CMsgVector2D::get_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "y",
                    CMsgVector2D::has_y,
                    CMsgVector2D::get_y,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgVector2D>(
                    "CMsgVector2D",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgVector2D {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CMsgVector2D {
    fn eq(&self, other: &CMsgVector2D) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CMsgVector2D {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CMsgQAngle {
    // message fields
    x: ::std::option::Option<f32>,
    y: ::std::option::Option<f32>,
    z: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgQAngle {}

impl CMsgQAngle {
    pub fn new() -> CMsgQAngle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgQAngle {
        static mut instance: ::protobuf::lazy::Lazy<CMsgQAngle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgQAngle,
        };
        unsafe {
            instance.get(|| {
                CMsgQAngle {
                    x: ::std::option::Option::None,
                    y: ::std::option::Option::None,
                    z: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional float x = 1;

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: f32) {
        self.x = ::std::option::Option::Some(v);
    }

    pub fn get_x(&self) -> f32 {
        self.x.unwrap_or(0.)
    }

    // optional float y = 2;

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: f32) {
        self.y = ::std::option::Option::Some(v);
    }

    pub fn get_y(&self) -> f32 {
        self.y.unwrap_or(0.)
    }

    // optional float z = 3;

    pub fn clear_z(&mut self) {
        self.z = ::std::option::Option::None;
    }

    pub fn has_z(&self) -> bool {
        self.z.is_some()
    }

    // Param is passed by value, moved
    pub fn set_z(&mut self, v: f32) {
        self.z = ::std::option::Option::Some(v);
    }

    pub fn get_z(&self) -> f32 {
        self.z.unwrap_or(0.)
    }
}

impl ::protobuf::Message for CMsgQAngle {
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
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.y = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.z = ::std::option::Option::Some(tmp);
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
        if self.x.is_some() {
            my_size += 5;
        };
        if self.y.is_some() {
            my_size += 5;
        };
        if self.z.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            try!(os.write_float(1, v));
        };
        if let Some(v) = self.y {
            try!(os.write_float(2, v));
        };
        if let Some(v) = self.z {
            try!(os.write_float(3, v));
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
        ::std::any::TypeId::of::<CMsgQAngle>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgQAngle {
    fn new() -> CMsgQAngle {
        CMsgQAngle::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgQAngle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "x",
                    CMsgQAngle::has_x,
                    CMsgQAngle::get_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "y",
                    CMsgQAngle::has_y,
                    CMsgQAngle::get_y,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "z",
                    CMsgQAngle::has_z,
                    CMsgQAngle::get_z,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgQAngle>(
                    "CMsgQAngle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgQAngle {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.clear_z();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CMsgQAngle {
    fn eq(&self, other: &CMsgQAngle) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CMsgQAngle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CMsg_CVars {
    // message fields
    cvars: ::protobuf::RepeatedField<CMsg_CVars_CVar>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsg_CVars {}

impl CMsg_CVars {
    pub fn new() -> CMsg_CVars {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsg_CVars {
        static mut instance: ::protobuf::lazy::Lazy<CMsg_CVars> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsg_CVars,
        };
        unsafe {
            instance.get(|| {
                CMsg_CVars {
                    cvars: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .CMsg_CVars.CVar cvars = 1;

    pub fn clear_cvars(&mut self) {
        self.cvars.clear();
    }

    // Param is passed by value, moved
    pub fn set_cvars(&mut self, v: ::protobuf::RepeatedField<CMsg_CVars_CVar>) {
        self.cvars = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cvars(&mut self) -> &mut ::protobuf::RepeatedField<CMsg_CVars_CVar> {
        &mut self.cvars
    }

    // Take field
    pub fn take_cvars(&mut self) -> ::protobuf::RepeatedField<CMsg_CVars_CVar> {
        ::std::mem::replace(&mut self.cvars, ::protobuf::RepeatedField::new())
    }

    pub fn get_cvars(&self) -> &[CMsg_CVars_CVar] {
        &self.cvars
    }
}

impl ::protobuf::Message for CMsg_CVars {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.cvars));
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
        for value in &self.cvars {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.cvars {
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
        ::std::any::TypeId::of::<CMsg_CVars>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsg_CVars {
    fn new() -> CMsg_CVars {
        CMsg_CVars::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsg_CVars>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "cvars",
                    CMsg_CVars::get_cvars,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsg_CVars>(
                    "CMsg_CVars",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsg_CVars {
    fn clear(&mut self) {
        self.clear_cvars();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CMsg_CVars {
    fn eq(&self, other: &CMsg_CVars) -> bool {
        self.cvars == other.cvars &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CMsg_CVars {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CMsg_CVars_CVar {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsg_CVars_CVar {}

impl CMsg_CVars_CVar {
    pub fn new() -> CMsg_CVars_CVar {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsg_CVars_CVar {
        static mut instance: ::protobuf::lazy::Lazy<CMsg_CVars_CVar> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsg_CVars_CVar,
        };
        unsafe {
            instance.get(|| {
                CMsg_CVars_CVar {
                    name: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularField::none(),
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

    // optional string value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        self.value.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_value(&self) -> &str {
        match self.value.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for CMsg_CVars_CVar {
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
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.value));
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
        for value in &self.value {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_string(2, &v));
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
        ::std::any::TypeId::of::<CMsg_CVars_CVar>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsg_CVars_CVar {
    fn new() -> CMsg_CVars_CVar {
        CMsg_CVars_CVar::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsg_CVars_CVar>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    CMsg_CVars_CVar::has_name,
                    CMsg_CVars_CVar::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "value",
                    CMsg_CVars_CVar::has_value,
                    CMsg_CVars_CVar::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsg_CVars_CVar>(
                    "CMsg_CVars_CVar",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsg_CVars_CVar {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CMsg_CVars_CVar {
    fn eq(&self, other: &CMsg_CVars_CVar) -> bool {
        self.name == other.name &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CMsg_CVars_CVar {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CNETMsg_NOP {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_NOP {}

impl CNETMsg_NOP {
    pub fn new() -> CNETMsg_NOP {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_NOP {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_NOP> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_NOP,
        };
        unsafe {
            instance.get(|| {
                CNETMsg_NOP {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for CNETMsg_NOP {
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
        ::std::any::TypeId::of::<CNETMsg_NOP>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CNETMsg_NOP {
    fn new() -> CNETMsg_NOP {
        CNETMsg_NOP::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_NOP>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_NOP>(
                    "CNETMsg_NOP",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_NOP {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CNETMsg_NOP {
    fn eq(&self, other: &CNETMsg_NOP) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CNETMsg_NOP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CNETMsg_Disconnect {
    // message fields
    text: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_Disconnect {}

impl CNETMsg_Disconnect {
    pub fn new() -> CNETMsg_Disconnect {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_Disconnect {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_Disconnect> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_Disconnect,
        };
        unsafe {
            instance.get(|| {
                CNETMsg_Disconnect {
                    text: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string text = 1;

    pub fn clear_text(&mut self) {
        self.text.clear();
    }

    pub fn has_text(&self) -> bool {
        self.text.is_some()
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: ::std::string::String) {
        self.text = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_text(&mut self) -> &mut ::std::string::String {
        if self.text.is_none() {
            self.text.set_default();
        };
        self.text.as_mut().unwrap()
    }

    // Take field
    pub fn take_text(&mut self) -> ::std::string::String {
        self.text.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_text(&self) -> &str {
        match self.text.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for CNETMsg_Disconnect {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.text));
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
        for value in &self.text {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.text.as_ref() {
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
        ::std::any::TypeId::of::<CNETMsg_Disconnect>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CNETMsg_Disconnect {
    fn new() -> CNETMsg_Disconnect {
        CNETMsg_Disconnect::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_Disconnect>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "text",
                    CNETMsg_Disconnect::has_text,
                    CNETMsg_Disconnect::get_text,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_Disconnect>(
                    "CNETMsg_Disconnect",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_Disconnect {
    fn clear(&mut self) {
        self.clear_text();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CNETMsg_Disconnect {
    fn eq(&self, other: &CNETMsg_Disconnect) -> bool {
        self.text == other.text &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CNETMsg_Disconnect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CNETMsg_File {
    // message fields
    transfer_id: ::std::option::Option<i32>,
    file_name: ::protobuf::SingularField<::std::string::String>,
    is_replay_demo_file: ::std::option::Option<bool>,
    deny: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_File {}

impl CNETMsg_File {
    pub fn new() -> CNETMsg_File {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_File {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_File> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_File,
        };
        unsafe {
            instance.get(|| {
                CNETMsg_File {
                    transfer_id: ::std::option::Option::None,
                    file_name: ::protobuf::SingularField::none(),
                    is_replay_demo_file: ::std::option::Option::None,
                    deny: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 transfer_id = 1;

    pub fn clear_transfer_id(&mut self) {
        self.transfer_id = ::std::option::Option::None;
    }

    pub fn has_transfer_id(&self) -> bool {
        self.transfer_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transfer_id(&mut self, v: i32) {
        self.transfer_id = ::std::option::Option::Some(v);
    }

    pub fn get_transfer_id(&self) -> i32 {
        self.transfer_id.unwrap_or(0)
    }

    // optional string file_name = 2;

    pub fn clear_file_name(&mut self) {
        self.file_name.clear();
    }

    pub fn has_file_name(&self) -> bool {
        self.file_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_file_name(&mut self, v: ::std::string::String) {
        self.file_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_file_name(&mut self) -> &mut ::std::string::String {
        if self.file_name.is_none() {
            self.file_name.set_default();
        };
        self.file_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_file_name(&mut self) -> ::std::string::String {
        self.file_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_file_name(&self) -> &str {
        match self.file_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bool is_replay_demo_file = 3;

    pub fn clear_is_replay_demo_file(&mut self) {
        self.is_replay_demo_file = ::std::option::Option::None;
    }

    pub fn has_is_replay_demo_file(&self) -> bool {
        self.is_replay_demo_file.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_replay_demo_file(&mut self, v: bool) {
        self.is_replay_demo_file = ::std::option::Option::Some(v);
    }

    pub fn get_is_replay_demo_file(&self) -> bool {
        self.is_replay_demo_file.unwrap_or(false)
    }

    // optional bool deny = 4;

    pub fn clear_deny(&mut self) {
        self.deny = ::std::option::Option::None;
    }

    pub fn has_deny(&self) -> bool {
        self.deny.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deny(&mut self, v: bool) {
        self.deny = ::std::option::Option::Some(v);
    }

    pub fn get_deny(&self) -> bool {
        self.deny.unwrap_or(false)
    }
}

impl ::protobuf::Message for CNETMsg_File {
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
                    self.transfer_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.file_name));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.is_replay_demo_file = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.deny = ::std::option::Option::Some(tmp);
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
        for value in &self.transfer_id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.file_name {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if self.is_replay_demo_file.is_some() {
            my_size += 2;
        };
        if self.deny.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.transfer_id {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.file_name.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.is_replay_demo_file {
            try!(os.write_bool(3, v));
        };
        if let Some(v) = self.deny {
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
        ::std::any::TypeId::of::<CNETMsg_File>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CNETMsg_File {
    fn new() -> CNETMsg_File {
        CNETMsg_File::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_File>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "transfer_id",
                    CNETMsg_File::has_transfer_id,
                    CNETMsg_File::get_transfer_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "file_name",
                    CNETMsg_File::has_file_name,
                    CNETMsg_File::get_file_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "is_replay_demo_file",
                    CNETMsg_File::has_is_replay_demo_file,
                    CNETMsg_File::get_is_replay_demo_file,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "deny",
                    CNETMsg_File::has_deny,
                    CNETMsg_File::get_deny,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_File>(
                    "CNETMsg_File",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_File {
    fn clear(&mut self) {
        self.clear_transfer_id();
        self.clear_file_name();
        self.clear_is_replay_demo_file();
        self.clear_deny();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CNETMsg_File {
    fn eq(&self, other: &CNETMsg_File) -> bool {
        self.transfer_id == other.transfer_id &&
        self.file_name == other.file_name &&
        self.is_replay_demo_file == other.is_replay_demo_file &&
        self.deny == other.deny &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CNETMsg_File {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CNETMsg_SplitScreenUser {
    // message fields
    slot: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_SplitScreenUser {}

impl CNETMsg_SplitScreenUser {
    pub fn new() -> CNETMsg_SplitScreenUser {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_SplitScreenUser {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_SplitScreenUser> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_SplitScreenUser,
        };
        unsafe {
            instance.get(|| {
                CNETMsg_SplitScreenUser {
                    slot: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 slot = 1;

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
}

impl ::protobuf::Message for CNETMsg_SplitScreenUser {
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
                    self.slot = ::std::option::Option::Some(tmp);
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
        for value in &self.slot {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.slot {
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
        ::std::any::TypeId::of::<CNETMsg_SplitScreenUser>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CNETMsg_SplitScreenUser {
    fn new() -> CNETMsg_SplitScreenUser {
        CNETMsg_SplitScreenUser::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_SplitScreenUser>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "slot",
                    CNETMsg_SplitScreenUser::has_slot,
                    CNETMsg_SplitScreenUser::get_slot,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_SplitScreenUser>(
                    "CNETMsg_SplitScreenUser",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_SplitScreenUser {
    fn clear(&mut self) {
        self.clear_slot();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CNETMsg_SplitScreenUser {
    fn eq(&self, other: &CNETMsg_SplitScreenUser) -> bool {
        self.slot == other.slot &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CNETMsg_SplitScreenUser {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CNETMsg_Tick {
    // message fields
    tick: ::std::option::Option<u32>,
    host_frametime: ::std::option::Option<u32>,
    host_frametime_std_deviation: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_Tick {}

impl CNETMsg_Tick {
    pub fn new() -> CNETMsg_Tick {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_Tick {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_Tick> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_Tick,
        };
        unsafe {
            instance.get(|| {
                CNETMsg_Tick {
                    tick: ::std::option::Option::None,
                    host_frametime: ::std::option::Option::None,
                    host_frametime_std_deviation: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 tick = 1;

    pub fn clear_tick(&mut self) {
        self.tick = ::std::option::Option::None;
    }

    pub fn has_tick(&self) -> bool {
        self.tick.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tick(&mut self, v: u32) {
        self.tick = ::std::option::Option::Some(v);
    }

    pub fn get_tick(&self) -> u32 {
        self.tick.unwrap_or(0)
    }

    // optional uint32 host_frametime = 2;

    pub fn clear_host_frametime(&mut self) {
        self.host_frametime = ::std::option::Option::None;
    }

    pub fn has_host_frametime(&self) -> bool {
        self.host_frametime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_host_frametime(&mut self, v: u32) {
        self.host_frametime = ::std::option::Option::Some(v);
    }

    pub fn get_host_frametime(&self) -> u32 {
        self.host_frametime.unwrap_or(0)
    }

    // optional uint32 host_frametime_std_deviation = 3;

    pub fn clear_host_frametime_std_deviation(&mut self) {
        self.host_frametime_std_deviation = ::std::option::Option::None;
    }

    pub fn has_host_frametime_std_deviation(&self) -> bool {
        self.host_frametime_std_deviation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_host_frametime_std_deviation(&mut self, v: u32) {
        self.host_frametime_std_deviation = ::std::option::Option::Some(v);
    }

    pub fn get_host_frametime_std_deviation(&self) -> u32 {
        self.host_frametime_std_deviation.unwrap_or(0)
    }
}

impl ::protobuf::Message for CNETMsg_Tick {
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
                    self.tick = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.host_frametime = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.host_frametime_std_deviation = ::std::option::Option::Some(tmp);
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
        for value in &self.tick {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.host_frametime {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.host_frametime_std_deviation {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tick {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.host_frametime {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.host_frametime_std_deviation {
            try!(os.write_uint32(3, v));
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
        ::std::any::TypeId::of::<CNETMsg_Tick>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CNETMsg_Tick {
    fn new() -> CNETMsg_Tick {
        CNETMsg_Tick::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_Tick>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "tick",
                    CNETMsg_Tick::has_tick,
                    CNETMsg_Tick::get_tick,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "host_frametime",
                    CNETMsg_Tick::has_host_frametime,
                    CNETMsg_Tick::get_host_frametime,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "host_frametime_std_deviation",
                    CNETMsg_Tick::has_host_frametime_std_deviation,
                    CNETMsg_Tick::get_host_frametime_std_deviation,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_Tick>(
                    "CNETMsg_Tick",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_Tick {
    fn clear(&mut self) {
        self.clear_tick();
        self.clear_host_frametime();
        self.clear_host_frametime_std_deviation();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CNETMsg_Tick {
    fn eq(&self, other: &CNETMsg_Tick) -> bool {
        self.tick == other.tick &&
        self.host_frametime == other.host_frametime &&
        self.host_frametime_std_deviation == other.host_frametime_std_deviation &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CNETMsg_Tick {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CNETMsg_StringCmd {
    // message fields
    command: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_StringCmd {}

impl CNETMsg_StringCmd {
    pub fn new() -> CNETMsg_StringCmd {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_StringCmd {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_StringCmd> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_StringCmd,
        };
        unsafe {
            instance.get(|| {
                CNETMsg_StringCmd {
                    command: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string command = 1;

    pub fn clear_command(&mut self) {
        self.command.clear();
    }

    pub fn has_command(&self) -> bool {
        self.command.is_some()
    }

    // Param is passed by value, moved
    pub fn set_command(&mut self, v: ::std::string::String) {
        self.command = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_command(&mut self) -> &mut ::std::string::String {
        if self.command.is_none() {
            self.command.set_default();
        };
        self.command.as_mut().unwrap()
    }

    // Take field
    pub fn take_command(&mut self) -> ::std::string::String {
        self.command.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_command(&self) -> &str {
        match self.command.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for CNETMsg_StringCmd {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.command));
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
        for value in &self.command {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.command.as_ref() {
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
        ::std::any::TypeId::of::<CNETMsg_StringCmd>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CNETMsg_StringCmd {
    fn new() -> CNETMsg_StringCmd {
        CNETMsg_StringCmd::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_StringCmd>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "command",
                    CNETMsg_StringCmd::has_command,
                    CNETMsg_StringCmd::get_command,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_StringCmd>(
                    "CNETMsg_StringCmd",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_StringCmd {
    fn clear(&mut self) {
        self.clear_command();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CNETMsg_StringCmd {
    fn eq(&self, other: &CNETMsg_StringCmd) -> bool {
        self.command == other.command &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CNETMsg_StringCmd {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CNETMsg_SetConVar {
    // message fields
    convars: ::protobuf::SingularPtrField<CMsg_CVars>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_SetConVar {}

impl CNETMsg_SetConVar {
    pub fn new() -> CNETMsg_SetConVar {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_SetConVar {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_SetConVar> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_SetConVar,
        };
        unsafe {
            instance.get(|| {
                CNETMsg_SetConVar {
                    convars: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .CMsg_CVars convars = 1;

    pub fn clear_convars(&mut self) {
        self.convars.clear();
    }

    pub fn has_convars(&self) -> bool {
        self.convars.is_some()
    }

    // Param is passed by value, moved
    pub fn set_convars(&mut self, v: CMsg_CVars) {
        self.convars = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_convars(&mut self) -> &mut CMsg_CVars {
        if self.convars.is_none() {
            self.convars.set_default();
        };
        self.convars.as_mut().unwrap()
    }

    // Take field
    pub fn take_convars(&mut self) -> CMsg_CVars {
        self.convars.take().unwrap_or_else(|| CMsg_CVars::new())
    }

    pub fn get_convars(&self) -> &CMsg_CVars {
        self.convars.as_ref().unwrap_or_else(|| CMsg_CVars::default_instance())
    }
}

impl ::protobuf::Message for CNETMsg_SetConVar {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.convars));
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
        for value in &self.convars {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.convars.as_ref() {
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
        ::std::any::TypeId::of::<CNETMsg_SetConVar>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CNETMsg_SetConVar {
    fn new() -> CNETMsg_SetConVar {
        CNETMsg_SetConVar::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_SetConVar>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "convars",
                    CNETMsg_SetConVar::has_convars,
                    CNETMsg_SetConVar::get_convars,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_SetConVar>(
                    "CNETMsg_SetConVar",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_SetConVar {
    fn clear(&mut self) {
        self.clear_convars();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CNETMsg_SetConVar {
    fn eq(&self, other: &CNETMsg_SetConVar) -> bool {
        self.convars == other.convars &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CNETMsg_SetConVar {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CNETMsg_SignonState {
    // message fields
    signon_state: ::std::option::Option<u32>,
    spawn_count: ::std::option::Option<u32>,
    num_server_players: ::std::option::Option<u32>,
    players_networkids: ::protobuf::RepeatedField<::std::string::String>,
    map_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_SignonState {}

impl CNETMsg_SignonState {
    pub fn new() -> CNETMsg_SignonState {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_SignonState {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_SignonState> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_SignonState,
        };
        unsafe {
            instance.get(|| {
                CNETMsg_SignonState {
                    signon_state: ::std::option::Option::None,
                    spawn_count: ::std::option::Option::None,
                    num_server_players: ::std::option::Option::None,
                    players_networkids: ::protobuf::RepeatedField::new(),
                    map_name: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 signon_state = 1;

    pub fn clear_signon_state(&mut self) {
        self.signon_state = ::std::option::Option::None;
    }

    pub fn has_signon_state(&self) -> bool {
        self.signon_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signon_state(&mut self, v: u32) {
        self.signon_state = ::std::option::Option::Some(v);
    }

    pub fn get_signon_state(&self) -> u32 {
        self.signon_state.unwrap_or(0)
    }

    // optional uint32 spawn_count = 2;

    pub fn clear_spawn_count(&mut self) {
        self.spawn_count = ::std::option::Option::None;
    }

    pub fn has_spawn_count(&self) -> bool {
        self.spawn_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spawn_count(&mut self, v: u32) {
        self.spawn_count = ::std::option::Option::Some(v);
    }

    pub fn get_spawn_count(&self) -> u32 {
        self.spawn_count.unwrap_or(0)
    }

    // optional uint32 num_server_players = 3;

    pub fn clear_num_server_players(&mut self) {
        self.num_server_players = ::std::option::Option::None;
    }

    pub fn has_num_server_players(&self) -> bool {
        self.num_server_players.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_server_players(&mut self, v: u32) {
        self.num_server_players = ::std::option::Option::Some(v);
    }

    pub fn get_num_server_players(&self) -> u32 {
        self.num_server_players.unwrap_or(0)
    }

    // repeated string players_networkids = 4;

    pub fn clear_players_networkids(&mut self) {
        self.players_networkids.clear();
    }

    // Param is passed by value, moved
    pub fn set_players_networkids(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.players_networkids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_players_networkids(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.players_networkids
    }

    // Take field
    pub fn take_players_networkids(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.players_networkids, ::protobuf::RepeatedField::new())
    }

    pub fn get_players_networkids(&self) -> &[::std::string::String] {
        &self.players_networkids
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
}

impl ::protobuf::Message for CNETMsg_SignonState {
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
                    self.signon_state = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.spawn_count = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.num_server_players = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.players_networkids));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.map_name));
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
        for value in &self.signon_state {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.spawn_count {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.num_server_players {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.players_networkids {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in &self.map_name {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.signon_state {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.spawn_count {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.num_server_players {
            try!(os.write_uint32(3, v));
        };
        for v in &self.players_networkids {
            try!(os.write_string(4, &v));
        };
        if let Some(v) = self.map_name.as_ref() {
            try!(os.write_string(5, &v));
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
        ::std::any::TypeId::of::<CNETMsg_SignonState>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CNETMsg_SignonState {
    fn new() -> CNETMsg_SignonState {
        CNETMsg_SignonState::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_SignonState>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "signon_state",
                    CNETMsg_SignonState::has_signon_state,
                    CNETMsg_SignonState::get_signon_state,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "spawn_count",
                    CNETMsg_SignonState::has_spawn_count,
                    CNETMsg_SignonState::get_spawn_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "num_server_players",
                    CNETMsg_SignonState::has_num_server_players,
                    CNETMsg_SignonState::get_num_server_players,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "players_networkids",
                    CNETMsg_SignonState::get_players_networkids,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "map_name",
                    CNETMsg_SignonState::has_map_name,
                    CNETMsg_SignonState::get_map_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_SignonState>(
                    "CNETMsg_SignonState",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_SignonState {
    fn clear(&mut self) {
        self.clear_signon_state();
        self.clear_spawn_count();
        self.clear_num_server_players();
        self.clear_players_networkids();
        self.clear_map_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CNETMsg_SignonState {
    fn eq(&self, other: &CNETMsg_SignonState) -> bool {
        self.signon_state == other.signon_state &&
        self.spawn_count == other.spawn_count &&
        self.num_server_players == other.num_server_players &&
        self.players_networkids == other.players_networkids &&
        self.map_name == other.map_name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CNETMsg_SignonState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CCLCMsg_ClientInfo {
    // message fields
    send_table_crc: ::std::option::Option<u32>,
    server_count: ::std::option::Option<u32>,
    is_hltv: ::std::option::Option<bool>,
    is_replay: ::std::option::Option<bool>,
    friends_id: ::std::option::Option<u32>,
    friends_name: ::protobuf::SingularField<::std::string::String>,
    custom_files: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_ClientInfo {}

impl CCLCMsg_ClientInfo {
    pub fn new() -> CCLCMsg_ClientInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_ClientInfo {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_ClientInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_ClientInfo,
        };
        unsafe {
            instance.get(|| {
                CCLCMsg_ClientInfo {
                    send_table_crc: ::std::option::Option::None,
                    server_count: ::std::option::Option::None,
                    is_hltv: ::std::option::Option::None,
                    is_replay: ::std::option::Option::None,
                    friends_id: ::std::option::Option::None,
                    friends_name: ::protobuf::SingularField::none(),
                    custom_files: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional fixed32 send_table_crc = 1;

    pub fn clear_send_table_crc(&mut self) {
        self.send_table_crc = ::std::option::Option::None;
    }

    pub fn has_send_table_crc(&self) -> bool {
        self.send_table_crc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_send_table_crc(&mut self, v: u32) {
        self.send_table_crc = ::std::option::Option::Some(v);
    }

    pub fn get_send_table_crc(&self) -> u32 {
        self.send_table_crc.unwrap_or(0)
    }

    // optional uint32 server_count = 2;

    pub fn clear_server_count(&mut self) {
        self.server_count = ::std::option::Option::None;
    }

    pub fn has_server_count(&self) -> bool {
        self.server_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_count(&mut self, v: u32) {
        self.server_count = ::std::option::Option::Some(v);
    }

    pub fn get_server_count(&self) -> u32 {
        self.server_count.unwrap_or(0)
    }

    // optional bool is_hltv = 3;

    pub fn clear_is_hltv(&mut self) {
        self.is_hltv = ::std::option::Option::None;
    }

    pub fn has_is_hltv(&self) -> bool {
        self.is_hltv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_hltv(&mut self, v: bool) {
        self.is_hltv = ::std::option::Option::Some(v);
    }

    pub fn get_is_hltv(&self) -> bool {
        self.is_hltv.unwrap_or(false)
    }

    // optional bool is_replay = 4;

    pub fn clear_is_replay(&mut self) {
        self.is_replay = ::std::option::Option::None;
    }

    pub fn has_is_replay(&self) -> bool {
        self.is_replay.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_replay(&mut self, v: bool) {
        self.is_replay = ::std::option::Option::Some(v);
    }

    pub fn get_is_replay(&self) -> bool {
        self.is_replay.unwrap_or(false)
    }

    // optional uint32 friends_id = 5;

    pub fn clear_friends_id(&mut self) {
        self.friends_id = ::std::option::Option::None;
    }

    pub fn has_friends_id(&self) -> bool {
        self.friends_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_friends_id(&mut self, v: u32) {
        self.friends_id = ::std::option::Option::Some(v);
    }

    pub fn get_friends_id(&self) -> u32 {
        self.friends_id.unwrap_or(0)
    }

    // optional string friends_name = 6;

    pub fn clear_friends_name(&mut self) {
        self.friends_name.clear();
    }

    pub fn has_friends_name(&self) -> bool {
        self.friends_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_friends_name(&mut self, v: ::std::string::String) {
        self.friends_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_friends_name(&mut self) -> &mut ::std::string::String {
        if self.friends_name.is_none() {
            self.friends_name.set_default();
        };
        self.friends_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_friends_name(&mut self) -> ::std::string::String {
        self.friends_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_friends_name(&self) -> &str {
        match self.friends_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated fixed32 custom_files = 7;

    pub fn clear_custom_files(&mut self) {
        self.custom_files.clear();
    }

    // Param is passed by value, moved
    pub fn set_custom_files(&mut self, v: ::std::vec::Vec<u32>) {
        self.custom_files = v;
    }

    // Mutable pointer to the field.
    pub fn mut_custom_files(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.custom_files
    }

    // Take field
    pub fn take_custom_files(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.custom_files, ::std::vec::Vec::new())
    }

    pub fn get_custom_files(&self) -> &[u32] {
        &self.custom_files
    }
}

impl ::protobuf::Message for CCLCMsg_ClientInfo {
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
                    let tmp = try!(is.read_fixed32());
                    self.send_table_crc = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.server_count = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.is_hltv = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.is_replay = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.friends_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.friends_name));
                },
                7 => {
                    try!(::protobuf::rt::read_repeated_fixed32_into(wire_type, is, &mut self.custom_files));
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
        if self.send_table_crc.is_some() {
            my_size += 5;
        };
        for value in &self.server_count {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.is_hltv.is_some() {
            my_size += 2;
        };
        if self.is_replay.is_some() {
            my_size += 2;
        };
        for value in &self.friends_id {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.friends_name {
            my_size += ::protobuf::rt::string_size(6, &value);
        };
        my_size += 5 * self.custom_files.len() as u32;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.send_table_crc {
            try!(os.write_fixed32(1, v));
        };
        if let Some(v) = self.server_count {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.is_hltv {
            try!(os.write_bool(3, v));
        };
        if let Some(v) = self.is_replay {
            try!(os.write_bool(4, v));
        };
        if let Some(v) = self.friends_id {
            try!(os.write_uint32(5, v));
        };
        if let Some(v) = self.friends_name.as_ref() {
            try!(os.write_string(6, &v));
        };
        for v in &self.custom_files {
            try!(os.write_fixed32(7, *v));
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
        ::std::any::TypeId::of::<CCLCMsg_ClientInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CCLCMsg_ClientInfo {
    fn new() -> CCLCMsg_ClientInfo {
        CCLCMsg_ClientInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_ClientInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "send_table_crc",
                    CCLCMsg_ClientInfo::has_send_table_crc,
                    CCLCMsg_ClientInfo::get_send_table_crc,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "server_count",
                    CCLCMsg_ClientInfo::has_server_count,
                    CCLCMsg_ClientInfo::get_server_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "is_hltv",
                    CCLCMsg_ClientInfo::has_is_hltv,
                    CCLCMsg_ClientInfo::get_is_hltv,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "is_replay",
                    CCLCMsg_ClientInfo::has_is_replay,
                    CCLCMsg_ClientInfo::get_is_replay,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "friends_id",
                    CCLCMsg_ClientInfo::has_friends_id,
                    CCLCMsg_ClientInfo::get_friends_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "friends_name",
                    CCLCMsg_ClientInfo::has_friends_name,
                    CCLCMsg_ClientInfo::get_friends_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_u32_accessor(
                    "custom_files",
                    CCLCMsg_ClientInfo::get_custom_files,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_ClientInfo>(
                    "CCLCMsg_ClientInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_ClientInfo {
    fn clear(&mut self) {
        self.clear_send_table_crc();
        self.clear_server_count();
        self.clear_is_hltv();
        self.clear_is_replay();
        self.clear_friends_id();
        self.clear_friends_name();
        self.clear_custom_files();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CCLCMsg_ClientInfo {
    fn eq(&self, other: &CCLCMsg_ClientInfo) -> bool {
        self.send_table_crc == other.send_table_crc &&
        self.server_count == other.server_count &&
        self.is_hltv == other.is_hltv &&
        self.is_replay == other.is_replay &&
        self.friends_id == other.friends_id &&
        self.friends_name == other.friends_name &&
        self.custom_files == other.custom_files &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CCLCMsg_ClientInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CCLCMsg_Move {
    // message fields
    num_backup_commands: ::std::option::Option<u32>,
    num_new_commands: ::std::option::Option<u32>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_Move {}

impl CCLCMsg_Move {
    pub fn new() -> CCLCMsg_Move {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_Move {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_Move> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_Move,
        };
        unsafe {
            instance.get(|| {
                CCLCMsg_Move {
                    num_backup_commands: ::std::option::Option::None,
                    num_new_commands: ::std::option::Option::None,
                    data: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 num_backup_commands = 1;

    pub fn clear_num_backup_commands(&mut self) {
        self.num_backup_commands = ::std::option::Option::None;
    }

    pub fn has_num_backup_commands(&self) -> bool {
        self.num_backup_commands.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_backup_commands(&mut self, v: u32) {
        self.num_backup_commands = ::std::option::Option::Some(v);
    }

    pub fn get_num_backup_commands(&self) -> u32 {
        self.num_backup_commands.unwrap_or(0)
    }

    // optional uint32 num_new_commands = 2;

    pub fn clear_num_new_commands(&mut self) {
        self.num_new_commands = ::std::option::Option::None;
    }

    pub fn has_num_new_commands(&self) -> bool {
        self.num_new_commands.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_new_commands(&mut self, v: u32) {
        self.num_new_commands = ::std::option::Option::Some(v);
    }

    pub fn get_num_new_commands(&self) -> u32 {
        self.num_new_commands.unwrap_or(0)
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

impl ::protobuf::Message for CCLCMsg_Move {
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
                    self.num_backup_commands = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.num_new_commands = ::std::option::Option::Some(tmp);
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
        for value in &self.num_backup_commands {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.num_new_commands {
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
        if let Some(v) = self.num_backup_commands {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.num_new_commands {
            try!(os.write_uint32(2, v));
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
        ::std::any::TypeId::of::<CCLCMsg_Move>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CCLCMsg_Move {
    fn new() -> CCLCMsg_Move {
        CCLCMsg_Move::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_Move>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "num_backup_commands",
                    CCLCMsg_Move::has_num_backup_commands,
                    CCLCMsg_Move::get_num_backup_commands,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "num_new_commands",
                    CCLCMsg_Move::has_num_new_commands,
                    CCLCMsg_Move::get_num_new_commands,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "data",
                    CCLCMsg_Move::has_data,
                    CCLCMsg_Move::get_data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_Move>(
                    "CCLCMsg_Move",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_Move {
    fn clear(&mut self) {
        self.clear_num_backup_commands();
        self.clear_num_new_commands();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CCLCMsg_Move {
    fn eq(&self, other: &CCLCMsg_Move) -> bool {
        self.num_backup_commands == other.num_backup_commands &&
        self.num_new_commands == other.num_new_commands &&
        self.data == other.data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CCLCMsg_Move {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CCLCMsg_VoiceData {
    // message fields
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    xuid: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_VoiceData {}

impl CCLCMsg_VoiceData {
    pub fn new() -> CCLCMsg_VoiceData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_VoiceData {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_VoiceData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_VoiceData,
        };
        unsafe {
            instance.get(|| {
                CCLCMsg_VoiceData {
                    data: ::protobuf::SingularField::none(),
                    xuid: ::std::option::Option::None,
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

    // optional fixed64 xuid = 2;

    pub fn clear_xuid(&mut self) {
        self.xuid = ::std::option::Option::None;
    }

    pub fn has_xuid(&self) -> bool {
        self.xuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_xuid(&mut self, v: u64) {
        self.xuid = ::std::option::Option::Some(v);
    }

    pub fn get_xuid(&self) -> u64 {
        self.xuid.unwrap_or(0)
    }
}

impl ::protobuf::Message for CCLCMsg_VoiceData {
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
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.xuid = ::std::option::Option::Some(tmp);
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
        if self.xuid.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.data.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.xuid {
            try!(os.write_fixed64(2, v));
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
        ::std::any::TypeId::of::<CCLCMsg_VoiceData>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CCLCMsg_VoiceData {
    fn new() -> CCLCMsg_VoiceData {
        CCLCMsg_VoiceData::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_VoiceData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "data",
                    CCLCMsg_VoiceData::has_data,
                    CCLCMsg_VoiceData::get_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "xuid",
                    CCLCMsg_VoiceData::has_xuid,
                    CCLCMsg_VoiceData::get_xuid,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_VoiceData>(
                    "CCLCMsg_VoiceData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_VoiceData {
    fn clear(&mut self) {
        self.clear_data();
        self.clear_xuid();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CCLCMsg_VoiceData {
    fn eq(&self, other: &CCLCMsg_VoiceData) -> bool {
        self.data == other.data &&
        self.xuid == other.xuid &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CCLCMsg_VoiceData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CCLCMsg_BaselineAck {
    // message fields
    baseline_tick: ::std::option::Option<i32>,
    baseline_nr: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_BaselineAck {}

impl CCLCMsg_BaselineAck {
    pub fn new() -> CCLCMsg_BaselineAck {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_BaselineAck {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_BaselineAck> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_BaselineAck,
        };
        unsafe {
            instance.get(|| {
                CCLCMsg_BaselineAck {
                    baseline_tick: ::std::option::Option::None,
                    baseline_nr: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 baseline_tick = 1;

    pub fn clear_baseline_tick(&mut self) {
        self.baseline_tick = ::std::option::Option::None;
    }

    pub fn has_baseline_tick(&self) -> bool {
        self.baseline_tick.is_some()
    }

    // Param is passed by value, moved
    pub fn set_baseline_tick(&mut self, v: i32) {
        self.baseline_tick = ::std::option::Option::Some(v);
    }

    pub fn get_baseline_tick(&self) -> i32 {
        self.baseline_tick.unwrap_or(0)
    }

    // optional int32 baseline_nr = 2;

    pub fn clear_baseline_nr(&mut self) {
        self.baseline_nr = ::std::option::Option::None;
    }

    pub fn has_baseline_nr(&self) -> bool {
        self.baseline_nr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_baseline_nr(&mut self, v: i32) {
        self.baseline_nr = ::std::option::Option::Some(v);
    }

    pub fn get_baseline_nr(&self) -> i32 {
        self.baseline_nr.unwrap_or(0)
    }
}

impl ::protobuf::Message for CCLCMsg_BaselineAck {
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
                    self.baseline_tick = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.baseline_nr = ::std::option::Option::Some(tmp);
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
        for value in &self.baseline_tick {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.baseline_nr {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.baseline_tick {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.baseline_nr {
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
        ::std::any::TypeId::of::<CCLCMsg_BaselineAck>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CCLCMsg_BaselineAck {
    fn new() -> CCLCMsg_BaselineAck {
        CCLCMsg_BaselineAck::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_BaselineAck>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "baseline_tick",
                    CCLCMsg_BaselineAck::has_baseline_tick,
                    CCLCMsg_BaselineAck::get_baseline_tick,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "baseline_nr",
                    CCLCMsg_BaselineAck::has_baseline_nr,
                    CCLCMsg_BaselineAck::get_baseline_nr,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_BaselineAck>(
                    "CCLCMsg_BaselineAck",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_BaselineAck {
    fn clear(&mut self) {
        self.clear_baseline_tick();
        self.clear_baseline_nr();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CCLCMsg_BaselineAck {
    fn eq(&self, other: &CCLCMsg_BaselineAck) -> bool {
        self.baseline_tick == other.baseline_tick &&
        self.baseline_nr == other.baseline_nr &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CCLCMsg_BaselineAck {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CCLCMsg_ListenEvents {
    // message fields
    event_mask: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_ListenEvents {}

impl CCLCMsg_ListenEvents {
    pub fn new() -> CCLCMsg_ListenEvents {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_ListenEvents {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_ListenEvents> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_ListenEvents,
        };
        unsafe {
            instance.get(|| {
                CCLCMsg_ListenEvents {
                    event_mask: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated fixed32 event_mask = 1;

    pub fn clear_event_mask(&mut self) {
        self.event_mask.clear();
    }

    // Param is passed by value, moved
    pub fn set_event_mask(&mut self, v: ::std::vec::Vec<u32>) {
        self.event_mask = v;
    }

    // Mutable pointer to the field.
    pub fn mut_event_mask(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.event_mask
    }

    // Take field
    pub fn take_event_mask(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.event_mask, ::std::vec::Vec::new())
    }

    pub fn get_event_mask(&self) -> &[u32] {
        &self.event_mask
    }
}

impl ::protobuf::Message for CCLCMsg_ListenEvents {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_fixed32_into(wire_type, is, &mut self.event_mask));
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
        my_size += 5 * self.event_mask.len() as u32;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.event_mask {
            try!(os.write_fixed32(1, *v));
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
        ::std::any::TypeId::of::<CCLCMsg_ListenEvents>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CCLCMsg_ListenEvents {
    fn new() -> CCLCMsg_ListenEvents {
        CCLCMsg_ListenEvents::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_ListenEvents>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_u32_accessor(
                    "event_mask",
                    CCLCMsg_ListenEvents::get_event_mask,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_ListenEvents>(
                    "CCLCMsg_ListenEvents",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_ListenEvents {
    fn clear(&mut self) {
        self.clear_event_mask();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CCLCMsg_ListenEvents {
    fn eq(&self, other: &CCLCMsg_ListenEvents) -> bool {
        self.event_mask == other.event_mask &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CCLCMsg_ListenEvents {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CCLCMsg_RespondCvarValue {
    // message fields
    cookie: ::std::option::Option<i32>,
    status_code: ::std::option::Option<i32>,
    name: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_RespondCvarValue {}

impl CCLCMsg_RespondCvarValue {
    pub fn new() -> CCLCMsg_RespondCvarValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_RespondCvarValue {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_RespondCvarValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_RespondCvarValue,
        };
        unsafe {
            instance.get(|| {
                CCLCMsg_RespondCvarValue {
                    cookie: ::std::option::Option::None,
                    status_code: ::std::option::Option::None,
                    name: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 cookie = 1;

    pub fn clear_cookie(&mut self) {
        self.cookie = ::std::option::Option::None;
    }

    pub fn has_cookie(&self) -> bool {
        self.cookie.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cookie(&mut self, v: i32) {
        self.cookie = ::std::option::Option::Some(v);
    }

    pub fn get_cookie(&self) -> i32 {
        self.cookie.unwrap_or(0)
    }

    // optional int32 status_code = 2;

    pub fn clear_status_code(&mut self) {
        self.status_code = ::std::option::Option::None;
    }

    pub fn has_status_code(&self) -> bool {
        self.status_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status_code(&mut self, v: i32) {
        self.status_code = ::std::option::Option::Some(v);
    }

    pub fn get_status_code(&self) -> i32 {
        self.status_code.unwrap_or(0)
    }

    // optional string name = 3;

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

    // optional string value = 4;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        self.value.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_value(&self) -> &str {
        match self.value.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for CCLCMsg_RespondCvarValue {
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
                    self.cookie = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.status_code = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.value));
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
        for value in &self.cookie {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.status_code {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.value {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cookie {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.status_code {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.value.as_ref() {
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
        ::std::any::TypeId::of::<CCLCMsg_RespondCvarValue>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CCLCMsg_RespondCvarValue {
    fn new() -> CCLCMsg_RespondCvarValue {
        CCLCMsg_RespondCvarValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_RespondCvarValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "cookie",
                    CCLCMsg_RespondCvarValue::has_cookie,
                    CCLCMsg_RespondCvarValue::get_cookie,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "status_code",
                    CCLCMsg_RespondCvarValue::has_status_code,
                    CCLCMsg_RespondCvarValue::get_status_code,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    CCLCMsg_RespondCvarValue::has_name,
                    CCLCMsg_RespondCvarValue::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "value",
                    CCLCMsg_RespondCvarValue::has_value,
                    CCLCMsg_RespondCvarValue::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_RespondCvarValue>(
                    "CCLCMsg_RespondCvarValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_RespondCvarValue {
    fn clear(&mut self) {
        self.clear_cookie();
        self.clear_status_code();
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CCLCMsg_RespondCvarValue {
    fn eq(&self, other: &CCLCMsg_RespondCvarValue) -> bool {
        self.cookie == other.cookie &&
        self.status_code == other.status_code &&
        self.name == other.name &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CCLCMsg_RespondCvarValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CCLCMsg_FileCRCCheck {
    // message fields
    code_path: ::std::option::Option<i32>,
    path: ::protobuf::SingularField<::std::string::String>,
    code_filename: ::std::option::Option<i32>,
    filename: ::protobuf::SingularField<::std::string::String>,
    crc: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_FileCRCCheck {}

impl CCLCMsg_FileCRCCheck {
    pub fn new() -> CCLCMsg_FileCRCCheck {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_FileCRCCheck {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_FileCRCCheck> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_FileCRCCheck,
        };
        unsafe {
            instance.get(|| {
                CCLCMsg_FileCRCCheck {
                    code_path: ::std::option::Option::None,
                    path: ::protobuf::SingularField::none(),
                    code_filename: ::std::option::Option::None,
                    filename: ::protobuf::SingularField::none(),
                    crc: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 code_path = 1;

    pub fn clear_code_path(&mut self) {
        self.code_path = ::std::option::Option::None;
    }

    pub fn has_code_path(&self) -> bool {
        self.code_path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_code_path(&mut self, v: i32) {
        self.code_path = ::std::option::Option::Some(v);
    }

    pub fn get_code_path(&self) -> i32 {
        self.code_path.unwrap_or(0)
    }

    // optional string path = 2;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        if self.path.is_none() {
            self.path.set_default();
        };
        self.path.as_mut().unwrap()
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        self.path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        match self.path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional int32 code_filename = 3;

    pub fn clear_code_filename(&mut self) {
        self.code_filename = ::std::option::Option::None;
    }

    pub fn has_code_filename(&self) -> bool {
        self.code_filename.is_some()
    }

    // Param is passed by value, moved
    pub fn set_code_filename(&mut self, v: i32) {
        self.code_filename = ::std::option::Option::Some(v);
    }

    pub fn get_code_filename(&self) -> i32 {
        self.code_filename.unwrap_or(0)
    }

    // optional string filename = 4;

    pub fn clear_filename(&mut self) {
        self.filename.clear();
    }

    pub fn has_filename(&self) -> bool {
        self.filename.is_some()
    }

    // Param is passed by value, moved
    pub fn set_filename(&mut self, v: ::std::string::String) {
        self.filename = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_filename(&mut self) -> &mut ::std::string::String {
        if self.filename.is_none() {
            self.filename.set_default();
        };
        self.filename.as_mut().unwrap()
    }

    // Take field
    pub fn take_filename(&mut self) -> ::std::string::String {
        self.filename.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_filename(&self) -> &str {
        match self.filename.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional fixed32 crc = 5;

    pub fn clear_crc(&mut self) {
        self.crc = ::std::option::Option::None;
    }

    pub fn has_crc(&self) -> bool {
        self.crc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_crc(&mut self, v: u32) {
        self.crc = ::std::option::Option::Some(v);
    }

    pub fn get_crc(&self) -> u32 {
        self.crc.unwrap_or(0)
    }
}

impl ::protobuf::Message for CCLCMsg_FileCRCCheck {
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
                    self.code_path = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.code_filename = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.filename));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed32());
                    self.crc = ::std::option::Option::Some(tmp);
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
        for value in &self.code_path {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.path {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.code_filename {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.filename {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        if self.crc.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.code_path {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.path.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.code_filename {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.filename.as_ref() {
            try!(os.write_string(4, &v));
        };
        if let Some(v) = self.crc {
            try!(os.write_fixed32(5, v));
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
        ::std::any::TypeId::of::<CCLCMsg_FileCRCCheck>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CCLCMsg_FileCRCCheck {
    fn new() -> CCLCMsg_FileCRCCheck {
        CCLCMsg_FileCRCCheck::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_FileCRCCheck>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "code_path",
                    CCLCMsg_FileCRCCheck::has_code_path,
                    CCLCMsg_FileCRCCheck::get_code_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path",
                    CCLCMsg_FileCRCCheck::has_path,
                    CCLCMsg_FileCRCCheck::get_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "code_filename",
                    CCLCMsg_FileCRCCheck::has_code_filename,
                    CCLCMsg_FileCRCCheck::get_code_filename,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "filename",
                    CCLCMsg_FileCRCCheck::has_filename,
                    CCLCMsg_FileCRCCheck::get_filename,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "crc",
                    CCLCMsg_FileCRCCheck::has_crc,
                    CCLCMsg_FileCRCCheck::get_crc,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_FileCRCCheck>(
                    "CCLCMsg_FileCRCCheck",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_FileCRCCheck {
    fn clear(&mut self) {
        self.clear_code_path();
        self.clear_path();
        self.clear_code_filename();
        self.clear_filename();
        self.clear_crc();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CCLCMsg_FileCRCCheck {
    fn eq(&self, other: &CCLCMsg_FileCRCCheck) -> bool {
        self.code_path == other.code_path &&
        self.path == other.path &&
        self.code_filename == other.code_filename &&
        self.filename == other.filename &&
        self.crc == other.crc &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CCLCMsg_FileCRCCheck {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CCLCMsg_LoadingProgress {
    // message fields
    progress: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_LoadingProgress {}

impl CCLCMsg_LoadingProgress {
    pub fn new() -> CCLCMsg_LoadingProgress {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_LoadingProgress {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_LoadingProgress> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_LoadingProgress,
        };
        unsafe {
            instance.get(|| {
                CCLCMsg_LoadingProgress {
                    progress: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 progress = 1;

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

impl ::protobuf::Message for CCLCMsg_LoadingProgress {
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
        for value in &self.progress {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.progress {
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
        ::std::any::TypeId::of::<CCLCMsg_LoadingProgress>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CCLCMsg_LoadingProgress {
    fn new() -> CCLCMsg_LoadingProgress {
        CCLCMsg_LoadingProgress::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_LoadingProgress>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "progress",
                    CCLCMsg_LoadingProgress::has_progress,
                    CCLCMsg_LoadingProgress::get_progress,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_LoadingProgress>(
                    "CCLCMsg_LoadingProgress",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_LoadingProgress {
    fn clear(&mut self) {
        self.clear_progress();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CCLCMsg_LoadingProgress {
    fn eq(&self, other: &CCLCMsg_LoadingProgress) -> bool {
        self.progress == other.progress &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CCLCMsg_LoadingProgress {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CCLCMsg_SplitPlayerConnect {
    // message fields
    convars: ::protobuf::SingularPtrField<CMsg_CVars>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_SplitPlayerConnect {}

impl CCLCMsg_SplitPlayerConnect {
    pub fn new() -> CCLCMsg_SplitPlayerConnect {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_SplitPlayerConnect {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_SplitPlayerConnect> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_SplitPlayerConnect,
        };
        unsafe {
            instance.get(|| {
                CCLCMsg_SplitPlayerConnect {
                    convars: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .CMsg_CVars convars = 1;

    pub fn clear_convars(&mut self) {
        self.convars.clear();
    }

    pub fn has_convars(&self) -> bool {
        self.convars.is_some()
    }

    // Param is passed by value, moved
    pub fn set_convars(&mut self, v: CMsg_CVars) {
        self.convars = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_convars(&mut self) -> &mut CMsg_CVars {
        if self.convars.is_none() {
            self.convars.set_default();
        };
        self.convars.as_mut().unwrap()
    }

    // Take field
    pub fn take_convars(&mut self) -> CMsg_CVars {
        self.convars.take().unwrap_or_else(|| CMsg_CVars::new())
    }

    pub fn get_convars(&self) -> &CMsg_CVars {
        self.convars.as_ref().unwrap_or_else(|| CMsg_CVars::default_instance())
    }
}

impl ::protobuf::Message for CCLCMsg_SplitPlayerConnect {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.convars));
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
        for value in &self.convars {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.convars.as_ref() {
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
        ::std::any::TypeId::of::<CCLCMsg_SplitPlayerConnect>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CCLCMsg_SplitPlayerConnect {
    fn new() -> CCLCMsg_SplitPlayerConnect {
        CCLCMsg_SplitPlayerConnect::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_SplitPlayerConnect>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "convars",
                    CCLCMsg_SplitPlayerConnect::has_convars,
                    CCLCMsg_SplitPlayerConnect::get_convars,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_SplitPlayerConnect>(
                    "CCLCMsg_SplitPlayerConnect",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_SplitPlayerConnect {
    fn clear(&mut self) {
        self.clear_convars();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CCLCMsg_SplitPlayerConnect {
    fn eq(&self, other: &CCLCMsg_SplitPlayerConnect) -> bool {
        self.convars == other.convars &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CCLCMsg_SplitPlayerConnect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CCLCMsg_ClientMessage {
    // message fields
    msg_type: ::std::option::Option<i32>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_ClientMessage {}

impl CCLCMsg_ClientMessage {
    pub fn new() -> CCLCMsg_ClientMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_ClientMessage {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_ClientMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_ClientMessage,
        };
        unsafe {
            instance.get(|| {
                CCLCMsg_ClientMessage {
                    msg_type: ::std::option::Option::None,
                    data: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 msg_type = 1;

    pub fn clear_msg_type(&mut self) {
        self.msg_type = ::std::option::Option::None;
    }

    pub fn has_msg_type(&self) -> bool {
        self.msg_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg_type(&mut self, v: i32) {
        self.msg_type = ::std::option::Option::Some(v);
    }

    pub fn get_msg_type(&self) -> i32 {
        self.msg_type.unwrap_or(0)
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

impl ::protobuf::Message for CCLCMsg_ClientMessage {
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
                    self.msg_type = ::std::option::Option::Some(tmp);
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
        for value in &self.msg_type {
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
        if let Some(v) = self.msg_type {
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
        ::std::any::TypeId::of::<CCLCMsg_ClientMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CCLCMsg_ClientMessage {
    fn new() -> CCLCMsg_ClientMessage {
        CCLCMsg_ClientMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_ClientMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "msg_type",
                    CCLCMsg_ClientMessage::has_msg_type,
                    CCLCMsg_ClientMessage::get_msg_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "data",
                    CCLCMsg_ClientMessage::has_data,
                    CCLCMsg_ClientMessage::get_data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_ClientMessage>(
                    "CCLCMsg_ClientMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_ClientMessage {
    fn clear(&mut self) {
        self.clear_msg_type();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CCLCMsg_ClientMessage {
    fn eq(&self, other: &CCLCMsg_ClientMessage) -> bool {
        self.msg_type == other.msg_type &&
        self.data == other.data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CCLCMsg_ClientMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_ServerInfo {
    // message fields
    protocol: ::std::option::Option<i32>,
    server_count: ::std::option::Option<i32>,
    is_dedicated: ::std::option::Option<bool>,
    is_hltv: ::std::option::Option<bool>,
    is_replay: ::std::option::Option<bool>,
    c_os: ::std::option::Option<i32>,
    map_crc: ::std::option::Option<u32>,
    client_crc: ::std::option::Option<u32>,
    string_table_crc: ::std::option::Option<u32>,
    max_clients: ::std::option::Option<i32>,
    max_classes: ::std::option::Option<i32>,
    player_slot: ::std::option::Option<i32>,
    tick_interval: ::std::option::Option<f32>,
    game_dir: ::protobuf::SingularField<::std::string::String>,
    map_name: ::protobuf::SingularField<::std::string::String>,
    sky_name: ::protobuf::SingularField<::std::string::String>,
    host_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_ServerInfo {}

impl CSVCMsg_ServerInfo {
    pub fn new() -> CSVCMsg_ServerInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_ServerInfo {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_ServerInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_ServerInfo,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_ServerInfo {
                    protocol: ::std::option::Option::None,
                    server_count: ::std::option::Option::None,
                    is_dedicated: ::std::option::Option::None,
                    is_hltv: ::std::option::Option::None,
                    is_replay: ::std::option::Option::None,
                    c_os: ::std::option::Option::None,
                    map_crc: ::std::option::Option::None,
                    client_crc: ::std::option::Option::None,
                    string_table_crc: ::std::option::Option::None,
                    max_clients: ::std::option::Option::None,
                    max_classes: ::std::option::Option::None,
                    player_slot: ::std::option::Option::None,
                    tick_interval: ::std::option::Option::None,
                    game_dir: ::protobuf::SingularField::none(),
                    map_name: ::protobuf::SingularField::none(),
                    sky_name: ::protobuf::SingularField::none(),
                    host_name: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 protocol = 1;

    pub fn clear_protocol(&mut self) {
        self.protocol = ::std::option::Option::None;
    }

    pub fn has_protocol(&self) -> bool {
        self.protocol.is_some()
    }

    // Param is passed by value, moved
    pub fn set_protocol(&mut self, v: i32) {
        self.protocol = ::std::option::Option::Some(v);
    }

    pub fn get_protocol(&self) -> i32 {
        self.protocol.unwrap_or(0)
    }

    // optional int32 server_count = 2;

    pub fn clear_server_count(&mut self) {
        self.server_count = ::std::option::Option::None;
    }

    pub fn has_server_count(&self) -> bool {
        self.server_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_count(&mut self, v: i32) {
        self.server_count = ::std::option::Option::Some(v);
    }

    pub fn get_server_count(&self) -> i32 {
        self.server_count.unwrap_or(0)
    }

    // optional bool is_dedicated = 3;

    pub fn clear_is_dedicated(&mut self) {
        self.is_dedicated = ::std::option::Option::None;
    }

    pub fn has_is_dedicated(&self) -> bool {
        self.is_dedicated.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_dedicated(&mut self, v: bool) {
        self.is_dedicated = ::std::option::Option::Some(v);
    }

    pub fn get_is_dedicated(&self) -> bool {
        self.is_dedicated.unwrap_or(false)
    }

    // optional bool is_hltv = 4;

    pub fn clear_is_hltv(&mut self) {
        self.is_hltv = ::std::option::Option::None;
    }

    pub fn has_is_hltv(&self) -> bool {
        self.is_hltv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_hltv(&mut self, v: bool) {
        self.is_hltv = ::std::option::Option::Some(v);
    }

    pub fn get_is_hltv(&self) -> bool {
        self.is_hltv.unwrap_or(false)
    }

    // optional bool is_replay = 5;

    pub fn clear_is_replay(&mut self) {
        self.is_replay = ::std::option::Option::None;
    }

    pub fn has_is_replay(&self) -> bool {
        self.is_replay.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_replay(&mut self, v: bool) {
        self.is_replay = ::std::option::Option::Some(v);
    }

    pub fn get_is_replay(&self) -> bool {
        self.is_replay.unwrap_or(false)
    }

    // optional int32 c_os = 6;

    pub fn clear_c_os(&mut self) {
        self.c_os = ::std::option::Option::None;
    }

    pub fn has_c_os(&self) -> bool {
        self.c_os.is_some()
    }

    // Param is passed by value, moved
    pub fn set_c_os(&mut self, v: i32) {
        self.c_os = ::std::option::Option::Some(v);
    }

    pub fn get_c_os(&self) -> i32 {
        self.c_os.unwrap_or(0)
    }

    // optional fixed32 map_crc = 7;

    pub fn clear_map_crc(&mut self) {
        self.map_crc = ::std::option::Option::None;
    }

    pub fn has_map_crc(&self) -> bool {
        self.map_crc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_map_crc(&mut self, v: u32) {
        self.map_crc = ::std::option::Option::Some(v);
    }

    pub fn get_map_crc(&self) -> u32 {
        self.map_crc.unwrap_or(0)
    }

    // optional fixed32 client_crc = 8;

    pub fn clear_client_crc(&mut self) {
        self.client_crc = ::std::option::Option::None;
    }

    pub fn has_client_crc(&self) -> bool {
        self.client_crc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_crc(&mut self, v: u32) {
        self.client_crc = ::std::option::Option::Some(v);
    }

    pub fn get_client_crc(&self) -> u32 {
        self.client_crc.unwrap_or(0)
    }

    // optional fixed32 string_table_crc = 9;

    pub fn clear_string_table_crc(&mut self) {
        self.string_table_crc = ::std::option::Option::None;
    }

    pub fn has_string_table_crc(&self) -> bool {
        self.string_table_crc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_string_table_crc(&mut self, v: u32) {
        self.string_table_crc = ::std::option::Option::Some(v);
    }

    pub fn get_string_table_crc(&self) -> u32 {
        self.string_table_crc.unwrap_or(0)
    }

    // optional int32 max_clients = 10;

    pub fn clear_max_clients(&mut self) {
        self.max_clients = ::std::option::Option::None;
    }

    pub fn has_max_clients(&self) -> bool {
        self.max_clients.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_clients(&mut self, v: i32) {
        self.max_clients = ::std::option::Option::Some(v);
    }

    pub fn get_max_clients(&self) -> i32 {
        self.max_clients.unwrap_or(0)
    }

    // optional int32 max_classes = 11;

    pub fn clear_max_classes(&mut self) {
        self.max_classes = ::std::option::Option::None;
    }

    pub fn has_max_classes(&self) -> bool {
        self.max_classes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_classes(&mut self, v: i32) {
        self.max_classes = ::std::option::Option::Some(v);
    }

    pub fn get_max_classes(&self) -> i32 {
        self.max_classes.unwrap_or(0)
    }

    // optional int32 player_slot = 12;

    pub fn clear_player_slot(&mut self) {
        self.player_slot = ::std::option::Option::None;
    }

    pub fn has_player_slot(&self) -> bool {
        self.player_slot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_slot(&mut self, v: i32) {
        self.player_slot = ::std::option::Option::Some(v);
    }

    pub fn get_player_slot(&self) -> i32 {
        self.player_slot.unwrap_or(0)
    }

    // optional float tick_interval = 13;

    pub fn clear_tick_interval(&mut self) {
        self.tick_interval = ::std::option::Option::None;
    }

    pub fn has_tick_interval(&self) -> bool {
        self.tick_interval.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tick_interval(&mut self, v: f32) {
        self.tick_interval = ::std::option::Option::Some(v);
    }

    pub fn get_tick_interval(&self) -> f32 {
        self.tick_interval.unwrap_or(0.)
    }

    // optional string game_dir = 14;

    pub fn clear_game_dir(&mut self) {
        self.game_dir.clear();
    }

    pub fn has_game_dir(&self) -> bool {
        self.game_dir.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_dir(&mut self, v: ::std::string::String) {
        self.game_dir = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_game_dir(&mut self) -> &mut ::std::string::String {
        if self.game_dir.is_none() {
            self.game_dir.set_default();
        };
        self.game_dir.as_mut().unwrap()
    }

    // Take field
    pub fn take_game_dir(&mut self) -> ::std::string::String {
        self.game_dir.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_game_dir(&self) -> &str {
        match self.game_dir.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string map_name = 15;

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

    // optional string sky_name = 16;

    pub fn clear_sky_name(&mut self) {
        self.sky_name.clear();
    }

    pub fn has_sky_name(&self) -> bool {
        self.sky_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sky_name(&mut self, v: ::std::string::String) {
        self.sky_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sky_name(&mut self) -> &mut ::std::string::String {
        if self.sky_name.is_none() {
            self.sky_name.set_default();
        };
        self.sky_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_sky_name(&mut self) -> ::std::string::String {
        self.sky_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_sky_name(&self) -> &str {
        match self.sky_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string host_name = 17;

    pub fn clear_host_name(&mut self) {
        self.host_name.clear();
    }

    pub fn has_host_name(&self) -> bool {
        self.host_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_host_name(&mut self, v: ::std::string::String) {
        self.host_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_host_name(&mut self) -> &mut ::std::string::String {
        if self.host_name.is_none() {
            self.host_name.set_default();
        };
        self.host_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_host_name(&mut self) -> ::std::string::String {
        self.host_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_host_name(&self) -> &str {
        match self.host_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for CSVCMsg_ServerInfo {
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
                    self.protocol = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.server_count = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.is_dedicated = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.is_hltv = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.is_replay = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.c_os = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed32());
                    self.map_crc = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed32());
                    self.client_crc = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed32());
                    self.string_table_crc = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.max_clients = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.max_classes = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.player_slot = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.tick_interval = ::std::option::Option::Some(tmp);
                },
                14 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.game_dir));
                },
                15 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.map_name));
                },
                16 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.sky_name));
                },
                17 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.host_name));
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
        for value in &self.protocol {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.server_count {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.is_dedicated.is_some() {
            my_size += 2;
        };
        if self.is_hltv.is_some() {
            my_size += 2;
        };
        if self.is_replay.is_some() {
            my_size += 2;
        };
        for value in &self.c_os {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.map_crc.is_some() {
            my_size += 5;
        };
        if self.client_crc.is_some() {
            my_size += 5;
        };
        if self.string_table_crc.is_some() {
            my_size += 5;
        };
        for value in &self.max_clients {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.max_classes {
            my_size += ::protobuf::rt::value_size(11, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.player_slot {
            my_size += ::protobuf::rt::value_size(12, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.tick_interval.is_some() {
            my_size += 5;
        };
        for value in &self.game_dir {
            my_size += ::protobuf::rt::string_size(14, &value);
        };
        for value in &self.map_name {
            my_size += ::protobuf::rt::string_size(15, &value);
        };
        for value in &self.sky_name {
            my_size += ::protobuf::rt::string_size(16, &value);
        };
        for value in &self.host_name {
            my_size += ::protobuf::rt::string_size(17, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.protocol {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.server_count {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.is_dedicated {
            try!(os.write_bool(3, v));
        };
        if let Some(v) = self.is_hltv {
            try!(os.write_bool(4, v));
        };
        if let Some(v) = self.is_replay {
            try!(os.write_bool(5, v));
        };
        if let Some(v) = self.c_os {
            try!(os.write_int32(6, v));
        };
        if let Some(v) = self.map_crc {
            try!(os.write_fixed32(7, v));
        };
        if let Some(v) = self.client_crc {
            try!(os.write_fixed32(8, v));
        };
        if let Some(v) = self.string_table_crc {
            try!(os.write_fixed32(9, v));
        };
        if let Some(v) = self.max_clients {
            try!(os.write_int32(10, v));
        };
        if let Some(v) = self.max_classes {
            try!(os.write_int32(11, v));
        };
        if let Some(v) = self.player_slot {
            try!(os.write_int32(12, v));
        };
        if let Some(v) = self.tick_interval {
            try!(os.write_float(13, v));
        };
        if let Some(v) = self.game_dir.as_ref() {
            try!(os.write_string(14, &v));
        };
        if let Some(v) = self.map_name.as_ref() {
            try!(os.write_string(15, &v));
        };
        if let Some(v) = self.sky_name.as_ref() {
            try!(os.write_string(16, &v));
        };
        if let Some(v) = self.host_name.as_ref() {
            try!(os.write_string(17, &v));
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
        ::std::any::TypeId::of::<CSVCMsg_ServerInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_ServerInfo {
    fn new() -> CSVCMsg_ServerInfo {
        CSVCMsg_ServerInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_ServerInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "protocol",
                    CSVCMsg_ServerInfo::has_protocol,
                    CSVCMsg_ServerInfo::get_protocol,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "server_count",
                    CSVCMsg_ServerInfo::has_server_count,
                    CSVCMsg_ServerInfo::get_server_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "is_dedicated",
                    CSVCMsg_ServerInfo::has_is_dedicated,
                    CSVCMsg_ServerInfo::get_is_dedicated,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "is_hltv",
                    CSVCMsg_ServerInfo::has_is_hltv,
                    CSVCMsg_ServerInfo::get_is_hltv,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "is_replay",
                    CSVCMsg_ServerInfo::has_is_replay,
                    CSVCMsg_ServerInfo::get_is_replay,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "c_os",
                    CSVCMsg_ServerInfo::has_c_os,
                    CSVCMsg_ServerInfo::get_c_os,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "map_crc",
                    CSVCMsg_ServerInfo::has_map_crc,
                    CSVCMsg_ServerInfo::get_map_crc,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "client_crc",
                    CSVCMsg_ServerInfo::has_client_crc,
                    CSVCMsg_ServerInfo::get_client_crc,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "string_table_crc",
                    CSVCMsg_ServerInfo::has_string_table_crc,
                    CSVCMsg_ServerInfo::get_string_table_crc,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "max_clients",
                    CSVCMsg_ServerInfo::has_max_clients,
                    CSVCMsg_ServerInfo::get_max_clients,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "max_classes",
                    CSVCMsg_ServerInfo::has_max_classes,
                    CSVCMsg_ServerInfo::get_max_classes,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "player_slot",
                    CSVCMsg_ServerInfo::has_player_slot,
                    CSVCMsg_ServerInfo::get_player_slot,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "tick_interval",
                    CSVCMsg_ServerInfo::has_tick_interval,
                    CSVCMsg_ServerInfo::get_tick_interval,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "game_dir",
                    CSVCMsg_ServerInfo::has_game_dir,
                    CSVCMsg_ServerInfo::get_game_dir,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "map_name",
                    CSVCMsg_ServerInfo::has_map_name,
                    CSVCMsg_ServerInfo::get_map_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "sky_name",
                    CSVCMsg_ServerInfo::has_sky_name,
                    CSVCMsg_ServerInfo::get_sky_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "host_name",
                    CSVCMsg_ServerInfo::has_host_name,
                    CSVCMsg_ServerInfo::get_host_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_ServerInfo>(
                    "CSVCMsg_ServerInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_ServerInfo {
    fn clear(&mut self) {
        self.clear_protocol();
        self.clear_server_count();
        self.clear_is_dedicated();
        self.clear_is_hltv();
        self.clear_is_replay();
        self.clear_c_os();
        self.clear_map_crc();
        self.clear_client_crc();
        self.clear_string_table_crc();
        self.clear_max_clients();
        self.clear_max_classes();
        self.clear_player_slot();
        self.clear_tick_interval();
        self.clear_game_dir();
        self.clear_map_name();
        self.clear_sky_name();
        self.clear_host_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_ServerInfo {
    fn eq(&self, other: &CSVCMsg_ServerInfo) -> bool {
        self.protocol == other.protocol &&
        self.server_count == other.server_count &&
        self.is_dedicated == other.is_dedicated &&
        self.is_hltv == other.is_hltv &&
        self.is_replay == other.is_replay &&
        self.c_os == other.c_os &&
        self.map_crc == other.map_crc &&
        self.client_crc == other.client_crc &&
        self.string_table_crc == other.string_table_crc &&
        self.max_clients == other.max_clients &&
        self.max_classes == other.max_classes &&
        self.player_slot == other.player_slot &&
        self.tick_interval == other.tick_interval &&
        self.game_dir == other.game_dir &&
        self.map_name == other.map_name &&
        self.sky_name == other.sky_name &&
        self.host_name == other.host_name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_ServerInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_ClassInfo {
    // message fields
    create_on_client: ::std::option::Option<bool>,
    classes: ::protobuf::RepeatedField<CSVCMsg_ClassInfo_class_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_ClassInfo {}

impl CSVCMsg_ClassInfo {
    pub fn new() -> CSVCMsg_ClassInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_ClassInfo {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_ClassInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_ClassInfo,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_ClassInfo {
                    create_on_client: ::std::option::Option::None,
                    classes: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool create_on_client = 1;

    pub fn clear_create_on_client(&mut self) {
        self.create_on_client = ::std::option::Option::None;
    }

    pub fn has_create_on_client(&self) -> bool {
        self.create_on_client.is_some()
    }

    // Param is passed by value, moved
    pub fn set_create_on_client(&mut self, v: bool) {
        self.create_on_client = ::std::option::Option::Some(v);
    }

    pub fn get_create_on_client(&self) -> bool {
        self.create_on_client.unwrap_or(false)
    }

    // repeated .CSVCMsg_ClassInfo.class_t classes = 2;

    pub fn clear_classes(&mut self) {
        self.classes.clear();
    }

    // Param is passed by value, moved
    pub fn set_classes(&mut self, v: ::protobuf::RepeatedField<CSVCMsg_ClassInfo_class_t>) {
        self.classes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_classes(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_ClassInfo_class_t> {
        &mut self.classes
    }

    // Take field
    pub fn take_classes(&mut self) -> ::protobuf::RepeatedField<CSVCMsg_ClassInfo_class_t> {
        ::std::mem::replace(&mut self.classes, ::protobuf::RepeatedField::new())
    }

    pub fn get_classes(&self) -> &[CSVCMsg_ClassInfo_class_t] {
        &self.classes
    }
}

impl ::protobuf::Message for CSVCMsg_ClassInfo {
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
                    self.create_on_client = ::std::option::Option::Some(tmp);
                },
                2 => {
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
        if self.create_on_client.is_some() {
            my_size += 2;
        };
        for value in &self.classes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.create_on_client {
            try!(os.write_bool(1, v));
        };
        for v in &self.classes {
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
        ::std::any::TypeId::of::<CSVCMsg_ClassInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_ClassInfo {
    fn new() -> CSVCMsg_ClassInfo {
        CSVCMsg_ClassInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_ClassInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "create_on_client",
                    CSVCMsg_ClassInfo::has_create_on_client,
                    CSVCMsg_ClassInfo::get_create_on_client,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "classes",
                    CSVCMsg_ClassInfo::get_classes,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_ClassInfo>(
                    "CSVCMsg_ClassInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_ClassInfo {
    fn clear(&mut self) {
        self.clear_create_on_client();
        self.clear_classes();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_ClassInfo {
    fn eq(&self, other: &CSVCMsg_ClassInfo) -> bool {
        self.create_on_client == other.create_on_client &&
        self.classes == other.classes &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_ClassInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_ClassInfo_class_t {
    // message fields
    class_id: ::std::option::Option<i32>,
    data_table_name: ::protobuf::SingularField<::std::string::String>,
    class_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_ClassInfo_class_t {}

impl CSVCMsg_ClassInfo_class_t {
    pub fn new() -> CSVCMsg_ClassInfo_class_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_ClassInfo_class_t {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_ClassInfo_class_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_ClassInfo_class_t,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_ClassInfo_class_t {
                    class_id: ::std::option::Option::None,
                    data_table_name: ::protobuf::SingularField::none(),
                    class_name: ::protobuf::SingularField::none(),
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

    // optional string data_table_name = 2;

    pub fn clear_data_table_name(&mut self) {
        self.data_table_name.clear();
    }

    pub fn has_data_table_name(&self) -> bool {
        self.data_table_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data_table_name(&mut self, v: ::std::string::String) {
        self.data_table_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data_table_name(&mut self) -> &mut ::std::string::String {
        if self.data_table_name.is_none() {
            self.data_table_name.set_default();
        };
        self.data_table_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_data_table_name(&mut self) -> ::std::string::String {
        self.data_table_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_data_table_name(&self) -> &str {
        match self.data_table_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string class_name = 3;

    pub fn clear_class_name(&mut self) {
        self.class_name.clear();
    }

    pub fn has_class_name(&self) -> bool {
        self.class_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_class_name(&mut self, v: ::std::string::String) {
        self.class_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_class_name(&mut self) -> &mut ::std::string::String {
        if self.class_name.is_none() {
            self.class_name.set_default();
        };
        self.class_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_class_name(&mut self) -> ::std::string::String {
        self.class_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_class_name(&self) -> &str {
        match self.class_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for CSVCMsg_ClassInfo_class_t {
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
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.data_table_name));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.class_name));
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
        for value in &self.data_table_name {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.class_name {
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
        if let Some(v) = self.data_table_name.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.class_name.as_ref() {
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
        ::std::any::TypeId::of::<CSVCMsg_ClassInfo_class_t>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_ClassInfo_class_t {
    fn new() -> CSVCMsg_ClassInfo_class_t {
        CSVCMsg_ClassInfo_class_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_ClassInfo_class_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "class_id",
                    CSVCMsg_ClassInfo_class_t::has_class_id,
                    CSVCMsg_ClassInfo_class_t::get_class_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "data_table_name",
                    CSVCMsg_ClassInfo_class_t::has_data_table_name,
                    CSVCMsg_ClassInfo_class_t::get_data_table_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "class_name",
                    CSVCMsg_ClassInfo_class_t::has_class_name,
                    CSVCMsg_ClassInfo_class_t::get_class_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_ClassInfo_class_t>(
                    "CSVCMsg_ClassInfo_class_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_ClassInfo_class_t {
    fn clear(&mut self) {
        self.clear_class_id();
        self.clear_data_table_name();
        self.clear_class_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_ClassInfo_class_t {
    fn eq(&self, other: &CSVCMsg_ClassInfo_class_t) -> bool {
        self.class_id == other.class_id &&
        self.data_table_name == other.data_table_name &&
        self.class_name == other.class_name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_ClassInfo_class_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_SetPause {
    // message fields
    paused: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_SetPause {}

impl CSVCMsg_SetPause {
    pub fn new() -> CSVCMsg_SetPause {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_SetPause {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_SetPause> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_SetPause,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_SetPause {
                    paused: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool paused = 1;

    pub fn clear_paused(&mut self) {
        self.paused = ::std::option::Option::None;
    }

    pub fn has_paused(&self) -> bool {
        self.paused.is_some()
    }

    // Param is passed by value, moved
    pub fn set_paused(&mut self, v: bool) {
        self.paused = ::std::option::Option::Some(v);
    }

    pub fn get_paused(&self) -> bool {
        self.paused.unwrap_or(false)
    }
}

impl ::protobuf::Message for CSVCMsg_SetPause {
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
                    self.paused = ::std::option::Option::Some(tmp);
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
        if self.paused.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.paused {
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
        ::std::any::TypeId::of::<CSVCMsg_SetPause>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_SetPause {
    fn new() -> CSVCMsg_SetPause {
        CSVCMsg_SetPause::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_SetPause>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "paused",
                    CSVCMsg_SetPause::has_paused,
                    CSVCMsg_SetPause::get_paused,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_SetPause>(
                    "CSVCMsg_SetPause",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_SetPause {
    fn clear(&mut self) {
        self.clear_paused();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_SetPause {
    fn eq(&self, other: &CSVCMsg_SetPause) -> bool {
        self.paused == other.paused &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_SetPause {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_VoiceInit {
    // message fields
    quality: ::std::option::Option<i32>,
    codec: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_VoiceInit {}

impl CSVCMsg_VoiceInit {
    pub fn new() -> CSVCMsg_VoiceInit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_VoiceInit {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_VoiceInit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_VoiceInit,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_VoiceInit {
                    quality: ::std::option::Option::None,
                    codec: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 quality = 1;

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

    // optional string codec = 2;

    pub fn clear_codec(&mut self) {
        self.codec.clear();
    }

    pub fn has_codec(&self) -> bool {
        self.codec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_codec(&mut self, v: ::std::string::String) {
        self.codec = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_codec(&mut self) -> &mut ::std::string::String {
        if self.codec.is_none() {
            self.codec.set_default();
        };
        self.codec.as_mut().unwrap()
    }

    // Take field
    pub fn take_codec(&mut self) -> ::std::string::String {
        self.codec.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_codec(&self) -> &str {
        match self.codec.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for CSVCMsg_VoiceInit {
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
                    self.quality = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.codec));
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
        for value in &self.quality {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.codec {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.quality {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.codec.as_ref() {
            try!(os.write_string(2, &v));
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
        ::std::any::TypeId::of::<CSVCMsg_VoiceInit>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_VoiceInit {
    fn new() -> CSVCMsg_VoiceInit {
        CSVCMsg_VoiceInit::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_VoiceInit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "quality",
                    CSVCMsg_VoiceInit::has_quality,
                    CSVCMsg_VoiceInit::get_quality,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "codec",
                    CSVCMsg_VoiceInit::has_codec,
                    CSVCMsg_VoiceInit::get_codec,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_VoiceInit>(
                    "CSVCMsg_VoiceInit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_VoiceInit {
    fn clear(&mut self) {
        self.clear_quality();
        self.clear_codec();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_VoiceInit {
    fn eq(&self, other: &CSVCMsg_VoiceInit) -> bool {
        self.quality == other.quality &&
        self.codec == other.codec &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_VoiceInit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_Print {
    // message fields
    text: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_Print {}

impl CSVCMsg_Print {
    pub fn new() -> CSVCMsg_Print {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_Print {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_Print> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_Print,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_Print {
                    text: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string text = 1;

    pub fn clear_text(&mut self) {
        self.text.clear();
    }

    pub fn has_text(&self) -> bool {
        self.text.is_some()
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: ::std::string::String) {
        self.text = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_text(&mut self) -> &mut ::std::string::String {
        if self.text.is_none() {
            self.text.set_default();
        };
        self.text.as_mut().unwrap()
    }

    // Take field
    pub fn take_text(&mut self) -> ::std::string::String {
        self.text.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_text(&self) -> &str {
        match self.text.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for CSVCMsg_Print {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.text));
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
        for value in &self.text {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.text.as_ref() {
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
        ::std::any::TypeId::of::<CSVCMsg_Print>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_Print {
    fn new() -> CSVCMsg_Print {
        CSVCMsg_Print::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_Print>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "text",
                    CSVCMsg_Print::has_text,
                    CSVCMsg_Print::get_text,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_Print>(
                    "CSVCMsg_Print",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_Print {
    fn clear(&mut self) {
        self.clear_text();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_Print {
    fn eq(&self, other: &CSVCMsg_Print) -> bool {
        self.text == other.text &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_Print {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_Sounds {
    // message fields
    reliable_sound: ::std::option::Option<bool>,
    sounds: ::protobuf::RepeatedField<CSVCMsg_Sounds_sounddata_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_Sounds {}

impl CSVCMsg_Sounds {
    pub fn new() -> CSVCMsg_Sounds {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_Sounds {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_Sounds> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_Sounds,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_Sounds {
                    reliable_sound: ::std::option::Option::None,
                    sounds: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool reliable_sound = 1;

    pub fn clear_reliable_sound(&mut self) {
        self.reliable_sound = ::std::option::Option::None;
    }

    pub fn has_reliable_sound(&self) -> bool {
        self.reliable_sound.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reliable_sound(&mut self, v: bool) {
        self.reliable_sound = ::std::option::Option::Some(v);
    }

    pub fn get_reliable_sound(&self) -> bool {
        self.reliable_sound.unwrap_or(false)
    }

    // repeated .CSVCMsg_Sounds.sounddata_t sounds = 2;

    pub fn clear_sounds(&mut self) {
        self.sounds.clear();
    }

    // Param is passed by value, moved
    pub fn set_sounds(&mut self, v: ::protobuf::RepeatedField<CSVCMsg_Sounds_sounddata_t>) {
        self.sounds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_sounds(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_Sounds_sounddata_t> {
        &mut self.sounds
    }

    // Take field
    pub fn take_sounds(&mut self) -> ::protobuf::RepeatedField<CSVCMsg_Sounds_sounddata_t> {
        ::std::mem::replace(&mut self.sounds, ::protobuf::RepeatedField::new())
    }

    pub fn get_sounds(&self) -> &[CSVCMsg_Sounds_sounddata_t] {
        &self.sounds
    }
}

impl ::protobuf::Message for CSVCMsg_Sounds {
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
                    self.reliable_sound = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.sounds));
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
        if self.reliable_sound.is_some() {
            my_size += 2;
        };
        for value in &self.sounds {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.reliable_sound {
            try!(os.write_bool(1, v));
        };
        for v in &self.sounds {
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
        ::std::any::TypeId::of::<CSVCMsg_Sounds>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_Sounds {
    fn new() -> CSVCMsg_Sounds {
        CSVCMsg_Sounds::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_Sounds>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "reliable_sound",
                    CSVCMsg_Sounds::has_reliable_sound,
                    CSVCMsg_Sounds::get_reliable_sound,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "sounds",
                    CSVCMsg_Sounds::get_sounds,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_Sounds>(
                    "CSVCMsg_Sounds",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_Sounds {
    fn clear(&mut self) {
        self.clear_reliable_sound();
        self.clear_sounds();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_Sounds {
    fn eq(&self, other: &CSVCMsg_Sounds) -> bool {
        self.reliable_sound == other.reliable_sound &&
        self.sounds == other.sounds &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_Sounds {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_Sounds_sounddata_t {
    // message fields
    origin_x: ::std::option::Option<i32>,
    origin_y: ::std::option::Option<i32>,
    origin_z: ::std::option::Option<i32>,
    volume: ::std::option::Option<u32>,
    delay_value: ::std::option::Option<f32>,
    sequence_number: ::std::option::Option<i32>,
    entity_index: ::std::option::Option<i32>,
    channel: ::std::option::Option<i32>,
    pitch: ::std::option::Option<i32>,
    flags: ::std::option::Option<i32>,
    sound_num: ::std::option::Option<u32>,
    sound_num_handle: ::std::option::Option<u32>,
    speaker_entity: ::std::option::Option<i32>,
    random_seed: ::std::option::Option<i32>,
    sound_level: ::std::option::Option<i32>,
    is_sentence: ::std::option::Option<bool>,
    is_ambient: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_Sounds_sounddata_t {}

impl CSVCMsg_Sounds_sounddata_t {
    pub fn new() -> CSVCMsg_Sounds_sounddata_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_Sounds_sounddata_t {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_Sounds_sounddata_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_Sounds_sounddata_t,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_Sounds_sounddata_t {
                    origin_x: ::std::option::Option::None,
                    origin_y: ::std::option::Option::None,
                    origin_z: ::std::option::Option::None,
                    volume: ::std::option::Option::None,
                    delay_value: ::std::option::Option::None,
                    sequence_number: ::std::option::Option::None,
                    entity_index: ::std::option::Option::None,
                    channel: ::std::option::Option::None,
                    pitch: ::std::option::Option::None,
                    flags: ::std::option::Option::None,
                    sound_num: ::std::option::Option::None,
                    sound_num_handle: ::std::option::Option::None,
                    speaker_entity: ::std::option::Option::None,
                    random_seed: ::std::option::Option::None,
                    sound_level: ::std::option::Option::None,
                    is_sentence: ::std::option::Option::None,
                    is_ambient: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional sint32 origin_x = 1;

    pub fn clear_origin_x(&mut self) {
        self.origin_x = ::std::option::Option::None;
    }

    pub fn has_origin_x(&self) -> bool {
        self.origin_x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_x(&mut self, v: i32) {
        self.origin_x = ::std::option::Option::Some(v);
    }

    pub fn get_origin_x(&self) -> i32 {
        self.origin_x.unwrap_or(0)
    }

    // optional sint32 origin_y = 2;

    pub fn clear_origin_y(&mut self) {
        self.origin_y = ::std::option::Option::None;
    }

    pub fn has_origin_y(&self) -> bool {
        self.origin_y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_y(&mut self, v: i32) {
        self.origin_y = ::std::option::Option::Some(v);
    }

    pub fn get_origin_y(&self) -> i32 {
        self.origin_y.unwrap_or(0)
    }

    // optional sint32 origin_z = 3;

    pub fn clear_origin_z(&mut self) {
        self.origin_z = ::std::option::Option::None;
    }

    pub fn has_origin_z(&self) -> bool {
        self.origin_z.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_z(&mut self, v: i32) {
        self.origin_z = ::std::option::Option::Some(v);
    }

    pub fn get_origin_z(&self) -> i32 {
        self.origin_z.unwrap_or(0)
    }

    // optional uint32 volume = 4;

    pub fn clear_volume(&mut self) {
        self.volume = ::std::option::Option::None;
    }

    pub fn has_volume(&self) -> bool {
        self.volume.is_some()
    }

    // Param is passed by value, moved
    pub fn set_volume(&mut self, v: u32) {
        self.volume = ::std::option::Option::Some(v);
    }

    pub fn get_volume(&self) -> u32 {
        self.volume.unwrap_or(0)
    }

    // optional float delay_value = 5;

    pub fn clear_delay_value(&mut self) {
        self.delay_value = ::std::option::Option::None;
    }

    pub fn has_delay_value(&self) -> bool {
        self.delay_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delay_value(&mut self, v: f32) {
        self.delay_value = ::std::option::Option::Some(v);
    }

    pub fn get_delay_value(&self) -> f32 {
        self.delay_value.unwrap_or(0.)
    }

    // optional int32 sequence_number = 6;

    pub fn clear_sequence_number(&mut self) {
        self.sequence_number = ::std::option::Option::None;
    }

    pub fn has_sequence_number(&self) -> bool {
        self.sequence_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sequence_number(&mut self, v: i32) {
        self.sequence_number = ::std::option::Option::Some(v);
    }

    pub fn get_sequence_number(&self) -> i32 {
        self.sequence_number.unwrap_or(0)
    }

    // optional int32 entity_index = 7;

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

    // optional int32 channel = 8;

    pub fn clear_channel(&mut self) {
        self.channel = ::std::option::Option::None;
    }

    pub fn has_channel(&self) -> bool {
        self.channel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel(&mut self, v: i32) {
        self.channel = ::std::option::Option::Some(v);
    }

    pub fn get_channel(&self) -> i32 {
        self.channel.unwrap_or(0)
    }

    // optional int32 pitch = 9;

    pub fn clear_pitch(&mut self) {
        self.pitch = ::std::option::Option::None;
    }

    pub fn has_pitch(&self) -> bool {
        self.pitch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pitch(&mut self, v: i32) {
        self.pitch = ::std::option::Option::Some(v);
    }

    pub fn get_pitch(&self) -> i32 {
        self.pitch.unwrap_or(0)
    }

    // optional int32 flags = 10;

    pub fn clear_flags(&mut self) {
        self.flags = ::std::option::Option::None;
    }

    pub fn has_flags(&self) -> bool {
        self.flags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flags(&mut self, v: i32) {
        self.flags = ::std::option::Option::Some(v);
    }

    pub fn get_flags(&self) -> i32 {
        self.flags.unwrap_or(0)
    }

    // optional uint32 sound_num = 11;

    pub fn clear_sound_num(&mut self) {
        self.sound_num = ::std::option::Option::None;
    }

    pub fn has_sound_num(&self) -> bool {
        self.sound_num.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sound_num(&mut self, v: u32) {
        self.sound_num = ::std::option::Option::Some(v);
    }

    pub fn get_sound_num(&self) -> u32 {
        self.sound_num.unwrap_or(0)
    }

    // optional fixed32 sound_num_handle = 12;

    pub fn clear_sound_num_handle(&mut self) {
        self.sound_num_handle = ::std::option::Option::None;
    }

    pub fn has_sound_num_handle(&self) -> bool {
        self.sound_num_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sound_num_handle(&mut self, v: u32) {
        self.sound_num_handle = ::std::option::Option::Some(v);
    }

    pub fn get_sound_num_handle(&self) -> u32 {
        self.sound_num_handle.unwrap_or(0)
    }

    // optional int32 speaker_entity = 13;

    pub fn clear_speaker_entity(&mut self) {
        self.speaker_entity = ::std::option::Option::None;
    }

    pub fn has_speaker_entity(&self) -> bool {
        self.speaker_entity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_speaker_entity(&mut self, v: i32) {
        self.speaker_entity = ::std::option::Option::Some(v);
    }

    pub fn get_speaker_entity(&self) -> i32 {
        self.speaker_entity.unwrap_or(0)
    }

    // optional int32 random_seed = 14;

    pub fn clear_random_seed(&mut self) {
        self.random_seed = ::std::option::Option::None;
    }

    pub fn has_random_seed(&self) -> bool {
        self.random_seed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_random_seed(&mut self, v: i32) {
        self.random_seed = ::std::option::Option::Some(v);
    }

    pub fn get_random_seed(&self) -> i32 {
        self.random_seed.unwrap_or(0)
    }

    // optional int32 sound_level = 15;

    pub fn clear_sound_level(&mut self) {
        self.sound_level = ::std::option::Option::None;
    }

    pub fn has_sound_level(&self) -> bool {
        self.sound_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sound_level(&mut self, v: i32) {
        self.sound_level = ::std::option::Option::Some(v);
    }

    pub fn get_sound_level(&self) -> i32 {
        self.sound_level.unwrap_or(0)
    }

    // optional bool is_sentence = 16;

    pub fn clear_is_sentence(&mut self) {
        self.is_sentence = ::std::option::Option::None;
    }

    pub fn has_is_sentence(&self) -> bool {
        self.is_sentence.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_sentence(&mut self, v: bool) {
        self.is_sentence = ::std::option::Option::Some(v);
    }

    pub fn get_is_sentence(&self) -> bool {
        self.is_sentence.unwrap_or(false)
    }

    // optional bool is_ambient = 17;

    pub fn clear_is_ambient(&mut self) {
        self.is_ambient = ::std::option::Option::None;
    }

    pub fn has_is_ambient(&self) -> bool {
        self.is_ambient.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_ambient(&mut self, v: bool) {
        self.is_ambient = ::std::option::Option::Some(v);
    }

    pub fn get_is_ambient(&self) -> bool {
        self.is_ambient.unwrap_or(false)
    }
}

impl ::protobuf::Message for CSVCMsg_Sounds_sounddata_t {
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
                    let tmp = try!(is.read_sint32());
                    self.origin_x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint32());
                    self.origin_y = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_sint32());
                    self.origin_z = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.volume = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.delay_value = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.sequence_number = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.entity_index = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.channel = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.pitch = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.flags = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.sound_num = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed32());
                    self.sound_num_handle = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.speaker_entity = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.random_seed = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.sound_level = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.is_sentence = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.is_ambient = ::std::option::Option::Some(tmp);
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
        for value in &self.origin_x {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, *value);
        };
        for value in &self.origin_y {
            my_size += ::protobuf::rt::value_varint_zigzag_size(2, *value);
        };
        for value in &self.origin_z {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, *value);
        };
        for value in &self.volume {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.delay_value.is_some() {
            my_size += 5;
        };
        for value in &self.sequence_number {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.entity_index {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.channel {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.pitch {
            my_size += ::protobuf::rt::value_size(9, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.flags {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.sound_num {
            my_size += ::protobuf::rt::value_size(11, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.sound_num_handle.is_some() {
            my_size += 5;
        };
        for value in &self.speaker_entity {
            my_size += ::protobuf::rt::value_size(13, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.random_seed {
            my_size += ::protobuf::rt::value_size(14, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.sound_level {
            my_size += ::protobuf::rt::value_size(15, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.is_sentence.is_some() {
            my_size += 3;
        };
        if self.is_ambient.is_some() {
            my_size += 3;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin_x {
            try!(os.write_sint32(1, v));
        };
        if let Some(v) = self.origin_y {
            try!(os.write_sint32(2, v));
        };
        if let Some(v) = self.origin_z {
            try!(os.write_sint32(3, v));
        };
        if let Some(v) = self.volume {
            try!(os.write_uint32(4, v));
        };
        if let Some(v) = self.delay_value {
            try!(os.write_float(5, v));
        };
        if let Some(v) = self.sequence_number {
            try!(os.write_int32(6, v));
        };
        if let Some(v) = self.entity_index {
            try!(os.write_int32(7, v));
        };
        if let Some(v) = self.channel {
            try!(os.write_int32(8, v));
        };
        if let Some(v) = self.pitch {
            try!(os.write_int32(9, v));
        };
        if let Some(v) = self.flags {
            try!(os.write_int32(10, v));
        };
        if let Some(v) = self.sound_num {
            try!(os.write_uint32(11, v));
        };
        if let Some(v) = self.sound_num_handle {
            try!(os.write_fixed32(12, v));
        };
        if let Some(v) = self.speaker_entity {
            try!(os.write_int32(13, v));
        };
        if let Some(v) = self.random_seed {
            try!(os.write_int32(14, v));
        };
        if let Some(v) = self.sound_level {
            try!(os.write_int32(15, v));
        };
        if let Some(v) = self.is_sentence {
            try!(os.write_bool(16, v));
        };
        if let Some(v) = self.is_ambient {
            try!(os.write_bool(17, v));
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
        ::std::any::TypeId::of::<CSVCMsg_Sounds_sounddata_t>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_Sounds_sounddata_t {
    fn new() -> CSVCMsg_Sounds_sounddata_t {
        CSVCMsg_Sounds_sounddata_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_Sounds_sounddata_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "origin_x",
                    CSVCMsg_Sounds_sounddata_t::has_origin_x,
                    CSVCMsg_Sounds_sounddata_t::get_origin_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "origin_y",
                    CSVCMsg_Sounds_sounddata_t::has_origin_y,
                    CSVCMsg_Sounds_sounddata_t::get_origin_y,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "origin_z",
                    CSVCMsg_Sounds_sounddata_t::has_origin_z,
                    CSVCMsg_Sounds_sounddata_t::get_origin_z,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "volume",
                    CSVCMsg_Sounds_sounddata_t::has_volume,
                    CSVCMsg_Sounds_sounddata_t::get_volume,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "delay_value",
                    CSVCMsg_Sounds_sounddata_t::has_delay_value,
                    CSVCMsg_Sounds_sounddata_t::get_delay_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "sequence_number",
                    CSVCMsg_Sounds_sounddata_t::has_sequence_number,
                    CSVCMsg_Sounds_sounddata_t::get_sequence_number,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "entity_index",
                    CSVCMsg_Sounds_sounddata_t::has_entity_index,
                    CSVCMsg_Sounds_sounddata_t::get_entity_index,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "channel",
                    CSVCMsg_Sounds_sounddata_t::has_channel,
                    CSVCMsg_Sounds_sounddata_t::get_channel,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "pitch",
                    CSVCMsg_Sounds_sounddata_t::has_pitch,
                    CSVCMsg_Sounds_sounddata_t::get_pitch,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "flags",
                    CSVCMsg_Sounds_sounddata_t::has_flags,
                    CSVCMsg_Sounds_sounddata_t::get_flags,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "sound_num",
                    CSVCMsg_Sounds_sounddata_t::has_sound_num,
                    CSVCMsg_Sounds_sounddata_t::get_sound_num,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "sound_num_handle",
                    CSVCMsg_Sounds_sounddata_t::has_sound_num_handle,
                    CSVCMsg_Sounds_sounddata_t::get_sound_num_handle,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "speaker_entity",
                    CSVCMsg_Sounds_sounddata_t::has_speaker_entity,
                    CSVCMsg_Sounds_sounddata_t::get_speaker_entity,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "random_seed",
                    CSVCMsg_Sounds_sounddata_t::has_random_seed,
                    CSVCMsg_Sounds_sounddata_t::get_random_seed,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "sound_level",
                    CSVCMsg_Sounds_sounddata_t::has_sound_level,
                    CSVCMsg_Sounds_sounddata_t::get_sound_level,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "is_sentence",
                    CSVCMsg_Sounds_sounddata_t::has_is_sentence,
                    CSVCMsg_Sounds_sounddata_t::get_is_sentence,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "is_ambient",
                    CSVCMsg_Sounds_sounddata_t::has_is_ambient,
                    CSVCMsg_Sounds_sounddata_t::get_is_ambient,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_Sounds_sounddata_t>(
                    "CSVCMsg_Sounds_sounddata_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_Sounds_sounddata_t {
    fn clear(&mut self) {
        self.clear_origin_x();
        self.clear_origin_y();
        self.clear_origin_z();
        self.clear_volume();
        self.clear_delay_value();
        self.clear_sequence_number();
        self.clear_entity_index();
        self.clear_channel();
        self.clear_pitch();
        self.clear_flags();
        self.clear_sound_num();
        self.clear_sound_num_handle();
        self.clear_speaker_entity();
        self.clear_random_seed();
        self.clear_sound_level();
        self.clear_is_sentence();
        self.clear_is_ambient();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_Sounds_sounddata_t {
    fn eq(&self, other: &CSVCMsg_Sounds_sounddata_t) -> bool {
        self.origin_x == other.origin_x &&
        self.origin_y == other.origin_y &&
        self.origin_z == other.origin_z &&
        self.volume == other.volume &&
        self.delay_value == other.delay_value &&
        self.sequence_number == other.sequence_number &&
        self.entity_index == other.entity_index &&
        self.channel == other.channel &&
        self.pitch == other.pitch &&
        self.flags == other.flags &&
        self.sound_num == other.sound_num &&
        self.sound_num_handle == other.sound_num_handle &&
        self.speaker_entity == other.speaker_entity &&
        self.random_seed == other.random_seed &&
        self.sound_level == other.sound_level &&
        self.is_sentence == other.is_sentence &&
        self.is_ambient == other.is_ambient &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_Sounds_sounddata_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_Prefetch {
    // message fields
    sound_index: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_Prefetch {}

impl CSVCMsg_Prefetch {
    pub fn new() -> CSVCMsg_Prefetch {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_Prefetch {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_Prefetch> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_Prefetch,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_Prefetch {
                    sound_index: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 sound_index = 1;

    pub fn clear_sound_index(&mut self) {
        self.sound_index = ::std::option::Option::None;
    }

    pub fn has_sound_index(&self) -> bool {
        self.sound_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sound_index(&mut self, v: i32) {
        self.sound_index = ::std::option::Option::Some(v);
    }

    pub fn get_sound_index(&self) -> i32 {
        self.sound_index.unwrap_or(0)
    }
}

impl ::protobuf::Message for CSVCMsg_Prefetch {
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
                    self.sound_index = ::std::option::Option::Some(tmp);
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
        for value in &self.sound_index {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sound_index {
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
        ::std::any::TypeId::of::<CSVCMsg_Prefetch>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_Prefetch {
    fn new() -> CSVCMsg_Prefetch {
        CSVCMsg_Prefetch::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_Prefetch>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "sound_index",
                    CSVCMsg_Prefetch::has_sound_index,
                    CSVCMsg_Prefetch::get_sound_index,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_Prefetch>(
                    "CSVCMsg_Prefetch",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_Prefetch {
    fn clear(&mut self) {
        self.clear_sound_index();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_Prefetch {
    fn eq(&self, other: &CSVCMsg_Prefetch) -> bool {
        self.sound_index == other.sound_index &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_Prefetch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_SetView {
    // message fields
    entity_index: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_SetView {}

impl CSVCMsg_SetView {
    pub fn new() -> CSVCMsg_SetView {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_SetView {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_SetView> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_SetView,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_SetView {
                    entity_index: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 entity_index = 1;

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
}

impl ::protobuf::Message for CSVCMsg_SetView {
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
                    self.entity_index = ::std::option::Option::Some(tmp);
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
        for value in &self.entity_index {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.entity_index {
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
        ::std::any::TypeId::of::<CSVCMsg_SetView>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_SetView {
    fn new() -> CSVCMsg_SetView {
        CSVCMsg_SetView::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_SetView>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "entity_index",
                    CSVCMsg_SetView::has_entity_index,
                    CSVCMsg_SetView::get_entity_index,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_SetView>(
                    "CSVCMsg_SetView",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_SetView {
    fn clear(&mut self) {
        self.clear_entity_index();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_SetView {
    fn eq(&self, other: &CSVCMsg_SetView) -> bool {
        self.entity_index == other.entity_index &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_SetView {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_FixAngle {
    // message fields
    relative: ::std::option::Option<bool>,
    angle: ::protobuf::SingularPtrField<CMsgQAngle>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_FixAngle {}

impl CSVCMsg_FixAngle {
    pub fn new() -> CSVCMsg_FixAngle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_FixAngle {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_FixAngle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_FixAngle,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_FixAngle {
                    relative: ::std::option::Option::None,
                    angle: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool relative = 1;

    pub fn clear_relative(&mut self) {
        self.relative = ::std::option::Option::None;
    }

    pub fn has_relative(&self) -> bool {
        self.relative.is_some()
    }

    // Param is passed by value, moved
    pub fn set_relative(&mut self, v: bool) {
        self.relative = ::std::option::Option::Some(v);
    }

    pub fn get_relative(&self) -> bool {
        self.relative.unwrap_or(false)
    }

    // optional .CMsgQAngle angle = 2;

    pub fn clear_angle(&mut self) {
        self.angle.clear();
    }

    pub fn has_angle(&self) -> bool {
        self.angle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_angle(&mut self, v: CMsgQAngle) {
        self.angle = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_angle(&mut self) -> &mut CMsgQAngle {
        if self.angle.is_none() {
            self.angle.set_default();
        };
        self.angle.as_mut().unwrap()
    }

    // Take field
    pub fn take_angle(&mut self) -> CMsgQAngle {
        self.angle.take().unwrap_or_else(|| CMsgQAngle::new())
    }

    pub fn get_angle(&self) -> &CMsgQAngle {
        self.angle.as_ref().unwrap_or_else(|| CMsgQAngle::default_instance())
    }
}

impl ::protobuf::Message for CSVCMsg_FixAngle {
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
                    self.relative = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.angle));
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
        if self.relative.is_some() {
            my_size += 2;
        };
        for value in &self.angle {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.relative {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.angle.as_ref() {
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
        ::std::any::TypeId::of::<CSVCMsg_FixAngle>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_FixAngle {
    fn new() -> CSVCMsg_FixAngle {
        CSVCMsg_FixAngle::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_FixAngle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "relative",
                    CSVCMsg_FixAngle::has_relative,
                    CSVCMsg_FixAngle::get_relative,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "angle",
                    CSVCMsg_FixAngle::has_angle,
                    CSVCMsg_FixAngle::get_angle,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_FixAngle>(
                    "CSVCMsg_FixAngle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_FixAngle {
    fn clear(&mut self) {
        self.clear_relative();
        self.clear_angle();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_FixAngle {
    fn eq(&self, other: &CSVCMsg_FixAngle) -> bool {
        self.relative == other.relative &&
        self.angle == other.angle &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_FixAngle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_CrosshairAngle {
    // message fields
    angle: ::protobuf::SingularPtrField<CMsgQAngle>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_CrosshairAngle {}

impl CSVCMsg_CrosshairAngle {
    pub fn new() -> CSVCMsg_CrosshairAngle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_CrosshairAngle {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_CrosshairAngle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_CrosshairAngle,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_CrosshairAngle {
                    angle: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .CMsgQAngle angle = 1;

    pub fn clear_angle(&mut self) {
        self.angle.clear();
    }

    pub fn has_angle(&self) -> bool {
        self.angle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_angle(&mut self, v: CMsgQAngle) {
        self.angle = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_angle(&mut self) -> &mut CMsgQAngle {
        if self.angle.is_none() {
            self.angle.set_default();
        };
        self.angle.as_mut().unwrap()
    }

    // Take field
    pub fn take_angle(&mut self) -> CMsgQAngle {
        self.angle.take().unwrap_or_else(|| CMsgQAngle::new())
    }

    pub fn get_angle(&self) -> &CMsgQAngle {
        self.angle.as_ref().unwrap_or_else(|| CMsgQAngle::default_instance())
    }
}

impl ::protobuf::Message for CSVCMsg_CrosshairAngle {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.angle));
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
        for value in &self.angle {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.angle.as_ref() {
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
        ::std::any::TypeId::of::<CSVCMsg_CrosshairAngle>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_CrosshairAngle {
    fn new() -> CSVCMsg_CrosshairAngle {
        CSVCMsg_CrosshairAngle::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_CrosshairAngle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "angle",
                    CSVCMsg_CrosshairAngle::has_angle,
                    CSVCMsg_CrosshairAngle::get_angle,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_CrosshairAngle>(
                    "CSVCMsg_CrosshairAngle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_CrosshairAngle {
    fn clear(&mut self) {
        self.clear_angle();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_CrosshairAngle {
    fn eq(&self, other: &CSVCMsg_CrosshairAngle) -> bool {
        self.angle == other.angle &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_CrosshairAngle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_BSPDecal {
    // message fields
    pos: ::protobuf::SingularPtrField<CMsgVector>,
    decal_texture_index: ::std::option::Option<i32>,
    entity_index: ::std::option::Option<i32>,
    model_index: ::std::option::Option<i32>,
    low_priority: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_BSPDecal {}

impl CSVCMsg_BSPDecal {
    pub fn new() -> CSVCMsg_BSPDecal {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_BSPDecal {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_BSPDecal> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_BSPDecal,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_BSPDecal {
                    pos: ::protobuf::SingularPtrField::none(),
                    decal_texture_index: ::std::option::Option::None,
                    entity_index: ::std::option::Option::None,
                    model_index: ::std::option::Option::None,
                    low_priority: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .CMsgVector pos = 1;

    pub fn clear_pos(&mut self) {
        self.pos.clear();
    }

    pub fn has_pos(&self) -> bool {
        self.pos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pos(&mut self, v: CMsgVector) {
        self.pos = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pos(&mut self) -> &mut CMsgVector {
        if self.pos.is_none() {
            self.pos.set_default();
        };
        self.pos.as_mut().unwrap()
    }

    // Take field
    pub fn take_pos(&mut self) -> CMsgVector {
        self.pos.take().unwrap_or_else(|| CMsgVector::new())
    }

    pub fn get_pos(&self) -> &CMsgVector {
        self.pos.as_ref().unwrap_or_else(|| CMsgVector::default_instance())
    }

    // optional int32 decal_texture_index = 2;

    pub fn clear_decal_texture_index(&mut self) {
        self.decal_texture_index = ::std::option::Option::None;
    }

    pub fn has_decal_texture_index(&self) -> bool {
        self.decal_texture_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_decal_texture_index(&mut self, v: i32) {
        self.decal_texture_index = ::std::option::Option::Some(v);
    }

    pub fn get_decal_texture_index(&self) -> i32 {
        self.decal_texture_index.unwrap_or(0)
    }

    // optional int32 entity_index = 3;

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

    // optional int32 model_index = 4;

    pub fn clear_model_index(&mut self) {
        self.model_index = ::std::option::Option::None;
    }

    pub fn has_model_index(&self) -> bool {
        self.model_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_model_index(&mut self, v: i32) {
        self.model_index = ::std::option::Option::Some(v);
    }

    pub fn get_model_index(&self) -> i32 {
        self.model_index.unwrap_or(0)
    }

    // optional bool low_priority = 5;

    pub fn clear_low_priority(&mut self) {
        self.low_priority = ::std::option::Option::None;
    }

    pub fn has_low_priority(&self) -> bool {
        self.low_priority.is_some()
    }

    // Param is passed by value, moved
    pub fn set_low_priority(&mut self, v: bool) {
        self.low_priority = ::std::option::Option::Some(v);
    }

    pub fn get_low_priority(&self) -> bool {
        self.low_priority.unwrap_or(false)
    }
}

impl ::protobuf::Message for CSVCMsg_BSPDecal {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pos));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.decal_texture_index = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.entity_index = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.model_index = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.low_priority = ::std::option::Option::Some(tmp);
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
        for value in &self.pos {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.decal_texture_index {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.entity_index {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.model_index {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.low_priority.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.pos.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.decal_texture_index {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.entity_index {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.model_index {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.low_priority {
            try!(os.write_bool(5, v));
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
        ::std::any::TypeId::of::<CSVCMsg_BSPDecal>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_BSPDecal {
    fn new() -> CSVCMsg_BSPDecal {
        CSVCMsg_BSPDecal::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_BSPDecal>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pos",
                    CSVCMsg_BSPDecal::has_pos,
                    CSVCMsg_BSPDecal::get_pos,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "decal_texture_index",
                    CSVCMsg_BSPDecal::has_decal_texture_index,
                    CSVCMsg_BSPDecal::get_decal_texture_index,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "entity_index",
                    CSVCMsg_BSPDecal::has_entity_index,
                    CSVCMsg_BSPDecal::get_entity_index,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "model_index",
                    CSVCMsg_BSPDecal::has_model_index,
                    CSVCMsg_BSPDecal::get_model_index,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "low_priority",
                    CSVCMsg_BSPDecal::has_low_priority,
                    CSVCMsg_BSPDecal::get_low_priority,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_BSPDecal>(
                    "CSVCMsg_BSPDecal",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_BSPDecal {
    fn clear(&mut self) {
        self.clear_pos();
        self.clear_decal_texture_index();
        self.clear_entity_index();
        self.clear_model_index();
        self.clear_low_priority();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_BSPDecal {
    fn eq(&self, other: &CSVCMsg_BSPDecal) -> bool {
        self.pos == other.pos &&
        self.decal_texture_index == other.decal_texture_index &&
        self.entity_index == other.entity_index &&
        self.model_index == other.model_index &&
        self.low_priority == other.low_priority &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_BSPDecal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_SplitScreen {
    // message fields
    field_type: ::std::option::Option<ESplitScreenMessageType>,
    slot: ::std::option::Option<i32>,
    player_index: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_SplitScreen {}

impl CSVCMsg_SplitScreen {
    pub fn new() -> CSVCMsg_SplitScreen {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_SplitScreen {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_SplitScreen> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_SplitScreen,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_SplitScreen {
                    field_type: ::std::option::Option::None,
                    slot: ::std::option::Option::None,
                    player_index: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .ESplitScreenMessageType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ESplitScreenMessageType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> ESplitScreenMessageType {
        self.field_type.unwrap_or(ESplitScreenMessageType::MSG_SPLITSCREEN_ADDUSER)
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

    // optional int32 player_index = 3;

    pub fn clear_player_index(&mut self) {
        self.player_index = ::std::option::Option::None;
    }

    pub fn has_player_index(&self) -> bool {
        self.player_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_index(&mut self, v: i32) {
        self.player_index = ::std::option::Option::Some(v);
    }

    pub fn get_player_index(&self) -> i32 {
        self.player_index.unwrap_or(0)
    }
}

impl ::protobuf::Message for CSVCMsg_SplitScreen {
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
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.slot = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.player_index = ::std::option::Option::Some(tmp);
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
        for value in &self.slot {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.player_index {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.slot {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.player_index {
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
        ::std::any::TypeId::of::<CSVCMsg_SplitScreen>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_SplitScreen {
    fn new() -> CSVCMsg_SplitScreen {
        CSVCMsg_SplitScreen::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_SplitScreen>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    CSVCMsg_SplitScreen::has_field_type,
                    CSVCMsg_SplitScreen::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "slot",
                    CSVCMsg_SplitScreen::has_slot,
                    CSVCMsg_SplitScreen::get_slot,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "player_index",
                    CSVCMsg_SplitScreen::has_player_index,
                    CSVCMsg_SplitScreen::get_player_index,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_SplitScreen>(
                    "CSVCMsg_SplitScreen",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_SplitScreen {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_slot();
        self.clear_player_index();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_SplitScreen {
    fn eq(&self, other: &CSVCMsg_SplitScreen) -> bool {
        self.field_type == other.field_type &&
        self.slot == other.slot &&
        self.player_index == other.player_index &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_SplitScreen {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_GetCvarValue {
    // message fields
    cookie: ::std::option::Option<i32>,
    cvar_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_GetCvarValue {}

impl CSVCMsg_GetCvarValue {
    pub fn new() -> CSVCMsg_GetCvarValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_GetCvarValue {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_GetCvarValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_GetCvarValue,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_GetCvarValue {
                    cookie: ::std::option::Option::None,
                    cvar_name: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 cookie = 1;

    pub fn clear_cookie(&mut self) {
        self.cookie = ::std::option::Option::None;
    }

    pub fn has_cookie(&self) -> bool {
        self.cookie.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cookie(&mut self, v: i32) {
        self.cookie = ::std::option::Option::Some(v);
    }

    pub fn get_cookie(&self) -> i32 {
        self.cookie.unwrap_or(0)
    }

    // optional string cvar_name = 2;

    pub fn clear_cvar_name(&mut self) {
        self.cvar_name.clear();
    }

    pub fn has_cvar_name(&self) -> bool {
        self.cvar_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cvar_name(&mut self, v: ::std::string::String) {
        self.cvar_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cvar_name(&mut self) -> &mut ::std::string::String {
        if self.cvar_name.is_none() {
            self.cvar_name.set_default();
        };
        self.cvar_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_cvar_name(&mut self) -> ::std::string::String {
        self.cvar_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_cvar_name(&self) -> &str {
        match self.cvar_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for CSVCMsg_GetCvarValue {
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
                    self.cookie = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.cvar_name));
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
        for value in &self.cookie {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.cvar_name {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cookie {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.cvar_name.as_ref() {
            try!(os.write_string(2, &v));
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
        ::std::any::TypeId::of::<CSVCMsg_GetCvarValue>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_GetCvarValue {
    fn new() -> CSVCMsg_GetCvarValue {
        CSVCMsg_GetCvarValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_GetCvarValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "cookie",
                    CSVCMsg_GetCvarValue::has_cookie,
                    CSVCMsg_GetCvarValue::get_cookie,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "cvar_name",
                    CSVCMsg_GetCvarValue::has_cvar_name,
                    CSVCMsg_GetCvarValue::get_cvar_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_GetCvarValue>(
                    "CSVCMsg_GetCvarValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_GetCvarValue {
    fn clear(&mut self) {
        self.clear_cookie();
        self.clear_cvar_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_GetCvarValue {
    fn eq(&self, other: &CSVCMsg_GetCvarValue) -> bool {
        self.cookie == other.cookie &&
        self.cvar_name == other.cvar_name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_GetCvarValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_Menu {
    // message fields
    dialog_type: ::std::option::Option<i32>,
    menu_key_values: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_Menu {}

impl CSVCMsg_Menu {
    pub fn new() -> CSVCMsg_Menu {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_Menu {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_Menu> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_Menu,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_Menu {
                    dialog_type: ::std::option::Option::None,
                    menu_key_values: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 dialog_type = 1;

    pub fn clear_dialog_type(&mut self) {
        self.dialog_type = ::std::option::Option::None;
    }

    pub fn has_dialog_type(&self) -> bool {
        self.dialog_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dialog_type(&mut self, v: i32) {
        self.dialog_type = ::std::option::Option::Some(v);
    }

    pub fn get_dialog_type(&self) -> i32 {
        self.dialog_type.unwrap_or(0)
    }

    // optional bytes menu_key_values = 2;

    pub fn clear_menu_key_values(&mut self) {
        self.menu_key_values.clear();
    }

    pub fn has_menu_key_values(&self) -> bool {
        self.menu_key_values.is_some()
    }

    // Param is passed by value, moved
    pub fn set_menu_key_values(&mut self, v: ::std::vec::Vec<u8>) {
        self.menu_key_values = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_menu_key_values(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.menu_key_values.is_none() {
            self.menu_key_values.set_default();
        };
        self.menu_key_values.as_mut().unwrap()
    }

    // Take field
    pub fn take_menu_key_values(&mut self) -> ::std::vec::Vec<u8> {
        self.menu_key_values.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_menu_key_values(&self) -> &[u8] {
        match self.menu_key_values.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for CSVCMsg_Menu {
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
                    self.dialog_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.menu_key_values));
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
        for value in &self.dialog_type {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.menu_key_values {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dialog_type {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.menu_key_values.as_ref() {
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
        ::std::any::TypeId::of::<CSVCMsg_Menu>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_Menu {
    fn new() -> CSVCMsg_Menu {
        CSVCMsg_Menu::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_Menu>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "dialog_type",
                    CSVCMsg_Menu::has_dialog_type,
                    CSVCMsg_Menu::get_dialog_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "menu_key_values",
                    CSVCMsg_Menu::has_menu_key_values,
                    CSVCMsg_Menu::get_menu_key_values,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_Menu>(
                    "CSVCMsg_Menu",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_Menu {
    fn clear(&mut self) {
        self.clear_dialog_type();
        self.clear_menu_key_values();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_Menu {
    fn eq(&self, other: &CSVCMsg_Menu) -> bool {
        self.dialog_type == other.dialog_type &&
        self.menu_key_values == other.menu_key_values &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_Menu {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_SendTable {
    // message fields
    is_end: ::std::option::Option<bool>,
    net_table_name: ::protobuf::SingularField<::std::string::String>,
    needs_decoder: ::std::option::Option<bool>,
    props: ::protobuf::RepeatedField<CSVCMsg_SendTable_sendprop_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_SendTable {}

impl CSVCMsg_SendTable {
    pub fn new() -> CSVCMsg_SendTable {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_SendTable {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_SendTable> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_SendTable,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_SendTable {
                    is_end: ::std::option::Option::None,
                    net_table_name: ::protobuf::SingularField::none(),
                    needs_decoder: ::std::option::Option::None,
                    props: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool is_end = 1;

    pub fn clear_is_end(&mut self) {
        self.is_end = ::std::option::Option::None;
    }

    pub fn has_is_end(&self) -> bool {
        self.is_end.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_end(&mut self, v: bool) {
        self.is_end = ::std::option::Option::Some(v);
    }

    pub fn get_is_end(&self) -> bool {
        self.is_end.unwrap_or(false)
    }

    // optional string net_table_name = 2;

    pub fn clear_net_table_name(&mut self) {
        self.net_table_name.clear();
    }

    pub fn has_net_table_name(&self) -> bool {
        self.net_table_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_net_table_name(&mut self, v: ::std::string::String) {
        self.net_table_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_net_table_name(&mut self) -> &mut ::std::string::String {
        if self.net_table_name.is_none() {
            self.net_table_name.set_default();
        };
        self.net_table_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_net_table_name(&mut self) -> ::std::string::String {
        self.net_table_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_net_table_name(&self) -> &str {
        match self.net_table_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bool needs_decoder = 3;

    pub fn clear_needs_decoder(&mut self) {
        self.needs_decoder = ::std::option::Option::None;
    }

    pub fn has_needs_decoder(&self) -> bool {
        self.needs_decoder.is_some()
    }

    // Param is passed by value, moved
    pub fn set_needs_decoder(&mut self, v: bool) {
        self.needs_decoder = ::std::option::Option::Some(v);
    }

    pub fn get_needs_decoder(&self) -> bool {
        self.needs_decoder.unwrap_or(false)
    }

    // repeated .CSVCMsg_SendTable.sendprop_t props = 4;

    pub fn clear_props(&mut self) {
        self.props.clear();
    }

    // Param is passed by value, moved
    pub fn set_props(&mut self, v: ::protobuf::RepeatedField<CSVCMsg_SendTable_sendprop_t>) {
        self.props = v;
    }

    // Mutable pointer to the field.
    pub fn mut_props(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_SendTable_sendprop_t> {
        &mut self.props
    }

    // Take field
    pub fn take_props(&mut self) -> ::protobuf::RepeatedField<CSVCMsg_SendTable_sendprop_t> {
        ::std::mem::replace(&mut self.props, ::protobuf::RepeatedField::new())
    }

    pub fn get_props(&self) -> &[CSVCMsg_SendTable_sendprop_t] {
        &self.props
    }
}

impl ::protobuf::Message for CSVCMsg_SendTable {
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
                    self.is_end = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.net_table_name));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.needs_decoder = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.props));
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
        if self.is_end.is_some() {
            my_size += 2;
        };
        for value in &self.net_table_name {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if self.needs_decoder.is_some() {
            my_size += 2;
        };
        for value in &self.props {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.is_end {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.net_table_name.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.needs_decoder {
            try!(os.write_bool(3, v));
        };
        for v in &self.props {
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
        ::std::any::TypeId::of::<CSVCMsg_SendTable>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_SendTable {
    fn new() -> CSVCMsg_SendTable {
        CSVCMsg_SendTable::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_SendTable>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "is_end",
                    CSVCMsg_SendTable::has_is_end,
                    CSVCMsg_SendTable::get_is_end,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "net_table_name",
                    CSVCMsg_SendTable::has_net_table_name,
                    CSVCMsg_SendTable::get_net_table_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "needs_decoder",
                    CSVCMsg_SendTable::has_needs_decoder,
                    CSVCMsg_SendTable::get_needs_decoder,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "props",
                    CSVCMsg_SendTable::get_props,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_SendTable>(
                    "CSVCMsg_SendTable",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_SendTable {
    fn clear(&mut self) {
        self.clear_is_end();
        self.clear_net_table_name();
        self.clear_needs_decoder();
        self.clear_props();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_SendTable {
    fn eq(&self, other: &CSVCMsg_SendTable) -> bool {
        self.is_end == other.is_end &&
        self.net_table_name == other.net_table_name &&
        self.needs_decoder == other.needs_decoder &&
        self.props == other.props &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_SendTable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_SendTable_sendprop_t {
    // message fields
    field_type: ::std::option::Option<i32>,
    var_name: ::protobuf::SingularField<::std::string::String>,
    flags: ::std::option::Option<i32>,
    priority: ::std::option::Option<i32>,
    dt_name: ::protobuf::SingularField<::std::string::String>,
    num_elements: ::std::option::Option<i32>,
    low_value: ::std::option::Option<f32>,
    high_value: ::std::option::Option<f32>,
    num_bits: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_SendTable_sendprop_t {}

impl CSVCMsg_SendTable_sendprop_t {
    pub fn new() -> CSVCMsg_SendTable_sendprop_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_SendTable_sendprop_t {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_SendTable_sendprop_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_SendTable_sendprop_t,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_SendTable_sendprop_t {
                    field_type: ::std::option::Option::None,
                    var_name: ::protobuf::SingularField::none(),
                    flags: ::std::option::Option::None,
                    priority: ::std::option::Option::None,
                    dt_name: ::protobuf::SingularField::none(),
                    num_elements: ::std::option::Option::None,
                    low_value: ::std::option::Option::None,
                    high_value: ::std::option::Option::None,
                    num_bits: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: i32) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> i32 {
        self.field_type.unwrap_or(0)
    }

    // optional string var_name = 2;

    pub fn clear_var_name(&mut self) {
        self.var_name.clear();
    }

    pub fn has_var_name(&self) -> bool {
        self.var_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_var_name(&mut self, v: ::std::string::String) {
        self.var_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_var_name(&mut self) -> &mut ::std::string::String {
        if self.var_name.is_none() {
            self.var_name.set_default();
        };
        self.var_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_var_name(&mut self) -> ::std::string::String {
        self.var_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_var_name(&self) -> &str {
        match self.var_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional int32 flags = 3;

    pub fn clear_flags(&mut self) {
        self.flags = ::std::option::Option::None;
    }

    pub fn has_flags(&self) -> bool {
        self.flags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flags(&mut self, v: i32) {
        self.flags = ::std::option::Option::Some(v);
    }

    pub fn get_flags(&self) -> i32 {
        self.flags.unwrap_or(0)
    }

    // optional int32 priority = 4;

    pub fn clear_priority(&mut self) {
        self.priority = ::std::option::Option::None;
    }

    pub fn has_priority(&self) -> bool {
        self.priority.is_some()
    }

    // Param is passed by value, moved
    pub fn set_priority(&mut self, v: i32) {
        self.priority = ::std::option::Option::Some(v);
    }

    pub fn get_priority(&self) -> i32 {
        self.priority.unwrap_or(0)
    }

    // optional string dt_name = 5;

    pub fn clear_dt_name(&mut self) {
        self.dt_name.clear();
    }

    pub fn has_dt_name(&self) -> bool {
        self.dt_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dt_name(&mut self, v: ::std::string::String) {
        self.dt_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dt_name(&mut self) -> &mut ::std::string::String {
        if self.dt_name.is_none() {
            self.dt_name.set_default();
        };
        self.dt_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_dt_name(&mut self) -> ::std::string::String {
        self.dt_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_dt_name(&self) -> &str {
        match self.dt_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional int32 num_elements = 6;

    pub fn clear_num_elements(&mut self) {
        self.num_elements = ::std::option::Option::None;
    }

    pub fn has_num_elements(&self) -> bool {
        self.num_elements.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_elements(&mut self, v: i32) {
        self.num_elements = ::std::option::Option::Some(v);
    }

    pub fn get_num_elements(&self) -> i32 {
        self.num_elements.unwrap_or(0)
    }

    // optional float low_value = 7;

    pub fn clear_low_value(&mut self) {
        self.low_value = ::std::option::Option::None;
    }

    pub fn has_low_value(&self) -> bool {
        self.low_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_low_value(&mut self, v: f32) {
        self.low_value = ::std::option::Option::Some(v);
    }

    pub fn get_low_value(&self) -> f32 {
        self.low_value.unwrap_or(0.)
    }

    // optional float high_value = 8;

    pub fn clear_high_value(&mut self) {
        self.high_value = ::std::option::Option::None;
    }

    pub fn has_high_value(&self) -> bool {
        self.high_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_high_value(&mut self, v: f32) {
        self.high_value = ::std::option::Option::Some(v);
    }

    pub fn get_high_value(&self) -> f32 {
        self.high_value.unwrap_or(0.)
    }

    // optional int32 num_bits = 9;

    pub fn clear_num_bits(&mut self) {
        self.num_bits = ::std::option::Option::None;
    }

    pub fn has_num_bits(&self) -> bool {
        self.num_bits.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_bits(&mut self, v: i32) {
        self.num_bits = ::std::option::Option::Some(v);
    }

    pub fn get_num_bits(&self) -> i32 {
        self.num_bits.unwrap_or(0)
    }
}

impl ::protobuf::Message for CSVCMsg_SendTable_sendprop_t {
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
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.var_name));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.flags = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.priority = ::std::option::Option::Some(tmp);
                },
                5 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.dt_name));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.num_elements = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.low_value = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.high_value = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.num_bits = ::std::option::Option::Some(tmp);
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
        for value in &self.var_name {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.flags {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.priority {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.dt_name {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        for value in &self.num_elements {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.low_value.is_some() {
            my_size += 5;
        };
        if self.high_value.is_some() {
            my_size += 5;
        };
        for value in &self.num_bits {
            my_size += ::protobuf::rt::value_size(9, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.var_name.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.flags {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.priority {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.dt_name.as_ref() {
            try!(os.write_string(5, &v));
        };
        if let Some(v) = self.num_elements {
            try!(os.write_int32(6, v));
        };
        if let Some(v) = self.low_value {
            try!(os.write_float(7, v));
        };
        if let Some(v) = self.high_value {
            try!(os.write_float(8, v));
        };
        if let Some(v) = self.num_bits {
            try!(os.write_int32(9, v));
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
        ::std::any::TypeId::of::<CSVCMsg_SendTable_sendprop_t>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_SendTable_sendprop_t {
    fn new() -> CSVCMsg_SendTable_sendprop_t {
        CSVCMsg_SendTable_sendprop_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_SendTable_sendprop_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "type",
                    CSVCMsg_SendTable_sendprop_t::has_field_type,
                    CSVCMsg_SendTable_sendprop_t::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "var_name",
                    CSVCMsg_SendTable_sendprop_t::has_var_name,
                    CSVCMsg_SendTable_sendprop_t::get_var_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "flags",
                    CSVCMsg_SendTable_sendprop_t::has_flags,
                    CSVCMsg_SendTable_sendprop_t::get_flags,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "priority",
                    CSVCMsg_SendTable_sendprop_t::has_priority,
                    CSVCMsg_SendTable_sendprop_t::get_priority,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "dt_name",
                    CSVCMsg_SendTable_sendprop_t::has_dt_name,
                    CSVCMsg_SendTable_sendprop_t::get_dt_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "num_elements",
                    CSVCMsg_SendTable_sendprop_t::has_num_elements,
                    CSVCMsg_SendTable_sendprop_t::get_num_elements,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "low_value",
                    CSVCMsg_SendTable_sendprop_t::has_low_value,
                    CSVCMsg_SendTable_sendprop_t::get_low_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "high_value",
                    CSVCMsg_SendTable_sendprop_t::has_high_value,
                    CSVCMsg_SendTable_sendprop_t::get_high_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "num_bits",
                    CSVCMsg_SendTable_sendprop_t::has_num_bits,
                    CSVCMsg_SendTable_sendprop_t::get_num_bits,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_SendTable_sendprop_t>(
                    "CSVCMsg_SendTable_sendprop_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_SendTable_sendprop_t {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_var_name();
        self.clear_flags();
        self.clear_priority();
        self.clear_dt_name();
        self.clear_num_elements();
        self.clear_low_value();
        self.clear_high_value();
        self.clear_num_bits();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_SendTable_sendprop_t {
    fn eq(&self, other: &CSVCMsg_SendTable_sendprop_t) -> bool {
        self.field_type == other.field_type &&
        self.var_name == other.var_name &&
        self.flags == other.flags &&
        self.priority == other.priority &&
        self.dt_name == other.dt_name &&
        self.num_elements == other.num_elements &&
        self.low_value == other.low_value &&
        self.high_value == other.high_value &&
        self.num_bits == other.num_bits &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_SendTable_sendprop_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_GameEvent {
    // message fields
    event_name: ::protobuf::SingularField<::std::string::String>,
    eventid: ::std::option::Option<i32>,
    keys: ::protobuf::RepeatedField<CSVCMsg_GameEvent_key_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_GameEvent {}

impl CSVCMsg_GameEvent {
    pub fn new() -> CSVCMsg_GameEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_GameEvent {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_GameEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_GameEvent,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_GameEvent {
                    event_name: ::protobuf::SingularField::none(),
                    eventid: ::std::option::Option::None,
                    keys: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string event_name = 1;

    pub fn clear_event_name(&mut self) {
        self.event_name.clear();
    }

    pub fn has_event_name(&self) -> bool {
        self.event_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_name(&mut self, v: ::std::string::String) {
        self.event_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event_name(&mut self) -> &mut ::std::string::String {
        if self.event_name.is_none() {
            self.event_name.set_default();
        };
        self.event_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_event_name(&mut self) -> ::std::string::String {
        self.event_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_event_name(&self) -> &str {
        match self.event_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional int32 eventid = 2;

    pub fn clear_eventid(&mut self) {
        self.eventid = ::std::option::Option::None;
    }

    pub fn has_eventid(&self) -> bool {
        self.eventid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eventid(&mut self, v: i32) {
        self.eventid = ::std::option::Option::Some(v);
    }

    pub fn get_eventid(&self) -> i32 {
        self.eventid.unwrap_or(0)
    }

    // repeated .CSVCMsg_GameEvent.key_t keys = 3;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<CSVCMsg_GameEvent_key_t>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_GameEvent_key_t> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<CSVCMsg_GameEvent_key_t> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[CSVCMsg_GameEvent_key_t] {
        &self.keys
    }
}

impl ::protobuf::Message for CSVCMsg_GameEvent {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.event_name));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.eventid = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.keys));
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
        for value in &self.event_name {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.eventid {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.keys {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.event_name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.eventid {
            try!(os.write_int32(2, v));
        };
        for v in &self.keys {
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
        ::std::any::TypeId::of::<CSVCMsg_GameEvent>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_GameEvent {
    fn new() -> CSVCMsg_GameEvent {
        CSVCMsg_GameEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_GameEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "event_name",
                    CSVCMsg_GameEvent::has_event_name,
                    CSVCMsg_GameEvent::get_event_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "eventid",
                    CSVCMsg_GameEvent::has_eventid,
                    CSVCMsg_GameEvent::get_eventid,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "keys",
                    CSVCMsg_GameEvent::get_keys,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_GameEvent>(
                    "CSVCMsg_GameEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_GameEvent {
    fn clear(&mut self) {
        self.clear_event_name();
        self.clear_eventid();
        self.clear_keys();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_GameEvent {
    fn eq(&self, other: &CSVCMsg_GameEvent) -> bool {
        self.event_name == other.event_name &&
        self.eventid == other.eventid &&
        self.keys == other.keys &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_GameEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_GameEvent_key_t {
    // message fields
    field_type: ::std::option::Option<i32>,
    val_string: ::protobuf::SingularField<::std::string::String>,
    val_float: ::std::option::Option<f32>,
    val_long: ::std::option::Option<i32>,
    val_short: ::std::option::Option<i32>,
    val_byte: ::std::option::Option<i32>,
    val_bool: ::std::option::Option<bool>,
    val_uint64: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_GameEvent_key_t {}

impl CSVCMsg_GameEvent_key_t {
    pub fn new() -> CSVCMsg_GameEvent_key_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_GameEvent_key_t {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_GameEvent_key_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_GameEvent_key_t,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_GameEvent_key_t {
                    field_type: ::std::option::Option::None,
                    val_string: ::protobuf::SingularField::none(),
                    val_float: ::std::option::Option::None,
                    val_long: ::std::option::Option::None,
                    val_short: ::std::option::Option::None,
                    val_byte: ::std::option::Option::None,
                    val_bool: ::std::option::Option::None,
                    val_uint64: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: i32) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> i32 {
        self.field_type.unwrap_or(0)
    }

    // optional string val_string = 2;

    pub fn clear_val_string(&mut self) {
        self.val_string.clear();
    }

    pub fn has_val_string(&self) -> bool {
        self.val_string.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val_string(&mut self, v: ::std::string::String) {
        self.val_string = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_val_string(&mut self) -> &mut ::std::string::String {
        if self.val_string.is_none() {
            self.val_string.set_default();
        };
        self.val_string.as_mut().unwrap()
    }

    // Take field
    pub fn take_val_string(&mut self) -> ::std::string::String {
        self.val_string.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_val_string(&self) -> &str {
        match self.val_string.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional float val_float = 3;

    pub fn clear_val_float(&mut self) {
        self.val_float = ::std::option::Option::None;
    }

    pub fn has_val_float(&self) -> bool {
        self.val_float.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val_float(&mut self, v: f32) {
        self.val_float = ::std::option::Option::Some(v);
    }

    pub fn get_val_float(&self) -> f32 {
        self.val_float.unwrap_or(0.)
    }

    // optional int32 val_long = 4;

    pub fn clear_val_long(&mut self) {
        self.val_long = ::std::option::Option::None;
    }

    pub fn has_val_long(&self) -> bool {
        self.val_long.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val_long(&mut self, v: i32) {
        self.val_long = ::std::option::Option::Some(v);
    }

    pub fn get_val_long(&self) -> i32 {
        self.val_long.unwrap_or(0)
    }

    // optional int32 val_short = 5;

    pub fn clear_val_short(&mut self) {
        self.val_short = ::std::option::Option::None;
    }

    pub fn has_val_short(&self) -> bool {
        self.val_short.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val_short(&mut self, v: i32) {
        self.val_short = ::std::option::Option::Some(v);
    }

    pub fn get_val_short(&self) -> i32 {
        self.val_short.unwrap_or(0)
    }

    // optional int32 val_byte = 6;

    pub fn clear_val_byte(&mut self) {
        self.val_byte = ::std::option::Option::None;
    }

    pub fn has_val_byte(&self) -> bool {
        self.val_byte.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val_byte(&mut self, v: i32) {
        self.val_byte = ::std::option::Option::Some(v);
    }

    pub fn get_val_byte(&self) -> i32 {
        self.val_byte.unwrap_or(0)
    }

    // optional bool val_bool = 7;

    pub fn clear_val_bool(&mut self) {
        self.val_bool = ::std::option::Option::None;
    }

    pub fn has_val_bool(&self) -> bool {
        self.val_bool.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val_bool(&mut self, v: bool) {
        self.val_bool = ::std::option::Option::Some(v);
    }

    pub fn get_val_bool(&self) -> bool {
        self.val_bool.unwrap_or(false)
    }

    // optional uint64 val_uint64 = 8;

    pub fn clear_val_uint64(&mut self) {
        self.val_uint64 = ::std::option::Option::None;
    }

    pub fn has_val_uint64(&self) -> bool {
        self.val_uint64.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val_uint64(&mut self, v: u64) {
        self.val_uint64 = ::std::option::Option::Some(v);
    }

    pub fn get_val_uint64(&self) -> u64 {
        self.val_uint64.unwrap_or(0)
    }
}

impl ::protobuf::Message for CSVCMsg_GameEvent_key_t {
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
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.val_string));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.val_float = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.val_long = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.val_short = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.val_byte = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.val_bool = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.val_uint64 = ::std::option::Option::Some(tmp);
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
        for value in &self.val_string {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if self.val_float.is_some() {
            my_size += 5;
        };
        for value in &self.val_long {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.val_short {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.val_byte {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.val_bool.is_some() {
            my_size += 2;
        };
        for value in &self.val_uint64 {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.val_string.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.val_float {
            try!(os.write_float(3, v));
        };
        if let Some(v) = self.val_long {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.val_short {
            try!(os.write_int32(5, v));
        };
        if let Some(v) = self.val_byte {
            try!(os.write_int32(6, v));
        };
        if let Some(v) = self.val_bool {
            try!(os.write_bool(7, v));
        };
        if let Some(v) = self.val_uint64 {
            try!(os.write_uint64(8, v));
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
        ::std::any::TypeId::of::<CSVCMsg_GameEvent_key_t>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_GameEvent_key_t {
    fn new() -> CSVCMsg_GameEvent_key_t {
        CSVCMsg_GameEvent_key_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_GameEvent_key_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "type",
                    CSVCMsg_GameEvent_key_t::has_field_type,
                    CSVCMsg_GameEvent_key_t::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "val_string",
                    CSVCMsg_GameEvent_key_t::has_val_string,
                    CSVCMsg_GameEvent_key_t::get_val_string,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "val_float",
                    CSVCMsg_GameEvent_key_t::has_val_float,
                    CSVCMsg_GameEvent_key_t::get_val_float,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "val_long",
                    CSVCMsg_GameEvent_key_t::has_val_long,
                    CSVCMsg_GameEvent_key_t::get_val_long,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "val_short",
                    CSVCMsg_GameEvent_key_t::has_val_short,
                    CSVCMsg_GameEvent_key_t::get_val_short,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "val_byte",
                    CSVCMsg_GameEvent_key_t::has_val_byte,
                    CSVCMsg_GameEvent_key_t::get_val_byte,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "val_bool",
                    CSVCMsg_GameEvent_key_t::has_val_bool,
                    CSVCMsg_GameEvent_key_t::get_val_bool,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "val_uint64",
                    CSVCMsg_GameEvent_key_t::has_val_uint64,
                    CSVCMsg_GameEvent_key_t::get_val_uint64,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_GameEvent_key_t>(
                    "CSVCMsg_GameEvent_key_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_GameEvent_key_t {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_val_string();
        self.clear_val_float();
        self.clear_val_long();
        self.clear_val_short();
        self.clear_val_byte();
        self.clear_val_bool();
        self.clear_val_uint64();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_GameEvent_key_t {
    fn eq(&self, other: &CSVCMsg_GameEvent_key_t) -> bool {
        self.field_type == other.field_type &&
        self.val_string == other.val_string &&
        self.val_float == other.val_float &&
        self.val_long == other.val_long &&
        self.val_short == other.val_short &&
        self.val_byte == other.val_byte &&
        self.val_bool == other.val_bool &&
        self.val_uint64 == other.val_uint64 &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_GameEvent_key_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_GameEventList {
    // message fields
    descriptors: ::protobuf::RepeatedField<CSVCMsg_GameEventList_descriptor_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_GameEventList {}

impl CSVCMsg_GameEventList {
    pub fn new() -> CSVCMsg_GameEventList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_GameEventList {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_GameEventList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_GameEventList,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_GameEventList {
                    descriptors: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .CSVCMsg_GameEventList.descriptor_t descriptors = 1;

    pub fn clear_descriptors(&mut self) {
        self.descriptors.clear();
    }

    // Param is passed by value, moved
    pub fn set_descriptors(&mut self, v: ::protobuf::RepeatedField<CSVCMsg_GameEventList_descriptor_t>) {
        self.descriptors = v;
    }

    // Mutable pointer to the field.
    pub fn mut_descriptors(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_GameEventList_descriptor_t> {
        &mut self.descriptors
    }

    // Take field
    pub fn take_descriptors(&mut self) -> ::protobuf::RepeatedField<CSVCMsg_GameEventList_descriptor_t> {
        ::std::mem::replace(&mut self.descriptors, ::protobuf::RepeatedField::new())
    }

    pub fn get_descriptors(&self) -> &[CSVCMsg_GameEventList_descriptor_t] {
        &self.descriptors
    }
}

impl ::protobuf::Message for CSVCMsg_GameEventList {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.descriptors));
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
        for value in &self.descriptors {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.descriptors {
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
        ::std::any::TypeId::of::<CSVCMsg_GameEventList>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_GameEventList {
    fn new() -> CSVCMsg_GameEventList {
        CSVCMsg_GameEventList::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_GameEventList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "descriptors",
                    CSVCMsg_GameEventList::get_descriptors,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_GameEventList>(
                    "CSVCMsg_GameEventList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_GameEventList {
    fn clear(&mut self) {
        self.clear_descriptors();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_GameEventList {
    fn eq(&self, other: &CSVCMsg_GameEventList) -> bool {
        self.descriptors == other.descriptors &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_GameEventList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_GameEventList_key_t {
    // message fields
    field_type: ::std::option::Option<i32>,
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_GameEventList_key_t {}

impl CSVCMsg_GameEventList_key_t {
    pub fn new() -> CSVCMsg_GameEventList_key_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_GameEventList_key_t {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_GameEventList_key_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_GameEventList_key_t,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_GameEventList_key_t {
                    field_type: ::std::option::Option::None,
                    name: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: i32) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> i32 {
        self.field_type.unwrap_or(0)
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
}

impl ::protobuf::Message for CSVCMsg_GameEventList_key_t {
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
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
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
        for value in &self.field_type {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(2, &v));
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
        ::std::any::TypeId::of::<CSVCMsg_GameEventList_key_t>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_GameEventList_key_t {
    fn new() -> CSVCMsg_GameEventList_key_t {
        CSVCMsg_GameEventList_key_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_GameEventList_key_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "type",
                    CSVCMsg_GameEventList_key_t::has_field_type,
                    CSVCMsg_GameEventList_key_t::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    CSVCMsg_GameEventList_key_t::has_name,
                    CSVCMsg_GameEventList_key_t::get_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_GameEventList_key_t>(
                    "CSVCMsg_GameEventList_key_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_GameEventList_key_t {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_GameEventList_key_t {
    fn eq(&self, other: &CSVCMsg_GameEventList_key_t) -> bool {
        self.field_type == other.field_type &&
        self.name == other.name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_GameEventList_key_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_GameEventList_descriptor_t {
    // message fields
    eventid: ::std::option::Option<i32>,
    name: ::protobuf::SingularField<::std::string::String>,
    keys: ::protobuf::RepeatedField<CSVCMsg_GameEventList_key_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_GameEventList_descriptor_t {}

impl CSVCMsg_GameEventList_descriptor_t {
    pub fn new() -> CSVCMsg_GameEventList_descriptor_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_GameEventList_descriptor_t {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_GameEventList_descriptor_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_GameEventList_descriptor_t,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_GameEventList_descriptor_t {
                    eventid: ::std::option::Option::None,
                    name: ::protobuf::SingularField::none(),
                    keys: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 eventid = 1;

    pub fn clear_eventid(&mut self) {
        self.eventid = ::std::option::Option::None;
    }

    pub fn has_eventid(&self) -> bool {
        self.eventid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eventid(&mut self, v: i32) {
        self.eventid = ::std::option::Option::Some(v);
    }

    pub fn get_eventid(&self) -> i32 {
        self.eventid.unwrap_or(0)
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

    // repeated .CSVCMsg_GameEventList.key_t keys = 3;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<CSVCMsg_GameEventList_key_t>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_GameEventList_key_t> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<CSVCMsg_GameEventList_key_t> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[CSVCMsg_GameEventList_key_t] {
        &self.keys
    }
}

impl ::protobuf::Message for CSVCMsg_GameEventList_descriptor_t {
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
                    self.eventid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.keys));
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
        for value in &self.eventid {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.keys {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.eventid {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(2, &v));
        };
        for v in &self.keys {
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
        ::std::any::TypeId::of::<CSVCMsg_GameEventList_descriptor_t>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_GameEventList_descriptor_t {
    fn new() -> CSVCMsg_GameEventList_descriptor_t {
        CSVCMsg_GameEventList_descriptor_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_GameEventList_descriptor_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "eventid",
                    CSVCMsg_GameEventList_descriptor_t::has_eventid,
                    CSVCMsg_GameEventList_descriptor_t::get_eventid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    CSVCMsg_GameEventList_descriptor_t::has_name,
                    CSVCMsg_GameEventList_descriptor_t::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "keys",
                    CSVCMsg_GameEventList_descriptor_t::get_keys,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_GameEventList_descriptor_t>(
                    "CSVCMsg_GameEventList_descriptor_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_GameEventList_descriptor_t {
    fn clear(&mut self) {
        self.clear_eventid();
        self.clear_name();
        self.clear_keys();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_GameEventList_descriptor_t {
    fn eq(&self, other: &CSVCMsg_GameEventList_descriptor_t) -> bool {
        self.eventid == other.eventid &&
        self.name == other.name &&
        self.keys == other.keys &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_GameEventList_descriptor_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_PacketEntities {
    // message fields
    max_entries: ::std::option::Option<i32>,
    updated_entries: ::std::option::Option<i32>,
    is_delta: ::std::option::Option<bool>,
    update_baseline: ::std::option::Option<bool>,
    baseline: ::std::option::Option<i32>,
    delta_from: ::std::option::Option<i32>,
    entity_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_PacketEntities {}

impl CSVCMsg_PacketEntities {
    pub fn new() -> CSVCMsg_PacketEntities {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_PacketEntities {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_PacketEntities> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_PacketEntities,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_PacketEntities {
                    max_entries: ::std::option::Option::None,
                    updated_entries: ::std::option::Option::None,
                    is_delta: ::std::option::Option::None,
                    update_baseline: ::std::option::Option::None,
                    baseline: ::std::option::Option::None,
                    delta_from: ::std::option::Option::None,
                    entity_data: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 max_entries = 1;

    pub fn clear_max_entries(&mut self) {
        self.max_entries = ::std::option::Option::None;
    }

    pub fn has_max_entries(&self) -> bool {
        self.max_entries.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_entries(&mut self, v: i32) {
        self.max_entries = ::std::option::Option::Some(v);
    }

    pub fn get_max_entries(&self) -> i32 {
        self.max_entries.unwrap_or(0)
    }

    // optional int32 updated_entries = 2;

    pub fn clear_updated_entries(&mut self) {
        self.updated_entries = ::std::option::Option::None;
    }

    pub fn has_updated_entries(&self) -> bool {
        self.updated_entries.is_some()
    }

    // Param is passed by value, moved
    pub fn set_updated_entries(&mut self, v: i32) {
        self.updated_entries = ::std::option::Option::Some(v);
    }

    pub fn get_updated_entries(&self) -> i32 {
        self.updated_entries.unwrap_or(0)
    }

    // optional bool is_delta = 3;

    pub fn clear_is_delta(&mut self) {
        self.is_delta = ::std::option::Option::None;
    }

    pub fn has_is_delta(&self) -> bool {
        self.is_delta.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_delta(&mut self, v: bool) {
        self.is_delta = ::std::option::Option::Some(v);
    }

    pub fn get_is_delta(&self) -> bool {
        self.is_delta.unwrap_or(false)
    }

    // optional bool update_baseline = 4;

    pub fn clear_update_baseline(&mut self) {
        self.update_baseline = ::std::option::Option::None;
    }

    pub fn has_update_baseline(&self) -> bool {
        self.update_baseline.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_baseline(&mut self, v: bool) {
        self.update_baseline = ::std::option::Option::Some(v);
    }

    pub fn get_update_baseline(&self) -> bool {
        self.update_baseline.unwrap_or(false)
    }

    // optional int32 baseline = 5;

    pub fn clear_baseline(&mut self) {
        self.baseline = ::std::option::Option::None;
    }

    pub fn has_baseline(&self) -> bool {
        self.baseline.is_some()
    }

    // Param is passed by value, moved
    pub fn set_baseline(&mut self, v: i32) {
        self.baseline = ::std::option::Option::Some(v);
    }

    pub fn get_baseline(&self) -> i32 {
        self.baseline.unwrap_or(0)
    }

    // optional int32 delta_from = 6;

    pub fn clear_delta_from(&mut self) {
        self.delta_from = ::std::option::Option::None;
    }

    pub fn has_delta_from(&self) -> bool {
        self.delta_from.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delta_from(&mut self, v: i32) {
        self.delta_from = ::std::option::Option::Some(v);
    }

    pub fn get_delta_from(&self) -> i32 {
        self.delta_from.unwrap_or(0)
    }

    // optional bytes entity_data = 7;

    pub fn clear_entity_data(&mut self) {
        self.entity_data.clear();
    }

    pub fn has_entity_data(&self) -> bool {
        self.entity_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.entity_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_entity_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.entity_data.is_none() {
            self.entity_data.set_default();
        };
        self.entity_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_entity_data(&mut self) -> ::std::vec::Vec<u8> {
        self.entity_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_entity_data(&self) -> &[u8] {
        match self.entity_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for CSVCMsg_PacketEntities {
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
                    self.max_entries = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.updated_entries = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.is_delta = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.update_baseline = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.baseline = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.delta_from = ::std::option::Option::Some(tmp);
                },
                7 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.entity_data));
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
        for value in &self.max_entries {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.updated_entries {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.is_delta.is_some() {
            my_size += 2;
        };
        if self.update_baseline.is_some() {
            my_size += 2;
        };
        for value in &self.baseline {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.delta_from {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.entity_data {
            my_size += ::protobuf::rt::bytes_size(7, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.max_entries {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.updated_entries {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.is_delta {
            try!(os.write_bool(3, v));
        };
        if let Some(v) = self.update_baseline {
            try!(os.write_bool(4, v));
        };
        if let Some(v) = self.baseline {
            try!(os.write_int32(5, v));
        };
        if let Some(v) = self.delta_from {
            try!(os.write_int32(6, v));
        };
        if let Some(v) = self.entity_data.as_ref() {
            try!(os.write_bytes(7, &v));
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
        ::std::any::TypeId::of::<CSVCMsg_PacketEntities>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_PacketEntities {
    fn new() -> CSVCMsg_PacketEntities {
        CSVCMsg_PacketEntities::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_PacketEntities>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "max_entries",
                    CSVCMsg_PacketEntities::has_max_entries,
                    CSVCMsg_PacketEntities::get_max_entries,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "updated_entries",
                    CSVCMsg_PacketEntities::has_updated_entries,
                    CSVCMsg_PacketEntities::get_updated_entries,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "is_delta",
                    CSVCMsg_PacketEntities::has_is_delta,
                    CSVCMsg_PacketEntities::get_is_delta,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "update_baseline",
                    CSVCMsg_PacketEntities::has_update_baseline,
                    CSVCMsg_PacketEntities::get_update_baseline,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "baseline",
                    CSVCMsg_PacketEntities::has_baseline,
                    CSVCMsg_PacketEntities::get_baseline,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "delta_from",
                    CSVCMsg_PacketEntities::has_delta_from,
                    CSVCMsg_PacketEntities::get_delta_from,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "entity_data",
                    CSVCMsg_PacketEntities::has_entity_data,
                    CSVCMsg_PacketEntities::get_entity_data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_PacketEntities>(
                    "CSVCMsg_PacketEntities",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_PacketEntities {
    fn clear(&mut self) {
        self.clear_max_entries();
        self.clear_updated_entries();
        self.clear_is_delta();
        self.clear_update_baseline();
        self.clear_baseline();
        self.clear_delta_from();
        self.clear_entity_data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_PacketEntities {
    fn eq(&self, other: &CSVCMsg_PacketEntities) -> bool {
        self.max_entries == other.max_entries &&
        self.updated_entries == other.updated_entries &&
        self.is_delta == other.is_delta &&
        self.update_baseline == other.update_baseline &&
        self.baseline == other.baseline &&
        self.delta_from == other.delta_from &&
        self.entity_data == other.entity_data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_PacketEntities {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_TempEntities {
    // message fields
    reliable: ::std::option::Option<bool>,
    num_entries: ::std::option::Option<i32>,
    entity_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_TempEntities {}

impl CSVCMsg_TempEntities {
    pub fn new() -> CSVCMsg_TempEntities {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_TempEntities {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_TempEntities> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_TempEntities,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_TempEntities {
                    reliable: ::std::option::Option::None,
                    num_entries: ::std::option::Option::None,
                    entity_data: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool reliable = 1;

    pub fn clear_reliable(&mut self) {
        self.reliable = ::std::option::Option::None;
    }

    pub fn has_reliable(&self) -> bool {
        self.reliable.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reliable(&mut self, v: bool) {
        self.reliable = ::std::option::Option::Some(v);
    }

    pub fn get_reliable(&self) -> bool {
        self.reliable.unwrap_or(false)
    }

    // optional int32 num_entries = 2;

    pub fn clear_num_entries(&mut self) {
        self.num_entries = ::std::option::Option::None;
    }

    pub fn has_num_entries(&self) -> bool {
        self.num_entries.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_entries(&mut self, v: i32) {
        self.num_entries = ::std::option::Option::Some(v);
    }

    pub fn get_num_entries(&self) -> i32 {
        self.num_entries.unwrap_or(0)
    }

    // optional bytes entity_data = 3;

    pub fn clear_entity_data(&mut self) {
        self.entity_data.clear();
    }

    pub fn has_entity_data(&self) -> bool {
        self.entity_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.entity_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_entity_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.entity_data.is_none() {
            self.entity_data.set_default();
        };
        self.entity_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_entity_data(&mut self) -> ::std::vec::Vec<u8> {
        self.entity_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_entity_data(&self) -> &[u8] {
        match self.entity_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for CSVCMsg_TempEntities {
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
                    self.reliable = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.num_entries = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.entity_data));
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
        if self.reliable.is_some() {
            my_size += 2;
        };
        for value in &self.num_entries {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.entity_data {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.reliable {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.num_entries {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.entity_data.as_ref() {
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
        ::std::any::TypeId::of::<CSVCMsg_TempEntities>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_TempEntities {
    fn new() -> CSVCMsg_TempEntities {
        CSVCMsg_TempEntities::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_TempEntities>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "reliable",
                    CSVCMsg_TempEntities::has_reliable,
                    CSVCMsg_TempEntities::get_reliable,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "num_entries",
                    CSVCMsg_TempEntities::has_num_entries,
                    CSVCMsg_TempEntities::get_num_entries,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "entity_data",
                    CSVCMsg_TempEntities::has_entity_data,
                    CSVCMsg_TempEntities::get_entity_data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_TempEntities>(
                    "CSVCMsg_TempEntities",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_TempEntities {
    fn clear(&mut self) {
        self.clear_reliable();
        self.clear_num_entries();
        self.clear_entity_data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_TempEntities {
    fn eq(&self, other: &CSVCMsg_TempEntities) -> bool {
        self.reliable == other.reliable &&
        self.num_entries == other.num_entries &&
        self.entity_data == other.entity_data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_TempEntities {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_CreateStringTable {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    max_entries: ::std::option::Option<i32>,
    num_entries: ::std::option::Option<i32>,
    user_data_fixed_size: ::std::option::Option<bool>,
    user_data_size: ::std::option::Option<i32>,
    user_data_size_bits: ::std::option::Option<i32>,
    flags: ::std::option::Option<i32>,
    string_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_CreateStringTable {}

impl CSVCMsg_CreateStringTable {
    pub fn new() -> CSVCMsg_CreateStringTable {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_CreateStringTable {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_CreateStringTable> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_CreateStringTable,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_CreateStringTable {
                    name: ::protobuf::SingularField::none(),
                    max_entries: ::std::option::Option::None,
                    num_entries: ::std::option::Option::None,
                    user_data_fixed_size: ::std::option::Option::None,
                    user_data_size: ::std::option::Option::None,
                    user_data_size_bits: ::std::option::Option::None,
                    flags: ::std::option::Option::None,
                    string_data: ::protobuf::SingularField::none(),
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

    // optional int32 max_entries = 2;

    pub fn clear_max_entries(&mut self) {
        self.max_entries = ::std::option::Option::None;
    }

    pub fn has_max_entries(&self) -> bool {
        self.max_entries.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_entries(&mut self, v: i32) {
        self.max_entries = ::std::option::Option::Some(v);
    }

    pub fn get_max_entries(&self) -> i32 {
        self.max_entries.unwrap_or(0)
    }

    // optional int32 num_entries = 3;

    pub fn clear_num_entries(&mut self) {
        self.num_entries = ::std::option::Option::None;
    }

    pub fn has_num_entries(&self) -> bool {
        self.num_entries.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_entries(&mut self, v: i32) {
        self.num_entries = ::std::option::Option::Some(v);
    }

    pub fn get_num_entries(&self) -> i32 {
        self.num_entries.unwrap_or(0)
    }

    // optional bool user_data_fixed_size = 4;

    pub fn clear_user_data_fixed_size(&mut self) {
        self.user_data_fixed_size = ::std::option::Option::None;
    }

    pub fn has_user_data_fixed_size(&self) -> bool {
        self.user_data_fixed_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user_data_fixed_size(&mut self, v: bool) {
        self.user_data_fixed_size = ::std::option::Option::Some(v);
    }

    pub fn get_user_data_fixed_size(&self) -> bool {
        self.user_data_fixed_size.unwrap_or(false)
    }

    // optional int32 user_data_size = 5;

    pub fn clear_user_data_size(&mut self) {
        self.user_data_size = ::std::option::Option::None;
    }

    pub fn has_user_data_size(&self) -> bool {
        self.user_data_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user_data_size(&mut self, v: i32) {
        self.user_data_size = ::std::option::Option::Some(v);
    }

    pub fn get_user_data_size(&self) -> i32 {
        self.user_data_size.unwrap_or(0)
    }

    // optional int32 user_data_size_bits = 6;

    pub fn clear_user_data_size_bits(&mut self) {
        self.user_data_size_bits = ::std::option::Option::None;
    }

    pub fn has_user_data_size_bits(&self) -> bool {
        self.user_data_size_bits.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user_data_size_bits(&mut self, v: i32) {
        self.user_data_size_bits = ::std::option::Option::Some(v);
    }

    pub fn get_user_data_size_bits(&self) -> i32 {
        self.user_data_size_bits.unwrap_or(0)
    }

    // optional int32 flags = 7;

    pub fn clear_flags(&mut self) {
        self.flags = ::std::option::Option::None;
    }

    pub fn has_flags(&self) -> bool {
        self.flags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flags(&mut self, v: i32) {
        self.flags = ::std::option::Option::Some(v);
    }

    pub fn get_flags(&self) -> i32 {
        self.flags.unwrap_or(0)
    }

    // optional bytes string_data = 8;

    pub fn clear_string_data(&mut self) {
        self.string_data.clear();
    }

    pub fn has_string_data(&self) -> bool {
        self.string_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_string_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.string_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_string_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.string_data.is_none() {
            self.string_data.set_default();
        };
        self.string_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_string_data(&mut self) -> ::std::vec::Vec<u8> {
        self.string_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_string_data(&self) -> &[u8] {
        match self.string_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for CSVCMsg_CreateStringTable {
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
                    self.max_entries = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.num_entries = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.user_data_fixed_size = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.user_data_size = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.user_data_size_bits = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.flags = ::std::option::Option::Some(tmp);
                },
                8 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.string_data));
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
        for value in &self.max_entries {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.num_entries {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.user_data_fixed_size.is_some() {
            my_size += 2;
        };
        for value in &self.user_data_size {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.user_data_size_bits {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.flags {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.string_data {
            my_size += ::protobuf::rt::bytes_size(8, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.max_entries {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.num_entries {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.user_data_fixed_size {
            try!(os.write_bool(4, v));
        };
        if let Some(v) = self.user_data_size {
            try!(os.write_int32(5, v));
        };
        if let Some(v) = self.user_data_size_bits {
            try!(os.write_int32(6, v));
        };
        if let Some(v) = self.flags {
            try!(os.write_int32(7, v));
        };
        if let Some(v) = self.string_data.as_ref() {
            try!(os.write_bytes(8, &v));
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
        ::std::any::TypeId::of::<CSVCMsg_CreateStringTable>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_CreateStringTable {
    fn new() -> CSVCMsg_CreateStringTable {
        CSVCMsg_CreateStringTable::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_CreateStringTable>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    CSVCMsg_CreateStringTable::has_name,
                    CSVCMsg_CreateStringTable::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "max_entries",
                    CSVCMsg_CreateStringTable::has_max_entries,
                    CSVCMsg_CreateStringTable::get_max_entries,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "num_entries",
                    CSVCMsg_CreateStringTable::has_num_entries,
                    CSVCMsg_CreateStringTable::get_num_entries,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "user_data_fixed_size",
                    CSVCMsg_CreateStringTable::has_user_data_fixed_size,
                    CSVCMsg_CreateStringTable::get_user_data_fixed_size,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "user_data_size",
                    CSVCMsg_CreateStringTable::has_user_data_size,
                    CSVCMsg_CreateStringTable::get_user_data_size,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "user_data_size_bits",
                    CSVCMsg_CreateStringTable::has_user_data_size_bits,
                    CSVCMsg_CreateStringTable::get_user_data_size_bits,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "flags",
                    CSVCMsg_CreateStringTable::has_flags,
                    CSVCMsg_CreateStringTable::get_flags,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "string_data",
                    CSVCMsg_CreateStringTable::has_string_data,
                    CSVCMsg_CreateStringTable::get_string_data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_CreateStringTable>(
                    "CSVCMsg_CreateStringTable",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_CreateStringTable {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_max_entries();
        self.clear_num_entries();
        self.clear_user_data_fixed_size();
        self.clear_user_data_size();
        self.clear_user_data_size_bits();
        self.clear_flags();
        self.clear_string_data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_CreateStringTable {
    fn eq(&self, other: &CSVCMsg_CreateStringTable) -> bool {
        self.name == other.name &&
        self.max_entries == other.max_entries &&
        self.num_entries == other.num_entries &&
        self.user_data_fixed_size == other.user_data_fixed_size &&
        self.user_data_size == other.user_data_size &&
        self.user_data_size_bits == other.user_data_size_bits &&
        self.flags == other.flags &&
        self.string_data == other.string_data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_CreateStringTable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_UpdateStringTable {
    // message fields
    table_id: ::std::option::Option<i32>,
    num_changed_entries: ::std::option::Option<i32>,
    string_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_UpdateStringTable {}

impl CSVCMsg_UpdateStringTable {
    pub fn new() -> CSVCMsg_UpdateStringTable {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_UpdateStringTable {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_UpdateStringTable> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_UpdateStringTable,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_UpdateStringTable {
                    table_id: ::std::option::Option::None,
                    num_changed_entries: ::std::option::Option::None,
                    string_data: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 table_id = 1;

    pub fn clear_table_id(&mut self) {
        self.table_id = ::std::option::Option::None;
    }

    pub fn has_table_id(&self) -> bool {
        self.table_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table_id(&mut self, v: i32) {
        self.table_id = ::std::option::Option::Some(v);
    }

    pub fn get_table_id(&self) -> i32 {
        self.table_id.unwrap_or(0)
    }

    // optional int32 num_changed_entries = 2;

    pub fn clear_num_changed_entries(&mut self) {
        self.num_changed_entries = ::std::option::Option::None;
    }

    pub fn has_num_changed_entries(&self) -> bool {
        self.num_changed_entries.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_changed_entries(&mut self, v: i32) {
        self.num_changed_entries = ::std::option::Option::Some(v);
    }

    pub fn get_num_changed_entries(&self) -> i32 {
        self.num_changed_entries.unwrap_or(0)
    }

    // optional bytes string_data = 3;

    pub fn clear_string_data(&mut self) {
        self.string_data.clear();
    }

    pub fn has_string_data(&self) -> bool {
        self.string_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_string_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.string_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_string_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.string_data.is_none() {
            self.string_data.set_default();
        };
        self.string_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_string_data(&mut self) -> ::std::vec::Vec<u8> {
        self.string_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_string_data(&self) -> &[u8] {
        match self.string_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for CSVCMsg_UpdateStringTable {
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
                    self.table_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.num_changed_entries = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.string_data));
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
        for value in &self.table_id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.num_changed_entries {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.string_data {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.table_id {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.num_changed_entries {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.string_data.as_ref() {
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
        ::std::any::TypeId::of::<CSVCMsg_UpdateStringTable>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_UpdateStringTable {
    fn new() -> CSVCMsg_UpdateStringTable {
        CSVCMsg_UpdateStringTable::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_UpdateStringTable>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "table_id",
                    CSVCMsg_UpdateStringTable::has_table_id,
                    CSVCMsg_UpdateStringTable::get_table_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "num_changed_entries",
                    CSVCMsg_UpdateStringTable::has_num_changed_entries,
                    CSVCMsg_UpdateStringTable::get_num_changed_entries,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "string_data",
                    CSVCMsg_UpdateStringTable::has_string_data,
                    CSVCMsg_UpdateStringTable::get_string_data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_UpdateStringTable>(
                    "CSVCMsg_UpdateStringTable",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_UpdateStringTable {
    fn clear(&mut self) {
        self.clear_table_id();
        self.clear_num_changed_entries();
        self.clear_string_data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_UpdateStringTable {
    fn eq(&self, other: &CSVCMsg_UpdateStringTable) -> bool {
        self.table_id == other.table_id &&
        self.num_changed_entries == other.num_changed_entries &&
        self.string_data == other.string_data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_UpdateStringTable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_UserMessage {
    // message fields
    msg_type: ::std::option::Option<i32>,
    msg_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_UserMessage {}

impl CSVCMsg_UserMessage {
    pub fn new() -> CSVCMsg_UserMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_UserMessage {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_UserMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_UserMessage,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_UserMessage {
                    msg_type: ::std::option::Option::None,
                    msg_data: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 msg_type = 1;

    pub fn clear_msg_type(&mut self) {
        self.msg_type = ::std::option::Option::None;
    }

    pub fn has_msg_type(&self) -> bool {
        self.msg_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg_type(&mut self, v: i32) {
        self.msg_type = ::std::option::Option::Some(v);
    }

    pub fn get_msg_type(&self) -> i32 {
        self.msg_type.unwrap_or(0)
    }

    // optional bytes msg_data = 2;

    pub fn clear_msg_data(&mut self) {
        self.msg_data.clear();
    }

    pub fn has_msg_data(&self) -> bool {
        self.msg_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.msg_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.msg_data.is_none() {
            self.msg_data.set_default();
        };
        self.msg_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_msg_data(&mut self) -> ::std::vec::Vec<u8> {
        self.msg_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_msg_data(&self) -> &[u8] {
        match self.msg_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for CSVCMsg_UserMessage {
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
                    self.msg_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.msg_data));
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
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.msg_data {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.msg_type {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.msg_data.as_ref() {
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
        ::std::any::TypeId::of::<CSVCMsg_UserMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_UserMessage {
    fn new() -> CSVCMsg_UserMessage {
        CSVCMsg_UserMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_UserMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "msg_type",
                    CSVCMsg_UserMessage::has_msg_type,
                    CSVCMsg_UserMessage::get_msg_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "msg_data",
                    CSVCMsg_UserMessage::has_msg_data,
                    CSVCMsg_UserMessage::get_msg_data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_UserMessage>(
                    "CSVCMsg_UserMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_UserMessage {
    fn clear(&mut self) {
        self.clear_msg_type();
        self.clear_msg_data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_UserMessage {
    fn eq(&self, other: &CSVCMsg_UserMessage) -> bool {
        self.msg_type == other.msg_type &&
        self.msg_data == other.msg_data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_UserMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsg_VoiceData {
    // message fields
    client: ::std::option::Option<i32>,
    proximity: ::std::option::Option<bool>,
    xuid: ::std::option::Option<u64>,
    audible_mask: ::std::option::Option<i32>,
    voice_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_VoiceData {}

impl CSVCMsg_VoiceData {
    pub fn new() -> CSVCMsg_VoiceData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_VoiceData {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_VoiceData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_VoiceData,
        };
        unsafe {
            instance.get(|| {
                CSVCMsg_VoiceData {
                    client: ::std::option::Option::None,
                    proximity: ::std::option::Option::None,
                    xuid: ::std::option::Option::None,
                    audible_mask: ::std::option::Option::None,
                    voice_data: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 client = 1;

    pub fn clear_client(&mut self) {
        self.client = ::std::option::Option::None;
    }

    pub fn has_client(&self) -> bool {
        self.client.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client(&mut self, v: i32) {
        self.client = ::std::option::Option::Some(v);
    }

    pub fn get_client(&self) -> i32 {
        self.client.unwrap_or(0)
    }

    // optional bool proximity = 2;

    pub fn clear_proximity(&mut self) {
        self.proximity = ::std::option::Option::None;
    }

    pub fn has_proximity(&self) -> bool {
        self.proximity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_proximity(&mut self, v: bool) {
        self.proximity = ::std::option::Option::Some(v);
    }

    pub fn get_proximity(&self) -> bool {
        self.proximity.unwrap_or(false)
    }

    // optional fixed64 xuid = 3;

    pub fn clear_xuid(&mut self) {
        self.xuid = ::std::option::Option::None;
    }

    pub fn has_xuid(&self) -> bool {
        self.xuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_xuid(&mut self, v: u64) {
        self.xuid = ::std::option::Option::Some(v);
    }

    pub fn get_xuid(&self) -> u64 {
        self.xuid.unwrap_or(0)
    }

    // optional int32 audible_mask = 4;

    pub fn clear_audible_mask(&mut self) {
        self.audible_mask = ::std::option::Option::None;
    }

    pub fn has_audible_mask(&self) -> bool {
        self.audible_mask.is_some()
    }

    // Param is passed by value, moved
    pub fn set_audible_mask(&mut self, v: i32) {
        self.audible_mask = ::std::option::Option::Some(v);
    }

    pub fn get_audible_mask(&self) -> i32 {
        self.audible_mask.unwrap_or(0)
    }

    // optional bytes voice_data = 5;

    pub fn clear_voice_data(&mut self) {
        self.voice_data.clear();
    }

    pub fn has_voice_data(&self) -> bool {
        self.voice_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_voice_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.voice_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_voice_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.voice_data.is_none() {
            self.voice_data.set_default();
        };
        self.voice_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_voice_data(&mut self) -> ::std::vec::Vec<u8> {
        self.voice_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_voice_data(&self) -> &[u8] {
        match self.voice_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for CSVCMsg_VoiceData {
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
                    self.client = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.proximity = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed64());
                    self.xuid = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.audible_mask = ::std::option::Option::Some(tmp);
                },
                5 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.voice_data));
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
        for value in &self.client {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.proximity.is_some() {
            my_size += 2;
        };
        if self.xuid.is_some() {
            my_size += 9;
        };
        for value in &self.audible_mask {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.voice_data {
            my_size += ::protobuf::rt::bytes_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.proximity {
            try!(os.write_bool(2, v));
        };
        if let Some(v) = self.xuid {
            try!(os.write_fixed64(3, v));
        };
        if let Some(v) = self.audible_mask {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.voice_data.as_ref() {
            try!(os.write_bytes(5, &v));
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
        ::std::any::TypeId::of::<CSVCMsg_VoiceData>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_VoiceData {
    fn new() -> CSVCMsg_VoiceData {
        CSVCMsg_VoiceData::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_VoiceData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "client",
                    CSVCMsg_VoiceData::has_client,
                    CSVCMsg_VoiceData::get_client,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "proximity",
                    CSVCMsg_VoiceData::has_proximity,
                    CSVCMsg_VoiceData::get_proximity,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "xuid",
                    CSVCMsg_VoiceData::has_xuid,
                    CSVCMsg_VoiceData::get_xuid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "audible_mask",
                    CSVCMsg_VoiceData::has_audible_mask,
                    CSVCMsg_VoiceData::get_audible_mask,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "voice_data",
                    CSVCMsg_VoiceData::has_voice_data,
                    CSVCMsg_VoiceData::get_voice_data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_VoiceData>(
                    "CSVCMsg_VoiceData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_VoiceData {
    fn clear(&mut self) {
        self.clear_client();
        self.clear_proximity();
        self.clear_xuid();
        self.clear_audible_mask();
        self.clear_voice_data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsg_VoiceData {
    fn eq(&self, other: &CSVCMsg_VoiceData) -> bool {
        self.client == other.client &&
        self.proximity == other.proximity &&
        self.xuid == other.xuid &&
        self.audible_mask == other.audible_mask &&
        self.voice_data == other.voice_data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsg_VoiceData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsgList_GameEvents {
    // message fields
    events: ::protobuf::RepeatedField<CSVCMsgList_GameEvents_event_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsgList_GameEvents {}

impl CSVCMsgList_GameEvents {
    pub fn new() -> CSVCMsgList_GameEvents {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsgList_GameEvents {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsgList_GameEvents> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsgList_GameEvents,
        };
        unsafe {
            instance.get(|| {
                CSVCMsgList_GameEvents {
                    events: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .CSVCMsgList_GameEvents.event_t events = 1;

    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    // Param is passed by value, moved
    pub fn set_events(&mut self, v: ::protobuf::RepeatedField<CSVCMsgList_GameEvents_event_t>) {
        self.events = v;
    }

    // Mutable pointer to the field.
    pub fn mut_events(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsgList_GameEvents_event_t> {
        &mut self.events
    }

    // Take field
    pub fn take_events(&mut self) -> ::protobuf::RepeatedField<CSVCMsgList_GameEvents_event_t> {
        ::std::mem::replace(&mut self.events, ::protobuf::RepeatedField::new())
    }

    pub fn get_events(&self) -> &[CSVCMsgList_GameEvents_event_t] {
        &self.events
    }
}

impl ::protobuf::Message for CSVCMsgList_GameEvents {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.events));
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
        for value in &self.events {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.events {
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
        ::std::any::TypeId::of::<CSVCMsgList_GameEvents>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsgList_GameEvents {
    fn new() -> CSVCMsgList_GameEvents {
        CSVCMsgList_GameEvents::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsgList_GameEvents>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "events",
                    CSVCMsgList_GameEvents::get_events,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsgList_GameEvents>(
                    "CSVCMsgList_GameEvents",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsgList_GameEvents {
    fn clear(&mut self) {
        self.clear_events();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsgList_GameEvents {
    fn eq(&self, other: &CSVCMsgList_GameEvents) -> bool {
        self.events == other.events &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsgList_GameEvents {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsgList_GameEvents_event_t {
    // message fields
    tick: ::std::option::Option<i32>,
    event: ::protobuf::SingularPtrField<CSVCMsg_GameEvent>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsgList_GameEvents_event_t {}

impl CSVCMsgList_GameEvents_event_t {
    pub fn new() -> CSVCMsgList_GameEvents_event_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsgList_GameEvents_event_t {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsgList_GameEvents_event_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsgList_GameEvents_event_t,
        };
        unsafe {
            instance.get(|| {
                CSVCMsgList_GameEvents_event_t {
                    tick: ::std::option::Option::None,
                    event: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 tick = 1;

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

    // optional .CSVCMsg_GameEvent event = 2;

    pub fn clear_event(&mut self) {
        self.event.clear();
    }

    pub fn has_event(&self) -> bool {
        self.event.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event(&mut self, v: CSVCMsg_GameEvent) {
        self.event = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event(&mut self) -> &mut CSVCMsg_GameEvent {
        if self.event.is_none() {
            self.event.set_default();
        };
        self.event.as_mut().unwrap()
    }

    // Take field
    pub fn take_event(&mut self) -> CSVCMsg_GameEvent {
        self.event.take().unwrap_or_else(|| CSVCMsg_GameEvent::new())
    }

    pub fn get_event(&self) -> &CSVCMsg_GameEvent {
        self.event.as_ref().unwrap_or_else(|| CSVCMsg_GameEvent::default_instance())
    }
}

impl ::protobuf::Message for CSVCMsgList_GameEvents_event_t {
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
                    self.tick = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.event));
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
        for value in &self.tick {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.event {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tick {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.event.as_ref() {
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
        ::std::any::TypeId::of::<CSVCMsgList_GameEvents_event_t>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsgList_GameEvents_event_t {
    fn new() -> CSVCMsgList_GameEvents_event_t {
        CSVCMsgList_GameEvents_event_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsgList_GameEvents_event_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "tick",
                    CSVCMsgList_GameEvents_event_t::has_tick,
                    CSVCMsgList_GameEvents_event_t::get_tick,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "event",
                    CSVCMsgList_GameEvents_event_t::has_event,
                    CSVCMsgList_GameEvents_event_t::get_event,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsgList_GameEvents_event_t>(
                    "CSVCMsgList_GameEvents_event_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsgList_GameEvents_event_t {
    fn clear(&mut self) {
        self.clear_tick();
        self.clear_event();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsgList_GameEvents_event_t {
    fn eq(&self, other: &CSVCMsgList_GameEvents_event_t) -> bool {
        self.tick == other.tick &&
        self.event == other.event &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsgList_GameEvents_event_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsgList_UserMessages {
    // message fields
    usermsgs: ::protobuf::RepeatedField<CSVCMsgList_UserMessages_usermsg_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsgList_UserMessages {}

impl CSVCMsgList_UserMessages {
    pub fn new() -> CSVCMsgList_UserMessages {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsgList_UserMessages {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsgList_UserMessages> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsgList_UserMessages,
        };
        unsafe {
            instance.get(|| {
                CSVCMsgList_UserMessages {
                    usermsgs: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .CSVCMsgList_UserMessages.usermsg_t usermsgs = 1;

    pub fn clear_usermsgs(&mut self) {
        self.usermsgs.clear();
    }

    // Param is passed by value, moved
    pub fn set_usermsgs(&mut self, v: ::protobuf::RepeatedField<CSVCMsgList_UserMessages_usermsg_t>) {
        self.usermsgs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_usermsgs(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsgList_UserMessages_usermsg_t> {
        &mut self.usermsgs
    }

    // Take field
    pub fn take_usermsgs(&mut self) -> ::protobuf::RepeatedField<CSVCMsgList_UserMessages_usermsg_t> {
        ::std::mem::replace(&mut self.usermsgs, ::protobuf::RepeatedField::new())
    }

    pub fn get_usermsgs(&self) -> &[CSVCMsgList_UserMessages_usermsg_t] {
        &self.usermsgs
    }
}

impl ::protobuf::Message for CSVCMsgList_UserMessages {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.usermsgs));
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
        for value in &self.usermsgs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.usermsgs {
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
        ::std::any::TypeId::of::<CSVCMsgList_UserMessages>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsgList_UserMessages {
    fn new() -> CSVCMsgList_UserMessages {
        CSVCMsgList_UserMessages::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsgList_UserMessages>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "usermsgs",
                    CSVCMsgList_UserMessages::get_usermsgs,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsgList_UserMessages>(
                    "CSVCMsgList_UserMessages",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsgList_UserMessages {
    fn clear(&mut self) {
        self.clear_usermsgs();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsgList_UserMessages {
    fn eq(&self, other: &CSVCMsgList_UserMessages) -> bool {
        self.usermsgs == other.usermsgs &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsgList_UserMessages {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CSVCMsgList_UserMessages_usermsg_t {
    // message fields
    tick: ::std::option::Option<i32>,
    msg: ::protobuf::SingularPtrField<CSVCMsg_UserMessage>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsgList_UserMessages_usermsg_t {}

impl CSVCMsgList_UserMessages_usermsg_t {
    pub fn new() -> CSVCMsgList_UserMessages_usermsg_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsgList_UserMessages_usermsg_t {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsgList_UserMessages_usermsg_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsgList_UserMessages_usermsg_t,
        };
        unsafe {
            instance.get(|| {
                CSVCMsgList_UserMessages_usermsg_t {
                    tick: ::std::option::Option::None,
                    msg: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 tick = 1;

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

    // optional .CSVCMsg_UserMessage msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    pub fn has_msg(&self) -> bool {
        self.msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: CSVCMsg_UserMessage) {
        self.msg = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut CSVCMsg_UserMessage {
        if self.msg.is_none() {
            self.msg.set_default();
        };
        self.msg.as_mut().unwrap()
    }

    // Take field
    pub fn take_msg(&mut self) -> CSVCMsg_UserMessage {
        self.msg.take().unwrap_or_else(|| CSVCMsg_UserMessage::new())
    }

    pub fn get_msg(&self) -> &CSVCMsg_UserMessage {
        self.msg.as_ref().unwrap_or_else(|| CSVCMsg_UserMessage::default_instance())
    }
}

impl ::protobuf::Message for CSVCMsgList_UserMessages_usermsg_t {
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
                    self.tick = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.msg));
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
        for value in &self.tick {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.msg {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tick {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.msg.as_ref() {
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
        ::std::any::TypeId::of::<CSVCMsgList_UserMessages_usermsg_t>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsgList_UserMessages_usermsg_t {
    fn new() -> CSVCMsgList_UserMessages_usermsg_t {
        CSVCMsgList_UserMessages_usermsg_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsgList_UserMessages_usermsg_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "tick",
                    CSVCMsgList_UserMessages_usermsg_t::has_tick,
                    CSVCMsgList_UserMessages_usermsg_t::get_tick,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "msg",
                    CSVCMsgList_UserMessages_usermsg_t::has_msg,
                    CSVCMsgList_UserMessages_usermsg_t::get_msg,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsgList_UserMessages_usermsg_t>(
                    "CSVCMsgList_UserMessages_usermsg_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsgList_UserMessages_usermsg_t {
    fn clear(&mut self) {
        self.clear_tick();
        self.clear_msg();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CSVCMsgList_UserMessages_usermsg_t {
    fn eq(&self, other: &CSVCMsgList_UserMessages_usermsg_t) -> bool {
        self.tick == other.tick &&
        self.msg == other.msg &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CSVCMsgList_UserMessages_usermsg_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum NET_Messages {
    net_NOP = 0,
    net_Disconnect = 1,
    net_File = 2,
    net_SplitScreenUser = 3,
    net_Tick = 4,
    net_StringCmd = 5,
    net_SetConVar = 6,
    net_SignonState = 7,
}

impl ::protobuf::ProtobufEnum for NET_Messages {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<NET_Messages> {
        match value {
            0 => ::std::option::Option::Some(NET_Messages::net_NOP),
            1 => ::std::option::Option::Some(NET_Messages::net_Disconnect),
            2 => ::std::option::Option::Some(NET_Messages::net_File),
            3 => ::std::option::Option::Some(NET_Messages::net_SplitScreenUser),
            4 => ::std::option::Option::Some(NET_Messages::net_Tick),
            5 => ::std::option::Option::Some(NET_Messages::net_StringCmd),
            6 => ::std::option::Option::Some(NET_Messages::net_SetConVar),
            7 => ::std::option::Option::Some(NET_Messages::net_SignonState),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [NET_Messages] = &[
            NET_Messages::net_NOP,
            NET_Messages::net_Disconnect,
            NET_Messages::net_File,
            NET_Messages::net_SplitScreenUser,
            NET_Messages::net_Tick,
            NET_Messages::net_StringCmd,
            NET_Messages::net_SetConVar,
            NET_Messages::net_SignonState,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<NET_Messages>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("NET_Messages", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for NET_Messages {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum SIGNONSTATE {
    SIGNONSTATE_NONE = 0,
    SIGNONSTATE_CHALLENGE = 1,
    SIGNONSTATE_CONNECTED = 2,
    SIGNONSTATE_NEW = 3,
    SIGNONSTATE_PRESPAWN = 4,
    SIGNONSTATE_SPAWN = 5,
    SIGNONSTATE_FULL = 6,
    SIGNONSTATE_CHANGELEVEL = 7,
}

impl ::protobuf::ProtobufEnum for SIGNONSTATE {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SIGNONSTATE> {
        match value {
            0 => ::std::option::Option::Some(SIGNONSTATE::SIGNONSTATE_NONE),
            1 => ::std::option::Option::Some(SIGNONSTATE::SIGNONSTATE_CHALLENGE),
            2 => ::std::option::Option::Some(SIGNONSTATE::SIGNONSTATE_CONNECTED),
            3 => ::std::option::Option::Some(SIGNONSTATE::SIGNONSTATE_NEW),
            4 => ::std::option::Option::Some(SIGNONSTATE::SIGNONSTATE_PRESPAWN),
            5 => ::std::option::Option::Some(SIGNONSTATE::SIGNONSTATE_SPAWN),
            6 => ::std::option::Option::Some(SIGNONSTATE::SIGNONSTATE_FULL),
            7 => ::std::option::Option::Some(SIGNONSTATE::SIGNONSTATE_CHANGELEVEL),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SIGNONSTATE] = &[
            SIGNONSTATE::SIGNONSTATE_NONE,
            SIGNONSTATE::SIGNONSTATE_CHALLENGE,
            SIGNONSTATE::SIGNONSTATE_CONNECTED,
            SIGNONSTATE::SIGNONSTATE_NEW,
            SIGNONSTATE::SIGNONSTATE_PRESPAWN,
            SIGNONSTATE::SIGNONSTATE_SPAWN,
            SIGNONSTATE::SIGNONSTATE_FULL,
            SIGNONSTATE::SIGNONSTATE_CHANGELEVEL,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<SIGNONSTATE>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("SIGNONSTATE", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for SIGNONSTATE {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CLC_Messages {
    clc_ClientInfo = 8,
    clc_Move = 9,
    clc_VoiceData = 10,
    clc_BaselineAck = 11,
    clc_ListenEvents = 12,
    clc_RespondCvarValue = 13,
    clc_FileCRCCheck = 14,
    clc_LoadingProgress = 15,
    clc_SplitPlayerConnect = 16,
    clc_ClientMessage = 17,
}

impl ::protobuf::ProtobufEnum for CLC_Messages {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CLC_Messages> {
        match value {
            8 => ::std::option::Option::Some(CLC_Messages::clc_ClientInfo),
            9 => ::std::option::Option::Some(CLC_Messages::clc_Move),
            10 => ::std::option::Option::Some(CLC_Messages::clc_VoiceData),
            11 => ::std::option::Option::Some(CLC_Messages::clc_BaselineAck),
            12 => ::std::option::Option::Some(CLC_Messages::clc_ListenEvents),
            13 => ::std::option::Option::Some(CLC_Messages::clc_RespondCvarValue),
            14 => ::std::option::Option::Some(CLC_Messages::clc_FileCRCCheck),
            15 => ::std::option::Option::Some(CLC_Messages::clc_LoadingProgress),
            16 => ::std::option::Option::Some(CLC_Messages::clc_SplitPlayerConnect),
            17 => ::std::option::Option::Some(CLC_Messages::clc_ClientMessage),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CLC_Messages] = &[
            CLC_Messages::clc_ClientInfo,
            CLC_Messages::clc_Move,
            CLC_Messages::clc_VoiceData,
            CLC_Messages::clc_BaselineAck,
            CLC_Messages::clc_ListenEvents,
            CLC_Messages::clc_RespondCvarValue,
            CLC_Messages::clc_FileCRCCheck,
            CLC_Messages::clc_LoadingProgress,
            CLC_Messages::clc_SplitPlayerConnect,
            CLC_Messages::clc_ClientMessage,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CLC_Messages>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CLC_Messages", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CLC_Messages {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum SVC_Messages {
    svc_ServerInfo = 8,
    svc_SendTable = 9,
    svc_ClassInfo = 10,
    svc_SetPause = 11,
    svc_CreateStringTable = 12,
    svc_UpdateStringTable = 13,
    svc_VoiceInit = 14,
    svc_VoiceData = 15,
    svc_Print = 16,
    svc_Sounds = 17,
    svc_SetView = 18,
    svc_FixAngle = 19,
    svc_CrosshairAngle = 20,
    svc_BSPDecal = 21,
    svc_SplitScreen = 22,
    svc_UserMessage = 23,
    svc_EntityMessage = 24,
    svc_GameEvent = 25,
    svc_PacketEntities = 26,
    svc_TempEntities = 27,
    svc_Prefetch = 28,
    svc_Menu = 29,
    svc_GameEventList = 30,
    svc_GetCvarValue = 31,
}

impl ::protobuf::ProtobufEnum for SVC_Messages {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SVC_Messages> {
        match value {
            8 => ::std::option::Option::Some(SVC_Messages::svc_ServerInfo),
            9 => ::std::option::Option::Some(SVC_Messages::svc_SendTable),
            10 => ::std::option::Option::Some(SVC_Messages::svc_ClassInfo),
            11 => ::std::option::Option::Some(SVC_Messages::svc_SetPause),
            12 => ::std::option::Option::Some(SVC_Messages::svc_CreateStringTable),
            13 => ::std::option::Option::Some(SVC_Messages::svc_UpdateStringTable),
            14 => ::std::option::Option::Some(SVC_Messages::svc_VoiceInit),
            15 => ::std::option::Option::Some(SVC_Messages::svc_VoiceData),
            16 => ::std::option::Option::Some(SVC_Messages::svc_Print),
            17 => ::std::option::Option::Some(SVC_Messages::svc_Sounds),
            18 => ::std::option::Option::Some(SVC_Messages::svc_SetView),
            19 => ::std::option::Option::Some(SVC_Messages::svc_FixAngle),
            20 => ::std::option::Option::Some(SVC_Messages::svc_CrosshairAngle),
            21 => ::std::option::Option::Some(SVC_Messages::svc_BSPDecal),
            22 => ::std::option::Option::Some(SVC_Messages::svc_SplitScreen),
            23 => ::std::option::Option::Some(SVC_Messages::svc_UserMessage),
            24 => ::std::option::Option::Some(SVC_Messages::svc_EntityMessage),
            25 => ::std::option::Option::Some(SVC_Messages::svc_GameEvent),
            26 => ::std::option::Option::Some(SVC_Messages::svc_PacketEntities),
            27 => ::std::option::Option::Some(SVC_Messages::svc_TempEntities),
            28 => ::std::option::Option::Some(SVC_Messages::svc_Prefetch),
            29 => ::std::option::Option::Some(SVC_Messages::svc_Menu),
            30 => ::std::option::Option::Some(SVC_Messages::svc_GameEventList),
            31 => ::std::option::Option::Some(SVC_Messages::svc_GetCvarValue),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SVC_Messages] = &[
            SVC_Messages::svc_ServerInfo,
            SVC_Messages::svc_SendTable,
            SVC_Messages::svc_ClassInfo,
            SVC_Messages::svc_SetPause,
            SVC_Messages::svc_CreateStringTable,
            SVC_Messages::svc_UpdateStringTable,
            SVC_Messages::svc_VoiceInit,
            SVC_Messages::svc_VoiceData,
            SVC_Messages::svc_Print,
            SVC_Messages::svc_Sounds,
            SVC_Messages::svc_SetView,
            SVC_Messages::svc_FixAngle,
            SVC_Messages::svc_CrosshairAngle,
            SVC_Messages::svc_BSPDecal,
            SVC_Messages::svc_SplitScreen,
            SVC_Messages::svc_UserMessage,
            SVC_Messages::svc_EntityMessage,
            SVC_Messages::svc_GameEvent,
            SVC_Messages::svc_PacketEntities,
            SVC_Messages::svc_TempEntities,
            SVC_Messages::svc_Prefetch,
            SVC_Messages::svc_Menu,
            SVC_Messages::svc_GameEventList,
            SVC_Messages::svc_GetCvarValue,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<SVC_Messages>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("SVC_Messages", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for SVC_Messages {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ESplitScreenMessageType {
    MSG_SPLITSCREEN_ADDUSER = 0,
    MSG_SPLITSCREEN_REMOVEUSER = 1,
    MSG_SPLITSCREEN_TYPE_BITS = 1,
}

impl ::protobuf::ProtobufEnum for ESplitScreenMessageType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ESplitScreenMessageType> {
        match value {
            0 => ::std::option::Option::Some(ESplitScreenMessageType::MSG_SPLITSCREEN_ADDUSER),
            1 => ::std::option::Option::Some(ESplitScreenMessageType::MSG_SPLITSCREEN_REMOVEUSER),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ESplitScreenMessageType] = &[
            ESplitScreenMessageType::MSG_SPLITSCREEN_ADDUSER,
            ESplitScreenMessageType::MSG_SPLITSCREEN_REMOVEUSER,
            ESplitScreenMessageType::MSG_SPLITSCREEN_TYPE_BITS,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ESplitScreenMessageType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ESplitScreenMessageType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ESplitScreenMessageType {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x11, 0x6e, 0x65, 0x74, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x1a, 0x20, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x62, 0x75, 0x66, 0x2f, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x2d, 0x0a, 0x0a, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63,
    0x74, 0x6f, 0x72, 0x12, 0x09, 0x0a, 0x01, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x02, 0x12, 0x09,
    0x0a, 0x01, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x12, 0x09, 0x0a, 0x01, 0x7a, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x02, 0x22, 0x24, 0x0a, 0x0c, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74,
    0x6f, 0x72, 0x32, 0x44, 0x12, 0x09, 0x0a, 0x01, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x02, 0x12,
    0x09, 0x0a, 0x01, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x22, 0x2d, 0x0a, 0x0a, 0x43, 0x4d,
    0x73, 0x67, 0x51, 0x41, 0x6e, 0x67, 0x6c, 0x65, 0x12, 0x09, 0x0a, 0x01, 0x78, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x02, 0x12, 0x09, 0x0a, 0x01, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x12, 0x09,
    0x0a, 0x01, 0x7a, 0x18, 0x03, 0x20, 0x01, 0x28, 0x02, 0x22, 0x52, 0x0a, 0x0a, 0x43, 0x4d, 0x73,
    0x67, 0x5f, 0x43, 0x56, 0x61, 0x72, 0x73, 0x12, 0x1f, 0x0a, 0x05, 0x63, 0x76, 0x61, 0x72, 0x73,
    0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x56,
    0x61, 0x72, 0x73, 0x2e, 0x43, 0x56, 0x61, 0x72, 0x1a, 0x23, 0x0a, 0x04, 0x43, 0x56, 0x61, 0x72,
    0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x12, 0x0d,
    0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x22, 0x0d, 0x0a,
    0x0b, 0x43, 0x4e, 0x45, 0x54, 0x4d, 0x73, 0x67, 0x5f, 0x4e, 0x4f, 0x50, 0x22, 0x22, 0x0a, 0x12,
    0x43, 0x4e, 0x45, 0x54, 0x4d, 0x73, 0x67, 0x5f, 0x44, 0x69, 0x73, 0x63, 0x6f, 0x6e, 0x6e, 0x65,
    0x63, 0x74, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x65, 0x78, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x22, 0x61, 0x0a, 0x0c, 0x43, 0x4e, 0x45, 0x54, 0x4d, 0x73, 0x67, 0x5f, 0x46, 0x69, 0x6c, 0x65,
    0x12, 0x13, 0x0a, 0x0b, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x11, 0x0a, 0x09, 0x66, 0x69, 0x6c, 0x65, 0x5f, 0x6e, 0x61,
    0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x1b, 0x0a, 0x13, 0x69, 0x73, 0x5f, 0x72,
    0x65, 0x70, 0x6c, 0x61, 0x79, 0x5f, 0x64, 0x65, 0x6d, 0x6f, 0x5f, 0x66, 0x69, 0x6c, 0x65, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x08, 0x12, 0x0c, 0x0a, 0x04, 0x64, 0x65, 0x6e, 0x79, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x08, 0x22, 0x27, 0x0a, 0x17, 0x43, 0x4e, 0x45, 0x54, 0x4d, 0x73, 0x67, 0x5f, 0x53,
    0x70, 0x6c, 0x69, 0x74, 0x53, 0x63, 0x72, 0x65, 0x65, 0x6e, 0x55, 0x73, 0x65, 0x72, 0x12, 0x0c,
    0x0a, 0x04, 0x73, 0x6c, 0x6f, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x22, 0x5a, 0x0a, 0x0c,
    0x43, 0x4e, 0x45, 0x54, 0x4d, 0x73, 0x67, 0x5f, 0x54, 0x69, 0x63, 0x6b, 0x12, 0x0c, 0x0a, 0x04,
    0x74, 0x69, 0x63, 0x6b, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x16, 0x0a, 0x0e, 0x68, 0x6f,
    0x73, 0x74, 0x5f, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0d, 0x12, 0x24, 0x0a, 0x1c, 0x68, 0x6f, 0x73, 0x74, 0x5f, 0x66, 0x72, 0x61, 0x6d, 0x65,
    0x74, 0x69, 0x6d, 0x65, 0x5f, 0x73, 0x74, 0x64, 0x5f, 0x64, 0x65, 0x76, 0x69, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x22, 0x24, 0x0a, 0x11, 0x43, 0x4e, 0x45, 0x54,
    0x4d, 0x73, 0x67, 0x5f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x43, 0x6d, 0x64, 0x12, 0x0f, 0x0a,
    0x07, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x22, 0x31,
    0x0a, 0x11, 0x43, 0x4e, 0x45, 0x54, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x65, 0x74, 0x43, 0x6f, 0x6e,
    0x56, 0x61, 0x72, 0x12, 0x1c, 0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x76, 0x61, 0x72, 0x73, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x56, 0x61, 0x72,
    0x73, 0x22, 0x8a, 0x01, 0x0a, 0x13, 0x43, 0x4e, 0x45, 0x54, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x69,
    0x67, 0x6e, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x14, 0x0a, 0x0c, 0x73, 0x69, 0x67,
    0x6e, 0x6f, 0x6e, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12,
    0x13, 0x0a, 0x0b, 0x73, 0x70, 0x61, 0x77, 0x6e, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0d, 0x12, 0x1a, 0x0a, 0x12, 0x6e, 0x75, 0x6d, 0x5f, 0x73, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x5f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d,
    0x12, 0x1a, 0x0a, 0x12, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x73, 0x5f, 0x6e, 0x65, 0x74, 0x77,
    0x6f, 0x72, 0x6b, 0x69, 0x64, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x09, 0x12, 0x10, 0x0a, 0x08,
    0x6d, 0x61, 0x70, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x22, 0xa6,
    0x01, 0x0a, 0x12, 0x43, 0x43, 0x4c, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x16, 0x0a, 0x0e, 0x73, 0x65, 0x6e, 0x64, 0x5f, 0x74, 0x61,
    0x62, 0x6c, 0x65, 0x5f, 0x63, 0x72, 0x63, 0x18, 0x01, 0x20, 0x01, 0x28, 0x07, 0x12, 0x14, 0x0a,
    0x0c, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0d, 0x12, 0x0f, 0x0a, 0x07, 0x69, 0x73, 0x5f, 0x68, 0x6c, 0x74, 0x76, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x08, 0x12, 0x11, 0x0a, 0x09, 0x69, 0x73, 0x5f, 0x72, 0x65, 0x70, 0x6c, 0x61,
    0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x12, 0x12, 0x0a, 0x0a, 0x66, 0x72, 0x69, 0x65, 0x6e,
    0x64, 0x73, 0x5f, 0x69, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x14, 0x0a, 0x0c, 0x66,
    0x72, 0x69, 0x65, 0x6e, 0x64, 0x73, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28,
    0x09, 0x12, 0x14, 0x0a, 0x0c, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x66, 0x69, 0x6c, 0x65,
    0x73, 0x18, 0x07, 0x20, 0x03, 0x28, 0x07, 0x22, 0x53, 0x0a, 0x0c, 0x43, 0x43, 0x4c, 0x43, 0x4d,
    0x73, 0x67, 0x5f, 0x4d, 0x6f, 0x76, 0x65, 0x12, 0x1b, 0x0a, 0x13, 0x6e, 0x75, 0x6d, 0x5f, 0x62,
    0x61, 0x63, 0x6b, 0x75, 0x70, 0x5f, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x73, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0d, 0x12, 0x18, 0x0a, 0x10, 0x6e, 0x75, 0x6d, 0x5f, 0x6e, 0x65, 0x77, 0x5f,
    0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0c,
    0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x2f, 0x0a, 0x11,
    0x43, 0x43, 0x4c, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x56, 0x6f, 0x69, 0x63, 0x65, 0x44, 0x61, 0x74,
    0x61, 0x12, 0x0c, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x12,
    0x0c, 0x0a, 0x04, 0x78, 0x75, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x06, 0x22, 0x41, 0x0a,
    0x13, 0x43, 0x43, 0x4c, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x42, 0x61, 0x73, 0x65, 0x6c, 0x69, 0x6e,
    0x65, 0x41, 0x63, 0x6b, 0x12, 0x15, 0x0a, 0x0d, 0x62, 0x61, 0x73, 0x65, 0x6c, 0x69, 0x6e, 0x65,
    0x5f, 0x74, 0x69, 0x63, 0x6b, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x13, 0x0a, 0x0b, 0x62,
    0x61, 0x73, 0x65, 0x6c, 0x69, 0x6e, 0x65, 0x5f, 0x6e, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05,
    0x22, 0x2a, 0x0a, 0x14, 0x43, 0x43, 0x4c, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x4c, 0x69, 0x73, 0x74,
    0x65, 0x6e, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x12, 0x12, 0x0a, 0x0a, 0x65, 0x76, 0x65, 0x6e,
    0x74, 0x5f, 0x6d, 0x61, 0x73, 0x6b, 0x18, 0x01, 0x20, 0x03, 0x28, 0x07, 0x22, 0x5c, 0x0a, 0x18,
    0x43, 0x43, 0x4c, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x64, 0x43,
    0x76, 0x61, 0x72, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x0e, 0x0a, 0x06, 0x63, 0x6f, 0x6f, 0x6b,
    0x69, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x13, 0x0a, 0x0b, 0x73, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x5f, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0c, 0x0a,
    0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x12, 0x0d, 0x0a, 0x05, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x22, 0x6d, 0x0a, 0x14, 0x43, 0x43,
    0x4c, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x46, 0x69, 0x6c, 0x65, 0x43, 0x52, 0x43, 0x43, 0x68, 0x65,
    0x63, 0x6b, 0x12, 0x11, 0x0a, 0x09, 0x63, 0x6f, 0x64, 0x65, 0x5f, 0x70, 0x61, 0x74, 0x68, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0c, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x09, 0x12, 0x15, 0x0a, 0x0d, 0x63, 0x6f, 0x64, 0x65, 0x5f, 0x66, 0x69, 0x6c, 0x65,
    0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x12, 0x10, 0x0a, 0x08, 0x66, 0x69,
    0x6c, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x12, 0x0b, 0x0a, 0x03,
    0x63, 0x72, 0x63, 0x18, 0x05, 0x20, 0x01, 0x28, 0x07, 0x22, 0x2b, 0x0a, 0x17, 0x43, 0x43, 0x4c,
    0x43, 0x4d, 0x73, 0x67, 0x5f, 0x4c, 0x6f, 0x61, 0x64, 0x69, 0x6e, 0x67, 0x50, 0x72, 0x6f, 0x67,
    0x72, 0x65, 0x73, 0x73, 0x12, 0x10, 0x0a, 0x08, 0x70, 0x72, 0x6f, 0x67, 0x72, 0x65, 0x73, 0x73,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x22, 0x3a, 0x0a, 0x1a, 0x43, 0x43, 0x4c, 0x43, 0x4d, 0x73,
    0x67, 0x5f, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x43, 0x6f, 0x6e,
    0x6e, 0x65, 0x63, 0x74, 0x12, 0x1c, 0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x76, 0x61, 0x72, 0x73, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x56, 0x61,
    0x72, 0x73, 0x22, 0x37, 0x0a, 0x15, 0x43, 0x43, 0x4c, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x10, 0x0a, 0x08, 0x6d,
    0x73, 0x67, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0c, 0x0a,
    0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x22, 0xe2, 0x02, 0x0a, 0x12,
    0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x49, 0x6e,
    0x66, 0x6f, 0x12, 0x10, 0x0a, 0x08, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x05, 0x12, 0x14, 0x0a, 0x0c, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12, 0x14, 0x0a, 0x0c, 0x69, 0x73,
    0x5f, 0x64, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08,
    0x12, 0x0f, 0x0a, 0x07, 0x69, 0x73, 0x5f, 0x68, 0x6c, 0x74, 0x76, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x08, 0x12, 0x11, 0x0a, 0x09, 0x69, 0x73, 0x5f, 0x72, 0x65, 0x70, 0x6c, 0x61, 0x79, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x08, 0x12, 0x0c, 0x0a, 0x04, 0x63, 0x5f, 0x6f, 0x73, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x05, 0x12, 0x0f, 0x0a, 0x07, 0x6d, 0x61, 0x70, 0x5f, 0x63, 0x72, 0x63, 0x18, 0x07, 0x20,
    0x01, 0x28, 0x07, 0x12, 0x12, 0x0a, 0x0a, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x63, 0x72,
    0x63, 0x18, 0x08, 0x20, 0x01, 0x28, 0x07, 0x12, 0x18, 0x0a, 0x10, 0x73, 0x74, 0x72, 0x69, 0x6e,
    0x67, 0x5f, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x63, 0x72, 0x63, 0x18, 0x09, 0x20, 0x01, 0x28,
    0x07, 0x12, 0x13, 0x0a, 0x0b, 0x6d, 0x61, 0x78, 0x5f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73,
    0x18, 0x0a, 0x20, 0x01, 0x28, 0x05, 0x12, 0x13, 0x0a, 0x0b, 0x6d, 0x61, 0x78, 0x5f, 0x63, 0x6c,
    0x61, 0x73, 0x73, 0x65, 0x73, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x05, 0x12, 0x13, 0x0a, 0x0b, 0x70,
    0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x73, 0x6c, 0x6f, 0x74, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x05,
    0x12, 0x15, 0x0a, 0x0d, 0x74, 0x69, 0x63, 0x6b, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61,
    0x6c, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x02, 0x12, 0x10, 0x0a, 0x08, 0x67, 0x61, 0x6d, 0x65, 0x5f,
    0x64, 0x69, 0x72, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x09, 0x12, 0x10, 0x0a, 0x08, 0x6d, 0x61, 0x70,
    0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x09, 0x12, 0x10, 0x0a, 0x08, 0x73,
    0x6b, 0x79, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x10, 0x20, 0x01, 0x28, 0x09, 0x12, 0x11, 0x0a,
    0x09, 0x68, 0x6f, 0x73, 0x74, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x11, 0x20, 0x01, 0x28, 0x09,
    0x22, 0xa4, 0x01, 0x0a, 0x11, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x6c, 0x61,
    0x73, 0x73, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x18, 0x0a, 0x10, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65,
    0x5f, 0x6f, 0x6e, 0x5f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08,
    0x12, 0x2b, 0x0a, 0x07, 0x63, 0x6c, 0x61, 0x73, 0x73, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x1a, 0x2e, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x6c, 0x61, 0x73,
    0x73, 0x49, 0x6e, 0x66, 0x6f, 0x2e, 0x63, 0x6c, 0x61, 0x73, 0x73, 0x5f, 0x74, 0x1a, 0x48, 0x0a,
    0x07, 0x63, 0x6c, 0x61, 0x73, 0x73, 0x5f, 0x74, 0x12, 0x10, 0x0a, 0x08, 0x63, 0x6c, 0x61, 0x73,
    0x73, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x17, 0x0a, 0x0f, 0x64, 0x61,
    0x74, 0x61, 0x5f, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x09, 0x12, 0x12, 0x0a, 0x0a, 0x63, 0x6c, 0x61, 0x73, 0x73, 0x5f, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x22, 0x22, 0x0a, 0x10, 0x43, 0x53, 0x56, 0x43, 0x4d,
    0x73, 0x67, 0x5f, 0x53, 0x65, 0x74, 0x50, 0x61, 0x75, 0x73, 0x65, 0x12, 0x0e, 0x0a, 0x06, 0x70,
    0x61, 0x75, 0x73, 0x65, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x22, 0x33, 0x0a, 0x11, 0x43,
    0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x56, 0x6f, 0x69, 0x63, 0x65, 0x49, 0x6e, 0x69, 0x74,
    0x12, 0x0f, 0x0a, 0x07, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x05, 0x12, 0x0d, 0x0a, 0x05, 0x63, 0x6f, 0x64, 0x65, 0x63, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09,
    0x22, 0x1d, 0x0a, 0x0d, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x50, 0x72, 0x69, 0x6e,
    0x74, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x65, 0x78, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x22,
    0xb6, 0x03, 0x0a, 0x0e, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x6f, 0x75, 0x6e,
    0x64, 0x73, 0x12, 0x16, 0x0a, 0x0e, 0x72, 0x65, 0x6c, 0x69, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x73,
    0x6f, 0x75, 0x6e, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x12, 0x2b, 0x0a, 0x06, 0x73, 0x6f,
    0x75, 0x6e, 0x64, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x43, 0x53, 0x56,
    0x43, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x6f, 0x75, 0x6e, 0x64, 0x73, 0x2e, 0x73, 0x6f, 0x75, 0x6e,
    0x64, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x74, 0x1a, 0xde, 0x02, 0x0a, 0x0b, 0x73, 0x6f, 0x75, 0x6e,
    0x64, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x74, 0x12, 0x10, 0x0a, 0x08, 0x6f, 0x72, 0x69, 0x67, 0x69,
    0x6e, 0x5f, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x11, 0x12, 0x10, 0x0a, 0x08, 0x6f, 0x72, 0x69,
    0x67, 0x69, 0x6e, 0x5f, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x11, 0x12, 0x10, 0x0a, 0x08, 0x6f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x5f, 0x7a, 0x18, 0x03, 0x20, 0x01, 0x28, 0x11, 0x12, 0x0e, 0x0a,
    0x06, 0x76, 0x6f, 0x6c, 0x75, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x13, 0x0a,
    0x0b, 0x64, 0x65, 0x6c, 0x61, 0x79, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x02, 0x12, 0x17, 0x0a, 0x0f, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x5f, 0x6e,
    0x75, 0x6d, 0x62, 0x65, 0x72, 0x18, 0x06, 0x20, 0x01, 0x28, 0x05, 0x12, 0x14, 0x0a, 0x0c, 0x65,
    0x6e, 0x74, 0x69, 0x74, 0x79, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x07, 0x20, 0x01, 0x28,
    0x05, 0x12, 0x0f, 0x0a, 0x07, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x18, 0x08, 0x20, 0x01,
    0x28, 0x05, 0x12, 0x0d, 0x0a, 0x05, 0x70, 0x69, 0x74, 0x63, 0x68, 0x18, 0x09, 0x20, 0x01, 0x28,
    0x05, 0x12, 0x0d, 0x0a, 0x05, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x05,
    0x12, 0x11, 0x0a, 0x09, 0x73, 0x6f, 0x75, 0x6e, 0x64, 0x5f, 0x6e, 0x75, 0x6d, 0x18, 0x0b, 0x20,
    0x01, 0x28, 0x0d, 0x12, 0x18, 0x0a, 0x10, 0x73, 0x6f, 0x75, 0x6e, 0x64, 0x5f, 0x6e, 0x75, 0x6d,
    0x5f, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x07, 0x12, 0x16, 0x0a,
    0x0e, 0x73, 0x70, 0x65, 0x61, 0x6b, 0x65, 0x72, 0x5f, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x18,
    0x0d, 0x20, 0x01, 0x28, 0x05, 0x12, 0x13, 0x0a, 0x0b, 0x72, 0x61, 0x6e, 0x64, 0x6f, 0x6d, 0x5f,
    0x73, 0x65, 0x65, 0x64, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x05, 0x12, 0x13, 0x0a, 0x0b, 0x73, 0x6f,
    0x75, 0x6e, 0x64, 0x5f, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x05, 0x12,
    0x13, 0x0a, 0x0b, 0x69, 0x73, 0x5f, 0x73, 0x65, 0x6e, 0x74, 0x65, 0x6e, 0x63, 0x65, 0x18, 0x10,
    0x20, 0x01, 0x28, 0x08, 0x12, 0x12, 0x0a, 0x0a, 0x69, 0x73, 0x5f, 0x61, 0x6d, 0x62, 0x69, 0x65,
    0x6e, 0x74, 0x18, 0x11, 0x20, 0x01, 0x28, 0x08, 0x22, 0x27, 0x0a, 0x10, 0x43, 0x53, 0x56, 0x43,
    0x4d, 0x73, 0x67, 0x5f, 0x50, 0x72, 0x65, 0x66, 0x65, 0x74, 0x63, 0x68, 0x12, 0x13, 0x0a, 0x0b,
    0x73, 0x6f, 0x75, 0x6e, 0x64, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x05, 0x22, 0x27, 0x0a, 0x0f, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x65, 0x74,
    0x56, 0x69, 0x65, 0x77, 0x12, 0x14, 0x0a, 0x0c, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x5f, 0x69,
    0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x22, 0x40, 0x0a, 0x10, 0x43, 0x53,
    0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x46, 0x69, 0x78, 0x41, 0x6e, 0x67, 0x6c, 0x65, 0x12, 0x10,
    0x0a, 0x08, 0x72, 0x65, 0x6c, 0x61, 0x74, 0x69, 0x76, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08,
    0x12, 0x1a, 0x0a, 0x05, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x0b, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x51, 0x41, 0x6e, 0x67, 0x6c, 0x65, 0x22, 0x34, 0x0a, 0x16,
    0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x72, 0x6f, 0x73, 0x73, 0x68, 0x61, 0x69,
    0x72, 0x41, 0x6e, 0x67, 0x6c, 0x65, 0x12, 0x1a, 0x0a, 0x05, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x51, 0x41, 0x6e, 0x67,
    0x6c, 0x65, 0x22, 0x8a, 0x01, 0x0a, 0x10, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x42,
    0x53, 0x50, 0x44, 0x65, 0x63, 0x61, 0x6c, 0x12, 0x18, 0x0a, 0x03, 0x70, 0x6f, 0x73, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f,
    0x72, 0x12, 0x1b, 0x0a, 0x13, 0x64, 0x65, 0x63, 0x61, 0x6c, 0x5f, 0x74, 0x65, 0x78, 0x74, 0x75,
    0x72, 0x65, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12, 0x14,
    0x0a, 0x0c, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x05, 0x12, 0x13, 0x0a, 0x0b, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x5f, 0x69, 0x6e,
    0x64, 0x65, 0x78, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05, 0x12, 0x14, 0x0a, 0x0c, 0x6c, 0x6f, 0x77,
    0x5f, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x08, 0x22,
    0x61, 0x0a, 0x13, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x70, 0x6c, 0x69, 0x74,
    0x53, 0x63, 0x72, 0x65, 0x65, 0x6e, 0x12, 0x26, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0e, 0x32, 0x18, 0x2e, 0x45, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x53, 0x63, 0x72,
    0x65, 0x65, 0x6e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0c,
    0x0a, 0x04, 0x73, 0x6c, 0x6f, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12, 0x14, 0x0a, 0x0c,
    0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x05, 0x22, 0x39, 0x0a, 0x14, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x47, 0x65,
    0x74, 0x43, 0x76, 0x61, 0x72, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x0e, 0x0a, 0x06, 0x63, 0x6f,
    0x6f, 0x6b, 0x69, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x11, 0x0a, 0x09, 0x63, 0x76,
    0x61, 0x72, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x22, 0x3c, 0x0a,
    0x0c, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x4d, 0x65, 0x6e, 0x75, 0x12, 0x13, 0x0a,
    0x0b, 0x64, 0x69, 0x61, 0x6c, 0x6f, 0x67, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x05, 0x12, 0x17, 0x0a, 0x0f, 0x6d, 0x65, 0x6e, 0x75, 0x5f, 0x6b, 0x65, 0x79, 0x5f, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x22, 0xb0, 0x02, 0x0a, 0x11,
    0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x65, 0x6e, 0x64, 0x54, 0x61, 0x62, 0x6c,
    0x65, 0x12, 0x0e, 0x0a, 0x06, 0x69, 0x73, 0x5f, 0x65, 0x6e, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x08, 0x12, 0x16, 0x0a, 0x0e, 0x6e, 0x65, 0x74, 0x5f, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x6e,
    0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x15, 0x0a, 0x0d, 0x6e, 0x65, 0x65,
    0x64, 0x73, 0x5f, 0x64, 0x65, 0x63, 0x6f, 0x64, 0x65, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08,
    0x12, 0x2c, 0x0a, 0x05, 0x70, 0x72, 0x6f, 0x70, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x1d, 0x2e, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x65, 0x6e, 0x64, 0x54, 0x61,
    0x62, 0x6c, 0x65, 0x2e, 0x73, 0x65, 0x6e, 0x64, 0x70, 0x72, 0x6f, 0x70, 0x5f, 0x74, 0x1a, 0xad,
    0x01, 0x0a, 0x0a, 0x73, 0x65, 0x6e, 0x64, 0x70, 0x72, 0x6f, 0x70, 0x5f, 0x74, 0x12, 0x0c, 0x0a,
    0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x10, 0x0a, 0x08, 0x76,
    0x61, 0x72, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x0d, 0x0a,
    0x05, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x12, 0x10, 0x0a, 0x08,
    0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0f,
    0x0a, 0x07, 0x64, 0x74, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x12,
    0x14, 0x0a, 0x0c, 0x6e, 0x75, 0x6d, 0x5f, 0x65, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x18,
    0x06, 0x20, 0x01, 0x28, 0x05, 0x12, 0x11, 0x0a, 0x09, 0x6c, 0x6f, 0x77, 0x5f, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x02, 0x12, 0x12, 0x0a, 0x0a, 0x68, 0x69, 0x67, 0x68,
    0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x02, 0x12, 0x10, 0x0a, 0x08,
    0x6e, 0x75, 0x6d, 0x5f, 0x62, 0x69, 0x74, 0x73, 0x18, 0x09, 0x20, 0x01, 0x28, 0x05, 0x22, 0xfc,
    0x01, 0x0a, 0x11, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x47, 0x61, 0x6d, 0x65, 0x45,
    0x76, 0x65, 0x6e, 0x74, 0x12, 0x12, 0x0a, 0x0a, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x5f, 0x6e, 0x61,
    0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x12, 0x0f, 0x0a, 0x07, 0x65, 0x76, 0x65, 0x6e,
    0x74, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12, 0x26, 0x0a, 0x04, 0x6b, 0x65, 0x79,
    0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73,
    0x67, 0x5f, 0x47, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x2e, 0x6b, 0x65, 0x79, 0x5f,
    0x74, 0x1a, 0x99, 0x01, 0x0a, 0x05, 0x6b, 0x65, 0x79, 0x5f, 0x74, 0x12, 0x0c, 0x0a, 0x04, 0x74,
    0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x12, 0x0a, 0x0a, 0x76, 0x61, 0x6c,
    0x5f, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x11, 0x0a,
    0x09, 0x76, 0x61, 0x6c, 0x5f, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x02,
    0x12, 0x10, 0x0a, 0x08, 0x76, 0x61, 0x6c, 0x5f, 0x6c, 0x6f, 0x6e, 0x67, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x05, 0x12, 0x11, 0x0a, 0x09, 0x76, 0x61, 0x6c, 0x5f, 0x73, 0x68, 0x6f, 0x72, 0x74, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x05, 0x12, 0x10, 0x0a, 0x08, 0x76, 0x61, 0x6c, 0x5f, 0x62, 0x79, 0x74,
    0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x05, 0x12, 0x10, 0x0a, 0x08, 0x76, 0x61, 0x6c, 0x5f, 0x62,
    0x6f, 0x6f, 0x6c, 0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x12, 0x12, 0x0a, 0x0a, 0x76, 0x61, 0x6c,
    0x5f, 0x75, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x18, 0x08, 0x20, 0x01, 0x28, 0x04, 0x22, 0xd1, 0x01,
    0x0a, 0x15, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x47, 0x61, 0x6d, 0x65, 0x45, 0x76,
    0x65, 0x6e, 0x74, 0x4c, 0x69, 0x73, 0x74, 0x12, 0x38, 0x0a, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72,
    0x69, 0x70, 0x74, 0x6f, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x43,
    0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x47, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74,
    0x4c, 0x69, 0x73, 0x74, 0x2e, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x5f,
    0x74, 0x1a, 0x23, 0x0a, 0x05, 0x6b, 0x65, 0x79, 0x5f, 0x74, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x79,
    0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x1a, 0x59, 0x0a, 0x0c, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69,
    0x70, 0x74, 0x6f, 0x72, 0x5f, 0x74, 0x12, 0x0f, 0x0a, 0x07, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x2a, 0x0a, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x18, 0x03, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x47, 0x61,
    0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x4c, 0x69, 0x73, 0x74, 0x2e, 0x6b, 0x65, 0x79, 0x5f,
    0x74, 0x22, 0xac, 0x01, 0x0a, 0x16, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x50, 0x61,
    0x63, 0x6b, 0x65, 0x74, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x69, 0x65, 0x73, 0x12, 0x13, 0x0a, 0x0b,
    0x6d, 0x61, 0x78, 0x5f, 0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x05, 0x12, 0x17, 0x0a, 0x0f, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x65, 0x6e, 0x74,
    0x72, 0x69, 0x65, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12, 0x10, 0x0a, 0x08, 0x69, 0x73,
    0x5f, 0x64, 0x65, 0x6c, 0x74, 0x61, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x12, 0x17, 0x0a, 0x0f,
    0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x62, 0x61, 0x73, 0x65, 0x6c, 0x69, 0x6e, 0x65, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x08, 0x12, 0x10, 0x0a, 0x08, 0x62, 0x61, 0x73, 0x65, 0x6c, 0x69, 0x6e,
    0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05, 0x12, 0x12, 0x0a, 0x0a, 0x64, 0x65, 0x6c, 0x74, 0x61,
    0x5f, 0x66, 0x72, 0x6f, 0x6d, 0x18, 0x06, 0x20, 0x01, 0x28, 0x05, 0x12, 0x13, 0x0a, 0x0b, 0x65,
    0x6e, 0x74, 0x69, 0x74, 0x79, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0c,
    0x22, 0x52, 0x0a, 0x14, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x54, 0x65, 0x6d, 0x70,
    0x45, 0x6e, 0x74, 0x69, 0x74, 0x69, 0x65, 0x73, 0x12, 0x10, 0x0a, 0x08, 0x72, 0x65, 0x6c, 0x69,
    0x61, 0x62, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x12, 0x13, 0x0a, 0x0b, 0x6e, 0x75,
    0x6d, 0x5f, 0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12,
    0x13, 0x0a, 0x0b, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0c, 0x22, 0xca, 0x01, 0x0a, 0x19, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67,
    0x5f, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x54, 0x61, 0x62,
    0x6c, 0x65, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x12, 0x13, 0x0a, 0x0b, 0x6d, 0x61, 0x78, 0x5f, 0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x05, 0x12, 0x13, 0x0a, 0x0b, 0x6e, 0x75, 0x6d, 0x5f, 0x65, 0x6e, 0x74,
    0x72, 0x69, 0x65, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x12, 0x1c, 0x0a, 0x14, 0x75, 0x73,
    0x65, 0x72, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x66, 0x69, 0x78, 0x65, 0x64, 0x5f, 0x73, 0x69,
    0x7a, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x12, 0x16, 0x0a, 0x0e, 0x75, 0x73, 0x65, 0x72,
    0x5f, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05,
    0x12, 0x1b, 0x0a, 0x13, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x73, 0x69,
    0x7a, 0x65, 0x5f, 0x62, 0x69, 0x74, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0d, 0x0a,
    0x05, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x18, 0x07, 0x20, 0x01, 0x28, 0x05, 0x12, 0x13, 0x0a, 0x0b,
    0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x08, 0x20, 0x01, 0x28,
    0x0c, 0x22, 0x5f, 0x0a, 0x19, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x55, 0x70, 0x64,
    0x61, 0x74, 0x65, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x12, 0x10,
    0x0a, 0x08, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05,
    0x12, 0x1b, 0x0a, 0x13, 0x6e, 0x75, 0x6d, 0x5f, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x64, 0x5f,
    0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12, 0x13, 0x0a,
    0x0b, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0c, 0x22, 0x39, 0x0a, 0x13, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x55, 0x73,
    0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x10, 0x0a, 0x08, 0x6d, 0x73, 0x67,
    0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x10, 0x0a, 0x08, 0x6d,
    0x73, 0x67, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x6e, 0x0a,
    0x11, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x56, 0x6f, 0x69, 0x63, 0x65, 0x44, 0x61,
    0x74, 0x61, 0x12, 0x0e, 0x0a, 0x06, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x05, 0x12, 0x11, 0x0a, 0x09, 0x70, 0x72, 0x6f, 0x78, 0x69, 0x6d, 0x69, 0x74, 0x79, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x08, 0x12, 0x0c, 0x0a, 0x04, 0x78, 0x75, 0x69, 0x64, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x06, 0x12, 0x14, 0x0a, 0x0c, 0x61, 0x75, 0x64, 0x69, 0x62, 0x6c, 0x65, 0x5f, 0x6d,
    0x61, 0x73, 0x6b, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05, 0x12, 0x12, 0x0a, 0x0a, 0x76, 0x6f, 0x69,
    0x63, 0x65, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x85, 0x01,
    0x0a, 0x16, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x4c, 0x69, 0x73, 0x74, 0x5f, 0x47, 0x61,
    0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x12, 0x2f, 0x0a, 0x06, 0x65, 0x76, 0x65, 0x6e,
    0x74, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x43, 0x53, 0x56, 0x43, 0x4d,
    0x73, 0x67, 0x4c, 0x69, 0x73, 0x74, 0x5f, 0x47, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74,
    0x73, 0x2e, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x1a, 0x3a, 0x0a, 0x07, 0x65, 0x76, 0x65,
    0x6e, 0x74, 0x5f, 0x74, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x69, 0x63, 0x6b, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x05, 0x12, 0x21, 0x0a, 0x05, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x12, 0x2e, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x47, 0x61, 0x6d, 0x65,
    0x45, 0x76, 0x65, 0x6e, 0x74, 0x22, 0x8f, 0x01, 0x0a, 0x18, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73,
    0x67, 0x4c, 0x69, 0x73, 0x74, 0x5f, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x73, 0x12, 0x35, 0x0a, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6d, 0x73, 0x67, 0x73, 0x18, 0x01,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x4c, 0x69,
    0x73, 0x74, 0x5f, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e,
    0x75, 0x73, 0x65, 0x72, 0x6d, 0x73, 0x67, 0x5f, 0x74, 0x1a, 0x3c, 0x0a, 0x09, 0x75, 0x73, 0x65,
    0x72, 0x6d, 0x73, 0x67, 0x5f, 0x74, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x69, 0x63, 0x6b, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x05, 0x12, 0x21, 0x0a, 0x03, 0x6d, 0x73, 0x67, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x14, 0x2e, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x55, 0x73, 0x65, 0x72,
    0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2a, 0x9f, 0x01, 0x0a, 0x0c, 0x4e, 0x45, 0x54, 0x5f,
    0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x12, 0x0b, 0x0a, 0x07, 0x6e, 0x65, 0x74, 0x5f,
    0x4e, 0x4f, 0x50, 0x10, 0x00, 0x12, 0x12, 0x0a, 0x0e, 0x6e, 0x65, 0x74, 0x5f, 0x44, 0x69, 0x73,
    0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x10, 0x01, 0x12, 0x0c, 0x0a, 0x08, 0x6e, 0x65, 0x74,
    0x5f, 0x46, 0x69, 0x6c, 0x65, 0x10, 0x02, 0x12, 0x17, 0x0a, 0x13, 0x6e, 0x65, 0x74, 0x5f, 0x53,
    0x70, 0x6c, 0x69, 0x74, 0x53, 0x63, 0x72, 0x65, 0x65, 0x6e, 0x55, 0x73, 0x65, 0x72, 0x10, 0x03,
    0x12, 0x0c, 0x0a, 0x08, 0x6e, 0x65, 0x74, 0x5f, 0x54, 0x69, 0x63, 0x6b, 0x10, 0x04, 0x12, 0x11,
    0x0a, 0x0d, 0x6e, 0x65, 0x74, 0x5f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x43, 0x6d, 0x64, 0x10,
    0x05, 0x12, 0x11, 0x0a, 0x0d, 0x6e, 0x65, 0x74, 0x5f, 0x53, 0x65, 0x74, 0x43, 0x6f, 0x6e, 0x56,
    0x61, 0x72, 0x10, 0x06, 0x12, 0x13, 0x0a, 0x0f, 0x6e, 0x65, 0x74, 0x5f, 0x53, 0x69, 0x67, 0x6e,
    0x6f, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x65, 0x10, 0x07, 0x2a, 0xd2, 0x01, 0x0a, 0x0b, 0x53, 0x49,
    0x47, 0x4e, 0x4f, 0x4e, 0x53, 0x54, 0x41, 0x54, 0x45, 0x12, 0x14, 0x0a, 0x10, 0x53, 0x49, 0x47,
    0x4e, 0x4f, 0x4e, 0x53, 0x54, 0x41, 0x54, 0x45, 0x5f, 0x4e, 0x4f, 0x4e, 0x45, 0x10, 0x00, 0x12,
    0x19, 0x0a, 0x15, 0x53, 0x49, 0x47, 0x4e, 0x4f, 0x4e, 0x53, 0x54, 0x41, 0x54, 0x45, 0x5f, 0x43,
    0x48, 0x41, 0x4c, 0x4c, 0x45, 0x4e, 0x47, 0x45, 0x10, 0x01, 0x12, 0x19, 0x0a, 0x15, 0x53, 0x49,
    0x47, 0x4e, 0x4f, 0x4e, 0x53, 0x54, 0x41, 0x54, 0x45, 0x5f, 0x43, 0x4f, 0x4e, 0x4e, 0x45, 0x43,
    0x54, 0x45, 0x44, 0x10, 0x02, 0x12, 0x13, 0x0a, 0x0f, 0x53, 0x49, 0x47, 0x4e, 0x4f, 0x4e, 0x53,
    0x54, 0x41, 0x54, 0x45, 0x5f, 0x4e, 0x45, 0x57, 0x10, 0x03, 0x12, 0x18, 0x0a, 0x14, 0x53, 0x49,
    0x47, 0x4e, 0x4f, 0x4e, 0x53, 0x54, 0x41, 0x54, 0x45, 0x5f, 0x50, 0x52, 0x45, 0x53, 0x50, 0x41,
    0x57, 0x4e, 0x10, 0x04, 0x12, 0x15, 0x0a, 0x11, 0x53, 0x49, 0x47, 0x4e, 0x4f, 0x4e, 0x53, 0x54,
    0x41, 0x54, 0x45, 0x5f, 0x53, 0x50, 0x41, 0x57, 0x4e, 0x10, 0x05, 0x12, 0x14, 0x0a, 0x10, 0x53,
    0x49, 0x47, 0x4e, 0x4f, 0x4e, 0x53, 0x54, 0x41, 0x54, 0x45, 0x5f, 0x46, 0x55, 0x4c, 0x4c, 0x10,
    0x06, 0x12, 0x1b, 0x0a, 0x17, 0x53, 0x49, 0x47, 0x4e, 0x4f, 0x4e, 0x53, 0x54, 0x41, 0x54, 0x45,
    0x5f, 0x43, 0x48, 0x41, 0x4e, 0x47, 0x45, 0x4c, 0x45, 0x56, 0x45, 0x4c, 0x10, 0x07, 0x2a, 0xea,
    0x01, 0x0a, 0x0c, 0x43, 0x4c, 0x43, 0x5f, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x12,
    0x12, 0x0a, 0x0e, 0x63, 0x6c, 0x63, 0x5f, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x6e, 0x66,
    0x6f, 0x10, 0x08, 0x12, 0x0c, 0x0a, 0x08, 0x63, 0x6c, 0x63, 0x5f, 0x4d, 0x6f, 0x76, 0x65, 0x10,
    0x09, 0x12, 0x11, 0x0a, 0x0d, 0x63, 0x6c, 0x63, 0x5f, 0x56, 0x6f, 0x69, 0x63, 0x65, 0x44, 0x61,
    0x74, 0x61, 0x10, 0x0a, 0x12, 0x13, 0x0a, 0x0f, 0x63, 0x6c, 0x63, 0x5f, 0x42, 0x61, 0x73, 0x65,
    0x6c, 0x69, 0x6e, 0x65, 0x41, 0x63, 0x6b, 0x10, 0x0b, 0x12, 0x14, 0x0a, 0x10, 0x63, 0x6c, 0x63,
    0x5f, 0x4c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x10, 0x0c, 0x12,
    0x18, 0x0a, 0x14, 0x63, 0x6c, 0x63, 0x5f, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x64, 0x43, 0x76,
    0x61, 0x72, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x10, 0x0d, 0x12, 0x14, 0x0a, 0x10, 0x63, 0x6c, 0x63,
    0x5f, 0x46, 0x69, 0x6c, 0x65, 0x43, 0x52, 0x43, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x10, 0x0e, 0x12,
    0x17, 0x0a, 0x13, 0x63, 0x6c, 0x63, 0x5f, 0x4c, 0x6f, 0x61, 0x64, 0x69, 0x6e, 0x67, 0x50, 0x72,
    0x6f, 0x67, 0x72, 0x65, 0x73, 0x73, 0x10, 0x0f, 0x12, 0x1a, 0x0a, 0x16, 0x63, 0x6c, 0x63, 0x5f,
    0x53, 0x70, 0x6c, 0x69, 0x74, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x6e, 0x65,
    0x63, 0x74, 0x10, 0x10, 0x12, 0x15, 0x0a, 0x11, 0x63, 0x6c, 0x63, 0x5f, 0x43, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x10, 0x11, 0x2a, 0xf1, 0x03, 0x0a, 0x0c,
    0x53, 0x56, 0x43, 0x5f, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x12, 0x12, 0x0a, 0x0e,
    0x73, 0x76, 0x63, 0x5f, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x10, 0x08,
    0x12, 0x11, 0x0a, 0x0d, 0x73, 0x76, 0x63, 0x5f, 0x53, 0x65, 0x6e, 0x64, 0x54, 0x61, 0x62, 0x6c,
    0x65, 0x10, 0x09, 0x12, 0x11, 0x0a, 0x0d, 0x73, 0x76, 0x63, 0x5f, 0x43, 0x6c, 0x61, 0x73, 0x73,
    0x49, 0x6e, 0x66, 0x6f, 0x10, 0x0a, 0x12, 0x10, 0x0a, 0x0c, 0x73, 0x76, 0x63, 0x5f, 0x53, 0x65,
    0x74, 0x50, 0x61, 0x75, 0x73, 0x65, 0x10, 0x0b, 0x12, 0x19, 0x0a, 0x15, 0x73, 0x76, 0x63, 0x5f,
    0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x54, 0x61, 0x62, 0x6c,
    0x65, 0x10, 0x0c, 0x12, 0x19, 0x0a, 0x15, 0x73, 0x76, 0x63, 0x5f, 0x55, 0x70, 0x64, 0x61, 0x74,
    0x65, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x10, 0x0d, 0x12, 0x11,
    0x0a, 0x0d, 0x73, 0x76, 0x63, 0x5f, 0x56, 0x6f, 0x69, 0x63, 0x65, 0x49, 0x6e, 0x69, 0x74, 0x10,
    0x0e, 0x12, 0x11, 0x0a, 0x0d, 0x73, 0x76, 0x63, 0x5f, 0x56, 0x6f, 0x69, 0x63, 0x65, 0x44, 0x61,
    0x74, 0x61, 0x10, 0x0f, 0x12, 0x0d, 0x0a, 0x09, 0x73, 0x76, 0x63, 0x5f, 0x50, 0x72, 0x69, 0x6e,
    0x74, 0x10, 0x10, 0x12, 0x0e, 0x0a, 0x0a, 0x73, 0x76, 0x63, 0x5f, 0x53, 0x6f, 0x75, 0x6e, 0x64,
    0x73, 0x10, 0x11, 0x12, 0x0f, 0x0a, 0x0b, 0x73, 0x76, 0x63, 0x5f, 0x53, 0x65, 0x74, 0x56, 0x69,
    0x65, 0x77, 0x10, 0x12, 0x12, 0x10, 0x0a, 0x0c, 0x73, 0x76, 0x63, 0x5f, 0x46, 0x69, 0x78, 0x41,
    0x6e, 0x67, 0x6c, 0x65, 0x10, 0x13, 0x12, 0x16, 0x0a, 0x12, 0x73, 0x76, 0x63, 0x5f, 0x43, 0x72,
    0x6f, 0x73, 0x73, 0x68, 0x61, 0x69, 0x72, 0x41, 0x6e, 0x67, 0x6c, 0x65, 0x10, 0x14, 0x12, 0x10,
    0x0a, 0x0c, 0x73, 0x76, 0x63, 0x5f, 0x42, 0x53, 0x50, 0x44, 0x65, 0x63, 0x61, 0x6c, 0x10, 0x15,
    0x12, 0x13, 0x0a, 0x0f, 0x73, 0x76, 0x63, 0x5f, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x53, 0x63, 0x72,
    0x65, 0x65, 0x6e, 0x10, 0x16, 0x12, 0x13, 0x0a, 0x0f, 0x73, 0x76, 0x63, 0x5f, 0x55, 0x73, 0x65,
    0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x10, 0x17, 0x12, 0x15, 0x0a, 0x11, 0x73, 0x76,
    0x63, 0x5f, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x10,
    0x18, 0x12, 0x11, 0x0a, 0x0d, 0x73, 0x76, 0x63, 0x5f, 0x47, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65,
    0x6e, 0x74, 0x10, 0x19, 0x12, 0x16, 0x0a, 0x12, 0x73, 0x76, 0x63, 0x5f, 0x50, 0x61, 0x63, 0x6b,
    0x65, 0x74, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x69, 0x65, 0x73, 0x10, 0x1a, 0x12, 0x14, 0x0a, 0x10,
    0x73, 0x76, 0x63, 0x5f, 0x54, 0x65, 0x6d, 0x70, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x69, 0x65, 0x73,
    0x10, 0x1b, 0x12, 0x10, 0x0a, 0x0c, 0x73, 0x76, 0x63, 0x5f, 0x50, 0x72, 0x65, 0x66, 0x65, 0x74,
    0x63, 0x68, 0x10, 0x1c, 0x12, 0x0c, 0x0a, 0x08, 0x73, 0x76, 0x63, 0x5f, 0x4d, 0x65, 0x6e, 0x75,
    0x10, 0x1d, 0x12, 0x15, 0x0a, 0x11, 0x73, 0x76, 0x63, 0x5f, 0x47, 0x61, 0x6d, 0x65, 0x45, 0x76,
    0x65, 0x6e, 0x74, 0x4c, 0x69, 0x73, 0x74, 0x10, 0x1e, 0x12, 0x14, 0x0a, 0x10, 0x73, 0x76, 0x63,
    0x5f, 0x47, 0x65, 0x74, 0x43, 0x76, 0x61, 0x72, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x10, 0x1f, 0x2a,
    0x75, 0x0a, 0x17, 0x45, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x53, 0x63, 0x72, 0x65, 0x65, 0x6e, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x1b, 0x0a, 0x17, 0x4d, 0x53,
    0x47, 0x5f, 0x53, 0x50, 0x4c, 0x49, 0x54, 0x53, 0x43, 0x52, 0x45, 0x45, 0x4e, 0x5f, 0x41, 0x44,
    0x44, 0x55, 0x53, 0x45, 0x52, 0x10, 0x00, 0x12, 0x1e, 0x0a, 0x1a, 0x4d, 0x53, 0x47, 0x5f, 0x53,
    0x50, 0x4c, 0x49, 0x54, 0x53, 0x43, 0x52, 0x45, 0x45, 0x4e, 0x5f, 0x52, 0x45, 0x4d, 0x4f, 0x56,
    0x45, 0x55, 0x53, 0x45, 0x52, 0x10, 0x01, 0x12, 0x1d, 0x0a, 0x19, 0x4d, 0x53, 0x47, 0x5f, 0x53,
    0x50, 0x4c, 0x49, 0x54, 0x53, 0x43, 0x52, 0x45, 0x45, 0x4e, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f,
    0x42, 0x49, 0x54, 0x53, 0x10, 0x01, 0x42, 0x03, 0x80, 0x01, 0x00,
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
