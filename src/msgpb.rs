// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct Message {
    // message fields
    msg_type: ::std::option::Option<MessageType>,
    cmd_req: ::protobuf::SingularPtrField<super::raft_cmdpb::RaftCmdRequest>,
    cmd_resp: ::protobuf::SingularPtrField<super::raft_cmdpb::RaftCmdResponse>,
    raft: ::protobuf::SingularPtrField<super::raft_serverpb::RaftMessage>,
    kv_req: ::protobuf::SingularPtrField<super::kvrpcpb::Request>,
    kv_resp: ::protobuf::SingularPtrField<super::kvrpcpb::Response>,
    cop_req: ::protobuf::SingularPtrField<super::coprocessor::Request>,
    cop_resp: ::protobuf::SingularPtrField<super::coprocessor::Response>,
    pd_req: ::protobuf::SingularPtrField<super::pdpb::Request>,
    pd_resp: ::protobuf::SingularPtrField<super::pdpb::Response>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Message {}

impl Message {
    pub fn new() -> Message {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Message {
        static mut instance: ::protobuf::lazy::Lazy<Message> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Message,
        };
        unsafe {
            instance.get(|| {
                Message {
                    msg_type: ::std::option::Option::None,
                    cmd_req: ::protobuf::SingularPtrField::none(),
                    cmd_resp: ::protobuf::SingularPtrField::none(),
                    raft: ::protobuf::SingularPtrField::none(),
                    kv_req: ::protobuf::SingularPtrField::none(),
                    kv_resp: ::protobuf::SingularPtrField::none(),
                    cop_req: ::protobuf::SingularPtrField::none(),
                    cop_resp: ::protobuf::SingularPtrField::none(),
                    pd_req: ::protobuf::SingularPtrField::none(),
                    pd_resp: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .msgpb.MessageType msg_type = 1;

    pub fn clear_msg_type(&mut self) {
        self.msg_type = ::std::option::Option::None;
    }

    pub fn has_msg_type(&self) -> bool {
        self.msg_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg_type(&mut self, v: MessageType) {
        self.msg_type = ::std::option::Option::Some(v);
    }

    pub fn get_msg_type(&self) -> MessageType {
        self.msg_type.unwrap_or(MessageType::None)
    }

    // optional .raft_cmdpb.RaftCmdRequest cmd_req = 2;

    pub fn clear_cmd_req(&mut self) {
        self.cmd_req.clear();
    }

    pub fn has_cmd_req(&self) -> bool {
        self.cmd_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_req(&mut self, v: super::raft_cmdpb::RaftCmdRequest) {
        self.cmd_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_req(&mut self) -> &mut super::raft_cmdpb::RaftCmdRequest {
        if self.cmd_req.is_none() {
            self.cmd_req.set_default();
        };
        self.cmd_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_req(&mut self) -> super::raft_cmdpb::RaftCmdRequest {
        self.cmd_req.take().unwrap_or_else(|| super::raft_cmdpb::RaftCmdRequest::new())
    }

    pub fn get_cmd_req(&self) -> &super::raft_cmdpb::RaftCmdRequest {
        self.cmd_req.as_ref().unwrap_or_else(|| super::raft_cmdpb::RaftCmdRequest::default_instance())
    }

    // optional .raft_cmdpb.RaftCmdResponse cmd_resp = 3;

    pub fn clear_cmd_resp(&mut self) {
        self.cmd_resp.clear();
    }

    pub fn has_cmd_resp(&self) -> bool {
        self.cmd_resp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_resp(&mut self, v: super::raft_cmdpb::RaftCmdResponse) {
        self.cmd_resp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_resp(&mut self) -> &mut super::raft_cmdpb::RaftCmdResponse {
        if self.cmd_resp.is_none() {
            self.cmd_resp.set_default();
        };
        self.cmd_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_resp(&mut self) -> super::raft_cmdpb::RaftCmdResponse {
        self.cmd_resp.take().unwrap_or_else(|| super::raft_cmdpb::RaftCmdResponse::new())
    }

    pub fn get_cmd_resp(&self) -> &super::raft_cmdpb::RaftCmdResponse {
        self.cmd_resp.as_ref().unwrap_or_else(|| super::raft_cmdpb::RaftCmdResponse::default_instance())
    }

    // optional .raft_serverpb.RaftMessage raft = 4;

    pub fn clear_raft(&mut self) {
        self.raft.clear();
    }

    pub fn has_raft(&self) -> bool {
        self.raft.is_some()
    }

    // Param is passed by value, moved
    pub fn set_raft(&mut self, v: super::raft_serverpb::RaftMessage) {
        self.raft = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_raft(&mut self) -> &mut super::raft_serverpb::RaftMessage {
        if self.raft.is_none() {
            self.raft.set_default();
        };
        self.raft.as_mut().unwrap()
    }

    // Take field
    pub fn take_raft(&mut self) -> super::raft_serverpb::RaftMessage {
        self.raft.take().unwrap_or_else(|| super::raft_serverpb::RaftMessage::new())
    }

    pub fn get_raft(&self) -> &super::raft_serverpb::RaftMessage {
        self.raft.as_ref().unwrap_or_else(|| super::raft_serverpb::RaftMessage::default_instance())
    }

    // optional .kvrpcpb.Request kv_req = 5;

    pub fn clear_kv_req(&mut self) {
        self.kv_req.clear();
    }

    pub fn has_kv_req(&self) -> bool {
        self.kv_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kv_req(&mut self, v: super::kvrpcpb::Request) {
        self.kv_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_kv_req(&mut self) -> &mut super::kvrpcpb::Request {
        if self.kv_req.is_none() {
            self.kv_req.set_default();
        };
        self.kv_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_kv_req(&mut self) -> super::kvrpcpb::Request {
        self.kv_req.take().unwrap_or_else(|| super::kvrpcpb::Request::new())
    }

    pub fn get_kv_req(&self) -> &super::kvrpcpb::Request {
        self.kv_req.as_ref().unwrap_or_else(|| super::kvrpcpb::Request::default_instance())
    }

    // optional .kvrpcpb.Response kv_resp = 6;

    pub fn clear_kv_resp(&mut self) {
        self.kv_resp.clear();
    }

    pub fn has_kv_resp(&self) -> bool {
        self.kv_resp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kv_resp(&mut self, v: super::kvrpcpb::Response) {
        self.kv_resp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_kv_resp(&mut self) -> &mut super::kvrpcpb::Response {
        if self.kv_resp.is_none() {
            self.kv_resp.set_default();
        };
        self.kv_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_kv_resp(&mut self) -> super::kvrpcpb::Response {
        self.kv_resp.take().unwrap_or_else(|| super::kvrpcpb::Response::new())
    }

    pub fn get_kv_resp(&self) -> &super::kvrpcpb::Response {
        self.kv_resp.as_ref().unwrap_or_else(|| super::kvrpcpb::Response::default_instance())
    }

    // optional .coprocessor.Request cop_req = 7;

    pub fn clear_cop_req(&mut self) {
        self.cop_req.clear();
    }

    pub fn has_cop_req(&self) -> bool {
        self.cop_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cop_req(&mut self, v: super::coprocessor::Request) {
        self.cop_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cop_req(&mut self) -> &mut super::coprocessor::Request {
        if self.cop_req.is_none() {
            self.cop_req.set_default();
        };
        self.cop_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cop_req(&mut self) -> super::coprocessor::Request {
        self.cop_req.take().unwrap_or_else(|| super::coprocessor::Request::new())
    }

    pub fn get_cop_req(&self) -> &super::coprocessor::Request {
        self.cop_req.as_ref().unwrap_or_else(|| super::coprocessor::Request::default_instance())
    }

    // optional .coprocessor.Response cop_resp = 8;

    pub fn clear_cop_resp(&mut self) {
        self.cop_resp.clear();
    }

    pub fn has_cop_resp(&self) -> bool {
        self.cop_resp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cop_resp(&mut self, v: super::coprocessor::Response) {
        self.cop_resp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cop_resp(&mut self) -> &mut super::coprocessor::Response {
        if self.cop_resp.is_none() {
            self.cop_resp.set_default();
        };
        self.cop_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cop_resp(&mut self) -> super::coprocessor::Response {
        self.cop_resp.take().unwrap_or_else(|| super::coprocessor::Response::new())
    }

    pub fn get_cop_resp(&self) -> &super::coprocessor::Response {
        self.cop_resp.as_ref().unwrap_or_else(|| super::coprocessor::Response::default_instance())
    }

    // optional .pdpb.Request pd_req = 9;

    pub fn clear_pd_req(&mut self) {
        self.pd_req.clear();
    }

    pub fn has_pd_req(&self) -> bool {
        self.pd_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pd_req(&mut self, v: super::pdpb::Request) {
        self.pd_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pd_req(&mut self) -> &mut super::pdpb::Request {
        if self.pd_req.is_none() {
            self.pd_req.set_default();
        };
        self.pd_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_pd_req(&mut self) -> super::pdpb::Request {
        self.pd_req.take().unwrap_or_else(|| super::pdpb::Request::new())
    }

    pub fn get_pd_req(&self) -> &super::pdpb::Request {
        self.pd_req.as_ref().unwrap_or_else(|| super::pdpb::Request::default_instance())
    }

    // optional .pdpb.Response pd_resp = 10;

    pub fn clear_pd_resp(&mut self) {
        self.pd_resp.clear();
    }

    pub fn has_pd_resp(&self) -> bool {
        self.pd_resp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pd_resp(&mut self, v: super::pdpb::Response) {
        self.pd_resp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pd_resp(&mut self) -> &mut super::pdpb::Response {
        if self.pd_resp.is_none() {
            self.pd_resp.set_default();
        };
        self.pd_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_pd_resp(&mut self) -> super::pdpb::Response {
        self.pd_resp.take().unwrap_or_else(|| super::pdpb::Response::new())
    }

    pub fn get_pd_resp(&self) -> &super::pdpb::Response {
        self.pd_resp.as_ref().unwrap_or_else(|| super::pdpb::Response::default_instance())
    }
}

impl ::protobuf::Message for Message {
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
                    self.msg_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_req));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_resp));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.raft));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.kv_req));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.kv_resp));
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cop_req));
                },
                8 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cop_resp));
                },
                9 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pd_req));
                },
                10 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pd_resp));
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
        for value in self.msg_type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.cmd_req.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.cmd_resp.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.raft.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.kv_req.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.kv_resp.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.cop_req.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.cop_resp.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.pd_req.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.pd_resp.iter() {
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
        if let Some(v) = self.cmd_req.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cmd_resp.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.raft.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.kv_req.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.kv_resp.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cop_req.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cop_resp.as_ref() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.pd_req.as_ref() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.pd_resp.as_ref() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<Message>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Message {
    fn new() -> Message {
        Message::new()
    }

    fn descriptor_static(_: ::std::option::Option<Message>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "msg_type",
                    Message::has_msg_type,
                    Message::get_msg_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cmd_req",
                    Message::has_cmd_req,
                    Message::get_cmd_req,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cmd_resp",
                    Message::has_cmd_resp,
                    Message::get_cmd_resp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "raft",
                    Message::has_raft,
                    Message::get_raft,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "kv_req",
                    Message::has_kv_req,
                    Message::get_kv_req,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "kv_resp",
                    Message::has_kv_resp,
                    Message::get_kv_resp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cop_req",
                    Message::has_cop_req,
                    Message::get_cop_req,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cop_resp",
                    Message::has_cop_resp,
                    Message::get_cop_resp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pd_req",
                    Message::has_pd_req,
                    Message::get_pd_req,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "pd_resp",
                    Message::has_pd_resp,
                    Message::get_pd_resp,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Message>(
                    "Message",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Message {
    fn clear(&mut self) {
        self.clear_msg_type();
        self.clear_cmd_req();
        self.clear_cmd_resp();
        self.clear_raft();
        self.clear_kv_req();
        self.clear_kv_resp();
        self.clear_cop_req();
        self.clear_cop_resp();
        self.clear_pd_req();
        self.clear_pd_resp();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Message {
    fn eq(&self, other: &Message) -> bool {
        self.msg_type == other.msg_type &&
        self.cmd_req == other.cmd_req &&
        self.cmd_resp == other.cmd_resp &&
        self.raft == other.raft &&
        self.kv_req == other.kv_req &&
        self.kv_resp == other.kv_resp &&
        self.cop_req == other.cop_req &&
        self.cop_resp == other.cop_resp &&
        self.pd_req == other.pd_req &&
        self.pd_resp == other.pd_resp &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MessageType {
    None = 0,
    Cmd = 1,
    CmdResp = 2,
    Raft = 3,
    KvReq = 4,
    KvResp = 5,
    CopReq = 6,
    CopResp = 7,
    PdReq = 8,
    PdResp = 9,
}

impl ::protobuf::ProtobufEnum for MessageType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MessageType> {
        match value {
            0 => ::std::option::Option::Some(MessageType::None),
            1 => ::std::option::Option::Some(MessageType::Cmd),
            2 => ::std::option::Option::Some(MessageType::CmdResp),
            3 => ::std::option::Option::Some(MessageType::Raft),
            4 => ::std::option::Option::Some(MessageType::KvReq),
            5 => ::std::option::Option::Some(MessageType::KvResp),
            6 => ::std::option::Option::Some(MessageType::CopReq),
            7 => ::std::option::Option::Some(MessageType::CopResp),
            8 => ::std::option::Option::Some(MessageType::PdReq),
            9 => ::std::option::Option::Some(MessageType::PdResp),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MessageType] = &[
            MessageType::None,
            MessageType::Cmd,
            MessageType::CmdResp,
            MessageType::Raft,
            MessageType::KvReq,
            MessageType::KvResp,
            MessageType::CopReq,
            MessageType::CopResp,
            MessageType::PdReq,
            MessageType::PdResp,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<MessageType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MessageType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MessageType {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0b, 0x6d, 0x73, 0x67, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x05, 0x6d,
    0x73, 0x67, 0x70, 0x62, 0x1a, 0x10, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x13, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x73, 0x65, 0x72,
    0x76, 0x65, 0x72, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0d, 0x6b, 0x76, 0x72,
    0x70, 0x63, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x11, 0x63, 0x6f, 0x70, 0x72,
    0x6f, 0x63, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0a, 0x70,
    0x64, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x8b, 0x03, 0x0a, 0x07, 0x4d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x24, 0x0a, 0x08, 0x6d, 0x73, 0x67, 0x5f, 0x74, 0x79, 0x70,
    0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x12, 0x2e, 0x6d, 0x73, 0x67, 0x70, 0x62, 0x2e,
    0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x2b, 0x0a, 0x07, 0x63,
    0x6d, 0x64, 0x5f, 0x72, 0x65, 0x71, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x72,
    0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x52, 0x61, 0x66, 0x74, 0x43, 0x6d,
    0x64, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x2d, 0x0a, 0x08, 0x63, 0x6d, 0x64, 0x5f,
    0x72, 0x65, 0x73, 0x70, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x72, 0x61, 0x66,
    0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x52, 0x61, 0x66, 0x74, 0x43, 0x6d, 0x64, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x28, 0x0a, 0x04, 0x72, 0x61, 0x66, 0x74, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x73, 0x65, 0x72,
    0x76, 0x65, 0x72, 0x70, 0x62, 0x2e, 0x52, 0x61, 0x66, 0x74, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x12, 0x20, 0x0a, 0x06, 0x6b, 0x76, 0x5f, 0x72, 0x65, 0x71, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x10, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x22, 0x0a, 0x07, 0x6b, 0x76, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x25, 0x0a, 0x07, 0x63, 0x6f, 0x70, 0x5f, 0x72,
    0x65, 0x71, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x63, 0x6f, 0x70, 0x72, 0x6f,
    0x63, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x27,
    0x0a, 0x08, 0x63, 0x6f, 0x70, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x15, 0x2e, 0x63, 0x6f, 0x70, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x2e, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1d, 0x0a, 0x06, 0x70, 0x64, 0x5f, 0x72, 0x65,
    0x71, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1f, 0x0a, 0x07, 0x70, 0x64, 0x5f, 0x72, 0x65, 0x73,
    0x70, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2a, 0x7e, 0x0a, 0x0b, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x08, 0x0a, 0x04, 0x4e, 0x6f, 0x6e, 0x65, 0x10, 0x00,
    0x12, 0x07, 0x0a, 0x03, 0x43, 0x6d, 0x64, 0x10, 0x01, 0x12, 0x0b, 0x0a, 0x07, 0x43, 0x6d, 0x64,
    0x52, 0x65, 0x73, 0x70, 0x10, 0x02, 0x12, 0x08, 0x0a, 0x04, 0x52, 0x61, 0x66, 0x74, 0x10, 0x03,
    0x12, 0x09, 0x0a, 0x05, 0x4b, 0x76, 0x52, 0x65, 0x71, 0x10, 0x04, 0x12, 0x0a, 0x0a, 0x06, 0x4b,
    0x76, 0x52, 0x65, 0x73, 0x70, 0x10, 0x05, 0x12, 0x0a, 0x0a, 0x06, 0x43, 0x6f, 0x70, 0x52, 0x65,
    0x71, 0x10, 0x06, 0x12, 0x0b, 0x0a, 0x07, 0x43, 0x6f, 0x70, 0x52, 0x65, 0x73, 0x70, 0x10, 0x07,
    0x12, 0x09, 0x0a, 0x05, 0x50, 0x64, 0x52, 0x65, 0x71, 0x10, 0x08, 0x12, 0x0a, 0x0a, 0x06, 0x50,
    0x64, 0x52, 0x65, 0x73, 0x70, 0x10, 0x09, 0x4a, 0xfc, 0x09, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x22, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x0d, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x00, 0x12, 0x03, 0x03, 0x07, 0x19, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x04,
    0x07, 0x1c, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x05, 0x07, 0x16, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x06, 0x07, 0x1a, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x04, 0x12, 0x03,
    0x07, 0x07, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x09, 0x00, 0x14, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x09, 0x05, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x0a, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x0a, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x0a, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x04,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x04, 0x07, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0b, 0x13, 0x14, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0c, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02,
    0x02, 0x12, 0x03, 0x0c, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03,
    0x0d, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0d, 0x04,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x0d, 0x13, 0x14, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0e, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0e, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x04, 0x02, 0x12, 0x03, 0x0e, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x05,
    0x12, 0x03, 0x0f, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x0f, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x0f, 0x13,
    0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x06, 0x12, 0x03, 0x10, 0x04, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x10, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x10, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x07, 0x12, 0x03, 0x11, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x01,
    0x12, 0x03, 0x11, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x02, 0x12, 0x03,
    0x11, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x08, 0x12, 0x03, 0x12, 0x04, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x12, 0x04, 0x09, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x02, 0x12, 0x03, 0x12, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x09, 0x12, 0x03, 0x13, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x09, 0x01, 0x12, 0x03, 0x13, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09, 0x02,
    0x12, 0x03, 0x13, 0x13, 0x14, 0x0a, 0x41, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x17, 0x00, 0x22,
    0x01, 0x1a, 0x35, 0x20, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x68, 0x6f, 0x6c, 0x64,
    0x73, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x20, 0x63,
    0x6f, 0x6d, 0x6d, 0x75, 0x6e, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x77, 0x69, 0x74,
    0x68, 0x20, 0x54, 0x69, 0x4b, 0x56, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x17, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x18, 0x04,
    0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x18, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x18, 0x0d, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18, 0x29, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x18, 0x35, 0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x19, 0x04, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x19, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x19,
    0x0d, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x19, 0x29, 0x30,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x19, 0x35, 0x36, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x1a, 0x04, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x06, 0x12, 0x03, 0x1a, 0x0d, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x1a, 0x29, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x1a, 0x35, 0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x1b, 0x04, 0x37,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x1b, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x1b, 0x0d, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1b, 0x29, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x1b, 0x35, 0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04,
    0x12, 0x03, 0x1c, 0x04, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03,
    0x1c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x1c, 0x0d,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x1c, 0x29, 0x2f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x1c, 0x35, 0x36, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x1d, 0x04, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x05, 0x04, 0x12, 0x03, 0x1d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05,
    0x06, 0x12, 0x03, 0x1d, 0x0d, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x1d, 0x29, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x1d,
    0x35, 0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x1e, 0x04, 0x37, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x1e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x06, 0x06, 0x12, 0x03, 0x1e, 0x0d, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x1e, 0x29, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x06, 0x03, 0x12, 0x03, 0x1e, 0x35, 0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12,
    0x03, 0x1f, 0x04, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x1f,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x06, 0x12, 0x03, 0x1f, 0x0d, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x1f, 0x29, 0x31, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x1f, 0x35, 0x36, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x20, 0x04, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x08, 0x04, 0x12, 0x03, 0x20, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x06,
    0x12, 0x03, 0x20, 0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03,
    0x20, 0x29, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03, 0x20, 0x35,
    0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x09, 0x12, 0x03, 0x21, 0x04, 0x38, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x04, 0x12, 0x03, 0x21, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x09, 0x06, 0x12, 0x03, 0x21, 0x0d, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x09, 0x01, 0x12, 0x03, 0x21, 0x29, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09,
    0x03, 0x12, 0x03, 0x21, 0x35, 0x37,
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
