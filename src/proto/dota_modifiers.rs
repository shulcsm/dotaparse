// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]

use descriptor::*;
use netmessages::*;

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
    0x4f, 0x56, 0x45, 0x44, 0x10, 0x02, 0x42, 0x05, 0x48, 0x01, 0x80, 0x01, 0x00, 0x4a, 0xc3, 0x13,
    0x0a, 0x06, 0x12, 0x04, 0x1e, 0x00, 0x61, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x1e,
    0x00, 0x1c, 0x0a, 0x37, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x1e, 0x00, 0x1c, 0x1a,
    0x2a, 0x20, 0x57, 0x65, 0x20, 0x63, 0x61, 0x72, 0x65, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x61,
    0x62, 0x6f, 0x75, 0x74, 0x20, 0x73, 0x70, 0x65, 0x65, 0x64, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x20,
    0x63, 0x6f, 0x64, 0x65, 0x20, 0x73, 0x69, 0x7a, 0x65, 0x0d, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x1e, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x1e, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00,
    0x03, 0x12, 0x03, 0x1e, 0x16, 0x1b, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x21, 0x00, 0x23,
    0x0a, 0x41, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01, 0x12, 0x03, 0x21, 0x00, 0x23, 0x1a, 0x34, 0x20,
    0x57, 0x65, 0x20, 0x64, 0x6f, 0x6e, 0x27, 0x74, 0x20, 0x75, 0x73, 0x65, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x66, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x69, 0x74,
    0x79, 0x0d, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x21, 0x07,
    0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x21, 0x07, 0x1a,
    0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x21, 0x07, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x03, 0x12, 0x03, 0x21, 0x1d, 0x22, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x3a, 0x07, 0x29, 0x0a, 0x22, 0x0a, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x3d, 0x07, 0x1a, 0x1a, 0x17, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x43, 0x4d, 0x73, 0x67, 0x56,
    0x65, 0x63, 0x74, 0x6f, 0x72, 0x2c, 0x20, 0x65, 0x74, 0x63, 0x2e, 0x0d, 0x0a, 0x0a, 0x0a, 0x0a,
    0x02, 0x05, 0x00, 0x12, 0x04, 0x3f, 0x00, 0x43, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01,
    0x12, 0x03, 0x3f, 0x05, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x41,
    0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x41, 0x08, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x41, 0x2a, 0x2b, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x42, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x42, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x01, 0x02, 0x12, 0x03, 0x42, 0x2b, 0x2c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x45,
    0x00, 0x61, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x45, 0x08, 0x23, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x47, 0x08, 0x39, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x47, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x47, 0x11, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x47, 0x2a, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x47, 0x37, 0x38, 0x0a, 0x27, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x48, 0x08,
    0x22, 0x22, 0x1a, 0x20, 0x65, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x70,
    0x61, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x0d, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x48, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x48, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x48, 0x17, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x48, 0x20, 0x21, 0x0a, 0x51, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x49,
    0x08, 0x21, 0x22, 0x44, 0x20, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x20, 0x69, 0x6e, 0x74, 0x6f, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x20, 0x6c, 0x69, 0x73,
    0x74, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x20,
    0x28, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x20, 0x74, 0x6f, 0x20, 0x65, 0x61, 0x63, 0x68, 0x20, 0x65,
    0x6e, 0x74, 0x69, 0x74, 0x79, 0x29, 0x0d, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x49, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x49, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x49,
    0x17, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x49, 0x1f, 0x20,
    0x0a, 0x24, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x4a, 0x08, 0x26, 0x22, 0x17, 0x20,
    0x67, 0x6c, 0x6f, 0x62, 0x61, 0x6c, 0x20, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x20, 0x6e, 0x75,
    0x6d, 0x62, 0x65, 0x72, 0x0d, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x4a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x4a,
    0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x4a, 0x17, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x4a, 0x24, 0x25, 0x0a, 0x39,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x4c, 0x08, 0x20, 0x22, 0x2c, 0x20, 0x69, 0x6e,
    0x64, 0x65, 0x78, 0x20, 0x69, 0x6e, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x6f, 0x64,
    0x69, 0x66, 0x69, 0x65, 0x72, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6e,
    0x67, 0x20, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x0d, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x04, 0x12, 0x03, 0x4c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05,
    0x12, 0x03, 0x4c, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x4c, 0x17, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x4c, 0x1e,
    0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x4d, 0x08, 0x29, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x4d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x4d, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x05, 0x01, 0x12, 0x03, 0x4d, 0x17, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05,
    0x03, 0x12, 0x03, 0x4d, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03,
    0x4e, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x4e, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x4e, 0x11, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x4e, 0x17, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x4e, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x07, 0x12, 0x03, 0x4f, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07,
    0x04, 0x12, 0x03, 0x4f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x05, 0x12,
    0x03, 0x4f, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x4f,
    0x17, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x4f, 0x27, 0x28,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x50, 0x08, 0x37, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x03, 0x50, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x08, 0x05, 0x12, 0x03, 0x50, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x08, 0x01, 0x12, 0x03, 0x50, 0x17, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x03,
    0x12, 0x03, 0x50, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x08, 0x12, 0x03,
    0x50, 0x24, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x07, 0x12, 0x03, 0x50, 0x30,
    0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x09, 0x12, 0x03, 0x51, 0x08, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x04, 0x12, 0x03, 0x51, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x09, 0x05, 0x12, 0x03, 0x51, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x09, 0x01, 0x12, 0x03, 0x51, 0x17, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09,
    0x03, 0x12, 0x03, 0x51, 0x20, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0a, 0x12, 0x03,
    0x52, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x52, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x52, 0x11, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x52, 0x17, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x52, 0x21, 0x23, 0x0a, 0x38, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x0b, 0x12, 0x03, 0x55, 0x08, 0x22, 0x1a, 0x17, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f,
    0x6e, 0x61, 0x6c, 0x20, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x20, 0x64, 0x61, 0x74, 0x61, 0x0d,
    0x0a, 0x22, 0x12, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x69, 0x6e, 0x76, 0x6f,
    0x6b, 0x65, 0x72, 0x0d, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x04, 0x12, 0x03,
    0x55, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x55, 0x11,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x55, 0x17, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x55, 0x1f, 0x21, 0x0a, 0x23, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x0c, 0x12, 0x03, 0x56, 0x08, 0x26, 0x22, 0x16, 0x20, 0x75, 0x73, 0x65,
    0x64, 0x20, 0x62, 0x79, 0x20, 0x69, 0x6e, 0x76, 0x69, 0x73, 0x69, 0x62, 0x6c, 0x69, 0x74, 0x79,
    0x0d, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x56, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x56, 0x11, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x56, 0x17, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x56, 0x23, 0x25, 0x0a, 0x23, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x0d, 0x12, 0x03, 0x57, 0x08, 0x22, 0x22, 0x16, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x62,
    0x79, 0x20, 0x69, 0x6e, 0x76, 0x69, 0x73, 0x69, 0x62, 0x6c, 0x69, 0x74, 0x79, 0x0d, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x57, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x57, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x57, 0x16, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0d, 0x03, 0x12, 0x03, 0x57, 0x1f, 0x21, 0x0a, 0x20, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0e, 0x12,
    0x03, 0x58, 0x08, 0x29, 0x22, 0x13, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74,
    0x65, 0x6c, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x0d, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0e, 0x04, 0x12, 0x03, 0x58, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x05,
    0x12, 0x03, 0x58, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x01, 0x12, 0x03,
    0x58, 0x17, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x58, 0x26,
    0x28, 0x0a, 0x20, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0f, 0x12, 0x03, 0x59, 0x08, 0x29, 0x22, 0x13,
    0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x65, 0x6c, 0x65, 0x70, 0x6f, 0x72,
    0x74, 0x0d, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0f, 0x04, 0x12, 0x03, 0x59, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0f, 0x06, 0x12, 0x03, 0x59, 0x11, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x59, 0x1c, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0f, 0x03, 0x12, 0x03, 0x59, 0x26, 0x28, 0x0a, 0x20, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x10, 0x12, 0x03, 0x5a, 0x08, 0x27, 0x22, 0x13, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20,
    0x62, 0x79, 0x20, 0x74, 0x65, 0x6c, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x0d, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x10, 0x04, 0x12, 0x03, 0x5a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x10, 0x06, 0x12, 0x03, 0x5a, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x10, 0x01, 0x12, 0x03, 0x5a, 0x1c, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x10, 0x03,
    0x12, 0x03, 0x5a, 0x24, 0x26, 0x0a, 0x20, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x11, 0x12, 0x03, 0x5b,
    0x08, 0x30, 0x22, 0x13, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x65, 0x6c,
    0x65, 0x70, 0x6f, 0x72, 0x74, 0x0d, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x11, 0x04,
    0x12, 0x03, 0x5b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x11, 0x05, 0x12, 0x03,
    0x5b, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x11, 0x01, 0x12, 0x03, 0x5b, 0x18,
    0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x11, 0x03, 0x12, 0x03, 0x5b, 0x2d, 0x2f, 0x0a,
    0x20, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x12, 0x12, 0x03, 0x5c, 0x08, 0x33, 0x22, 0x13, 0x20, 0x75,
    0x73, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x65, 0x6c, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x0d,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x12, 0x04, 0x12, 0x03, 0x5c, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x12, 0x05, 0x12, 0x03, 0x5c, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x12, 0x01, 0x12, 0x03, 0x5c, 0x18, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x12, 0x03, 0x12, 0x03, 0x5c, 0x30, 0x32, 0x0a, 0x20, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x13, 0x12, 0x03, 0x5d, 0x08, 0x2e, 0x22, 0x13, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x62, 0x79,
    0x20, 0x74, 0x65, 0x6c, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x0d, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x13, 0x04, 0x12, 0x03, 0x5d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x13, 0x05, 0x12, 0x03, 0x5d, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x13, 0x01,
    0x12, 0x03, 0x5d, 0x18, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x13, 0x03, 0x12, 0x03,
    0x5d, 0x2b, 0x2d, 0x0a, 0x20, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x14, 0x12, 0x03, 0x5e, 0x08, 0x31,
    0x22, 0x13, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x65, 0x6c, 0x65, 0x70,
    0x6f, 0x72, 0x74, 0x0d, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x14, 0x04, 0x12, 0x03,
    0x5e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x14, 0x05, 0x12, 0x03, 0x5e, 0x11,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x14, 0x01, 0x12, 0x03, 0x5e, 0x18, 0x2b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x14, 0x03, 0x12, 0x03, 0x5e, 0x2e, 0x30, 0x0a, 0x27, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x15, 0x12, 0x03, 0x5f, 0x08, 0x2b, 0x22, 0x1a, 0x20, 0x75, 0x73, 0x65,
    0x64, 0x20, 0x62, 0x79, 0x20, 0x73, 0x6d, 0x6f, 0x6b, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x64, 0x65,
    0x63, 0x65, 0x69, 0x74, 0x0d, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x15, 0x04, 0x12,
    0x03, 0x5f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x15, 0x05, 0x12, 0x03, 0x5f,
    0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x15, 0x01, 0x12, 0x03, 0x5f, 0x17, 0x25,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x15, 0x03, 0x12, 0x03, 0x5f, 0x28, 0x2a, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x16, 0x12, 0x03, 0x60, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x16, 0x04, 0x12, 0x03, 0x60, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x16, 0x05, 0x12, 0x03, 0x60, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x16, 0x01,
    0x12, 0x03, 0x60, 0x16, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x16, 0x03, 0x12, 0x03,
    0x60, 0x1d, 0x1f,
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
pub struct CDOTAModifierBuffTableEntry {
    entry_type: Option<DOTA_MODIFIER_ENTRY_TYPE>,
    parent: Option<i32>,
    index: Option<i32>,
    serial_num: Option<i32>,
    name: Option<i32>,
    ability_level: Option<i32>,
    stack_count: Option<i32>,
    creation_time: Option<f32>,
    duration: Option<f32>,
    caster: Option<i32>,
    ability: Option<i32>,
    armor: Option<i32>,
    fade_time: Option<f32>,
    subtle: Option<bool>,
    channel_time: Option<f32>,
    v_start: ::protobuf::SingularPtrField<CMsgVector>,
    v_end: ::protobuf::SingularPtrField<CMsgVector>,
    portal_loop_appear: ::protobuf::SingularField<String>,
    portal_loop_disappear: ::protobuf::SingularField<String>,
    hero_loop_appear: ::protobuf::SingularField<String>,
    hero_loop_disappear: ::protobuf::SingularField<String>,
    movement_speed: Option<i32>,
    aura: Option<bool>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> CDOTAModifierBuffTableEntry {
    pub fn new() -> CDOTAModifierBuffTableEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAModifierBuffTableEntry {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAModifierBuffTableEntry> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const CDOTAModifierBuffTableEntry };
        unsafe {
            instance.get(|| {
                CDOTAModifierBuffTableEntry {
                    entry_type: None,
                    parent: None,
                    index: None,
                    serial_num: None,
                    name: None,
                    ability_level: None,
                    stack_count: None,
                    creation_time: None,
                    duration: None,
                    caster: None,
                    ability: None,
                    armor: None,
                    fade_time: None,
                    subtle: None,
                    channel_time: None,
                    v_start: ::protobuf::SingularPtrField::none(),
                    v_end: ::protobuf::SingularPtrField::none(),
                    portal_loop_appear: ::protobuf::SingularField::none(),
                    portal_loop_disappear: ::protobuf::SingularField::none(),
                    hero_loop_appear: ::protobuf::SingularField::none(),
                    hero_loop_disappear: ::protobuf::SingularField::none(),
                    movement_speed: None,
                    aura: None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    pub fn clear_entry_type(&mut self) {
        self.entry_type = None;
    }

    pub fn has_entry_type(&self) -> bool {
        self.entry_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entry_type(&mut self, v: DOTA_MODIFIER_ENTRY_TYPE) {
        self.entry_type = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_entry_type(&'a mut self) -> &'a mut DOTA_MODIFIER_ENTRY_TYPE {
        if self.entry_type.is_none() {
            self.entry_type = Some(DOTA_MODIFIER_ENTRY_TYPE::new(0));
        };
        self.entry_type.as_mut().unwrap()
    }

    pub fn get_entry_type(&self) -> DOTA_MODIFIER_ENTRY_TYPE {
        self.entry_type.unwrap_or_else(|| DOTA_MODIFIER_ENTRY_TYPE::new(0))
    }

    pub fn clear_parent(&mut self) {
        self.parent = None;
    }

    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parent(&mut self, v: i32) {
        self.parent = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_parent(&'a mut self) -> &'a mut i32 {
        if self.parent.is_none() {
            self.parent = Some(0);
        };
        self.parent.as_mut().unwrap()
    }

    pub fn get_parent(&self) -> i32 {
        self.parent.unwrap_or_else(|| 0)
    }

    pub fn clear_index(&mut self) {
        self.index = None;
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: i32) {
        self.index = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_index(&'a mut self) -> &'a mut i32 {
        if self.index.is_none() {
            self.index = Some(0);
        };
        self.index.as_mut().unwrap()
    }

    pub fn get_index(&self) -> i32 {
        self.index.unwrap_or_else(|| 0)
    }

    pub fn clear_serial_num(&mut self) {
        self.serial_num = None;
    }

    pub fn has_serial_num(&self) -> bool {
        self.serial_num.is_some()
    }

    // Param is passed by value, moved
    pub fn set_serial_num(&mut self, v: i32) {
        self.serial_num = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_serial_num(&'a mut self) -> &'a mut i32 {
        if self.serial_num.is_none() {
            self.serial_num = Some(0);
        };
        self.serial_num.as_mut().unwrap()
    }

    pub fn get_serial_num(&self) -> i32 {
        self.serial_num.unwrap_or_else(|| 0)
    }

    pub fn clear_name(&mut self) {
        self.name = None;
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: i32) {
        self.name = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&'a mut self) -> &'a mut i32 {
        if self.name.is_none() {
            self.name = Some(0);
        };
        self.name.as_mut().unwrap()
    }

    pub fn get_name(&self) -> i32 {
        self.name.unwrap_or_else(|| 0)
    }

    pub fn clear_ability_level(&mut self) {
        self.ability_level = None;
    }

    pub fn has_ability_level(&self) -> bool {
        self.ability_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ability_level(&mut self, v: i32) {
        self.ability_level = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ability_level(&'a mut self) -> &'a mut i32 {
        if self.ability_level.is_none() {
            self.ability_level = Some(0);
        };
        self.ability_level.as_mut().unwrap()
    }

    pub fn get_ability_level(&self) -> i32 {
        self.ability_level.unwrap_or_else(|| 0)
    }

    pub fn clear_stack_count(&mut self) {
        self.stack_count = None;
    }

    pub fn has_stack_count(&self) -> bool {
        self.stack_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stack_count(&mut self, v: i32) {
        self.stack_count = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stack_count(&'a mut self) -> &'a mut i32 {
        if self.stack_count.is_none() {
            self.stack_count = Some(0);
        };
        self.stack_count.as_mut().unwrap()
    }

    pub fn get_stack_count(&self) -> i32 {
        self.stack_count.unwrap_or_else(|| 0)
    }

    pub fn clear_creation_time(&mut self) {
        self.creation_time = None;
    }

    pub fn has_creation_time(&self) -> bool {
        self.creation_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_creation_time(&mut self, v: f32) {
        self.creation_time = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_creation_time(&'a mut self) -> &'a mut f32 {
        if self.creation_time.is_none() {
            self.creation_time = Some(0.);
        };
        self.creation_time.as_mut().unwrap()
    }

    pub fn get_creation_time(&self) -> f32 {
        self.creation_time.unwrap_or_else(|| 0.)
    }

    pub fn clear_duration(&mut self) {
        self.duration = None;
    }

    pub fn has_duration(&self) -> bool {
        self.duration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration(&mut self, v: f32) {
        self.duration = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_duration(&'a mut self) -> &'a mut f32 {
        if self.duration.is_none() {
            self.duration = Some(0.);
        };
        self.duration.as_mut().unwrap()
    }

    pub fn get_duration(&self) -> f32 {
        self.duration.unwrap_or_else(|| 0.)
    }

    pub fn clear_caster(&mut self) {
        self.caster = None;
    }

    pub fn has_caster(&self) -> bool {
        self.caster.is_some()
    }

    // Param is passed by value, moved
    pub fn set_caster(&mut self, v: i32) {
        self.caster = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_caster(&'a mut self) -> &'a mut i32 {
        if self.caster.is_none() {
            self.caster = Some(0);
        };
        self.caster.as_mut().unwrap()
    }

    pub fn get_caster(&self) -> i32 {
        self.caster.unwrap_or_else(|| 0)
    }

    pub fn clear_ability(&mut self) {
        self.ability = None;
    }

    pub fn has_ability(&self) -> bool {
        self.ability.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ability(&mut self, v: i32) {
        self.ability = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ability(&'a mut self) -> &'a mut i32 {
        if self.ability.is_none() {
            self.ability = Some(0);
        };
        self.ability.as_mut().unwrap()
    }

    pub fn get_ability(&self) -> i32 {
        self.ability.unwrap_or_else(|| 0)
    }

    pub fn clear_armor(&mut self) {
        self.armor = None;
    }

    pub fn has_armor(&self) -> bool {
        self.armor.is_some()
    }

    // Param is passed by value, moved
    pub fn set_armor(&mut self, v: i32) {
        self.armor = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_armor(&'a mut self) -> &'a mut i32 {
        if self.armor.is_none() {
            self.armor = Some(0);
        };
        self.armor.as_mut().unwrap()
    }

    pub fn get_armor(&self) -> i32 {
        self.armor.unwrap_or_else(|| 0)
    }

    pub fn clear_fade_time(&mut self) {
        self.fade_time = None;
    }

    pub fn has_fade_time(&self) -> bool {
        self.fade_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fade_time(&mut self, v: f32) {
        self.fade_time = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fade_time(&'a mut self) -> &'a mut f32 {
        if self.fade_time.is_none() {
            self.fade_time = Some(0.);
        };
        self.fade_time.as_mut().unwrap()
    }

    pub fn get_fade_time(&self) -> f32 {
        self.fade_time.unwrap_or_else(|| 0.)
    }

    pub fn clear_subtle(&mut self) {
        self.subtle = None;
    }

    pub fn has_subtle(&self) -> bool {
        self.subtle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_subtle(&mut self, v: bool) {
        self.subtle = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_subtle(&'a mut self) -> &'a mut bool {
        if self.subtle.is_none() {
            self.subtle = Some(false);
        };
        self.subtle.as_mut().unwrap()
    }

    pub fn get_subtle(&self) -> bool {
        self.subtle.unwrap_or_else(|| false)
    }

    pub fn clear_channel_time(&mut self) {
        self.channel_time = None;
    }

    pub fn has_channel_time(&self) -> bool {
        self.channel_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_time(&mut self, v: f32) {
        self.channel_time = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_channel_time(&'a mut self) -> &'a mut f32 {
        if self.channel_time.is_none() {
            self.channel_time = Some(0.);
        };
        self.channel_time.as_mut().unwrap()
    }

    pub fn get_channel_time(&self) -> f32 {
        self.channel_time.unwrap_or_else(|| 0.)
    }

    pub fn clear_v_start(&mut self) {
        self.v_start.clear();
    }

    pub fn has_v_start(&self) -> bool {
        self.v_start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_v_start(&mut self, v: CMsgVector) {
        self.v_start = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_v_start(&'a mut self) -> &'a mut CMsgVector {
        if self.v_start.is_none() {
            self.v_start.set_default();
        };
        self.v_start.as_mut().unwrap()
    }

    pub fn get_v_start(&'a self) -> &'a CMsgVector {
        self.v_start.as_ref().unwrap_or_else(|| CMsgVector::default_instance())
    }

    pub fn clear_v_end(&mut self) {
        self.v_end.clear();
    }

    pub fn has_v_end(&self) -> bool {
        self.v_end.is_some()
    }

    // Param is passed by value, moved
    pub fn set_v_end(&mut self, v: CMsgVector) {
        self.v_end = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_v_end(&'a mut self) -> &'a mut CMsgVector {
        if self.v_end.is_none() {
            self.v_end.set_default();
        };
        self.v_end.as_mut().unwrap()
    }

    pub fn get_v_end(&'a self) -> &'a CMsgVector {
        self.v_end.as_ref().unwrap_or_else(|| CMsgVector::default_instance())
    }

    pub fn clear_portal_loop_appear(&mut self) {
        self.portal_loop_appear.clear();
    }

    pub fn has_portal_loop_appear(&self) -> bool {
        self.portal_loop_appear.is_some()
    }

    // Param is passed by value, moved
    pub fn set_portal_loop_appear(&mut self, v: String) {
        self.portal_loop_appear = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_portal_loop_appear(&'a mut self) -> &'a mut String {
        if self.portal_loop_appear.is_none() {
            self.portal_loop_appear.set_default();
        };
        self.portal_loop_appear.as_mut().unwrap()
    }

    pub fn get_portal_loop_appear(&'a self) -> &'a str {
        match self.portal_loop_appear.as_ref() {
            Some(ref v) => v.as_slice(),
            None => "",
        }
    }

    pub fn clear_portal_loop_disappear(&mut self) {
        self.portal_loop_disappear.clear();
    }

    pub fn has_portal_loop_disappear(&self) -> bool {
        self.portal_loop_disappear.is_some()
    }

    // Param is passed by value, moved
    pub fn set_portal_loop_disappear(&mut self, v: String) {
        self.portal_loop_disappear = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_portal_loop_disappear(&'a mut self) -> &'a mut String {
        if self.portal_loop_disappear.is_none() {
            self.portal_loop_disappear.set_default();
        };
        self.portal_loop_disappear.as_mut().unwrap()
    }

    pub fn get_portal_loop_disappear(&'a self) -> &'a str {
        match self.portal_loop_disappear.as_ref() {
            Some(ref v) => v.as_slice(),
            None => "",
        }
    }

    pub fn clear_hero_loop_appear(&mut self) {
        self.hero_loop_appear.clear();
    }

    pub fn has_hero_loop_appear(&self) -> bool {
        self.hero_loop_appear.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hero_loop_appear(&mut self, v: String) {
        self.hero_loop_appear = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hero_loop_appear(&'a mut self) -> &'a mut String {
        if self.hero_loop_appear.is_none() {
            self.hero_loop_appear.set_default();
        };
        self.hero_loop_appear.as_mut().unwrap()
    }

    pub fn get_hero_loop_appear(&'a self) -> &'a str {
        match self.hero_loop_appear.as_ref() {
            Some(ref v) => v.as_slice(),
            None => "",
        }
    }

    pub fn clear_hero_loop_disappear(&mut self) {
        self.hero_loop_disappear.clear();
    }

    pub fn has_hero_loop_disappear(&self) -> bool {
        self.hero_loop_disappear.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hero_loop_disappear(&mut self, v: String) {
        self.hero_loop_disappear = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hero_loop_disappear(&'a mut self) -> &'a mut String {
        if self.hero_loop_disappear.is_none() {
            self.hero_loop_disappear.set_default();
        };
        self.hero_loop_disappear.as_mut().unwrap()
    }

    pub fn get_hero_loop_disappear(&'a self) -> &'a str {
        match self.hero_loop_disappear.as_ref() {
            Some(ref v) => v.as_slice(),
            None => "",
        }
    }

    pub fn clear_movement_speed(&mut self) {
        self.movement_speed = None;
    }

    pub fn has_movement_speed(&self) -> bool {
        self.movement_speed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_movement_speed(&mut self, v: i32) {
        self.movement_speed = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_movement_speed(&'a mut self) -> &'a mut i32 {
        if self.movement_speed.is_none() {
            self.movement_speed = Some(0);
        };
        self.movement_speed.as_mut().unwrap()
    }

    pub fn get_movement_speed(&self) -> i32 {
        self.movement_speed.unwrap_or_else(|| 0)
    }

    pub fn clear_aura(&mut self) {
        self.aura = None;
    }

    pub fn has_aura(&self) -> bool {
        self.aura.is_some()
    }

    // Param is passed by value, moved
    pub fn set_aura(&mut self, v: bool) {
        self.aura = Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_aura(&'a mut self) -> &'a mut bool {
        if self.aura.is_none() {
            self.aura = Some(false);
        };
        self.aura.as_mut().unwrap()
    }

    pub fn get_aura(&self) -> bool {
        self.aura.unwrap_or_else(|| false)
    }
}

impl ::protobuf::Message for CDOTAModifierBuffTableEntry {
    fn new() -> CDOTAModifierBuffTableEntry {
        CDOTAModifierBuffTableEntry::new()
    }

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

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) {
        while !is.eof() {
            let (field_number, wire_type) = is.read_tag_unpack();
            match field_number {
                1 => {
                    assert_eq!(::protobuf::wire_format::WireTypeVarint, wire_type);
                    let tmp = DOTA_MODIFIER_ENTRY_TYPE::new(is.read_int32());
                    self.entry_type = Some(tmp);
                },
                2 => {
                    assert_eq!(::protobuf::wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int32();
                    self.parent = Some(tmp);
                },
                3 => {
                    assert_eq!(::protobuf::wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int32();
                    self.index = Some(tmp);
                },
                4 => {
                    assert_eq!(::protobuf::wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int32();
                    self.serial_num = Some(tmp);
                },
                5 => {
                    assert_eq!(::protobuf::wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int32();
                    self.name = Some(tmp);
                },
                6 => {
                    assert_eq!(::protobuf::wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int32();
                    self.ability_level = Some(tmp);
                },
                7 => {
                    assert_eq!(::protobuf::wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int32();
                    self.stack_count = Some(tmp);
                },
                8 => {
                    assert_eq!(::protobuf::wire_format::WireTypeFixed32, wire_type);
                    let tmp = is.read_float();
                    self.creation_time = Some(tmp);
                },
                9 => {
                    assert_eq!(::protobuf::wire_format::WireTypeFixed32, wire_type);
                    let tmp = is.read_float();
                    self.duration = Some(tmp);
                },
                10 => {
                    assert_eq!(::protobuf::wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int32();
                    self.caster = Some(tmp);
                },
                11 => {
                    assert_eq!(::protobuf::wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int32();
                    self.ability = Some(tmp);
                },
                12 => {
                    assert_eq!(::protobuf::wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int32();
                    self.armor = Some(tmp);
                },
                13 => {
                    assert_eq!(::protobuf::wire_format::WireTypeFixed32, wire_type);
                    let tmp = is.read_float();
                    self.fade_time = Some(tmp);
                },
                14 => {
                    assert_eq!(::protobuf::wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_bool();
                    self.subtle = Some(tmp);
                },
                15 => {
                    assert_eq!(::protobuf::wire_format::WireTypeFixed32, wire_type);
                    let tmp = is.read_float();
                    self.channel_time = Some(tmp);
                },
                16 => {
                    assert_eq!(::protobuf::wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = self.v_start.set_default();
                    is.merge_message(tmp)
                },
                17 => {
                    assert_eq!(::protobuf::wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = self.v_end.set_default();
                    is.merge_message(tmp)
                },
                18 => {
                    assert_eq!(::protobuf::wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = self.portal_loop_appear.set_default();
                    is.read_string_into(tmp)
                },
                19 => {
                    assert_eq!(::protobuf::wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = self.portal_loop_disappear.set_default();
                    is.read_string_into(tmp)
                },
                20 => {
                    assert_eq!(::protobuf::wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = self.hero_loop_appear.set_default();
                    is.read_string_into(tmp)
                },
                21 => {
                    assert_eq!(::protobuf::wire_format::WireTypeLengthDelimited, wire_type);
                    let tmp = self.hero_loop_disappear.set_default();
                    is.read_string_into(tmp)
                },
                22 => {
                    assert_eq!(::protobuf::wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_int32();
                    self.movement_speed = Some(tmp);
                },
                23 => {
                    assert_eq!(::protobuf::wire_format::WireTypeVarint, wire_type);
                    let tmp = is.read_bool();
                    self.aura = Some(tmp);
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
        for value in self.entry_type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.parent.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.index.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.serial_num.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.name.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.ability_level.iter() {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.stack_count.iter() {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.creation_time.is_some() {
            my_size += 5;
        };
        if self.duration.is_some() {
            my_size += 5;
        };
        for value in self.caster.iter() {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.ability.iter() {
            my_size += ::protobuf::rt::value_size(11, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.armor.iter() {
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
        for value in self.v_start.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.v_end.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.portal_loop_appear.iter() {
            my_size += ::protobuf::rt::string_size(18, value.as_slice());
        };
        for value in self.portal_loop_disappear.iter() {
            my_size += ::protobuf::rt::string_size(19, value.as_slice());
        };
        for value in self.hero_loop_appear.iter() {
            my_size += ::protobuf::rt::string_size(20, value.as_slice());
        };
        for value in self.hero_loop_disappear.iter() {
            my_size += ::protobuf::rt::string_size(21, value.as_slice());
        };
        for value in self.movement_speed.iter() {
            my_size += ::protobuf::rt::value_size(22, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.aura.is_some() {
            my_size += 3;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        *sizes.get_mut(pos) = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) {
        use protobuf::{Message};
        match self.entry_type {
            Some(ref v) => {
                os.write_enum(1, *v as i32);
            },
            None => {},
        };
        match self.parent {
            Some(ref v) => {
                os.write_int32(2, *v);
            },
            None => {},
        };
        match self.index {
            Some(ref v) => {
                os.write_int32(3, *v);
            },
            None => {},
        };
        match self.serial_num {
            Some(ref v) => {
                os.write_int32(4, *v);
            },
            None => {},
        };
        match self.name {
            Some(ref v) => {
                os.write_int32(5, *v);
            },
            None => {},
        };
        match self.ability_level {
            Some(ref v) => {
                os.write_int32(6, *v);
            },
            None => {},
        };
        match self.stack_count {
            Some(ref v) => {
                os.write_int32(7, *v);
            },
            None => {},
        };
        match self.creation_time {
            Some(ref v) => {
                os.write_float(8, *v);
            },
            None => {},
        };
        match self.duration {
            Some(ref v) => {
                os.write_float(9, *v);
            },
            None => {},
        };
        match self.caster {
            Some(ref v) => {
                os.write_int32(10, *v);
            },
            None => {},
        };
        match self.ability {
            Some(ref v) => {
                os.write_int32(11, *v);
            },
            None => {},
        };
        match self.armor {
            Some(ref v) => {
                os.write_int32(12, *v);
            },
            None => {},
        };
        match self.fade_time {
            Some(ref v) => {
                os.write_float(13, *v);
            },
            None => {},
        };
        match self.subtle {
            Some(ref v) => {
                os.write_bool(14, *v);
            },
            None => {},
        };
        match self.channel_time {
            Some(ref v) => {
                os.write_float(15, *v);
            },
            None => {},
        };
        match self.v_start.as_ref() {
            Some(ref v) => {
                os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos);
            },
            None => {},
        };
        match self.v_end.as_ref() {
            Some(ref v) => {
                os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited);
                os.write_raw_varint32(sizes[*sizes_pos]);
                *sizes_pos += 1;
                v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos);
            },
            None => {},
        };
        match self.portal_loop_appear.as_ref() {
            Some(ref v) => {
                os.write_string(18, v.as_slice());
            },
            None => {},
        };
        match self.portal_loop_disappear.as_ref() {
            Some(ref v) => {
                os.write_string(19, v.as_slice());
            },
            None => {},
        };
        match self.hero_loop_appear.as_ref() {
            Some(ref v) => {
                os.write_string(20, v.as_slice());
            },
            None => {},
        };
        match self.hero_loop_disappear.as_ref() {
            Some(ref v) => {
                os.write_string(21, v.as_slice());
            },
            None => {},
        };
        match self.movement_speed {
            Some(ref v) => {
                os.write_int32(22, *v);
            },
            None => {},
        };
        match self.aura {
            Some(ref v) => {
                os.write_bool(23, *v);
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
    fn descriptor_static(_: Option<CDOTAModifierBuffTableEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::MessageDescriptor };
        unsafe {
            descriptor.get(|| {
                let mut fields: Vec<&'static ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry>> = Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&CDOTAModifierBuffTableEntry_entry_type_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry>) });
                fields.push(unsafe { ::std::mem::transmute(&CDOTAModifierBuffTableEntry_parent_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry>) });
                fields.push(unsafe { ::std::mem::transmute(&CDOTAModifierBuffTableEntry_index_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry>) });
                fields.push(unsafe { ::std::mem::transmute(&CDOTAModifierBuffTableEntry_serial_num_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry>) });
                fields.push(unsafe { ::std::mem::transmute(&CDOTAModifierBuffTableEntry_name_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry>) });
                fields.push(unsafe { ::std::mem::transmute(&CDOTAModifierBuffTableEntry_ability_level_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry>) });
                fields.push(unsafe { ::std::mem::transmute(&CDOTAModifierBuffTableEntry_stack_count_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry>) });
                fields.push(unsafe { ::std::mem::transmute(&CDOTAModifierBuffTableEntry_creation_time_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry>) });
                fields.push(unsafe { ::std::mem::transmute(&CDOTAModifierBuffTableEntry_duration_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry>) });
                fields.push(unsafe { ::std::mem::transmute(&CDOTAModifierBuffTableEntry_caster_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry>) });
                fields.push(unsafe { ::std::mem::transmute(&CDOTAModifierBuffTableEntry_ability_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry>) });
                fields.push(unsafe { ::std::mem::transmute(&CDOTAModifierBuffTableEntry_armor_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry>) });
                fields.push(unsafe { ::std::mem::transmute(&CDOTAModifierBuffTableEntry_fade_time_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry>) });
                fields.push(unsafe { ::std::mem::transmute(&CDOTAModifierBuffTableEntry_subtle_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry>) });
                fields.push(unsafe { ::std::mem::transmute(&CDOTAModifierBuffTableEntry_channel_time_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry>) });
                fields.push(unsafe { ::std::mem::transmute(&CDOTAModifierBuffTableEntry_v_start_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry>) });
                fields.push(unsafe { ::std::mem::transmute(&CDOTAModifierBuffTableEntry_v_end_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry>) });
                fields.push(unsafe { ::std::mem::transmute(&CDOTAModifierBuffTableEntry_portal_loop_appear_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry>) });
                fields.push(unsafe { ::std::mem::transmute(&CDOTAModifierBuffTableEntry_portal_loop_disappear_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry>) });
                fields.push(unsafe { ::std::mem::transmute(&CDOTAModifierBuffTableEntry_hero_loop_appear_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry>) });
                fields.push(unsafe { ::std::mem::transmute(&CDOTAModifierBuffTableEntry_hero_loop_disappear_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry>) });
                fields.push(unsafe { ::std::mem::transmute(&CDOTAModifierBuffTableEntry_movement_speed_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry>) });
                fields.push(unsafe { ::std::mem::transmute(&CDOTAModifierBuffTableEntry_aura_acc as &'static ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry>) });
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAModifierBuffTableEntry>(
                    "CDOTAModifierBuffTableEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<CDOTAModifierBuffTableEntry>()
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

impl ::std::fmt::Show for CDOTAModifierBuffTableEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct CDOTAModifierBuffTableEntry_entry_type_acc_type;
static CDOTAModifierBuffTableEntry_entry_type_acc: CDOTAModifierBuffTableEntry_entry_type_acc_type = CDOTAModifierBuffTableEntry_entry_type_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry> for CDOTAModifierBuffTableEntry_entry_type_acc_type {
    fn name(&self) -> &'static str {
        "entry_type"
    }

    fn has_field(&self, m: &CDOTAModifierBuffTableEntry) -> bool {
        m.has_entry_type()
    }

    fn get_enum<'a>(&self, m: &CDOTAModifierBuffTableEntry) -> &'static ::protobuf::reflect::EnumValueDescriptor {
        use protobuf::{ProtobufEnum};
        m.get_entry_type().descriptor()
    }
}

#[allow(non_camel_case_types)]
struct CDOTAModifierBuffTableEntry_parent_acc_type;
static CDOTAModifierBuffTableEntry_parent_acc: CDOTAModifierBuffTableEntry_parent_acc_type = CDOTAModifierBuffTableEntry_parent_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry> for CDOTAModifierBuffTableEntry_parent_acc_type {
    fn name(&self) -> &'static str {
        "parent"
    }

    fn has_field(&self, m: &CDOTAModifierBuffTableEntry) -> bool {
        m.has_parent()
    }

    fn get_i32(&self, m: &CDOTAModifierBuffTableEntry) -> i32 {
        m.get_parent()
    }
}

#[allow(non_camel_case_types)]
struct CDOTAModifierBuffTableEntry_index_acc_type;
static CDOTAModifierBuffTableEntry_index_acc: CDOTAModifierBuffTableEntry_index_acc_type = CDOTAModifierBuffTableEntry_index_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry> for CDOTAModifierBuffTableEntry_index_acc_type {
    fn name(&self) -> &'static str {
        "index"
    }

    fn has_field(&self, m: &CDOTAModifierBuffTableEntry) -> bool {
        m.has_index()
    }

    fn get_i32(&self, m: &CDOTAModifierBuffTableEntry) -> i32 {
        m.get_index()
    }
}

#[allow(non_camel_case_types)]
struct CDOTAModifierBuffTableEntry_serial_num_acc_type;
static CDOTAModifierBuffTableEntry_serial_num_acc: CDOTAModifierBuffTableEntry_serial_num_acc_type = CDOTAModifierBuffTableEntry_serial_num_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry> for CDOTAModifierBuffTableEntry_serial_num_acc_type {
    fn name(&self) -> &'static str {
        "serial_num"
    }

    fn has_field(&self, m: &CDOTAModifierBuffTableEntry) -> bool {
        m.has_serial_num()
    }

    fn get_i32(&self, m: &CDOTAModifierBuffTableEntry) -> i32 {
        m.get_serial_num()
    }
}

#[allow(non_camel_case_types)]
struct CDOTAModifierBuffTableEntry_name_acc_type;
static CDOTAModifierBuffTableEntry_name_acc: CDOTAModifierBuffTableEntry_name_acc_type = CDOTAModifierBuffTableEntry_name_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry> for CDOTAModifierBuffTableEntry_name_acc_type {
    fn name(&self) -> &'static str {
        "name"
    }

    fn has_field(&self, m: &CDOTAModifierBuffTableEntry) -> bool {
        m.has_name()
    }

    fn get_i32(&self, m: &CDOTAModifierBuffTableEntry) -> i32 {
        m.get_name()
    }
}

#[allow(non_camel_case_types)]
struct CDOTAModifierBuffTableEntry_ability_level_acc_type;
static CDOTAModifierBuffTableEntry_ability_level_acc: CDOTAModifierBuffTableEntry_ability_level_acc_type = CDOTAModifierBuffTableEntry_ability_level_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry> for CDOTAModifierBuffTableEntry_ability_level_acc_type {
    fn name(&self) -> &'static str {
        "ability_level"
    }

    fn has_field(&self, m: &CDOTAModifierBuffTableEntry) -> bool {
        m.has_ability_level()
    }

    fn get_i32(&self, m: &CDOTAModifierBuffTableEntry) -> i32 {
        m.get_ability_level()
    }
}

#[allow(non_camel_case_types)]
struct CDOTAModifierBuffTableEntry_stack_count_acc_type;
static CDOTAModifierBuffTableEntry_stack_count_acc: CDOTAModifierBuffTableEntry_stack_count_acc_type = CDOTAModifierBuffTableEntry_stack_count_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry> for CDOTAModifierBuffTableEntry_stack_count_acc_type {
    fn name(&self) -> &'static str {
        "stack_count"
    }

    fn has_field(&self, m: &CDOTAModifierBuffTableEntry) -> bool {
        m.has_stack_count()
    }

    fn get_i32(&self, m: &CDOTAModifierBuffTableEntry) -> i32 {
        m.get_stack_count()
    }
}

#[allow(non_camel_case_types)]
struct CDOTAModifierBuffTableEntry_creation_time_acc_type;
static CDOTAModifierBuffTableEntry_creation_time_acc: CDOTAModifierBuffTableEntry_creation_time_acc_type = CDOTAModifierBuffTableEntry_creation_time_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry> for CDOTAModifierBuffTableEntry_creation_time_acc_type {
    fn name(&self) -> &'static str {
        "creation_time"
    }

    fn has_field(&self, m: &CDOTAModifierBuffTableEntry) -> bool {
        m.has_creation_time()
    }

    fn get_f32(&self, m: &CDOTAModifierBuffTableEntry) -> f32 {
        m.get_creation_time()
    }
}

#[allow(non_camel_case_types)]
struct CDOTAModifierBuffTableEntry_duration_acc_type;
static CDOTAModifierBuffTableEntry_duration_acc: CDOTAModifierBuffTableEntry_duration_acc_type = CDOTAModifierBuffTableEntry_duration_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry> for CDOTAModifierBuffTableEntry_duration_acc_type {
    fn name(&self) -> &'static str {
        "duration"
    }

    fn has_field(&self, m: &CDOTAModifierBuffTableEntry) -> bool {
        m.has_duration()
    }

    fn get_f32(&self, m: &CDOTAModifierBuffTableEntry) -> f32 {
        m.get_duration()
    }
}

#[allow(non_camel_case_types)]
struct CDOTAModifierBuffTableEntry_caster_acc_type;
static CDOTAModifierBuffTableEntry_caster_acc: CDOTAModifierBuffTableEntry_caster_acc_type = CDOTAModifierBuffTableEntry_caster_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry> for CDOTAModifierBuffTableEntry_caster_acc_type {
    fn name(&self) -> &'static str {
        "caster"
    }

    fn has_field(&self, m: &CDOTAModifierBuffTableEntry) -> bool {
        m.has_caster()
    }

    fn get_i32(&self, m: &CDOTAModifierBuffTableEntry) -> i32 {
        m.get_caster()
    }
}

#[allow(non_camel_case_types)]
struct CDOTAModifierBuffTableEntry_ability_acc_type;
static CDOTAModifierBuffTableEntry_ability_acc: CDOTAModifierBuffTableEntry_ability_acc_type = CDOTAModifierBuffTableEntry_ability_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry> for CDOTAModifierBuffTableEntry_ability_acc_type {
    fn name(&self) -> &'static str {
        "ability"
    }

    fn has_field(&self, m: &CDOTAModifierBuffTableEntry) -> bool {
        m.has_ability()
    }

    fn get_i32(&self, m: &CDOTAModifierBuffTableEntry) -> i32 {
        m.get_ability()
    }
}

#[allow(non_camel_case_types)]
struct CDOTAModifierBuffTableEntry_armor_acc_type;
static CDOTAModifierBuffTableEntry_armor_acc: CDOTAModifierBuffTableEntry_armor_acc_type = CDOTAModifierBuffTableEntry_armor_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry> for CDOTAModifierBuffTableEntry_armor_acc_type {
    fn name(&self) -> &'static str {
        "armor"
    }

    fn has_field(&self, m: &CDOTAModifierBuffTableEntry) -> bool {
        m.has_armor()
    }

    fn get_i32(&self, m: &CDOTAModifierBuffTableEntry) -> i32 {
        m.get_armor()
    }
}

#[allow(non_camel_case_types)]
struct CDOTAModifierBuffTableEntry_fade_time_acc_type;
static CDOTAModifierBuffTableEntry_fade_time_acc: CDOTAModifierBuffTableEntry_fade_time_acc_type = CDOTAModifierBuffTableEntry_fade_time_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry> for CDOTAModifierBuffTableEntry_fade_time_acc_type {
    fn name(&self) -> &'static str {
        "fade_time"
    }

    fn has_field(&self, m: &CDOTAModifierBuffTableEntry) -> bool {
        m.has_fade_time()
    }

    fn get_f32(&self, m: &CDOTAModifierBuffTableEntry) -> f32 {
        m.get_fade_time()
    }
}

#[allow(non_camel_case_types)]
struct CDOTAModifierBuffTableEntry_subtle_acc_type;
static CDOTAModifierBuffTableEntry_subtle_acc: CDOTAModifierBuffTableEntry_subtle_acc_type = CDOTAModifierBuffTableEntry_subtle_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry> for CDOTAModifierBuffTableEntry_subtle_acc_type {
    fn name(&self) -> &'static str {
        "subtle"
    }

    fn has_field(&self, m: &CDOTAModifierBuffTableEntry) -> bool {
        m.has_subtle()
    }

    fn get_bool(&self, m: &CDOTAModifierBuffTableEntry) -> bool {
        m.get_subtle()
    }
}

#[allow(non_camel_case_types)]
struct CDOTAModifierBuffTableEntry_channel_time_acc_type;
static CDOTAModifierBuffTableEntry_channel_time_acc: CDOTAModifierBuffTableEntry_channel_time_acc_type = CDOTAModifierBuffTableEntry_channel_time_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry> for CDOTAModifierBuffTableEntry_channel_time_acc_type {
    fn name(&self) -> &'static str {
        "channel_time"
    }

    fn has_field(&self, m: &CDOTAModifierBuffTableEntry) -> bool {
        m.has_channel_time()
    }

    fn get_f32(&self, m: &CDOTAModifierBuffTableEntry) -> f32 {
        m.get_channel_time()
    }
}

#[allow(non_camel_case_types)]
struct CDOTAModifierBuffTableEntry_v_start_acc_type;
static CDOTAModifierBuffTableEntry_v_start_acc: CDOTAModifierBuffTableEntry_v_start_acc_type = CDOTAModifierBuffTableEntry_v_start_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry> for CDOTAModifierBuffTableEntry_v_start_acc_type {
    fn name(&self) -> &'static str {
        "v_start"
    }

    fn has_field(&self, m: &CDOTAModifierBuffTableEntry) -> bool {
        m.has_v_start()
    }

    fn get_message<'a>(&self, m: &'a CDOTAModifierBuffTableEntry) -> &'a ::protobuf::Message {
        m.get_v_start() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct CDOTAModifierBuffTableEntry_v_end_acc_type;
static CDOTAModifierBuffTableEntry_v_end_acc: CDOTAModifierBuffTableEntry_v_end_acc_type = CDOTAModifierBuffTableEntry_v_end_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry> for CDOTAModifierBuffTableEntry_v_end_acc_type {
    fn name(&self) -> &'static str {
        "v_end"
    }

    fn has_field(&self, m: &CDOTAModifierBuffTableEntry) -> bool {
        m.has_v_end()
    }

    fn get_message<'a>(&self, m: &'a CDOTAModifierBuffTableEntry) -> &'a ::protobuf::Message {
        m.get_v_end() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct CDOTAModifierBuffTableEntry_portal_loop_appear_acc_type;
static CDOTAModifierBuffTableEntry_portal_loop_appear_acc: CDOTAModifierBuffTableEntry_portal_loop_appear_acc_type = CDOTAModifierBuffTableEntry_portal_loop_appear_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry> for CDOTAModifierBuffTableEntry_portal_loop_appear_acc_type {
    fn name(&self) -> &'static str {
        "portal_loop_appear"
    }

    fn has_field(&self, m: &CDOTAModifierBuffTableEntry) -> bool {
        m.has_portal_loop_appear()
    }

    fn get_str<'a>(&self, m: &'a CDOTAModifierBuffTableEntry) -> &'a str {
        m.get_portal_loop_appear()
    }
}

#[allow(non_camel_case_types)]
struct CDOTAModifierBuffTableEntry_portal_loop_disappear_acc_type;
static CDOTAModifierBuffTableEntry_portal_loop_disappear_acc: CDOTAModifierBuffTableEntry_portal_loop_disappear_acc_type = CDOTAModifierBuffTableEntry_portal_loop_disappear_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry> for CDOTAModifierBuffTableEntry_portal_loop_disappear_acc_type {
    fn name(&self) -> &'static str {
        "portal_loop_disappear"
    }

    fn has_field(&self, m: &CDOTAModifierBuffTableEntry) -> bool {
        m.has_portal_loop_disappear()
    }

    fn get_str<'a>(&self, m: &'a CDOTAModifierBuffTableEntry) -> &'a str {
        m.get_portal_loop_disappear()
    }
}

#[allow(non_camel_case_types)]
struct CDOTAModifierBuffTableEntry_hero_loop_appear_acc_type;
static CDOTAModifierBuffTableEntry_hero_loop_appear_acc: CDOTAModifierBuffTableEntry_hero_loop_appear_acc_type = CDOTAModifierBuffTableEntry_hero_loop_appear_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry> for CDOTAModifierBuffTableEntry_hero_loop_appear_acc_type {
    fn name(&self) -> &'static str {
        "hero_loop_appear"
    }

    fn has_field(&self, m: &CDOTAModifierBuffTableEntry) -> bool {
        m.has_hero_loop_appear()
    }

    fn get_str<'a>(&self, m: &'a CDOTAModifierBuffTableEntry) -> &'a str {
        m.get_hero_loop_appear()
    }
}

#[allow(non_camel_case_types)]
struct CDOTAModifierBuffTableEntry_hero_loop_disappear_acc_type;
static CDOTAModifierBuffTableEntry_hero_loop_disappear_acc: CDOTAModifierBuffTableEntry_hero_loop_disappear_acc_type = CDOTAModifierBuffTableEntry_hero_loop_disappear_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry> for CDOTAModifierBuffTableEntry_hero_loop_disappear_acc_type {
    fn name(&self) -> &'static str {
        "hero_loop_disappear"
    }

    fn has_field(&self, m: &CDOTAModifierBuffTableEntry) -> bool {
        m.has_hero_loop_disappear()
    }

    fn get_str<'a>(&self, m: &'a CDOTAModifierBuffTableEntry) -> &'a str {
        m.get_hero_loop_disappear()
    }
}

#[allow(non_camel_case_types)]
struct CDOTAModifierBuffTableEntry_movement_speed_acc_type;
static CDOTAModifierBuffTableEntry_movement_speed_acc: CDOTAModifierBuffTableEntry_movement_speed_acc_type = CDOTAModifierBuffTableEntry_movement_speed_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry> for CDOTAModifierBuffTableEntry_movement_speed_acc_type {
    fn name(&self) -> &'static str {
        "movement_speed"
    }

    fn has_field(&self, m: &CDOTAModifierBuffTableEntry) -> bool {
        m.has_movement_speed()
    }

    fn get_i32(&self, m: &CDOTAModifierBuffTableEntry) -> i32 {
        m.get_movement_speed()
    }
}

#[allow(non_camel_case_types)]
struct CDOTAModifierBuffTableEntry_aura_acc_type;
static CDOTAModifierBuffTableEntry_aura_acc: CDOTAModifierBuffTableEntry_aura_acc_type = CDOTAModifierBuffTableEntry_aura_acc_type;

impl ::protobuf::reflect::FieldAccessor<CDOTAModifierBuffTableEntry> for CDOTAModifierBuffTableEntry_aura_acc_type {
    fn name(&self) -> &'static str {
        "aura"
    }

    fn has_field(&self, m: &CDOTAModifierBuffTableEntry) -> bool {
        m.has_aura()
    }

    fn get_bool(&self, m: &CDOTAModifierBuffTableEntry) -> bool {
        m.get_aura()
    }
}

#[deriving(Clone,PartialEq,Eq,Show)]
pub enum DOTA_MODIFIER_ENTRY_TYPE {
    DOTA_MODIFIER_ENTRY_TYPE_ACTIVE = 1,
    DOTA_MODIFIER_ENTRY_TYPE_REMOVED = 2,
}

impl DOTA_MODIFIER_ENTRY_TYPE {
    pub fn new(value: i32) -> DOTA_MODIFIER_ENTRY_TYPE {
        match value {
            1 => DOTA_MODIFIER_ENTRY_TYPE_ACTIVE,
            2 => DOTA_MODIFIER_ENTRY_TYPE_REMOVED,
            _ => fail!()
        }
    }
}

impl ::protobuf::ProtobufEnum for DOTA_MODIFIER_ENTRY_TYPE {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn enum_descriptor_static(_: Option<DOTA_MODIFIER_ENTRY_TYPE>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy { lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const ::protobuf::reflect::EnumDescriptor };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DOTA_MODIFIER_ENTRY_TYPE", file_descriptor_proto())
            })
        }
    }
}
