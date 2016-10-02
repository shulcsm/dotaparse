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
pub struct CDOTAModifierBuffTableEntry {
    // message fields
    entry_type: ::std::option::Option<DOTA_MODIFIER_ENTRY_TYPE>,
    parent: ::std::option::Option<i32>,
    index: ::std::option::Option<i32>,
    serial_num: ::std::option::Option<i32>,
    name: ::std::option::Option<i32>,
    ability_level: ::std::option::Option<i32>,
    stack_count: ::std::option::Option<i32>,
    creation_time: ::std::option::Option<f32>,
    duration: ::std::option::Option<f32>,
    caster: ::std::option::Option<i32>,
    ability: ::std::option::Option<i32>,
    armor: ::std::option::Option<i32>,
    fade_time: ::std::option::Option<f32>,
    subtle: ::std::option::Option<bool>,
    channel_time: ::std::option::Option<f32>,
    v_start: ::protobuf::SingularPtrField<super::netmessages::CMsgVector>,
    v_end: ::protobuf::SingularPtrField<super::netmessages::CMsgVector>,
    portal_loop_appear: ::protobuf::SingularField<::std::string::String>,
    portal_loop_disappear: ::protobuf::SingularField<::std::string::String>,
    hero_loop_appear: ::protobuf::SingularField<::std::string::String>,
    hero_loop_disappear: ::protobuf::SingularField<::std::string::String>,
    movement_speed: ::std::option::Option<i32>,
    aura: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAModifierBuffTableEntry {}

impl CDOTAModifierBuffTableEntry {
    pub fn new() -> CDOTAModifierBuffTableEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAModifierBuffTableEntry {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAModifierBuffTableEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAModifierBuffTableEntry,
        };
        unsafe {
            instance.get(|| {
                CDOTAModifierBuffTableEntry {
                    entry_type: ::std::option::Option::None,
                    parent: ::std::option::Option::None,
                    index: ::std::option::Option::None,
                    serial_num: ::std::option::Option::None,
                    name: ::std::option::Option::None,
                    ability_level: ::std::option::Option::None,
                    stack_count: ::std::option::Option::None,
                    creation_time: ::std::option::Option::None,
                    duration: ::std::option::Option::None,
                    caster: ::std::option::Option::None,
                    ability: ::std::option::Option::None,
                    armor: ::std::option::Option::None,
                    fade_time: ::std::option::Option::None,
                    subtle: ::std::option::Option::None,
                    channel_time: ::std::option::Option::None,
                    v_start: ::protobuf::SingularPtrField::none(),
                    v_end: ::protobuf::SingularPtrField::none(),
                    portal_loop_appear: ::protobuf::SingularField::none(),
                    portal_loop_disappear: ::protobuf::SingularField::none(),
                    hero_loop_appear: ::protobuf::SingularField::none(),
                    hero_loop_disappear: ::protobuf::SingularField::none(),
                    movement_speed: ::std::option::Option::None,
                    aura: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .DOTA_MODIFIER_ENTRY_TYPE entry_type = 1;

    pub fn clear_entry_type(&mut self) {
        self.entry_type = ::std::option::Option::None;
    }

    pub fn has_entry_type(&self) -> bool {
        self.entry_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entry_type(&mut self, v: DOTA_MODIFIER_ENTRY_TYPE) {
        self.entry_type = ::std::option::Option::Some(v);
    }

    pub fn get_entry_type(&self) -> DOTA_MODIFIER_ENTRY_TYPE {
        self.entry_type.unwrap_or(DOTA_MODIFIER_ENTRY_TYPE::DOTA_MODIFIER_ENTRY_TYPE_ACTIVE)
    }

    // required int32 parent = 2;

    pub fn clear_parent(&mut self) {
        self.parent = ::std::option::Option::None;
    }

    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parent(&mut self, v: i32) {
        self.parent = ::std::option::Option::Some(v);
    }

    pub fn get_parent(&self) -> i32 {
        self.parent.unwrap_or(0)
    }

    // required int32 index = 3;

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

    // required int32 serial_num = 4;

    pub fn clear_serial_num(&mut self) {
        self.serial_num = ::std::option::Option::None;
    }

    pub fn has_serial_num(&self) -> bool {
        self.serial_num.is_some()
    }

    // Param is passed by value, moved
    pub fn set_serial_num(&mut self, v: i32) {
        self.serial_num = ::std::option::Option::Some(v);
    }

    pub fn get_serial_num(&self) -> i32 {
        self.serial_num.unwrap_or(0)
    }

    // optional int32 name = 5;

    pub fn clear_name(&mut self) {
        self.name = ::std::option::Option::None;
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: i32) {
        self.name = ::std::option::Option::Some(v);
    }

    pub fn get_name(&self) -> i32 {
        self.name.unwrap_or(0)
    }

    // optional int32 ability_level = 6;

    pub fn clear_ability_level(&mut self) {
        self.ability_level = ::std::option::Option::None;
    }

    pub fn has_ability_level(&self) -> bool {
        self.ability_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ability_level(&mut self, v: i32) {
        self.ability_level = ::std::option::Option::Some(v);
    }

    pub fn get_ability_level(&self) -> i32 {
        self.ability_level.unwrap_or(0)
    }

    // optional int32 stack_count = 7;

    pub fn clear_stack_count(&mut self) {
        self.stack_count = ::std::option::Option::None;
    }

    pub fn has_stack_count(&self) -> bool {
        self.stack_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stack_count(&mut self, v: i32) {
        self.stack_count = ::std::option::Option::Some(v);
    }

    pub fn get_stack_count(&self) -> i32 {
        self.stack_count.unwrap_or(0)
    }

    // optional float creation_time = 8;

    pub fn clear_creation_time(&mut self) {
        self.creation_time = ::std::option::Option::None;
    }

    pub fn has_creation_time(&self) -> bool {
        self.creation_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_creation_time(&mut self, v: f32) {
        self.creation_time = ::std::option::Option::Some(v);
    }

    pub fn get_creation_time(&self) -> f32 {
        self.creation_time.unwrap_or(0.)
    }

    // optional float duration = 9;

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
        self.duration.unwrap_or(-1f32)
    }

    // optional int32 caster = 10;

    pub fn clear_caster(&mut self) {
        self.caster = ::std::option::Option::None;
    }

    pub fn has_caster(&self) -> bool {
        self.caster.is_some()
    }

    // Param is passed by value, moved
    pub fn set_caster(&mut self, v: i32) {
        self.caster = ::std::option::Option::Some(v);
    }

    pub fn get_caster(&self) -> i32 {
        self.caster.unwrap_or(0)
    }

    // optional int32 ability = 11;

    pub fn clear_ability(&mut self) {
        self.ability = ::std::option::Option::None;
    }

    pub fn has_ability(&self) -> bool {
        self.ability.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ability(&mut self, v: i32) {
        self.ability = ::std::option::Option::Some(v);
    }

    pub fn get_ability(&self) -> i32 {
        self.ability.unwrap_or(0)
    }

    // optional int32 armor = 12;

    pub fn clear_armor(&mut self) {
        self.armor = ::std::option::Option::None;
    }

    pub fn has_armor(&self) -> bool {
        self.armor.is_some()
    }

    // Param is passed by value, moved
    pub fn set_armor(&mut self, v: i32) {
        self.armor = ::std::option::Option::Some(v);
    }

    pub fn get_armor(&self) -> i32 {
        self.armor.unwrap_or(0)
    }

    // optional float fade_time = 13;

    pub fn clear_fade_time(&mut self) {
        self.fade_time = ::std::option::Option::None;
    }

    pub fn has_fade_time(&self) -> bool {
        self.fade_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fade_time(&mut self, v: f32) {
        self.fade_time = ::std::option::Option::Some(v);
    }

    pub fn get_fade_time(&self) -> f32 {
        self.fade_time.unwrap_or(0.)
    }

    // optional bool subtle = 14;

    pub fn clear_subtle(&mut self) {
        self.subtle = ::std::option::Option::None;
    }

    pub fn has_subtle(&self) -> bool {
        self.subtle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_subtle(&mut self, v: bool) {
        self.subtle = ::std::option::Option::Some(v);
    }

    pub fn get_subtle(&self) -> bool {
        self.subtle.unwrap_or(false)
    }

    // optional float channel_time = 15;

    pub fn clear_channel_time(&mut self) {
        self.channel_time = ::std::option::Option::None;
    }

    pub fn has_channel_time(&self) -> bool {
        self.channel_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_time(&mut self, v: f32) {
        self.channel_time = ::std::option::Option::Some(v);
    }

    pub fn get_channel_time(&self) -> f32 {
        self.channel_time.unwrap_or(0.)
    }

    // optional .CMsgVector v_start = 16;

    pub fn clear_v_start(&mut self) {
        self.v_start.clear();
    }

    pub fn has_v_start(&self) -> bool {
        self.v_start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_v_start(&mut self, v: super::netmessages::CMsgVector) {
        self.v_start = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_v_start(&mut self) -> &mut super::netmessages::CMsgVector {
        if self.v_start.is_none() {
            self.v_start.set_default();
        };
        self.v_start.as_mut().unwrap()
    }

    // Take field
    pub fn take_v_start(&mut self) -> super::netmessages::CMsgVector {
        self.v_start.take().unwrap_or_else(|| super::netmessages::CMsgVector::new())
    }

    pub fn get_v_start(&self) -> &super::netmessages::CMsgVector {
        self.v_start.as_ref().unwrap_or_else(|| super::netmessages::CMsgVector::default_instance())
    }

    // optional .CMsgVector v_end = 17;

    pub fn clear_v_end(&mut self) {
        self.v_end.clear();
    }

    pub fn has_v_end(&self) -> bool {
        self.v_end.is_some()
    }

    // Param is passed by value, moved
    pub fn set_v_end(&mut self, v: super::netmessages::CMsgVector) {
        self.v_end = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_v_end(&mut self) -> &mut super::netmessages::CMsgVector {
        if self.v_end.is_none() {
            self.v_end.set_default();
        };
        self.v_end.as_mut().unwrap()
    }

    // Take field
    pub fn take_v_end(&mut self) -> super::netmessages::CMsgVector {
        self.v_end.take().unwrap_or_else(|| super::netmessages::CMsgVector::new())
    }

    pub fn get_v_end(&self) -> &super::netmessages::CMsgVector {
        self.v_end.as_ref().unwrap_or_else(|| super::netmessages::CMsgVector::default_instance())
    }

    // optional string portal_loop_appear = 18;

    pub fn clear_portal_loop_appear(&mut self) {
        self.portal_loop_appear.clear();
    }

    pub fn has_portal_loop_appear(&self) -> bool {
        self.portal_loop_appear.is_some()
    }

    // Param is passed by value, moved
    pub fn set_portal_loop_appear(&mut self, v: ::std::string::String) {
        self.portal_loop_appear = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_portal_loop_appear(&mut self) -> &mut ::std::string::String {
        if self.portal_loop_appear.is_none() {
            self.portal_loop_appear.set_default();
        };
        self.portal_loop_appear.as_mut().unwrap()
    }

    // Take field
    pub fn take_portal_loop_appear(&mut self) -> ::std::string::String {
        self.portal_loop_appear.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_portal_loop_appear(&self) -> &str {
        match self.portal_loop_appear.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string portal_loop_disappear = 19;

    pub fn clear_portal_loop_disappear(&mut self) {
        self.portal_loop_disappear.clear();
    }

    pub fn has_portal_loop_disappear(&self) -> bool {
        self.portal_loop_disappear.is_some()
    }

    // Param is passed by value, moved
    pub fn set_portal_loop_disappear(&mut self, v: ::std::string::String) {
        self.portal_loop_disappear = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_portal_loop_disappear(&mut self) -> &mut ::std::string::String {
        if self.portal_loop_disappear.is_none() {
            self.portal_loop_disappear.set_default();
        };
        self.portal_loop_disappear.as_mut().unwrap()
    }

    // Take field
    pub fn take_portal_loop_disappear(&mut self) -> ::std::string::String {
        self.portal_loop_disappear.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_portal_loop_disappear(&self) -> &str {
        match self.portal_loop_disappear.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string hero_loop_appear = 20;

    pub fn clear_hero_loop_appear(&mut self) {
        self.hero_loop_appear.clear();
    }

    pub fn has_hero_loop_appear(&self) -> bool {
        self.hero_loop_appear.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hero_loop_appear(&mut self, v: ::std::string::String) {
        self.hero_loop_appear = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hero_loop_appear(&mut self) -> &mut ::std::string::String {
        if self.hero_loop_appear.is_none() {
            self.hero_loop_appear.set_default();
        };
        self.hero_loop_appear.as_mut().unwrap()
    }

    // Take field
    pub fn take_hero_loop_appear(&mut self) -> ::std::string::String {
        self.hero_loop_appear.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_hero_loop_appear(&self) -> &str {
        match self.hero_loop_appear.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string hero_loop_disappear = 21;

    pub fn clear_hero_loop_disappear(&mut self) {
        self.hero_loop_disappear.clear();
    }

    pub fn has_hero_loop_disappear(&self) -> bool {
        self.hero_loop_disappear.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hero_loop_disappear(&mut self, v: ::std::string::String) {
        self.hero_loop_disappear = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hero_loop_disappear(&mut self) -> &mut ::std::string::String {
        if self.hero_loop_disappear.is_none() {
            self.hero_loop_disappear.set_default();
        };
        self.hero_loop_disappear.as_mut().unwrap()
    }

    // Take field
    pub fn take_hero_loop_disappear(&mut self) -> ::std::string::String {
        self.hero_loop_disappear.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_hero_loop_disappear(&self) -> &str {
        match self.hero_loop_disappear.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional int32 movement_speed = 22;

    pub fn clear_movement_speed(&mut self) {
        self.movement_speed = ::std::option::Option::None;
    }

    pub fn has_movement_speed(&self) -> bool {
        self.movement_speed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_movement_speed(&mut self, v: i32) {
        self.movement_speed = ::std::option::Option::Some(v);
    }

    pub fn get_movement_speed(&self) -> i32 {
        self.movement_speed.unwrap_or(0)
    }

    // optional bool aura = 23;

    pub fn clear_aura(&mut self) {
        self.aura = ::std::option::Option::None;
    }

    pub fn has_aura(&self) -> bool {
        self.aura.is_some()
    }

    // Param is passed by value, moved
    pub fn set_aura(&mut self, v: bool) {
        self.aura = ::std::option::Option::Some(v);
    }

    pub fn get_aura(&self) -> bool {
        self.aura.unwrap_or(false)
    }
}

impl ::protobuf::Message for CDOTAModifierBuffTableEntry {
    fn is_initialized(&self) -> bool {
        if self.entry_type.is_none() {
            return false;
        };
        if self.parent.is_none() {
            return false;
        };
        if self.index.is_none() {
            return false;
        };
        if self.serial_num.is_none() {
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
                    self.entry_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.parent = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.index = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.serial_num = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.name = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.ability_level = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.stack_count = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.creation_time = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.duration = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.caster = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.ability = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.armor = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.fade_time = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.subtle = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_float());
                    self.channel_time = ::std::option::Option::Some(tmp);
                },
                16 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.v_start));
                },
                17 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.v_end));
                },
                18 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.portal_loop_appear));
                },
                19 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.portal_loop_disappear));
                },
                20 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.hero_loop_appear));
                },
                21 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.hero_loop_disappear));
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.movement_speed = ::std::option::Option::Some(tmp);
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.aura = ::std::option::Option::Some(tmp);
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
        for value in &self.entry_type {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.parent {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.index {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.serial_num {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.name {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.ability_level {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.stack_count {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.creation_time.is_some() {
            my_size += 5;
        };
        if self.duration.is_some() {
            my_size += 5;
        };
        for value in &self.caster {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.ability {
            my_size += ::protobuf::rt::value_size(11, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.armor {
            my_size += ::protobuf::rt::value_size(12, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.fade_time.is_some() {
            my_size += 5;
        };
        if self.subtle.is_some() {
            my_size += 2;
        };
        if self.channel_time.is_some() {
            my_size += 5;
        };
        for value in &self.v_start {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.v_end {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.portal_loop_appear {
            my_size += ::protobuf::rt::string_size(18, &value);
        };
        for value in &self.portal_loop_disappear {
            my_size += ::protobuf::rt::string_size(19, &value);
        };
        for value in &self.hero_loop_appear {
            my_size += ::protobuf::rt::string_size(20, &value);
        };
        for value in &self.hero_loop_disappear {
            my_size += ::protobuf::rt::string_size(21, &value);
        };
        for value in &self.movement_speed {
            my_size += ::protobuf::rt::value_size(22, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.aura.is_some() {
            my_size += 3;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.entry_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.parent {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.index {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.serial_num {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.name {
            try!(os.write_int32(5, v));
        };
        if let Some(v) = self.ability_level {
            try!(os.write_int32(6, v));
        };
        if let Some(v) = self.stack_count {
            try!(os.write_int32(7, v));
        };
        if let Some(v) = self.creation_time {
            try!(os.write_float(8, v));
        };
        if let Some(v) = self.duration {
            try!(os.write_float(9, v));
        };
        if let Some(v) = self.caster {
            try!(os.write_int32(10, v));
        };
        if let Some(v) = self.ability {
            try!(os.write_int32(11, v));
        };
        if let Some(v) = self.armor {
            try!(os.write_int32(12, v));
        };
        if let Some(v) = self.fade_time {
            try!(os.write_float(13, v));
        };
        if let Some(v) = self.subtle {
            try!(os.write_bool(14, v));
        };
        if let Some(v) = self.channel_time {
            try!(os.write_float(15, v));
        };
        if let Some(v) = self.v_start.as_ref() {
            try!(os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.v_end.as_ref() {
            try!(os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.portal_loop_appear.as_ref() {
            try!(os.write_string(18, &v));
        };
        if let Some(v) = self.portal_loop_disappear.as_ref() {
            try!(os.write_string(19, &v));
        };
        if let Some(v) = self.hero_loop_appear.as_ref() {
            try!(os.write_string(20, &v));
        };
        if let Some(v) = self.hero_loop_disappear.as_ref() {
            try!(os.write_string(21, &v));
        };
        if let Some(v) = self.movement_speed {
            try!(os.write_int32(22, v));
        };
        if let Some(v) = self.aura {
            try!(os.write_bool(23, v));
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
        ::std::any::TypeId::of::<CDOTAModifierBuffTableEntry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAModifierBuffTableEntry {
    fn new() -> CDOTAModifierBuffTableEntry {
        CDOTAModifierBuffTableEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAModifierBuffTableEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "entry_type",
                    CDOTAModifierBuffTableEntry::has_entry_type,
                    CDOTAModifierBuffTableEntry::get_entry_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "parent",
                    CDOTAModifierBuffTableEntry::has_parent,
                    CDOTAModifierBuffTableEntry::get_parent,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "index",
                    CDOTAModifierBuffTableEntry::has_index,
                    CDOTAModifierBuffTableEntry::get_index,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "serial_num",
                    CDOTAModifierBuffTableEntry::has_serial_num,
                    CDOTAModifierBuffTableEntry::get_serial_num,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "name",
                    CDOTAModifierBuffTableEntry::has_name,
                    CDOTAModifierBuffTableEntry::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "ability_level",
                    CDOTAModifierBuffTableEntry::has_ability_level,
                    CDOTAModifierBuffTableEntry::get_ability_level,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "stack_count",
                    CDOTAModifierBuffTableEntry::has_stack_count,
                    CDOTAModifierBuffTableEntry::get_stack_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "creation_time",
                    CDOTAModifierBuffTableEntry::has_creation_time,
                    CDOTAModifierBuffTableEntry::get_creation_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "duration",
                    CDOTAModifierBuffTableEntry::has_duration,
                    CDOTAModifierBuffTableEntry::get_duration,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "caster",
                    CDOTAModifierBuffTableEntry::has_caster,
                    CDOTAModifierBuffTableEntry::get_caster,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "ability",
                    CDOTAModifierBuffTableEntry::has_ability,
                    CDOTAModifierBuffTableEntry::get_ability,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "armor",
                    CDOTAModifierBuffTableEntry::has_armor,
                    CDOTAModifierBuffTableEntry::get_armor,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "fade_time",
                    CDOTAModifierBuffTableEntry::has_fade_time,
                    CDOTAModifierBuffTableEntry::get_fade_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "subtle",
                    CDOTAModifierBuffTableEntry::has_subtle,
                    CDOTAModifierBuffTableEntry::get_subtle,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor(
                    "channel_time",
                    CDOTAModifierBuffTableEntry::has_channel_time,
                    CDOTAModifierBuffTableEntry::get_channel_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "v_start",
                    CDOTAModifierBuffTableEntry::has_v_start,
                    CDOTAModifierBuffTableEntry::get_v_start,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "v_end",
                    CDOTAModifierBuffTableEntry::has_v_end,
                    CDOTAModifierBuffTableEntry::get_v_end,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "portal_loop_appear",
                    CDOTAModifierBuffTableEntry::has_portal_loop_appear,
                    CDOTAModifierBuffTableEntry::get_portal_loop_appear,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "portal_loop_disappear",
                    CDOTAModifierBuffTableEntry::has_portal_loop_disappear,
                    CDOTAModifierBuffTableEntry::get_portal_loop_disappear,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "hero_loop_appear",
                    CDOTAModifierBuffTableEntry::has_hero_loop_appear,
                    CDOTAModifierBuffTableEntry::get_hero_loop_appear,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "hero_loop_disappear",
                    CDOTAModifierBuffTableEntry::has_hero_loop_disappear,
                    CDOTAModifierBuffTableEntry::get_hero_loop_disappear,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "movement_speed",
                    CDOTAModifierBuffTableEntry::has_movement_speed,
                    CDOTAModifierBuffTableEntry::get_movement_speed,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "aura",
                    CDOTAModifierBuffTableEntry::has_aura,
                    CDOTAModifierBuffTableEntry::get_aura,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAModifierBuffTableEntry>(
                    "CDOTAModifierBuffTableEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAModifierBuffTableEntry {
    fn clear(&mut self) {
        self.clear_entry_type();
        self.clear_parent();
        self.clear_index();
        self.clear_serial_num();
        self.clear_name();
        self.clear_ability_level();
        self.clear_stack_count();
        self.clear_creation_time();
        self.clear_duration();
        self.clear_caster();
        self.clear_ability();
        self.clear_armor();
        self.clear_fade_time();
        self.clear_subtle();
        self.clear_channel_time();
        self.clear_v_start();
        self.clear_v_end();
        self.clear_portal_loop_appear();
        self.clear_portal_loop_disappear();
        self.clear_hero_loop_appear();
        self.clear_hero_loop_disappear();
        self.clear_movement_speed();
        self.clear_aura();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CDOTAModifierBuffTableEntry {
    fn eq(&self, other: &CDOTAModifierBuffTableEntry) -> bool {
        self.entry_type == other.entry_type &&
        self.parent == other.parent &&
        self.index == other.index &&
        self.serial_num == other.serial_num &&
        self.name == other.name &&
        self.ability_level == other.ability_level &&
        self.stack_count == other.stack_count &&
        self.creation_time == other.creation_time &&
        self.duration == other.duration &&
        self.caster == other.caster &&
        self.ability == other.ability &&
        self.armor == other.armor &&
        self.fade_time == other.fade_time &&
        self.subtle == other.subtle &&
        self.channel_time == other.channel_time &&
        self.v_start == other.v_start &&
        self.v_end == other.v_end &&
        self.portal_loop_appear == other.portal_loop_appear &&
        self.portal_loop_disappear == other.portal_loop_disappear &&
        self.hero_loop_appear == other.hero_loop_appear &&
        self.hero_loop_disappear == other.hero_loop_disappear &&
        self.movement_speed == other.movement_speed &&
        self.aura == other.aura &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CDOTAModifierBuffTableEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DOTA_MODIFIER_ENTRY_TYPE {
    DOTA_MODIFIER_ENTRY_TYPE_ACTIVE = 1,
    DOTA_MODIFIER_ENTRY_TYPE_REMOVED = 2,
}

impl ::protobuf::ProtobufEnum for DOTA_MODIFIER_ENTRY_TYPE {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DOTA_MODIFIER_ENTRY_TYPE> {
        match value {
            1 => ::std::option::Option::Some(DOTA_MODIFIER_ENTRY_TYPE::DOTA_MODIFIER_ENTRY_TYPE_ACTIVE),
            2 => ::std::option::Option::Some(DOTA_MODIFIER_ENTRY_TYPE::DOTA_MODIFIER_ENTRY_TYPE_REMOVED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DOTA_MODIFIER_ENTRY_TYPE] = &[
            DOTA_MODIFIER_ENTRY_TYPE::DOTA_MODIFIER_ENTRY_TYPE_ACTIVE,
            DOTA_MODIFIER_ENTRY_TYPE::DOTA_MODIFIER_ENTRY_TYPE_REMOVED,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<DOTA_MODIFIER_ENTRY_TYPE>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DOTA_MODIFIER_ENTRY_TYPE", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DOTA_MODIFIER_ENTRY_TYPE {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x14, 0x64, 0x6f, 0x74, 0x61, 0x5f, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x73,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x20, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74,
    0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x11, 0x6e, 0x65, 0x74, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xa1, 0x04, 0x0a, 0x1b,
    0x43, 0x44, 0x4f, 0x54, 0x41, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x42, 0x75, 0x66,
    0x66, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x2d, 0x0a, 0x0a, 0x65,
    0x6e, 0x74, 0x72, 0x79, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32,
    0x19, 0x2e, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x4d, 0x4f, 0x44, 0x49, 0x46, 0x49, 0x45, 0x52, 0x5f,
    0x45, 0x4e, 0x54, 0x52, 0x59, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x12, 0x0e, 0x0a, 0x06, 0x70, 0x61,
    0x72, 0x65, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x02, 0x28, 0x05, 0x12, 0x0d, 0x0a, 0x05, 0x69, 0x6e,
    0x64, 0x65, 0x78, 0x18, 0x03, 0x20, 0x02, 0x28, 0x05, 0x12, 0x12, 0x0a, 0x0a, 0x73, 0x65, 0x72,
    0x69, 0x61, 0x6c, 0x5f, 0x6e, 0x75, 0x6d, 0x18, 0x04, 0x20, 0x02, 0x28, 0x05, 0x12, 0x0c, 0x0a,
    0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05, 0x12, 0x15, 0x0a, 0x0d, 0x61,
    0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x5f, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x05, 0x12, 0x13, 0x0a, 0x0b, 0x73, 0x74, 0x61, 0x63, 0x6b, 0x5f, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x05, 0x12, 0x15, 0x0a, 0x0d, 0x63, 0x72, 0x65, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x02, 0x12, 0x14,
    0x0a, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x09, 0x20, 0x01, 0x28, 0x02,
    0x3a, 0x02, 0x2d, 0x31, 0x12, 0x0e, 0x0a, 0x06, 0x63, 0x61, 0x73, 0x74, 0x65, 0x72, 0x18, 0x0a,
    0x20, 0x01, 0x28, 0x05, 0x12, 0x0f, 0x0a, 0x07, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x18,
    0x0b, 0x20, 0x01, 0x28, 0x05, 0x12, 0x0d, 0x0a, 0x05, 0x61, 0x72, 0x6d, 0x6f, 0x72, 0x18, 0x0c,
    0x20, 0x01, 0x28, 0x05, 0x12, 0x11, 0x0a, 0x09, 0x66, 0x61, 0x64, 0x65, 0x5f, 0x74, 0x69, 0x6d,
    0x65, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x02, 0x12, 0x0e, 0x0a, 0x06, 0x73, 0x75, 0x62, 0x74, 0x6c,
    0x65, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x08, 0x12, 0x14, 0x0a, 0x0c, 0x63, 0x68, 0x61, 0x6e, 0x6e,
    0x65, 0x6c, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x02, 0x12, 0x1c, 0x0a,
    0x07, 0x76, 0x5f, 0x73, 0x74, 0x61, 0x72, 0x74, 0x18, 0x10, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b,
    0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x12, 0x1a, 0x0a, 0x05, 0x76,
    0x5f, 0x65, 0x6e, 0x64, 0x18, 0x11, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x43, 0x4d, 0x73,
    0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x12, 0x1a, 0x0a, 0x12, 0x70, 0x6f, 0x72, 0x74, 0x61,
    0x6c, 0x5f, 0x6c, 0x6f, 0x6f, 0x70, 0x5f, 0x61, 0x70, 0x70, 0x65, 0x61, 0x72, 0x18, 0x12, 0x20,
    0x01, 0x28, 0x09, 0x12, 0x1d, 0x0a, 0x15, 0x70, 0x6f, 0x72, 0x74, 0x61, 0x6c, 0x5f, 0x6c, 0x6f,
    0x6f, 0x70, 0x5f, 0x64, 0x69, 0x73, 0x61, 0x70, 0x70, 0x65, 0x61, 0x72, 0x18, 0x13, 0x20, 0x01,
    0x28, 0x09, 0x12, 0x18, 0x0a, 0x10, 0x68, 0x65, 0x72, 0x6f, 0x5f, 0x6c, 0x6f, 0x6f, 0x70, 0x5f,
    0x61, 0x70, 0x70, 0x65, 0x61, 0x72, 0x18, 0x14, 0x20, 0x01, 0x28, 0x09, 0x12, 0x1b, 0x0a, 0x13,
    0x68, 0x65, 0x72, 0x6f, 0x5f, 0x6c, 0x6f, 0x6f, 0x70, 0x5f, 0x64, 0x69, 0x73, 0x61, 0x70, 0x70,
    0x65, 0x61, 0x72, 0x18, 0x15, 0x20, 0x01, 0x28, 0x09, 0x12, 0x16, 0x0a, 0x0e, 0x6d, 0x6f, 0x76,
    0x65, 0x6d, 0x65, 0x6e, 0x74, 0x5f, 0x73, 0x70, 0x65, 0x65, 0x64, 0x18, 0x16, 0x20, 0x01, 0x28,
    0x05, 0x12, 0x0c, 0x0a, 0x04, 0x61, 0x75, 0x72, 0x61, 0x18, 0x17, 0x20, 0x01, 0x28, 0x08, 0x2a,
    0x65, 0x0a, 0x18, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x4d, 0x4f, 0x44, 0x49, 0x46, 0x49, 0x45, 0x52,
    0x5f, 0x45, 0x4e, 0x54, 0x52, 0x59, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x12, 0x23, 0x0a, 0x1f, 0x44,
    0x4f, 0x54, 0x41, 0x5f, 0x4d, 0x4f, 0x44, 0x49, 0x46, 0x49, 0x45, 0x52, 0x5f, 0x45, 0x4e, 0x54,
    0x52, 0x59, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x41, 0x43, 0x54, 0x49, 0x56, 0x45, 0x10, 0x01,
    0x12, 0x24, 0x0a, 0x20, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x4d, 0x4f, 0x44, 0x49, 0x46, 0x49, 0x45,
    0x52, 0x5f, 0x45, 0x4e, 0x54, 0x52, 0x59, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x52, 0x45, 0x4d,
    0x4f, 0x56, 0x45, 0x44, 0x10, 0x02, 0x42, 0x05, 0x48, 0x01, 0x80, 0x01, 0x00,
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
