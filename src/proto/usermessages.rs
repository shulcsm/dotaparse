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
pub struct CUserMsg_AchievementEvent {
    // message fields
    achievement: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_AchievementEvent {}

impl CUserMsg_AchievementEvent {
    pub fn new() -> CUserMsg_AchievementEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_AchievementEvent {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_AchievementEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_AchievementEvent,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_AchievementEvent {
                    achievement: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 achievement = 1;

    pub fn clear_achievement(&mut self) {
        self.achievement = ::std::option::Option::None;
    }

    pub fn has_achievement(&self) -> bool {
        self.achievement.is_some()
    }

    // Param is passed by value, moved
    pub fn set_achievement(&mut self, v: u32) {
        self.achievement = ::std::option::Option::Some(v);
    }

    pub fn get_achievement(&self) -> u32 {
        self.achievement.unwrap_or(0)
    }
}

impl ::protobuf::Message for CUserMsg_AchievementEvent {
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
                    self.achievement = ::std::option::Option::Some(tmp);
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
        for value in &self.achievement {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.achievement {
            try!(os.write_uint32(1, v));
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
        ::std::any::TypeId::of::<CUserMsg_AchievementEvent>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_AchievementEvent {
    fn new() -> CUserMsg_AchievementEvent {
        CUserMsg_AchievementEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_AchievementEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "achievement",
                    CUserMsg_AchievementEvent::has_achievement,
                    CUserMsg_AchievementEvent::get_achievement,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_AchievementEvent>(
                    "CUserMsg_AchievementEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_AchievementEvent {
    fn clear(&mut self) {
        self.clear_achievement();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_AchievementEvent {
    fn eq(&self, other: &CUserMsg_AchievementEvent) -> bool {
        self.achievement == other.achievement &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_AchievementEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_CloseCaption {
    // message fields
    hash: ::std::option::Option<u32>,
    duration: ::std::option::Option<f32>,
    from_player: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_CloseCaption {}

impl CUserMsg_CloseCaption {
    pub fn new() -> CUserMsg_CloseCaption {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_CloseCaption {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_CloseCaption> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_CloseCaption,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_CloseCaption {
                    hash: ::std::option::Option::None,
                    duration: ::std::option::Option::None,
                    from_player: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional fixed32 hash = 1;

    pub fn clear_hash(&mut self) {
        self.hash = ::std::option::Option::None;
    }

    pub fn has_hash(&self) -> bool {
        self.hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: u32) {
        self.hash = ::std::option::Option::Some(v);
    }

    pub fn get_hash(&self) -> u32 {
        self.hash.unwrap_or(0)
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

    // optional bool from_player = 3;

    pub fn clear_from_player(&mut self) {
        self.from_player = ::std::option::Option::None;
    }

    pub fn has_from_player(&self) -> bool {
        self.from_player.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_player(&mut self, v: bool) {
        self.from_player = ::std::option::Option::Some(v);
    }

    pub fn get_from_player(&self) -> bool {
        self.from_player.unwrap_or(false)
    }
}

impl ::protobuf::Message for CUserMsg_CloseCaption {
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
                    self.hash = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.duration = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.from_player = ::std::option::Option::Some(tmp);
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
        if self.hash.is_some() {
            my_size += 5;
        };
        if self.duration.is_some() {
            my_size += 5;
        };
        if self.from_player.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hash {
            try!(os.write_fixed32(1, v));
        };
        if let Some(v) = self.duration {
            try!(os.write_float(2, v));
        };
        if let Some(v) = self.from_player {
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
        ::std::any::TypeId::of::<CUserMsg_CloseCaption>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_CloseCaption {
    fn new() -> CUserMsg_CloseCaption {
        CUserMsg_CloseCaption::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_CloseCaption>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "hash",
                    CUserMsg_CloseCaption::has_hash,
                    CUserMsg_CloseCaption::get_hash,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "duration",
                    CUserMsg_CloseCaption::has_duration,
                    CUserMsg_CloseCaption::get_duration,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "from_player",
                    CUserMsg_CloseCaption::has_from_player,
                    CUserMsg_CloseCaption::get_from_player,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_CloseCaption>(
                    "CUserMsg_CloseCaption",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_CloseCaption {
    fn clear(&mut self) {
        self.clear_hash();
        self.clear_duration();
        self.clear_from_player();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_CloseCaption {
    fn eq(&self, other: &CUserMsg_CloseCaption) -> bool {
        self.hash == other.hash &&
        self.duration == other.duration &&
        self.from_player == other.from_player &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_CloseCaption {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_CurrentTimescale {
    // message fields
    current: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_CurrentTimescale {}

impl CUserMsg_CurrentTimescale {
    pub fn new() -> CUserMsg_CurrentTimescale {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_CurrentTimescale {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_CurrentTimescale> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_CurrentTimescale,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_CurrentTimescale {
                    current: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional float current = 1;

    pub fn clear_current(&mut self) {
        self.current = ::std::option::Option::None;
    }

    pub fn has_current(&self) -> bool {
        self.current.is_some()
    }

    // Param is passed by value, moved
    pub fn set_current(&mut self, v: f32) {
        self.current = ::std::option::Option::Some(v);
    }

    pub fn get_current(&self) -> f32 {
        self.current.unwrap_or(0.)
    }
}

impl ::protobuf::Message for CUserMsg_CurrentTimescale {
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
                    self.current = ::std::option::Option::Some(tmp);
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
        if self.current.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.current {
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
        ::std::any::TypeId::of::<CUserMsg_CurrentTimescale>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_CurrentTimescale {
    fn new() -> CUserMsg_CurrentTimescale {
        CUserMsg_CurrentTimescale::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_CurrentTimescale>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "current",
                    CUserMsg_CurrentTimescale::has_current,
                    CUserMsg_CurrentTimescale::get_current,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_CurrentTimescale>(
                    "CUserMsg_CurrentTimescale",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_CurrentTimescale {
    fn clear(&mut self) {
        self.clear_current();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_CurrentTimescale {
    fn eq(&self, other: &CUserMsg_CurrentTimescale) -> bool {
        self.current == other.current &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_CurrentTimescale {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_DesiredTimescale {
    // message fields
    desired: ::std::option::Option<f32>,
    duration: ::std::option::Option<f32>,
    interpolator: ::std::option::Option<u32>,
    start_blend_time: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_DesiredTimescale {}

impl CUserMsg_DesiredTimescale {
    pub fn new() -> CUserMsg_DesiredTimescale {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_DesiredTimescale {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_DesiredTimescale> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_DesiredTimescale,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_DesiredTimescale {
                    desired: ::std::option::Option::None,
                    duration: ::std::option::Option::None,
                    interpolator: ::std::option::Option::None,
                    start_blend_time: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional float desired = 1;

    pub fn clear_desired(&mut self) {
        self.desired = ::std::option::Option::None;
    }

    pub fn has_desired(&self) -> bool {
        self.desired.is_some()
    }

    // Param is passed by value, moved
    pub fn set_desired(&mut self, v: f32) {
        self.desired = ::std::option::Option::Some(v);
    }

    pub fn get_desired(&self) -> f32 {
        self.desired.unwrap_or(0.)
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

    // optional uint32 interpolator = 3;

    pub fn clear_interpolator(&mut self) {
        self.interpolator = ::std::option::Option::None;
    }

    pub fn has_interpolator(&self) -> bool {
        self.interpolator.is_some()
    }

    // Param is passed by value, moved
    pub fn set_interpolator(&mut self, v: u32) {
        self.interpolator = ::std::option::Option::Some(v);
    }

    pub fn get_interpolator(&self) -> u32 {
        self.interpolator.unwrap_or(0)
    }

    // optional float start_blend_time = 4;

    pub fn clear_start_blend_time(&mut self) {
        self.start_blend_time = ::std::option::Option::None;
    }

    pub fn has_start_blend_time(&self) -> bool {
        self.start_blend_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_blend_time(&mut self, v: f32) {
        self.start_blend_time = ::std::option::Option::Some(v);
    }

    pub fn get_start_blend_time(&self) -> f32 {
        self.start_blend_time.unwrap_or(0.)
    }
}

impl ::protobuf::Message for CUserMsg_DesiredTimescale {
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
                    self.desired = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.duration = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.interpolator = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.start_blend_time = ::std::option::Option::Some(tmp);
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
        if self.desired.is_some() {
            my_size += 5;
        };
        if self.duration.is_some() {
            my_size += 5;
        };
        for value in &self.interpolator {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.start_blend_time.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.desired {
            try!(os.write_float(1, v));
        };
        if let Some(v) = self.duration {
            try!(os.write_float(2, v));
        };
        if let Some(v) = self.interpolator {
            try!(os.write_uint32(3, v));
        };
        if let Some(v) = self.start_blend_time {
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
        ::std::any::TypeId::of::<CUserMsg_DesiredTimescale>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_DesiredTimescale {
    fn new() -> CUserMsg_DesiredTimescale {
        CUserMsg_DesiredTimescale::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_DesiredTimescale>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "desired",
                    CUserMsg_DesiredTimescale::has_desired,
                    CUserMsg_DesiredTimescale::get_desired,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "duration",
                    CUserMsg_DesiredTimescale::has_duration,
                    CUserMsg_DesiredTimescale::get_duration,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "interpolator",
                    CUserMsg_DesiredTimescale::has_interpolator,
                    CUserMsg_DesiredTimescale::get_interpolator,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "start_blend_time",
                    CUserMsg_DesiredTimescale::has_start_blend_time,
                    CUserMsg_DesiredTimescale::get_start_blend_time,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_DesiredTimescale>(
                    "CUserMsg_DesiredTimescale",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_DesiredTimescale {
    fn clear(&mut self) {
        self.clear_desired();
        self.clear_duration();
        self.clear_interpolator();
        self.clear_start_blend_time();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_DesiredTimescale {
    fn eq(&self, other: &CUserMsg_DesiredTimescale) -> bool {
        self.desired == other.desired &&
        self.duration == other.duration &&
        self.interpolator == other.interpolator &&
        self.start_blend_time == other.start_blend_time &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_DesiredTimescale {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_Fade {
    // message fields
    duration: ::std::option::Option<u32>,
    hold_time: ::std::option::Option<u32>,
    flags: ::std::option::Option<u32>,
    color: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_Fade {}

impl CUserMsg_Fade {
    pub fn new() -> CUserMsg_Fade {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_Fade {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_Fade> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_Fade,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_Fade {
                    duration: ::std::option::Option::None,
                    hold_time: ::std::option::Option::None,
                    flags: ::std::option::Option::None,
                    color: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 duration = 1;

    pub fn clear_duration(&mut self) {
        self.duration = ::std::option::Option::None;
    }

    pub fn has_duration(&self) -> bool {
        self.duration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration(&mut self, v: u32) {
        self.duration = ::std::option::Option::Some(v);
    }

    pub fn get_duration(&self) -> u32 {
        self.duration.unwrap_or(0)
    }

    // optional uint32 hold_time = 2;

    pub fn clear_hold_time(&mut self) {
        self.hold_time = ::std::option::Option::None;
    }

    pub fn has_hold_time(&self) -> bool {
        self.hold_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hold_time(&mut self, v: u32) {
        self.hold_time = ::std::option::Option::Some(v);
    }

    pub fn get_hold_time(&self) -> u32 {
        self.hold_time.unwrap_or(0)
    }

    // optional uint32 flags = 3;

    pub fn clear_flags(&mut self) {
        self.flags = ::std::option::Option::None;
    }

    pub fn has_flags(&self) -> bool {
        self.flags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flags(&mut self, v: u32) {
        self.flags = ::std::option::Option::Some(v);
    }

    pub fn get_flags(&self) -> u32 {
        self.flags.unwrap_or(0)
    }

    // optional fixed32 color = 4;

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
}

impl ::protobuf::Message for CUserMsg_Fade {
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
                    self.duration = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.hold_time = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.flags = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_fixed32());
                    self.color = ::std::option::Option::Some(tmp);
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
        for value in &self.duration {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.hold_time {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.flags {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.color.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.duration {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.hold_time {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.flags {
            try!(os.write_uint32(3, v));
        };
        if let Some(v) = self.color {
            try!(os.write_fixed32(4, v));
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
        ::std::any::TypeId::of::<CUserMsg_Fade>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_Fade {
    fn new() -> CUserMsg_Fade {
        CUserMsg_Fade::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_Fade>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "duration",
                    CUserMsg_Fade::has_duration,
                    CUserMsg_Fade::get_duration,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "hold_time",
                    CUserMsg_Fade::has_hold_time,
                    CUserMsg_Fade::get_hold_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "flags",
                    CUserMsg_Fade::has_flags,
                    CUserMsg_Fade::get_flags,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "color",
                    CUserMsg_Fade::has_color,
                    CUserMsg_Fade::get_color,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_Fade>(
                    "CUserMsg_Fade",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_Fade {
    fn clear(&mut self) {
        self.clear_duration();
        self.clear_hold_time();
        self.clear_flags();
        self.clear_color();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_Fade {
    fn eq(&self, other: &CUserMsg_Fade) -> bool {
        self.duration == other.duration &&
        self.hold_time == other.hold_time &&
        self.flags == other.flags &&
        self.color == other.color &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_Fade {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_Shake {
    // message fields
    command: ::std::option::Option<u32>,
    amplitude: ::std::option::Option<f32>,
    frequency: ::std::option::Option<f32>,
    duration: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_Shake {}

impl CUserMsg_Shake {
    pub fn new() -> CUserMsg_Shake {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_Shake {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_Shake> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_Shake,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_Shake {
                    command: ::std::option::Option::None,
                    amplitude: ::std::option::Option::None,
                    frequency: ::std::option::Option::None,
                    duration: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 command = 1;

    pub fn clear_command(&mut self) {
        self.command = ::std::option::Option::None;
    }

    pub fn has_command(&self) -> bool {
        self.command.is_some()
    }

    // Param is passed by value, moved
    pub fn set_command(&mut self, v: u32) {
        self.command = ::std::option::Option::Some(v);
    }

    pub fn get_command(&self) -> u32 {
        self.command.unwrap_or(0)
    }

    // optional float amplitude = 2;

    pub fn clear_amplitude(&mut self) {
        self.amplitude = ::std::option::Option::None;
    }

    pub fn has_amplitude(&self) -> bool {
        self.amplitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_amplitude(&mut self, v: f32) {
        self.amplitude = ::std::option::Option::Some(v);
    }

    pub fn get_amplitude(&self) -> f32 {
        self.amplitude.unwrap_or(0.)
    }

    // optional float frequency = 3;

    pub fn clear_frequency(&mut self) {
        self.frequency = ::std::option::Option::None;
    }

    pub fn has_frequency(&self) -> bool {
        self.frequency.is_some()
    }

    // Param is passed by value, moved
    pub fn set_frequency(&mut self, v: f32) {
        self.frequency = ::std::option::Option::Some(v);
    }

    pub fn get_frequency(&self) -> f32 {
        self.frequency.unwrap_or(0.)
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

impl ::protobuf::Message for CUserMsg_Shake {
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
                    self.command = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.amplitude = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.frequency = ::std::option::Option::Some(tmp);
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
        for value in &self.command {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.amplitude.is_some() {
            my_size += 5;
        };
        if self.frequency.is_some() {
            my_size += 5;
        };
        if self.duration.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.command {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.amplitude {
            try!(os.write_float(2, v));
        };
        if let Some(v) = self.frequency {
            try!(os.write_float(3, v));
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
        ::std::any::TypeId::of::<CUserMsg_Shake>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_Shake {
    fn new() -> CUserMsg_Shake {
        CUserMsg_Shake::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_Shake>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "command",
                    CUserMsg_Shake::has_command,
                    CUserMsg_Shake::get_command,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "amplitude",
                    CUserMsg_Shake::has_amplitude,
                    CUserMsg_Shake::get_amplitude,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "frequency",
                    CUserMsg_Shake::has_frequency,
                    CUserMsg_Shake::get_frequency,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "duration",
                    CUserMsg_Shake::has_duration,
                    CUserMsg_Shake::get_duration,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_Shake>(
                    "CUserMsg_Shake",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_Shake {
    fn clear(&mut self) {
        self.clear_command();
        self.clear_amplitude();
        self.clear_frequency();
        self.clear_duration();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_Shake {
    fn eq(&self, other: &CUserMsg_Shake) -> bool {
        self.command == other.command &&
        self.amplitude == other.amplitude &&
        self.frequency == other.frequency &&
        self.duration == other.duration &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_Shake {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_ShakeDir {
    // message fields
    shake: ::protobuf::SingularPtrField<CUserMsg_Shake>,
    direction: ::protobuf::SingularPtrField<super::netmessages::CMsgVector>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_ShakeDir {}

impl CUserMsg_ShakeDir {
    pub fn new() -> CUserMsg_ShakeDir {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_ShakeDir {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_ShakeDir> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_ShakeDir,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_ShakeDir {
                    shake: ::protobuf::SingularPtrField::none(),
                    direction: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .CUserMsg_Shake shake = 1;

    pub fn clear_shake(&mut self) {
        self.shake.clear();
    }

    pub fn has_shake(&self) -> bool {
        self.shake.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shake(&mut self, v: CUserMsg_Shake) {
        self.shake = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_shake(&mut self) -> &mut CUserMsg_Shake {
        if self.shake.is_none() {
            self.shake.set_default();
        };
        self.shake.as_mut().unwrap()
    }

    // Take field
    pub fn take_shake(&mut self) -> CUserMsg_Shake {
        self.shake.take().unwrap_or_else(|| CUserMsg_Shake::new())
    }

    pub fn get_shake(&self) -> &CUserMsg_Shake {
        self.shake.as_ref().unwrap_or_else(|| CUserMsg_Shake::default_instance())
    }

    // optional .CMsgVector direction = 2;

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
}

impl ::protobuf::Message for CUserMsg_ShakeDir {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.shake));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.direction));
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
        for value in &self.shake {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.direction {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.shake.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.direction.as_ref() {
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
        ::std::any::TypeId::of::<CUserMsg_ShakeDir>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_ShakeDir {
    fn new() -> CUserMsg_ShakeDir {
        CUserMsg_ShakeDir::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_ShakeDir>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "shake",
                    CUserMsg_ShakeDir::has_shake,
                    CUserMsg_ShakeDir::get_shake,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "direction",
                    CUserMsg_ShakeDir::has_direction,
                    CUserMsg_ShakeDir::get_direction,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_ShakeDir>(
                    "CUserMsg_ShakeDir",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_ShakeDir {
    fn clear(&mut self) {
        self.clear_shake();
        self.clear_direction();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_ShakeDir {
    fn eq(&self, other: &CUserMsg_ShakeDir) -> bool {
        self.shake == other.shake &&
        self.direction == other.direction &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_ShakeDir {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_Tilt {
    // message fields
    command: ::std::option::Option<u32>,
    ease_in_out: ::std::option::Option<bool>,
    angle: ::protobuf::SingularPtrField<super::netmessages::CMsgVector>,
    duration: ::std::option::Option<f32>,
    time: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_Tilt {}

impl CUserMsg_Tilt {
    pub fn new() -> CUserMsg_Tilt {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_Tilt {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_Tilt> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_Tilt,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_Tilt {
                    command: ::std::option::Option::None,
                    ease_in_out: ::std::option::Option::None,
                    angle: ::protobuf::SingularPtrField::none(),
                    duration: ::std::option::Option::None,
                    time: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 command = 1;

    pub fn clear_command(&mut self) {
        self.command = ::std::option::Option::None;
    }

    pub fn has_command(&self) -> bool {
        self.command.is_some()
    }

    // Param is passed by value, moved
    pub fn set_command(&mut self, v: u32) {
        self.command = ::std::option::Option::Some(v);
    }

    pub fn get_command(&self) -> u32 {
        self.command.unwrap_or(0)
    }

    // optional bool ease_in_out = 2;

    pub fn clear_ease_in_out(&mut self) {
        self.ease_in_out = ::std::option::Option::None;
    }

    pub fn has_ease_in_out(&self) -> bool {
        self.ease_in_out.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ease_in_out(&mut self, v: bool) {
        self.ease_in_out = ::std::option::Option::Some(v);
    }

    pub fn get_ease_in_out(&self) -> bool {
        self.ease_in_out.unwrap_or(false)
    }

    // optional .CMsgVector angle = 3;

    pub fn clear_angle(&mut self) {
        self.angle.clear();
    }

    pub fn has_angle(&self) -> bool {
        self.angle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_angle(&mut self, v: super::netmessages::CMsgVector) {
        self.angle = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_angle(&mut self) -> &mut super::netmessages::CMsgVector {
        if self.angle.is_none() {
            self.angle.set_default();
        };
        self.angle.as_mut().unwrap()
    }

    // Take field
    pub fn take_angle(&mut self) -> super::netmessages::CMsgVector {
        self.angle.take().unwrap_or_else(|| super::netmessages::CMsgVector::new())
    }

    pub fn get_angle(&self) -> &super::netmessages::CMsgVector {
        self.angle.as_ref().unwrap_or_else(|| super::netmessages::CMsgVector::default_instance())
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

    // optional float time = 5;

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

impl ::protobuf::Message for CUserMsg_Tilt {
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
                    self.command = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.ease_in_out = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.angle));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.duration = ::std::option::Option::Some(tmp);
                },
                5 => {
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
        for value in &self.command {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.ease_in_out.is_some() {
            my_size += 2;
        };
        for value in &self.angle {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.duration.is_some() {
            my_size += 5;
        };
        if self.time.is_some() {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.command {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.ease_in_out {
            try!(os.write_bool(2, v));
        };
        if let Some(v) = self.angle.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.duration {
            try!(os.write_float(4, v));
        };
        if let Some(v) = self.time {
            try!(os.write_float(5, v));
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
        ::std::any::TypeId::of::<CUserMsg_Tilt>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_Tilt {
    fn new() -> CUserMsg_Tilt {
        CUserMsg_Tilt::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_Tilt>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "command",
                    CUserMsg_Tilt::has_command,
                    CUserMsg_Tilt::get_command,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "ease_in_out",
                    CUserMsg_Tilt::has_ease_in_out,
                    CUserMsg_Tilt::get_ease_in_out,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "angle",
                    CUserMsg_Tilt::has_angle,
                    CUserMsg_Tilt::get_angle,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "duration",
                    CUserMsg_Tilt::has_duration,
                    CUserMsg_Tilt::get_duration,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "time",
                    CUserMsg_Tilt::has_time,
                    CUserMsg_Tilt::get_time,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_Tilt>(
                    "CUserMsg_Tilt",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_Tilt {
    fn clear(&mut self) {
        self.clear_command();
        self.clear_ease_in_out();
        self.clear_angle();
        self.clear_duration();
        self.clear_time();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_Tilt {
    fn eq(&self, other: &CUserMsg_Tilt) -> bool {
        self.command == other.command &&
        self.ease_in_out == other.ease_in_out &&
        self.angle == other.angle &&
        self.duration == other.duration &&
        self.time == other.time &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_Tilt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_SayText {
    // message fields
    client: ::std::option::Option<u32>,
    text: ::protobuf::SingularField<::std::string::String>,
    chat: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_SayText {}

impl CUserMsg_SayText {
    pub fn new() -> CUserMsg_SayText {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_SayText {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_SayText> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_SayText,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_SayText {
                    client: ::std::option::Option::None,
                    text: ::protobuf::SingularField::none(),
                    chat: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 client = 1;

    pub fn clear_client(&mut self) {
        self.client = ::std::option::Option::None;
    }

    pub fn has_client(&self) -> bool {
        self.client.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client(&mut self, v: u32) {
        self.client = ::std::option::Option::Some(v);
    }

    pub fn get_client(&self) -> u32 {
        self.client.unwrap_or(0)
    }

    // optional string text = 2;

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

    // optional bool chat = 3;

    pub fn clear_chat(&mut self) {
        self.chat = ::std::option::Option::None;
    }

    pub fn has_chat(&self) -> bool {
        self.chat.is_some()
    }

    // Param is passed by value, moved
    pub fn set_chat(&mut self, v: bool) {
        self.chat = ::std::option::Option::Some(v);
    }

    pub fn get_chat(&self) -> bool {
        self.chat.unwrap_or(false)
    }
}

impl ::protobuf::Message for CUserMsg_SayText {
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
                    self.client = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.text));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.chat = ::std::option::Option::Some(tmp);
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
        for value in &self.text {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if self.chat.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.text.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.chat {
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
        ::std::any::TypeId::of::<CUserMsg_SayText>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_SayText {
    fn new() -> CUserMsg_SayText {
        CUserMsg_SayText::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_SayText>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "client",
                    CUserMsg_SayText::has_client,
                    CUserMsg_SayText::get_client,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "text",
                    CUserMsg_SayText::has_text,
                    CUserMsg_SayText::get_text,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "chat",
                    CUserMsg_SayText::has_chat,
                    CUserMsg_SayText::get_chat,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_SayText>(
                    "CUserMsg_SayText",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_SayText {
    fn clear(&mut self) {
        self.clear_client();
        self.clear_text();
        self.clear_chat();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_SayText {
    fn eq(&self, other: &CUserMsg_SayText) -> bool {
        self.client == other.client &&
        self.text == other.text &&
        self.chat == other.chat &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_SayText {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_SayText2 {
    // message fields
    client: ::std::option::Option<u32>,
    chat: ::std::option::Option<bool>,
    format: ::protobuf::SingularField<::std::string::String>,
    prefix: ::protobuf::SingularField<::std::string::String>,
    text: ::protobuf::SingularField<::std::string::String>,
    location: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_SayText2 {}

impl CUserMsg_SayText2 {
    pub fn new() -> CUserMsg_SayText2 {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_SayText2 {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_SayText2> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_SayText2,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_SayText2 {
                    client: ::std::option::Option::None,
                    chat: ::std::option::Option::None,
                    format: ::protobuf::SingularField::none(),
                    prefix: ::protobuf::SingularField::none(),
                    text: ::protobuf::SingularField::none(),
                    location: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 client = 1;

    pub fn clear_client(&mut self) {
        self.client = ::std::option::Option::None;
    }

    pub fn has_client(&self) -> bool {
        self.client.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client(&mut self, v: u32) {
        self.client = ::std::option::Option::Some(v);
    }

    pub fn get_client(&self) -> u32 {
        self.client.unwrap_or(0)
    }

    // optional bool chat = 2;

    pub fn clear_chat(&mut self) {
        self.chat = ::std::option::Option::None;
    }

    pub fn has_chat(&self) -> bool {
        self.chat.is_some()
    }

    // Param is passed by value, moved
    pub fn set_chat(&mut self, v: bool) {
        self.chat = ::std::option::Option::Some(v);
    }

    pub fn get_chat(&self) -> bool {
        self.chat.unwrap_or(false)
    }

    // optional string format = 3;

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

    // optional string prefix = 4;

    pub fn clear_prefix(&mut self) {
        self.prefix.clear();
    }

    pub fn has_prefix(&self) -> bool {
        self.prefix.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prefix(&mut self, v: ::std::string::String) {
        self.prefix = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_prefix(&mut self) -> &mut ::std::string::String {
        if self.prefix.is_none() {
            self.prefix.set_default();
        };
        self.prefix.as_mut().unwrap()
    }

    // Take field
    pub fn take_prefix(&mut self) -> ::std::string::String {
        self.prefix.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_prefix(&self) -> &str {
        match self.prefix.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string text = 5;

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

    // optional string location = 6;

    pub fn clear_location(&mut self) {
        self.location.clear();
    }

    pub fn has_location(&self) -> bool {
        self.location.is_some()
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: ::std::string::String) {
        self.location = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_location(&mut self) -> &mut ::std::string::String {
        if self.location.is_none() {
            self.location.set_default();
        };
        self.location.as_mut().unwrap()
    }

    // Take field
    pub fn take_location(&mut self) -> ::std::string::String {
        self.location.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_location(&self) -> &str {
        match self.location.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for CUserMsg_SayText2 {
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
                    self.client = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.chat = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.format));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.prefix));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.text));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.location));
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
        if self.chat.is_some() {
            my_size += 2;
        };
        for value in &self.format {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.prefix {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in &self.text {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        for value in &self.location {
            my_size += ::protobuf::rt::string_size(6, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.chat {
            try!(os.write_bool(2, v));
        };
        if let Some(v) = self.format.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.prefix.as_ref() {
            try!(os.write_string(4, &v));
        };
        if let Some(v) = self.text.as_ref() {
            try!(os.write_string(5, &v));
        };
        if let Some(v) = self.location.as_ref() {
            try!(os.write_string(6, &v));
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
        ::std::any::TypeId::of::<CUserMsg_SayText2>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_SayText2 {
    fn new() -> CUserMsg_SayText2 {
        CUserMsg_SayText2::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_SayText2>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "client",
                    CUserMsg_SayText2::has_client,
                    CUserMsg_SayText2::get_client,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "chat",
                    CUserMsg_SayText2::has_chat,
                    CUserMsg_SayText2::get_chat,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "format",
                    CUserMsg_SayText2::has_format,
                    CUserMsg_SayText2::get_format,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "prefix",
                    CUserMsg_SayText2::has_prefix,
                    CUserMsg_SayText2::get_prefix,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "text",
                    CUserMsg_SayText2::has_text,
                    CUserMsg_SayText2::get_text,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "location",
                    CUserMsg_SayText2::has_location,
                    CUserMsg_SayText2::get_location,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_SayText2>(
                    "CUserMsg_SayText2",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_SayText2 {
    fn clear(&mut self) {
        self.clear_client();
        self.clear_chat();
        self.clear_format();
        self.clear_prefix();
        self.clear_text();
        self.clear_location();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_SayText2 {
    fn eq(&self, other: &CUserMsg_SayText2) -> bool {
        self.client == other.client &&
        self.chat == other.chat &&
        self.format == other.format &&
        self.prefix == other.prefix &&
        self.text == other.text &&
        self.location == other.location &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_SayText2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_HudMsg {
    // message fields
    channel: ::std::option::Option<u32>,
    x: ::std::option::Option<f32>,
    y: ::std::option::Option<f32>,
    color1: ::std::option::Option<u32>,
    color2: ::std::option::Option<u32>,
    effect: ::std::option::Option<u32>,
    fade_in_time: ::std::option::Option<f32>,
    fade_out_time: ::std::option::Option<f32>,
    hold_time: ::std::option::Option<f32>,
    fx_time: ::std::option::Option<f32>,
    message: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_HudMsg {}

impl CUserMsg_HudMsg {
    pub fn new() -> CUserMsg_HudMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_HudMsg {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_HudMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_HudMsg,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_HudMsg {
                    channel: ::std::option::Option::None,
                    x: ::std::option::Option::None,
                    y: ::std::option::Option::None,
                    color1: ::std::option::Option::None,
                    color2: ::std::option::Option::None,
                    effect: ::std::option::Option::None,
                    fade_in_time: ::std::option::Option::None,
                    fade_out_time: ::std::option::Option::None,
                    hold_time: ::std::option::Option::None,
                    fx_time: ::std::option::Option::None,
                    message: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 channel = 1;

    pub fn clear_channel(&mut self) {
        self.channel = ::std::option::Option::None;
    }

    pub fn has_channel(&self) -> bool {
        self.channel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel(&mut self, v: u32) {
        self.channel = ::std::option::Option::Some(v);
    }

    pub fn get_channel(&self) -> u32 {
        self.channel.unwrap_or(0)
    }

    // optional float x = 2;

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

    // optional float y = 3;

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

    // optional uint32 color1 = 4;

    pub fn clear_color1(&mut self) {
        self.color1 = ::std::option::Option::None;
    }

    pub fn has_color1(&self) -> bool {
        self.color1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_color1(&mut self, v: u32) {
        self.color1 = ::std::option::Option::Some(v);
    }

    pub fn get_color1(&self) -> u32 {
        self.color1.unwrap_or(0)
    }

    // optional uint32 color2 = 5;

    pub fn clear_color2(&mut self) {
        self.color2 = ::std::option::Option::None;
    }

    pub fn has_color2(&self) -> bool {
        self.color2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_color2(&mut self, v: u32) {
        self.color2 = ::std::option::Option::Some(v);
    }

    pub fn get_color2(&self) -> u32 {
        self.color2.unwrap_or(0)
    }

    // optional uint32 effect = 6;

    pub fn clear_effect(&mut self) {
        self.effect = ::std::option::Option::None;
    }

    pub fn has_effect(&self) -> bool {
        self.effect.is_some()
    }

    // Param is passed by value, moved
    pub fn set_effect(&mut self, v: u32) {
        self.effect = ::std::option::Option::Some(v);
    }

    pub fn get_effect(&self) -> u32 {
        self.effect.unwrap_or(0)
    }

    // optional float fade_in_time = 7;

    pub fn clear_fade_in_time(&mut self) {
        self.fade_in_time = ::std::option::Option::None;
    }

    pub fn has_fade_in_time(&self) -> bool {
        self.fade_in_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fade_in_time(&mut self, v: f32) {
        self.fade_in_time = ::std::option::Option::Some(v);
    }

    pub fn get_fade_in_time(&self) -> f32 {
        self.fade_in_time.unwrap_or(0.)
    }

    // optional float fade_out_time = 8;

    pub fn clear_fade_out_time(&mut self) {
        self.fade_out_time = ::std::option::Option::None;
    }

    pub fn has_fade_out_time(&self) -> bool {
        self.fade_out_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fade_out_time(&mut self, v: f32) {
        self.fade_out_time = ::std::option::Option::Some(v);
    }

    pub fn get_fade_out_time(&self) -> f32 {
        self.fade_out_time.unwrap_or(0.)
    }

    // optional float hold_time = 9;

    pub fn clear_hold_time(&mut self) {
        self.hold_time = ::std::option::Option::None;
    }

    pub fn has_hold_time(&self) -> bool {
        self.hold_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hold_time(&mut self, v: f32) {
        self.hold_time = ::std::option::Option::Some(v);
    }

    pub fn get_hold_time(&self) -> f32 {
        self.hold_time.unwrap_or(0.)
    }

    // optional float fx_time = 10;

    pub fn clear_fx_time(&mut self) {
        self.fx_time = ::std::option::Option::None;
    }

    pub fn has_fx_time(&self) -> bool {
        self.fx_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fx_time(&mut self, v: f32) {
        self.fx_time = ::std::option::Option::Some(v);
    }

    pub fn get_fx_time(&self) -> f32 {
        self.fx_time.unwrap_or(0.)
    }

    // optional string message = 11;

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

impl ::protobuf::Message for CUserMsg_HudMsg {
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
                    self.channel = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.x = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.y = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.color1 = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.color2 = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.effect = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.fade_in_time = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.fade_out_time = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.hold_time = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.fx_time = ::std::option::Option::Some(tmp);
                },
                11 => {
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
        for value in &self.channel {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.x.is_some() {
            my_size += 5;
        };
        if self.y.is_some() {
            my_size += 5;
        };
        for value in &self.color1 {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.color2 {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.effect {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.fade_in_time.is_some() {
            my_size += 5;
        };
        if self.fade_out_time.is_some() {
            my_size += 5;
        };
        if self.hold_time.is_some() {
            my_size += 5;
        };
        if self.fx_time.is_some() {
            my_size += 5;
        };
        for value in &self.message {
            my_size += ::protobuf::rt::string_size(11, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.channel {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.x {
            try!(os.write_float(2, v));
        };
        if let Some(v) = self.y {
            try!(os.write_float(3, v));
        };
        if let Some(v) = self.color1 {
            try!(os.write_uint32(4, v));
        };
        if let Some(v) = self.color2 {
            try!(os.write_uint32(5, v));
        };
        if let Some(v) = self.effect {
            try!(os.write_uint32(6, v));
        };
        if let Some(v) = self.fade_in_time {
            try!(os.write_float(7, v));
        };
        if let Some(v) = self.fade_out_time {
            try!(os.write_float(8, v));
        };
        if let Some(v) = self.hold_time {
            try!(os.write_float(9, v));
        };
        if let Some(v) = self.fx_time {
            try!(os.write_float(10, v));
        };
        if let Some(v) = self.message.as_ref() {
            try!(os.write_string(11, &v));
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
        ::std::any::TypeId::of::<CUserMsg_HudMsg>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_HudMsg {
    fn new() -> CUserMsg_HudMsg {
        CUserMsg_HudMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_HudMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "channel",
                    CUserMsg_HudMsg::has_channel,
                    CUserMsg_HudMsg::get_channel,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "x",
                    CUserMsg_HudMsg::has_x,
                    CUserMsg_HudMsg::get_x,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "y",
                    CUserMsg_HudMsg::has_y,
                    CUserMsg_HudMsg::get_y,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "color1",
                    CUserMsg_HudMsg::has_color1,
                    CUserMsg_HudMsg::get_color1,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "color2",
                    CUserMsg_HudMsg::has_color2,
                    CUserMsg_HudMsg::get_color2,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "effect",
                    CUserMsg_HudMsg::has_effect,
                    CUserMsg_HudMsg::get_effect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "fade_in_time",
                    CUserMsg_HudMsg::has_fade_in_time,
                    CUserMsg_HudMsg::get_fade_in_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "fade_out_time",
                    CUserMsg_HudMsg::has_fade_out_time,
                    CUserMsg_HudMsg::get_fade_out_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "hold_time",
                    CUserMsg_HudMsg::has_hold_time,
                    CUserMsg_HudMsg::get_hold_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "fx_time",
                    CUserMsg_HudMsg::has_fx_time,
                    CUserMsg_HudMsg::get_fx_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "message",
                    CUserMsg_HudMsg::has_message,
                    CUserMsg_HudMsg::get_message,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_HudMsg>(
                    "CUserMsg_HudMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_HudMsg {
    fn clear(&mut self) {
        self.clear_channel();
        self.clear_x();
        self.clear_y();
        self.clear_color1();
        self.clear_color2();
        self.clear_effect();
        self.clear_fade_in_time();
        self.clear_fade_out_time();
        self.clear_hold_time();
        self.clear_fx_time();
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_HudMsg {
    fn eq(&self, other: &CUserMsg_HudMsg) -> bool {
        self.channel == other.channel &&
        self.x == other.x &&
        self.y == other.y &&
        self.color1 == other.color1 &&
        self.color2 == other.color2 &&
        self.effect == other.effect &&
        self.fade_in_time == other.fade_in_time &&
        self.fade_out_time == other.fade_out_time &&
        self.hold_time == other.hold_time &&
        self.fx_time == other.fx_time &&
        self.message == other.message &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_HudMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_HudText {
    // message fields
    message: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_HudText {}

impl CUserMsg_HudText {
    pub fn new() -> CUserMsg_HudText {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_HudText {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_HudText> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_HudText,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_HudText {
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

impl ::protobuf::Message for CUserMsg_HudText {
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
        ::std::any::TypeId::of::<CUserMsg_HudText>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_HudText {
    fn new() -> CUserMsg_HudText {
        CUserMsg_HudText::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_HudText>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "message",
                    CUserMsg_HudText::has_message,
                    CUserMsg_HudText::get_message,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_HudText>(
                    "CUserMsg_HudText",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_HudText {
    fn clear(&mut self) {
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_HudText {
    fn eq(&self, other: &CUserMsg_HudText) -> bool {
        self.message == other.message &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_HudText {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_TextMsg {
    // message fields
    dest: ::std::option::Option<u32>,
    param: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_TextMsg {}

impl CUserMsg_TextMsg {
    pub fn new() -> CUserMsg_TextMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_TextMsg {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_TextMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_TextMsg,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_TextMsg {
                    dest: ::std::option::Option::None,
                    param: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 dest = 1;

    pub fn clear_dest(&mut self) {
        self.dest = ::std::option::Option::None;
    }

    pub fn has_dest(&self) -> bool {
        self.dest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dest(&mut self, v: u32) {
        self.dest = ::std::option::Option::Some(v);
    }

    pub fn get_dest(&self) -> u32 {
        self.dest.unwrap_or(0)
    }

    // repeated string param = 2;

    pub fn clear_param(&mut self) {
        self.param.clear();
    }

    // Param is passed by value, moved
    pub fn set_param(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.param = v;
    }

    // Mutable pointer to the field.
    pub fn mut_param(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.param
    }

    // Take field
    pub fn take_param(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.param, ::protobuf::RepeatedField::new())
    }

    pub fn get_param(&self) -> &[::std::string::String] {
        &self.param
    }
}

impl ::protobuf::Message for CUserMsg_TextMsg {
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
                    self.dest = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.param));
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
        for value in &self.dest {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.param {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dest {
            try!(os.write_uint32(1, v));
        };
        for v in &self.param {
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
        ::std::any::TypeId::of::<CUserMsg_TextMsg>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_TextMsg {
    fn new() -> CUserMsg_TextMsg {
        CUserMsg_TextMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_TextMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "dest",
                    CUserMsg_TextMsg::has_dest,
                    CUserMsg_TextMsg::get_dest,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "param",
                    CUserMsg_TextMsg::get_param,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_TextMsg>(
                    "CUserMsg_TextMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_TextMsg {
    fn clear(&mut self) {
        self.clear_dest();
        self.clear_param();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_TextMsg {
    fn eq(&self, other: &CUserMsg_TextMsg) -> bool {
        self.dest == other.dest &&
        self.param == other.param &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_TextMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_GameTitle {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_GameTitle {}

impl CUserMsg_GameTitle {
    pub fn new() -> CUserMsg_GameTitle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_GameTitle {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_GameTitle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_GameTitle,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_GameTitle {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for CUserMsg_GameTitle {
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
        ::std::any::TypeId::of::<CUserMsg_GameTitle>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_GameTitle {
    fn new() -> CUserMsg_GameTitle {
        CUserMsg_GameTitle::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_GameTitle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_GameTitle>(
                    "CUserMsg_GameTitle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_GameTitle {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_GameTitle {
    fn eq(&self, other: &CUserMsg_GameTitle) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_GameTitle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_ResetHUD {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_ResetHUD {}

impl CUserMsg_ResetHUD {
    pub fn new() -> CUserMsg_ResetHUD {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_ResetHUD {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_ResetHUD> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_ResetHUD,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_ResetHUD {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for CUserMsg_ResetHUD {
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
        ::std::any::TypeId::of::<CUserMsg_ResetHUD>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_ResetHUD {
    fn new() -> CUserMsg_ResetHUD {
        CUserMsg_ResetHUD::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_ResetHUD>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_ResetHUD>(
                    "CUserMsg_ResetHUD",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_ResetHUD {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_ResetHUD {
    fn eq(&self, other: &CUserMsg_ResetHUD) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_ResetHUD {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_SendAudio {
    // message fields
    stop: ::std::option::Option<bool>,
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_SendAudio {}

impl CUserMsg_SendAudio {
    pub fn new() -> CUserMsg_SendAudio {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_SendAudio {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_SendAudio> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_SendAudio,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_SendAudio {
                    stop: ::std::option::Option::None,
                    name: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool stop = 2;

    pub fn clear_stop(&mut self) {
        self.stop = ::std::option::Option::None;
    }

    pub fn has_stop(&self) -> bool {
        self.stop.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stop(&mut self, v: bool) {
        self.stop = ::std::option::Option::Some(v);
    }

    pub fn get_stop(&self) -> bool {
        self.stop.unwrap_or(false)
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
}

impl ::protobuf::Message for CUserMsg_SendAudio {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.stop = ::std::option::Option::Some(tmp);
                },
                3 => {
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
        if self.stop.is_some() {
            my_size += 2;
        };
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.stop {
            try!(os.write_bool(2, v));
        };
        if let Some(v) = self.name.as_ref() {
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
        ::std::any::TypeId::of::<CUserMsg_SendAudio>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_SendAudio {
    fn new() -> CUserMsg_SendAudio {
        CUserMsg_SendAudio::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_SendAudio>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "stop",
                    CUserMsg_SendAudio::has_stop,
                    CUserMsg_SendAudio::get_stop,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    CUserMsg_SendAudio::has_name,
                    CUserMsg_SendAudio::get_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_SendAudio>(
                    "CUserMsg_SendAudio",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_SendAudio {
    fn clear(&mut self) {
        self.clear_stop();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_SendAudio {
    fn eq(&self, other: &CUserMsg_SendAudio) -> bool {
        self.stop == other.stop &&
        self.name == other.name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_SendAudio {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_VoiceMask {
    // message fields
    audible_players_mask: ::std::vec::Vec<i32>,
    player_mod_enabled: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_VoiceMask {}

impl CUserMsg_VoiceMask {
    pub fn new() -> CUserMsg_VoiceMask {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_VoiceMask {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_VoiceMask> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_VoiceMask,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_VoiceMask {
                    audible_players_mask: ::std::vec::Vec::new(),
                    player_mod_enabled: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated int32 audible_players_mask = 1;

    pub fn clear_audible_players_mask(&mut self) {
        self.audible_players_mask.clear();
    }

    // Param is passed by value, moved
    pub fn set_audible_players_mask(&mut self, v: ::std::vec::Vec<i32>) {
        self.audible_players_mask = v;
    }

    // Mutable pointer to the field.
    pub fn mut_audible_players_mask(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.audible_players_mask
    }

    // Take field
    pub fn take_audible_players_mask(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.audible_players_mask, ::std::vec::Vec::new())
    }

    pub fn get_audible_players_mask(&self) -> &[i32] {
        &self.audible_players_mask
    }

    // optional bool player_mod_enabled = 2;

    pub fn clear_player_mod_enabled(&mut self) {
        self.player_mod_enabled = ::std::option::Option::None;
    }

    pub fn has_player_mod_enabled(&self) -> bool {
        self.player_mod_enabled.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_mod_enabled(&mut self, v: bool) {
        self.player_mod_enabled = ::std::option::Option::Some(v);
    }

    pub fn get_player_mod_enabled(&self) -> bool {
        self.player_mod_enabled.unwrap_or(false)
    }
}

impl ::protobuf::Message for CUserMsg_VoiceMask {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.audible_players_mask));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.player_mod_enabled = ::std::option::Option::Some(tmp);
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
        for value in &self.audible_players_mask {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.player_mod_enabled.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.audible_players_mask {
            try!(os.write_int32(1, *v));
        };
        if let Some(v) = self.player_mod_enabled {
            try!(os.write_bool(2, v));
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
        ::std::any::TypeId::of::<CUserMsg_VoiceMask>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_VoiceMask {
    fn new() -> CUserMsg_VoiceMask {
        CUserMsg_VoiceMask::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_VoiceMask>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_i32_accessor(
                    "audible_players_mask",
                    CUserMsg_VoiceMask::get_audible_players_mask,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "player_mod_enabled",
                    CUserMsg_VoiceMask::has_player_mod_enabled,
                    CUserMsg_VoiceMask::get_player_mod_enabled,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_VoiceMask>(
                    "CUserMsg_VoiceMask",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_VoiceMask {
    fn clear(&mut self) {
        self.clear_audible_players_mask();
        self.clear_player_mod_enabled();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_VoiceMask {
    fn eq(&self, other: &CUserMsg_VoiceMask) -> bool {
        self.audible_players_mask == other.audible_players_mask &&
        self.player_mod_enabled == other.player_mod_enabled &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_VoiceMask {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_RequestState {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_RequestState {}

impl CUserMsg_RequestState {
    pub fn new() -> CUserMsg_RequestState {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_RequestState {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_RequestState> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_RequestState,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_RequestState {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for CUserMsg_RequestState {
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
        ::std::any::TypeId::of::<CUserMsg_RequestState>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_RequestState {
    fn new() -> CUserMsg_RequestState {
        CUserMsg_RequestState::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_RequestState>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_RequestState>(
                    "CUserMsg_RequestState",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_RequestState {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_RequestState {
    fn eq(&self, other: &CUserMsg_RequestState) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_RequestState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_HintText {
    // message fields
    message: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_HintText {}

impl CUserMsg_HintText {
    pub fn new() -> CUserMsg_HintText {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_HintText {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_HintText> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_HintText,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_HintText {
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

impl ::protobuf::Message for CUserMsg_HintText {
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
        ::std::any::TypeId::of::<CUserMsg_HintText>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_HintText {
    fn new() -> CUserMsg_HintText {
        CUserMsg_HintText::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_HintText>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "message",
                    CUserMsg_HintText::has_message,
                    CUserMsg_HintText::get_message,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_HintText>(
                    "CUserMsg_HintText",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_HintText {
    fn clear(&mut self) {
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_HintText {
    fn eq(&self, other: &CUserMsg_HintText) -> bool {
        self.message == other.message &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_HintText {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_KeyHintText {
    // message fields
    messages: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_KeyHintText {}

impl CUserMsg_KeyHintText {
    pub fn new() -> CUserMsg_KeyHintText {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_KeyHintText {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_KeyHintText> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_KeyHintText,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_KeyHintText {
                    messages: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated string messages = 1;

    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }

    // Param is passed by value, moved
    pub fn set_messages(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.messages = v;
    }

    // Mutable pointer to the field.
    pub fn mut_messages(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.messages
    }

    // Take field
    pub fn take_messages(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.messages, ::protobuf::RepeatedField::new())
    }

    pub fn get_messages(&self) -> &[::std::string::String] {
        &self.messages
    }
}

impl ::protobuf::Message for CUserMsg_KeyHintText {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.messages));
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
        for value in &self.messages {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.messages {
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
        ::std::any::TypeId::of::<CUserMsg_KeyHintText>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_KeyHintText {
    fn new() -> CUserMsg_KeyHintText {
        CUserMsg_KeyHintText::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_KeyHintText>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "messages",
                    CUserMsg_KeyHintText::get_messages,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_KeyHintText>(
                    "CUserMsg_KeyHintText",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_KeyHintText {
    fn clear(&mut self) {
        self.clear_messages();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_KeyHintText {
    fn eq(&self, other: &CUserMsg_KeyHintText) -> bool {
        self.messages == other.messages &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_KeyHintText {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_StatsCrawlMsg {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_StatsCrawlMsg {}

impl CUserMsg_StatsCrawlMsg {
    pub fn new() -> CUserMsg_StatsCrawlMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_StatsCrawlMsg {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_StatsCrawlMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_StatsCrawlMsg,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_StatsCrawlMsg {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for CUserMsg_StatsCrawlMsg {
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
        ::std::any::TypeId::of::<CUserMsg_StatsCrawlMsg>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_StatsCrawlMsg {
    fn new() -> CUserMsg_StatsCrawlMsg {
        CUserMsg_StatsCrawlMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_StatsCrawlMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_StatsCrawlMsg>(
                    "CUserMsg_StatsCrawlMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_StatsCrawlMsg {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_StatsCrawlMsg {
    fn eq(&self, other: &CUserMsg_StatsCrawlMsg) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_StatsCrawlMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_StatsSkipState {
    // message fields
    num_skips: ::std::option::Option<i32>,
    num_players: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_StatsSkipState {}

impl CUserMsg_StatsSkipState {
    pub fn new() -> CUserMsg_StatsSkipState {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_StatsSkipState {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_StatsSkipState> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_StatsSkipState,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_StatsSkipState {
                    num_skips: ::std::option::Option::None,
                    num_players: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 num_skips = 1;

    pub fn clear_num_skips(&mut self) {
        self.num_skips = ::std::option::Option::None;
    }

    pub fn has_num_skips(&self) -> bool {
        self.num_skips.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_skips(&mut self, v: i32) {
        self.num_skips = ::std::option::Option::Some(v);
    }

    pub fn get_num_skips(&self) -> i32 {
        self.num_skips.unwrap_or(0)
    }

    // optional int32 num_players = 2;

    pub fn clear_num_players(&mut self) {
        self.num_players = ::std::option::Option::None;
    }

    pub fn has_num_players(&self) -> bool {
        self.num_players.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_players(&mut self, v: i32) {
        self.num_players = ::std::option::Option::Some(v);
    }

    pub fn get_num_players(&self) -> i32 {
        self.num_players.unwrap_or(0)
    }
}

impl ::protobuf::Message for CUserMsg_StatsSkipState {
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
                    self.num_skips = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.num_players = ::std::option::Option::Some(tmp);
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
        for value in &self.num_skips {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.num_players {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.num_skips {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.num_players {
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
        ::std::any::TypeId::of::<CUserMsg_StatsSkipState>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_StatsSkipState {
    fn new() -> CUserMsg_StatsSkipState {
        CUserMsg_StatsSkipState::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_StatsSkipState>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "num_skips",
                    CUserMsg_StatsSkipState::has_num_skips,
                    CUserMsg_StatsSkipState::get_num_skips,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "num_players",
                    CUserMsg_StatsSkipState::has_num_players,
                    CUserMsg_StatsSkipState::get_num_players,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_StatsSkipState>(
                    "CUserMsg_StatsSkipState",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_StatsSkipState {
    fn clear(&mut self) {
        self.clear_num_skips();
        self.clear_num_players();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_StatsSkipState {
    fn eq(&self, other: &CUserMsg_StatsSkipState) -> bool {
        self.num_skips == other.num_skips &&
        self.num_players == other.num_players &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_StatsSkipState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_VoiceSubtitle {
    // message fields
    ent_index: ::std::option::Option<i32>,
    menu: ::std::option::Option<i32>,
    item: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_VoiceSubtitle {}

impl CUserMsg_VoiceSubtitle {
    pub fn new() -> CUserMsg_VoiceSubtitle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_VoiceSubtitle {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_VoiceSubtitle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_VoiceSubtitle,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_VoiceSubtitle {
                    ent_index: ::std::option::Option::None,
                    menu: ::std::option::Option::None,
                    item: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 ent_index = 1;

    pub fn clear_ent_index(&mut self) {
        self.ent_index = ::std::option::Option::None;
    }

    pub fn has_ent_index(&self) -> bool {
        self.ent_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ent_index(&mut self, v: i32) {
        self.ent_index = ::std::option::Option::Some(v);
    }

    pub fn get_ent_index(&self) -> i32 {
        self.ent_index.unwrap_or(0)
    }

    // optional int32 menu = 2;

    pub fn clear_menu(&mut self) {
        self.menu = ::std::option::Option::None;
    }

    pub fn has_menu(&self) -> bool {
        self.menu.is_some()
    }

    // Param is passed by value, moved
    pub fn set_menu(&mut self, v: i32) {
        self.menu = ::std::option::Option::Some(v);
    }

    pub fn get_menu(&self) -> i32 {
        self.menu.unwrap_or(0)
    }

    // optional int32 item = 3;

    pub fn clear_item(&mut self) {
        self.item = ::std::option::Option::None;
    }

    pub fn has_item(&self) -> bool {
        self.item.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item(&mut self, v: i32) {
        self.item = ::std::option::Option::Some(v);
    }

    pub fn get_item(&self) -> i32 {
        self.item.unwrap_or(0)
    }
}

impl ::protobuf::Message for CUserMsg_VoiceSubtitle {
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
                    self.ent_index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.menu = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.item = ::std::option::Option::Some(tmp);
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
        for value in &self.ent_index {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.menu {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.item {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ent_index {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.menu {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.item {
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
        ::std::any::TypeId::of::<CUserMsg_VoiceSubtitle>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_VoiceSubtitle {
    fn new() -> CUserMsg_VoiceSubtitle {
        CUserMsg_VoiceSubtitle::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_VoiceSubtitle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "ent_index",
                    CUserMsg_VoiceSubtitle::has_ent_index,
                    CUserMsg_VoiceSubtitle::get_ent_index,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "menu",
                    CUserMsg_VoiceSubtitle::has_menu,
                    CUserMsg_VoiceSubtitle::get_menu,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "item",
                    CUserMsg_VoiceSubtitle::has_item,
                    CUserMsg_VoiceSubtitle::get_item,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_VoiceSubtitle>(
                    "CUserMsg_VoiceSubtitle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_VoiceSubtitle {
    fn clear(&mut self) {
        self.clear_ent_index();
        self.clear_menu();
        self.clear_item();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_VoiceSubtitle {
    fn eq(&self, other: &CUserMsg_VoiceSubtitle) -> bool {
        self.ent_index == other.ent_index &&
        self.menu == other.menu &&
        self.item == other.item &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_VoiceSubtitle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_VGUIMenu {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    show: ::std::option::Option<bool>,
    keys: ::protobuf::RepeatedField<CUserMsg_VGUIMenu_Keys>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_VGUIMenu {}

impl CUserMsg_VGUIMenu {
    pub fn new() -> CUserMsg_VGUIMenu {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_VGUIMenu {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_VGUIMenu> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_VGUIMenu,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_VGUIMenu {
                    name: ::protobuf::SingularField::none(),
                    show: ::std::option::Option::None,
                    keys: ::protobuf::RepeatedField::new(),
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

    // optional bool show = 2;

    pub fn clear_show(&mut self) {
        self.show = ::std::option::Option::None;
    }

    pub fn has_show(&self) -> bool {
        self.show.is_some()
    }

    // Param is passed by value, moved
    pub fn set_show(&mut self, v: bool) {
        self.show = ::std::option::Option::Some(v);
    }

    pub fn get_show(&self) -> bool {
        self.show.unwrap_or(false)
    }

    // repeated .CUserMsg_VGUIMenu.Keys keys = 3;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<CUserMsg_VGUIMenu_Keys>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<CUserMsg_VGUIMenu_Keys> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<CUserMsg_VGUIMenu_Keys> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[CUserMsg_VGUIMenu_Keys] {
        &self.keys
    }
}

impl ::protobuf::Message for CUserMsg_VGUIMenu {
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
                    let tmp = try!(is.read_bool());
                    self.show = ::std::option::Option::Some(tmp);
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
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if self.show.is_some() {
            my_size += 2;
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
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.show {
            try!(os.write_bool(2, v));
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
        ::std::any::TypeId::of::<CUserMsg_VGUIMenu>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_VGUIMenu {
    fn new() -> CUserMsg_VGUIMenu {
        CUserMsg_VGUIMenu::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_VGUIMenu>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    CUserMsg_VGUIMenu::has_name,
                    CUserMsg_VGUIMenu::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "show",
                    CUserMsg_VGUIMenu::has_show,
                    CUserMsg_VGUIMenu::get_show,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "keys",
                    CUserMsg_VGUIMenu::get_keys,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_VGUIMenu>(
                    "CUserMsg_VGUIMenu",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_VGUIMenu {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_show();
        self.clear_keys();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_VGUIMenu {
    fn eq(&self, other: &CUserMsg_VGUIMenu) -> bool {
        self.name == other.name &&
        self.show == other.show &&
        self.keys == other.keys &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_VGUIMenu {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_VGUIMenu_Keys {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_VGUIMenu_Keys {}

impl CUserMsg_VGUIMenu_Keys {
    pub fn new() -> CUserMsg_VGUIMenu_Keys {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_VGUIMenu_Keys {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_VGUIMenu_Keys> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_VGUIMenu_Keys,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_VGUIMenu_Keys {
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

impl ::protobuf::Message for CUserMsg_VGUIMenu_Keys {
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
        ::std::any::TypeId::of::<CUserMsg_VGUIMenu_Keys>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_VGUIMenu_Keys {
    fn new() -> CUserMsg_VGUIMenu_Keys {
        CUserMsg_VGUIMenu_Keys::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_VGUIMenu_Keys>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    CUserMsg_VGUIMenu_Keys::has_name,
                    CUserMsg_VGUIMenu_Keys::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "value",
                    CUserMsg_VGUIMenu_Keys::has_value,
                    CUserMsg_VGUIMenu_Keys::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_VGUIMenu_Keys>(
                    "CUserMsg_VGUIMenu_Keys",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_VGUIMenu_Keys {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_VGUIMenu_Keys {
    fn eq(&self, other: &CUserMsg_VGUIMenu_Keys) -> bool {
        self.name == other.name &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_VGUIMenu_Keys {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_Geiger {
    // message fields
    range: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_Geiger {}

impl CUserMsg_Geiger {
    pub fn new() -> CUserMsg_Geiger {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_Geiger {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_Geiger> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_Geiger,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_Geiger {
                    range: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 range = 1;

    pub fn clear_range(&mut self) {
        self.range = ::std::option::Option::None;
    }

    pub fn has_range(&self) -> bool {
        self.range.is_some()
    }

    // Param is passed by value, moved
    pub fn set_range(&mut self, v: i32) {
        self.range = ::std::option::Option::Some(v);
    }

    pub fn get_range(&self) -> i32 {
        self.range.unwrap_or(0)
    }
}

impl ::protobuf::Message for CUserMsg_Geiger {
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
                    self.range = ::std::option::Option::Some(tmp);
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
        for value in &self.range {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.range {
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
        ::std::any::TypeId::of::<CUserMsg_Geiger>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_Geiger {
    fn new() -> CUserMsg_Geiger {
        CUserMsg_Geiger::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_Geiger>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "range",
                    CUserMsg_Geiger::has_range,
                    CUserMsg_Geiger::get_range,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_Geiger>(
                    "CUserMsg_Geiger",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_Geiger {
    fn clear(&mut self) {
        self.clear_range();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_Geiger {
    fn eq(&self, other: &CUserMsg_Geiger) -> bool {
        self.range == other.range &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_Geiger {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_Rumble {
    // message fields
    index: ::std::option::Option<i32>,
    data: ::std::option::Option<i32>,
    flags: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_Rumble {}

impl CUserMsg_Rumble {
    pub fn new() -> CUserMsg_Rumble {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_Rumble {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_Rumble> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_Rumble,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_Rumble {
                    index: ::std::option::Option::None,
                    data: ::std::option::Option::None,
                    flags: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 index = 1;

    pub fn clear_index(&mut self) {
        self.index = ::std::option::Option::None;
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: i32) {
        self.index = ::std::option::Option::Some(v);
    }

    pub fn get_index(&self) -> i32 {
        self.index.unwrap_or(0)
    }

    // optional int32 data = 2;

    pub fn clear_data(&mut self) {
        self.data = ::std::option::Option::None;
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: i32) {
        self.data = ::std::option::Option::Some(v);
    }

    pub fn get_data(&self) -> i32 {
        self.data.unwrap_or(0)
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
}

impl ::protobuf::Message for CUserMsg_Rumble {
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
                    self.index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.data = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.flags = ::std::option::Option::Some(tmp);
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
        for value in &self.data {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.flags {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.index {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.data {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.flags {
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
        ::std::any::TypeId::of::<CUserMsg_Rumble>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_Rumble {
    fn new() -> CUserMsg_Rumble {
        CUserMsg_Rumble::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_Rumble>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "index",
                    CUserMsg_Rumble::has_index,
                    CUserMsg_Rumble::get_index,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "data",
                    CUserMsg_Rumble::has_data,
                    CUserMsg_Rumble::get_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "flags",
                    CUserMsg_Rumble::has_flags,
                    CUserMsg_Rumble::get_flags,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_Rumble>(
                    "CUserMsg_Rumble",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_Rumble {
    fn clear(&mut self) {
        self.clear_index();
        self.clear_data();
        self.clear_flags();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_Rumble {
    fn eq(&self, other: &CUserMsg_Rumble) -> bool {
        self.index == other.index &&
        self.data == other.data &&
        self.flags == other.flags &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_Rumble {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_Train {
    // message fields
    train: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_Train {}

impl CUserMsg_Train {
    pub fn new() -> CUserMsg_Train {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_Train {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_Train> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_Train,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_Train {
                    train: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 train = 1;

    pub fn clear_train(&mut self) {
        self.train = ::std::option::Option::None;
    }

    pub fn has_train(&self) -> bool {
        self.train.is_some()
    }

    // Param is passed by value, moved
    pub fn set_train(&mut self, v: i32) {
        self.train = ::std::option::Option::Some(v);
    }

    pub fn get_train(&self) -> i32 {
        self.train.unwrap_or(0)
    }
}

impl ::protobuf::Message for CUserMsg_Train {
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
                    self.train = ::std::option::Option::Some(tmp);
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
        for value in &self.train {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.train {
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
        ::std::any::TypeId::of::<CUserMsg_Train>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_Train {
    fn new() -> CUserMsg_Train {
        CUserMsg_Train::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_Train>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "train",
                    CUserMsg_Train::has_train,
                    CUserMsg_Train::get_train,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_Train>(
                    "CUserMsg_Train",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_Train {
    fn clear(&mut self) {
        self.clear_train();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_Train {
    fn eq(&self, other: &CUserMsg_Train) -> bool {
        self.train == other.train &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_Train {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_SayTextChannel {
    // message fields
    player: ::std::option::Option<i32>,
    channel: ::std::option::Option<i32>,
    text: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_SayTextChannel {}

impl CUserMsg_SayTextChannel {
    pub fn new() -> CUserMsg_SayTextChannel {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_SayTextChannel {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_SayTextChannel> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_SayTextChannel,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_SayTextChannel {
                    player: ::std::option::Option::None,
                    channel: ::std::option::Option::None,
                    text: ::protobuf::SingularField::none(),
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

    // optional int32 channel = 2;

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

    // optional string text = 3;

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

impl ::protobuf::Message for CUserMsg_SayTextChannel {
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
                    self.channel = ::std::option::Option::Some(tmp);
                },
                3 => {
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
        for value in &self.player {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.channel {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.text {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.player {
            try!(os.write_int32(1, v));
        };
        if let Some(v) = self.channel {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.text.as_ref() {
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
        ::std::any::TypeId::of::<CUserMsg_SayTextChannel>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_SayTextChannel {
    fn new() -> CUserMsg_SayTextChannel {
        CUserMsg_SayTextChannel::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_SayTextChannel>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "player",
                    CUserMsg_SayTextChannel::has_player,
                    CUserMsg_SayTextChannel::get_player,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "channel",
                    CUserMsg_SayTextChannel::has_channel,
                    CUserMsg_SayTextChannel::get_channel,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "text",
                    CUserMsg_SayTextChannel::has_text,
                    CUserMsg_SayTextChannel::get_text,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_SayTextChannel>(
                    "CUserMsg_SayTextChannel",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_SayTextChannel {
    fn clear(&mut self) {
        self.clear_player();
        self.clear_channel();
        self.clear_text();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_SayTextChannel {
    fn eq(&self, other: &CUserMsg_SayTextChannel) -> bool {
        self.player == other.player &&
        self.channel == other.channel &&
        self.text == other.text &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_SayTextChannel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CUserMsg_MessageText {
    // message fields
    color: ::std::option::Option<u32>,
    text: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_MessageText {}

impl CUserMsg_MessageText {
    pub fn new() -> CUserMsg_MessageText {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_MessageText {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_MessageText> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_MessageText,
        };
        unsafe {
            instance.get(|| {
                CUserMsg_MessageText {
                    color: ::std::option::Option::None,
                    text: ::protobuf::SingularField::none(),
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

    // optional string text = 2;

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

impl ::protobuf::Message for CUserMsg_MessageText {
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
        for value in &self.color {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.text {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.color {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.text.as_ref() {
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
        ::std::any::TypeId::of::<CUserMsg_MessageText>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMsg_MessageText {
    fn new() -> CUserMsg_MessageText {
        CUserMsg_MessageText::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_MessageText>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "color",
                    CUserMsg_MessageText::has_color,
                    CUserMsg_MessageText::get_color,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "text",
                    CUserMsg_MessageText::has_text,
                    CUserMsg_MessageText::get_text,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_MessageText>(
                    "CUserMsg_MessageText",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_MessageText {
    fn clear(&mut self) {
        self.clear_color();
        self.clear_text();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CUserMsg_MessageText {
    fn eq(&self, other: &CUserMsg_MessageText) -> bool {
        self.color == other.color &&
        self.text == other.text &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CUserMsg_MessageText {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EBaseUserMessages {
    UM_AchievementEvent = 1,
    UM_CloseCaption = 2,
    UM_CloseCaptionDirect = 3,
    UM_CurrentTimescale = 4,
    UM_DesiredTimescale = 5,
    UM_Fade = 6,
    UM_GameTitle = 7,
    UM_Geiger = 8,
    UM_HintText = 9,
    UM_HudMsg = 10,
    UM_HudText = 11,
    UM_KeyHintText = 12,
    UM_MessageText = 13,
    UM_RequestState = 14,
    UM_ResetHUD = 15,
    UM_Rumble = 16,
    UM_SayText = 17,
    UM_SayText2 = 18,
    UM_SayTextChannel = 19,
    UM_Shake = 20,
    UM_ShakeDir = 21,
    UM_StatsCrawlMsg = 22,
    UM_StatsSkipState = 23,
    UM_TextMsg = 24,
    UM_Tilt = 25,
    UM_Train = 26,
    UM_VGUIMenu = 27,
    UM_VoiceMask = 28,
    UM_VoiceSubtitle = 29,
    UM_SendAudio = 30,
    UM_MAX_BASE = 63,
}

impl ::protobuf::ProtobufEnum for EBaseUserMessages {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EBaseUserMessages> {
        match value {
            1 => ::std::option::Option::Some(EBaseUserMessages::UM_AchievementEvent),
            2 => ::std::option::Option::Some(EBaseUserMessages::UM_CloseCaption),
            3 => ::std::option::Option::Some(EBaseUserMessages::UM_CloseCaptionDirect),
            4 => ::std::option::Option::Some(EBaseUserMessages::UM_CurrentTimescale),
            5 => ::std::option::Option::Some(EBaseUserMessages::UM_DesiredTimescale),
            6 => ::std::option::Option::Some(EBaseUserMessages::UM_Fade),
            7 => ::std::option::Option::Some(EBaseUserMessages::UM_GameTitle),
            8 => ::std::option::Option::Some(EBaseUserMessages::UM_Geiger),
            9 => ::std::option::Option::Some(EBaseUserMessages::UM_HintText),
            10 => ::std::option::Option::Some(EBaseUserMessages::UM_HudMsg),
            11 => ::std::option::Option::Some(EBaseUserMessages::UM_HudText),
            12 => ::std::option::Option::Some(EBaseUserMessages::UM_KeyHintText),
            13 => ::std::option::Option::Some(EBaseUserMessages::UM_MessageText),
            14 => ::std::option::Option::Some(EBaseUserMessages::UM_RequestState),
            15 => ::std::option::Option::Some(EBaseUserMessages::UM_ResetHUD),
            16 => ::std::option::Option::Some(EBaseUserMessages::UM_Rumble),
            17 => ::std::option::Option::Some(EBaseUserMessages::UM_SayText),
            18 => ::std::option::Option::Some(EBaseUserMessages::UM_SayText2),
            19 => ::std::option::Option::Some(EBaseUserMessages::UM_SayTextChannel),
            20 => ::std::option::Option::Some(EBaseUserMessages::UM_Shake),
            21 => ::std::option::Option::Some(EBaseUserMessages::UM_ShakeDir),
            22 => ::std::option::Option::Some(EBaseUserMessages::UM_StatsCrawlMsg),
            23 => ::std::option::Option::Some(EBaseUserMessages::UM_StatsSkipState),
            24 => ::std::option::Option::Some(EBaseUserMessages::UM_TextMsg),
            25 => ::std::option::Option::Some(EBaseUserMessages::UM_Tilt),
            26 => ::std::option::Option::Some(EBaseUserMessages::UM_Train),
            27 => ::std::option::Option::Some(EBaseUserMessages::UM_VGUIMenu),
            28 => ::std::option::Option::Some(EBaseUserMessages::UM_VoiceMask),
            29 => ::std::option::Option::Some(EBaseUserMessages::UM_VoiceSubtitle),
            30 => ::std::option::Option::Some(EBaseUserMessages::UM_SendAudio),
            63 => ::std::option::Option::Some(EBaseUserMessages::UM_MAX_BASE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EBaseUserMessages] = &[
            EBaseUserMessages::UM_AchievementEvent,
            EBaseUserMessages::UM_CloseCaption,
            EBaseUserMessages::UM_CloseCaptionDirect,
            EBaseUserMessages::UM_CurrentTimescale,
            EBaseUserMessages::UM_DesiredTimescale,
            EBaseUserMessages::UM_Fade,
            EBaseUserMessages::UM_GameTitle,
            EBaseUserMessages::UM_Geiger,
            EBaseUserMessages::UM_HintText,
            EBaseUserMessages::UM_HudMsg,
            EBaseUserMessages::UM_HudText,
            EBaseUserMessages::UM_KeyHintText,
            EBaseUserMessages::UM_MessageText,
            EBaseUserMessages::UM_RequestState,
            EBaseUserMessages::UM_ResetHUD,
            EBaseUserMessages::UM_Rumble,
            EBaseUserMessages::UM_SayText,
            EBaseUserMessages::UM_SayText2,
            EBaseUserMessages::UM_SayTextChannel,
            EBaseUserMessages::UM_Shake,
            EBaseUserMessages::UM_ShakeDir,
            EBaseUserMessages::UM_StatsCrawlMsg,
            EBaseUserMessages::UM_StatsSkipState,
            EBaseUserMessages::UM_TextMsg,
            EBaseUserMessages::UM_Tilt,
            EBaseUserMessages::UM_Train,
            EBaseUserMessages::UM_VGUIMenu,
            EBaseUserMessages::UM_VoiceMask,
            EBaseUserMessages::UM_VoiceSubtitle,
            EBaseUserMessages::UM_SendAudio,
            EBaseUserMessages::UM_MAX_BASE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<EBaseUserMessages>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EBaseUserMessages", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EBaseUserMessages {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x12, 0x75, 0x73, 0x65, 0x72, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x20, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x11, 0x6e, 0x65, 0x74, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x30, 0x0a, 0x19, 0x43, 0x55, 0x73,
    0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x41, 0x63, 0x68, 0x69, 0x65, 0x76, 0x65, 0x6d, 0x65, 0x6e,
    0x74, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x13, 0x0a, 0x0b, 0x61, 0x63, 0x68, 0x69, 0x65, 0x76,
    0x65, 0x6d, 0x65, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x22, 0x4c, 0x0a, 0x15, 0x43,
    0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x6c, 0x6f, 0x73, 0x65, 0x43, 0x61, 0x70,
    0x74, 0x69, 0x6f, 0x6e, 0x12, 0x0c, 0x0a, 0x04, 0x68, 0x61, 0x73, 0x68, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x07, 0x12, 0x10, 0x0a, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x02, 0x12, 0x13, 0x0a, 0x0b, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x70, 0x6c, 0x61,
    0x79, 0x65, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x22, 0x2c, 0x0a, 0x19, 0x43, 0x55, 0x73,
    0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x54, 0x69, 0x6d,
    0x65, 0x73, 0x63, 0x61, 0x6c, 0x65, 0x12, 0x0f, 0x0a, 0x07, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e,
    0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x02, 0x22, 0x6e, 0x0a, 0x19, 0x43, 0x55, 0x73, 0x65, 0x72,
    0x4d, 0x73, 0x67, 0x5f, 0x44, 0x65, 0x73, 0x69, 0x72, 0x65, 0x64, 0x54, 0x69, 0x6d, 0x65, 0x73,
    0x63, 0x61, 0x6c, 0x65, 0x12, 0x0f, 0x0a, 0x07, 0x64, 0x65, 0x73, 0x69, 0x72, 0x65, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x02, 0x12, 0x10, 0x0a, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x12, 0x14, 0x0a, 0x0c, 0x69, 0x6e, 0x74, 0x65, 0x72,
    0x70, 0x6f, 0x6c, 0x61, 0x74, 0x6f, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x18, 0x0a,
    0x10, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x62, 0x6c, 0x65, 0x6e, 0x64, 0x5f, 0x74, 0x69, 0x6d,
    0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x02, 0x22, 0x52, 0x0a, 0x0d, 0x43, 0x55, 0x73, 0x65, 0x72,
    0x4d, 0x73, 0x67, 0x5f, 0x46, 0x61, 0x64, 0x65, 0x12, 0x10, 0x0a, 0x08, 0x64, 0x75, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x11, 0x0a, 0x09, 0x68, 0x6f,
    0x6c, 0x64, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0d, 0x0a,
    0x05, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0d, 0x0a, 0x05,
    0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x07, 0x22, 0x59, 0x0a, 0x0e, 0x43,
    0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x68, 0x61, 0x6b, 0x65, 0x12, 0x0f, 0x0a,
    0x07, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x11,
    0x0a, 0x09, 0x61, 0x6d, 0x70, 0x6c, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x02, 0x12, 0x11, 0x0a, 0x09, 0x66, 0x72, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x79, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x02, 0x12, 0x10, 0x0a, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x02, 0x22, 0x53, 0x0a, 0x11, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d,
    0x73, 0x67, 0x5f, 0x53, 0x68, 0x61, 0x6b, 0x65, 0x44, 0x69, 0x72, 0x12, 0x1e, 0x0a, 0x05, 0x73,
    0x68, 0x61, 0x6b, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x43, 0x55, 0x73,
    0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x68, 0x61, 0x6b, 0x65, 0x12, 0x1e, 0x0a, 0x09, 0x64,
    0x69, 0x72, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b,
    0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x22, 0x71, 0x0a, 0x0d, 0x43,
    0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x54, 0x69, 0x6c, 0x74, 0x12, 0x0f, 0x0a, 0x07,
    0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x13, 0x0a,
    0x0b, 0x65, 0x61, 0x73, 0x65, 0x5f, 0x69, 0x6e, 0x5f, 0x6f, 0x75, 0x74, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x08, 0x12, 0x1a, 0x0a, 0x05, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x0b, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x12, 0x10,
    0x0a, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x04, 0x20, 0x01, 0x28, 0x02,
    0x12, 0x0c, 0x0a, 0x04, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x02, 0x22, 0x3e,
    0x0a, 0x10, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x61, 0x79, 0x54, 0x65,
    0x78, 0x74, 0x12, 0x0e, 0x0a, 0x06, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0d, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x65, 0x78, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09,
    0x12, 0x0c, 0x0a, 0x04, 0x63, 0x68, 0x61, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x22, 0x71,
    0x0a, 0x11, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x61, 0x79, 0x54, 0x65,
    0x78, 0x74, 0x32, 0x12, 0x0e, 0x0a, 0x06, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0d, 0x12, 0x0c, 0x0a, 0x04, 0x63, 0x68, 0x61, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x08, 0x12, 0x0e, 0x0a, 0x06, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x09, 0x12, 0x0e, 0x0a, 0x06, 0x70, 0x72, 0x65, 0x66, 0x69, 0x78, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x09, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x65, 0x78, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x12,
    0x10, 0x0a, 0x08, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x06, 0x20, 0x01, 0x28,
    0x09, 0x22, 0xca, 0x01, 0x0a, 0x0f, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x48,
    0x75, 0x64, 0x4d, 0x73, 0x67, 0x12, 0x0f, 0x0a, 0x07, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x09, 0x0a, 0x01, 0x78, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x02, 0x12, 0x09, 0x0a, 0x01, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x02, 0x12, 0x0e, 0x0a, 0x06,
    0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x31, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0e, 0x0a, 0x06,
    0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x32, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0e, 0x0a, 0x06,
    0x65, 0x66, 0x66, 0x65, 0x63, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x14, 0x0a, 0x0c,
    0x66, 0x61, 0x64, 0x65, 0x5f, 0x69, 0x6e, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x07, 0x20, 0x01,
    0x28, 0x02, 0x12, 0x15, 0x0a, 0x0d, 0x66, 0x61, 0x64, 0x65, 0x5f, 0x6f, 0x75, 0x74, 0x5f, 0x74,
    0x69, 0x6d, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x02, 0x12, 0x11, 0x0a, 0x09, 0x68, 0x6f, 0x6c,
    0x64, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x09, 0x20, 0x01, 0x28, 0x02, 0x12, 0x0f, 0x0a, 0x07,
    0x66, 0x78, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x02, 0x12, 0x0f, 0x0a,
    0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x09, 0x22, 0x23,
    0x0a, 0x10, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x48, 0x75, 0x64, 0x54, 0x65,
    0x78, 0x74, 0x12, 0x0f, 0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x09, 0x22, 0x2f, 0x0a, 0x10, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f,
    0x54, 0x65, 0x78, 0x74, 0x4d, 0x73, 0x67, 0x12, 0x0c, 0x0a, 0x04, 0x64, 0x65, 0x73, 0x74, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0d, 0x0a, 0x05, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x18, 0x02,
    0x20, 0x03, 0x28, 0x09, 0x22, 0x14, 0x0a, 0x12, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67,
    0x5f, 0x47, 0x61, 0x6d, 0x65, 0x54, 0x69, 0x74, 0x6c, 0x65, 0x22, 0x13, 0x0a, 0x11, 0x43, 0x55,
    0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x52, 0x65, 0x73, 0x65, 0x74, 0x48, 0x55, 0x44, 0x22,
    0x30, 0x0a, 0x12, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x65, 0x6e, 0x64,
    0x41, 0x75, 0x64, 0x69, 0x6f, 0x12, 0x0c, 0x0a, 0x04, 0x73, 0x74, 0x6f, 0x70, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x08, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x09, 0x22, 0x4e, 0x0a, 0x12, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x56, 0x6f,
    0x69, 0x63, 0x65, 0x4d, 0x61, 0x73, 0x6b, 0x12, 0x1c, 0x0a, 0x14, 0x61, 0x75, 0x64, 0x69, 0x62,
    0x6c, 0x65, 0x5f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x73, 0x5f, 0x6d, 0x61, 0x73, 0x6b, 0x18,
    0x01, 0x20, 0x03, 0x28, 0x05, 0x12, 0x1a, 0x0a, 0x12, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f,
    0x6d, 0x6f, 0x64, 0x5f, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x08, 0x22, 0x17, 0x0a, 0x15, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x53, 0x74, 0x61, 0x74, 0x65, 0x22, 0x24, 0x0a, 0x11, 0x43, 0x55,
    0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x48, 0x69, 0x6e, 0x74, 0x54, 0x65, 0x78, 0x74, 0x12,
    0x0f, 0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x22, 0x28, 0x0a, 0x14, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x4b, 0x65, 0x79,
    0x48, 0x69, 0x6e, 0x74, 0x54, 0x65, 0x78, 0x74, 0x12, 0x10, 0x0a, 0x08, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x09, 0x22, 0x18, 0x0a, 0x16, 0x43, 0x55,
    0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x74, 0x61, 0x74, 0x73, 0x43, 0x72, 0x61, 0x77,
    0x6c, 0x4d, 0x73, 0x67, 0x22, 0x41, 0x0a, 0x17, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67,
    0x5f, 0x53, 0x74, 0x61, 0x74, 0x73, 0x53, 0x6b, 0x69, 0x70, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12,
    0x11, 0x0a, 0x09, 0x6e, 0x75, 0x6d, 0x5f, 0x73, 0x6b, 0x69, 0x70, 0x73, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x05, 0x12, 0x13, 0x0a, 0x0b, 0x6e, 0x75, 0x6d, 0x5f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72,
    0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x22, 0x47, 0x0a, 0x16, 0x43, 0x55, 0x73, 0x65, 0x72,
    0x4d, 0x73, 0x67, 0x5f, 0x56, 0x6f, 0x69, 0x63, 0x65, 0x53, 0x75, 0x62, 0x74, 0x69, 0x74, 0x6c,
    0x65, 0x12, 0x11, 0x0a, 0x09, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x05, 0x12, 0x0c, 0x0a, 0x04, 0x6d, 0x65, 0x6e, 0x75, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x05, 0x12, 0x0c, 0x0a, 0x04, 0x69, 0x74, 0x65, 0x6d, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05,
    0x22, 0x7b, 0x0a, 0x11, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x56, 0x47, 0x55,
    0x49, 0x4d, 0x65, 0x6e, 0x75, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x09, 0x12, 0x0c, 0x0a, 0x04, 0x73, 0x68, 0x6f, 0x77, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x08, 0x12, 0x25, 0x0a, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x17, 0x2e, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x56, 0x47, 0x55, 0x49, 0x4d,
    0x65, 0x6e, 0x75, 0x2e, 0x4b, 0x65, 0x79, 0x73, 0x1a, 0x23, 0x0a, 0x04, 0x4b, 0x65, 0x79, 0x73,
    0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x12, 0x0d,
    0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x22, 0x20, 0x0a,
    0x0f, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x47, 0x65, 0x69, 0x67, 0x65, 0x72,
    0x12, 0x0d, 0x0a, 0x05, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x22,
    0x3d, 0x0a, 0x0f, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x52, 0x75, 0x6d, 0x62,
    0x6c, 0x65, 0x12, 0x0d, 0x0a, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x05, 0x12, 0x0c, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12,
    0x0d, 0x0a, 0x05, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x22, 0x1f,
    0x0a, 0x0e, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x54, 0x72, 0x61, 0x69, 0x6e,
    0x12, 0x0d, 0x0a, 0x05, 0x74, 0x72, 0x61, 0x69, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x22,
    0x48, 0x0a, 0x17, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x61, 0x79, 0x54,
    0x65, 0x78, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x12, 0x0e, 0x0a, 0x06, 0x70, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0f, 0x0a, 0x07, 0x63, 0x68,
    0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0c, 0x0a, 0x04, 0x74,
    0x65, 0x78, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x22, 0x33, 0x0a, 0x14, 0x43, 0x55, 0x73,
    0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x65, 0x78,
    0x74, 0x12, 0x0d, 0x0a, 0x05, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d,
    0x12, 0x0c, 0x0a, 0x04, 0x74, 0x65, 0x78, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x2a, 0xd4,
    0x04, 0x0a, 0x11, 0x45, 0x42, 0x61, 0x73, 0x65, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x73, 0x12, 0x17, 0x0a, 0x13, 0x55, 0x4d, 0x5f, 0x41, 0x63, 0x68, 0x69, 0x65,
    0x76, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x10, 0x01, 0x12, 0x13, 0x0a,
    0x0f, 0x55, 0x4d, 0x5f, 0x43, 0x6c, 0x6f, 0x73, 0x65, 0x43, 0x61, 0x70, 0x74, 0x69, 0x6f, 0x6e,
    0x10, 0x02, 0x12, 0x19, 0x0a, 0x15, 0x55, 0x4d, 0x5f, 0x43, 0x6c, 0x6f, 0x73, 0x65, 0x43, 0x61,
    0x70, 0x74, 0x69, 0x6f, 0x6e, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74, 0x10, 0x03, 0x12, 0x17, 0x0a,
    0x13, 0x55, 0x4d, 0x5f, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x73,
    0x63, 0x61, 0x6c, 0x65, 0x10, 0x04, 0x12, 0x17, 0x0a, 0x13, 0x55, 0x4d, 0x5f, 0x44, 0x65, 0x73,
    0x69, 0x72, 0x65, 0x64, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x63, 0x61, 0x6c, 0x65, 0x10, 0x05, 0x12,
    0x0b, 0x0a, 0x07, 0x55, 0x4d, 0x5f, 0x46, 0x61, 0x64, 0x65, 0x10, 0x06, 0x12, 0x10, 0x0a, 0x0c,
    0x55, 0x4d, 0x5f, 0x47, 0x61, 0x6d, 0x65, 0x54, 0x69, 0x74, 0x6c, 0x65, 0x10, 0x07, 0x12, 0x0d,
    0x0a, 0x09, 0x55, 0x4d, 0x5f, 0x47, 0x65, 0x69, 0x67, 0x65, 0x72, 0x10, 0x08, 0x12, 0x0f, 0x0a,
    0x0b, 0x55, 0x4d, 0x5f, 0x48, 0x69, 0x6e, 0x74, 0x54, 0x65, 0x78, 0x74, 0x10, 0x09, 0x12, 0x0d,
    0x0a, 0x09, 0x55, 0x4d, 0x5f, 0x48, 0x75, 0x64, 0x4d, 0x73, 0x67, 0x10, 0x0a, 0x12, 0x0e, 0x0a,
    0x0a, 0x55, 0x4d, 0x5f, 0x48, 0x75, 0x64, 0x54, 0x65, 0x78, 0x74, 0x10, 0x0b, 0x12, 0x12, 0x0a,
    0x0e, 0x55, 0x4d, 0x5f, 0x4b, 0x65, 0x79, 0x48, 0x69, 0x6e, 0x74, 0x54, 0x65, 0x78, 0x74, 0x10,
    0x0c, 0x12, 0x12, 0x0a, 0x0e, 0x55, 0x4d, 0x5f, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54,
    0x65, 0x78, 0x74, 0x10, 0x0d, 0x12, 0x13, 0x0a, 0x0f, 0x55, 0x4d, 0x5f, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x53, 0x74, 0x61, 0x74, 0x65, 0x10, 0x0e, 0x12, 0x0f, 0x0a, 0x0b, 0x55, 0x4d,
    0x5f, 0x52, 0x65, 0x73, 0x65, 0x74, 0x48, 0x55, 0x44, 0x10, 0x0f, 0x12, 0x0d, 0x0a, 0x09, 0x55,
    0x4d, 0x5f, 0x52, 0x75, 0x6d, 0x62, 0x6c, 0x65, 0x10, 0x10, 0x12, 0x0e, 0x0a, 0x0a, 0x55, 0x4d,
    0x5f, 0x53, 0x61, 0x79, 0x54, 0x65, 0x78, 0x74, 0x10, 0x11, 0x12, 0x0f, 0x0a, 0x0b, 0x55, 0x4d,
    0x5f, 0x53, 0x61, 0x79, 0x54, 0x65, 0x78, 0x74, 0x32, 0x10, 0x12, 0x12, 0x15, 0x0a, 0x11, 0x55,
    0x4d, 0x5f, 0x53, 0x61, 0x79, 0x54, 0x65, 0x78, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c,
    0x10, 0x13, 0x12, 0x0c, 0x0a, 0x08, 0x55, 0x4d, 0x5f, 0x53, 0x68, 0x61, 0x6b, 0x65, 0x10, 0x14,
    0x12, 0x0f, 0x0a, 0x0b, 0x55, 0x4d, 0x5f, 0x53, 0x68, 0x61, 0x6b, 0x65, 0x44, 0x69, 0x72, 0x10,
    0x15, 0x12, 0x14, 0x0a, 0x10, 0x55, 0x4d, 0x5f, 0x53, 0x74, 0x61, 0x74, 0x73, 0x43, 0x72, 0x61,
    0x77, 0x6c, 0x4d, 0x73, 0x67, 0x10, 0x16, 0x12, 0x15, 0x0a, 0x11, 0x55, 0x4d, 0x5f, 0x53, 0x74,
    0x61, 0x74, 0x73, 0x53, 0x6b, 0x69, 0x70, 0x53, 0x74, 0x61, 0x74, 0x65, 0x10, 0x17, 0x12, 0x0e,
    0x0a, 0x0a, 0x55, 0x4d, 0x5f, 0x54, 0x65, 0x78, 0x74, 0x4d, 0x73, 0x67, 0x10, 0x18, 0x12, 0x0b,
    0x0a, 0x07, 0x55, 0x4d, 0x5f, 0x54, 0x69, 0x6c, 0x74, 0x10, 0x19, 0x12, 0x0c, 0x0a, 0x08, 0x55,
    0x4d, 0x5f, 0x54, 0x72, 0x61, 0x69, 0x6e, 0x10, 0x1a, 0x12, 0x0f, 0x0a, 0x0b, 0x55, 0x4d, 0x5f,
    0x56, 0x47, 0x55, 0x49, 0x4d, 0x65, 0x6e, 0x75, 0x10, 0x1b, 0x12, 0x10, 0x0a, 0x0c, 0x55, 0x4d,
    0x5f, 0x56, 0x6f, 0x69, 0x63, 0x65, 0x4d, 0x61, 0x73, 0x6b, 0x10, 0x1c, 0x12, 0x14, 0x0a, 0x10,
    0x55, 0x4d, 0x5f, 0x56, 0x6f, 0x69, 0x63, 0x65, 0x53, 0x75, 0x62, 0x74, 0x69, 0x74, 0x6c, 0x65,
    0x10, 0x1d, 0x12, 0x10, 0x0a, 0x0c, 0x55, 0x4d, 0x5f, 0x53, 0x65, 0x6e, 0x64, 0x41, 0x75, 0x64,
    0x69, 0x6f, 0x10, 0x1e, 0x12, 0x0f, 0x0a, 0x0b, 0x55, 0x4d, 0x5f, 0x4d, 0x41, 0x58, 0x5f, 0x42,
    0x41, 0x53, 0x45, 0x10, 0x3f, 0x42, 0x05, 0x48, 0x01, 0x80, 0x01, 0x00,
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
