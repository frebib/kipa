// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Key {
    // message fields
    pub key_id: ::std::string::String,
    pub data: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Key {}

impl Key {
    pub fn new() -> Key {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Key {
        static mut instance: ::protobuf::lazy::Lazy<Key> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Key,
        };
        unsafe {
            instance.get(Key::new)
        }
    }

    // string key_id = 1;

    pub fn clear_key_id(&mut self) {
        self.key_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_key_id(&mut self, v: ::std::string::String) {
        self.key_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key_id(&mut self) -> &mut ::std::string::String {
        &mut self.key_id
    }

    // Take field
    pub fn take_key_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.key_id, ::std::string::String::new())
    }

    pub fn get_key_id(&self) -> &str {
        &self.key_id
    }

    fn get_key_id_for_reflect(&self) -> &::std::string::String {
        &self.key_id
    }

    fn mut_key_id_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.key_id
    }

    // bytes data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }
}

impl ::protobuf::Message for Key {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.key_id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.key_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.key_id);
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key_id.is_empty() {
            os.write_string(1, &self.key_id)?;
        }
        if !self.data.is_empty() {
            os.write_bytes(2, &self.data)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Key {
    fn new() -> Key {
        Key::new()
    }

    fn descriptor_static(_: ::std::option::Option<Key>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "key_id",
                    Key::get_key_id_for_reflect,
                    Key::mut_key_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    Key::get_data_for_reflect,
                    Key::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Key>(
                    "Key",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Key {
    fn clear(&mut self) {
        self.clear_key_id();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Key {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Key {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Address {
    // message fields
    pub ip_data: ::std::vec::Vec<u8>,
    pub port: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Address {}

impl Address {
    pub fn new() -> Address {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Address {
        static mut instance: ::protobuf::lazy::Lazy<Address> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Address,
        };
        unsafe {
            instance.get(Address::new)
        }
    }

    // bytes ip_data = 1;

    pub fn clear_ip_data(&mut self) {
        self.ip_data.clear();
    }

    // Param is passed by value, moved
    pub fn set_ip_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.ip_data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ip_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.ip_data
    }

    // Take field
    pub fn take_ip_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.ip_data, ::std::vec::Vec::new())
    }

    pub fn get_ip_data(&self) -> &[u8] {
        &self.ip_data
    }

    fn get_ip_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.ip_data
    }

    fn mut_ip_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.ip_data
    }

    // uint32 port = 2;

    pub fn clear_port(&mut self) {
        self.port = 0;
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: u32) {
        self.port = v;
    }

    pub fn get_port(&self) -> u32 {
        self.port
    }

    fn get_port_for_reflect(&self) -> &u32 {
        &self.port
    }

    fn mut_port_for_reflect(&mut self) -> &mut u32 {
        &mut self.port
    }
}

impl ::protobuf::Message for Address {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.ip_data)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.port = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.ip_data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.ip_data);
        }
        if self.port != 0 {
            my_size += ::protobuf::rt::value_size(2, self.port, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.ip_data.is_empty() {
            os.write_bytes(1, &self.ip_data)?;
        }
        if self.port != 0 {
            os.write_uint32(2, self.port)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Address {
    fn new() -> Address {
        Address::new()
    }

    fn descriptor_static(_: ::std::option::Option<Address>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "ip_data",
                    Address::get_ip_data_for_reflect,
                    Address::mut_ip_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "port",
                    Address::get_port_for_reflect,
                    Address::mut_port_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Address>(
                    "Address",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Address {
    fn clear(&mut self) {
        self.clear_ip_data();
        self.clear_port();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Address {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Address {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Node {
    // message fields
    pub key: ::protobuf::SingularPtrField<Key>,
    pub address: ::protobuf::SingularPtrField<Address>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Node {}

impl Node {
    pub fn new() -> Node {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Node {
        static mut instance: ::protobuf::lazy::Lazy<Node> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Node,
        };
        unsafe {
            instance.get(Node::new)
        }
    }

    // .Key key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: Key) {
        self.key = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut Key {
        if self.key.is_none() {
            self.key.set_default();
        }
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> Key {
        self.key.take().unwrap_or_else(|| Key::new())
    }

    pub fn get_key(&self) -> &Key {
        self.key.as_ref().unwrap_or_else(|| Key::default_instance())
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularPtrField<Key> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Key> {
        &mut self.key
    }

    // .Address address = 2;

    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    pub fn has_address(&self) -> bool {
        self.address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: Address) {
        self.address = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut Address {
        if self.address.is_none() {
            self.address.set_default();
        }
        self.address.as_mut().unwrap()
    }

    // Take field
    pub fn take_address(&mut self) -> Address {
        self.address.take().unwrap_or_else(|| Address::new())
    }

    pub fn get_address(&self) -> &Address {
        self.address.as_ref().unwrap_or_else(|| Address::default_instance())
    }

    fn get_address_for_reflect(&self) -> &::protobuf::SingularPtrField<Address> {
        &self.address
    }

    fn mut_address_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Address> {
        &mut self.address
    }
}

impl ::protobuf::Message for Node {
    fn is_initialized(&self) -> bool {
        for v in &self.key {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.address {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.address)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.key.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.address.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.key.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.address.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Node {
    fn new() -> Node {
        Node::new()
    }

    fn descriptor_static(_: ::std::option::Option<Node>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Key>>(
                    "key",
                    Node::get_key_for_reflect,
                    Node::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Address>>(
                    "address",
                    Node::get_address_for_reflect,
                    Node::mut_address_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Node>(
                    "Node",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Node {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_address();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Node {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Node {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ApiError {
    // message fields
    pub msg: ::std::string::String,
    pub error_type: ApiErrorType,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ApiError {}

impl ApiError {
    pub fn new() -> ApiError {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ApiError {
        static mut instance: ::protobuf::lazy::Lazy<ApiError> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ApiError,
        };
        unsafe {
            instance.get(ApiError::new)
        }
    }

    // string msg = 1;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_msg_for_reflect(&self) -> &::std::string::String {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // .ApiErrorType error_type = 2;

    pub fn clear_error_type(&mut self) {
        self.error_type = ApiErrorType::NoError;
    }

    // Param is passed by value, moved
    pub fn set_error_type(&mut self, v: ApiErrorType) {
        self.error_type = v;
    }

    pub fn get_error_type(&self) -> ApiErrorType {
        self.error_type
    }

    fn get_error_type_for_reflect(&self) -> &ApiErrorType {
        &self.error_type
    }

    fn mut_error_type_for_reflect(&mut self) -> &mut ApiErrorType {
        &mut self.error_type
    }
}

impl ::protobuf::Message for ApiError {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                2 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.error_type, 2, &mut self.unknown_fields)?
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.msg);
        }
        if self.error_type != ApiErrorType::NoError {
            my_size += ::protobuf::rt::enum_size(2, self.error_type);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.msg.is_empty() {
            os.write_string(1, &self.msg)?;
        }
        if self.error_type != ApiErrorType::NoError {
            os.write_enum(2, self.error_type.value())?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ApiError {
    fn new() -> ApiError {
        ApiError::new()
    }

    fn descriptor_static(_: ::std::option::Option<ApiError>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    ApiError::get_msg_for_reflect,
                    ApiError::mut_msg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ApiErrorType>>(
                    "error_type",
                    ApiError::get_error_type_for_reflect,
                    ApiError::mut_error_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ApiError>(
                    "ApiError",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ApiError {
    fn clear(&mut self) {
        self.clear_msg();
        self.clear_error_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ApiError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ApiError {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MessageSender {
    // message fields
    pub key: ::protobuf::SingularPtrField<Key>,
    pub port: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MessageSender {}

impl MessageSender {
    pub fn new() -> MessageSender {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MessageSender {
        static mut instance: ::protobuf::lazy::Lazy<MessageSender> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MessageSender,
        };
        unsafe {
            instance.get(MessageSender::new)
        }
    }

    // .Key key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: Key) {
        self.key = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut Key {
        if self.key.is_none() {
            self.key.set_default();
        }
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> Key {
        self.key.take().unwrap_or_else(|| Key::new())
    }

    pub fn get_key(&self) -> &Key {
        self.key.as_ref().unwrap_or_else(|| Key::default_instance())
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularPtrField<Key> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Key> {
        &mut self.key
    }

    // uint32 port = 2;

    pub fn clear_port(&mut self) {
        self.port = 0;
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: u32) {
        self.port = v;
    }

    pub fn get_port(&self) -> u32 {
        self.port
    }

    fn get_port_for_reflect(&self) -> &u32 {
        &self.port
    }

    fn mut_port_for_reflect(&mut self) -> &mut u32 {
        &mut self.port
    }
}

impl ::protobuf::Message for MessageSender {
    fn is_initialized(&self) -> bool {
        for v in &self.key {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.port = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.key.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.port != 0 {
            my_size += ::protobuf::rt::value_size(2, self.port, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.key.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.port != 0 {
            os.write_uint32(2, self.port)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MessageSender {
    fn new() -> MessageSender {
        MessageSender::new()
    }

    fn descriptor_static(_: ::std::option::Option<MessageSender>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Key>>(
                    "key",
                    MessageSender::get_key_for_reflect,
                    MessageSender::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "port",
                    MessageSender::get_port_for_reflect,
                    MessageSender::mut_port_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MessageSender>(
                    "MessageSender",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MessageSender {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_port();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MessageSender {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MessageSender {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GeneralRequest {
    // message fields
    pub query_request: ::protobuf::SingularPtrField<QueryRequest>,
    pub search_request: ::protobuf::SingularPtrField<SearchRequest>,
    pub connect_request: ::protobuf::SingularPtrField<ConnectRequest>,
    pub list_neighbours_request: ::protobuf::SingularPtrField<ListNeighboursRequest>,
    pub sender: ::protobuf::SingularPtrField<MessageSender>,
    pub id: u32,
    pub version: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GeneralRequest {}

impl GeneralRequest {
    pub fn new() -> GeneralRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GeneralRequest {
        static mut instance: ::protobuf::lazy::Lazy<GeneralRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GeneralRequest,
        };
        unsafe {
            instance.get(GeneralRequest::new)
        }
    }

    // .QueryRequest query_request = 1;

    pub fn clear_query_request(&mut self) {
        self.query_request.clear();
    }

    pub fn has_query_request(&self) -> bool {
        self.query_request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_query_request(&mut self, v: QueryRequest) {
        self.query_request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_query_request(&mut self) -> &mut QueryRequest {
        if self.query_request.is_none() {
            self.query_request.set_default();
        }
        self.query_request.as_mut().unwrap()
    }

    // Take field
    pub fn take_query_request(&mut self) -> QueryRequest {
        self.query_request.take().unwrap_or_else(|| QueryRequest::new())
    }

    pub fn get_query_request(&self) -> &QueryRequest {
        self.query_request.as_ref().unwrap_or_else(|| QueryRequest::default_instance())
    }

    fn get_query_request_for_reflect(&self) -> &::protobuf::SingularPtrField<QueryRequest> {
        &self.query_request
    }

    fn mut_query_request_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<QueryRequest> {
        &mut self.query_request
    }

    // .SearchRequest search_request = 2;

    pub fn clear_search_request(&mut self) {
        self.search_request.clear();
    }

    pub fn has_search_request(&self) -> bool {
        self.search_request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_search_request(&mut self, v: SearchRequest) {
        self.search_request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_search_request(&mut self) -> &mut SearchRequest {
        if self.search_request.is_none() {
            self.search_request.set_default();
        }
        self.search_request.as_mut().unwrap()
    }

    // Take field
    pub fn take_search_request(&mut self) -> SearchRequest {
        self.search_request.take().unwrap_or_else(|| SearchRequest::new())
    }

    pub fn get_search_request(&self) -> &SearchRequest {
        self.search_request.as_ref().unwrap_or_else(|| SearchRequest::default_instance())
    }

    fn get_search_request_for_reflect(&self) -> &::protobuf::SingularPtrField<SearchRequest> {
        &self.search_request
    }

    fn mut_search_request_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SearchRequest> {
        &mut self.search_request
    }

    // .ConnectRequest connect_request = 3;

    pub fn clear_connect_request(&mut self) {
        self.connect_request.clear();
    }

    pub fn has_connect_request(&self) -> bool {
        self.connect_request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_connect_request(&mut self, v: ConnectRequest) {
        self.connect_request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_connect_request(&mut self) -> &mut ConnectRequest {
        if self.connect_request.is_none() {
            self.connect_request.set_default();
        }
        self.connect_request.as_mut().unwrap()
    }

    // Take field
    pub fn take_connect_request(&mut self) -> ConnectRequest {
        self.connect_request.take().unwrap_or_else(|| ConnectRequest::new())
    }

    pub fn get_connect_request(&self) -> &ConnectRequest {
        self.connect_request.as_ref().unwrap_or_else(|| ConnectRequest::default_instance())
    }

    fn get_connect_request_for_reflect(&self) -> &::protobuf::SingularPtrField<ConnectRequest> {
        &self.connect_request
    }

    fn mut_connect_request_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ConnectRequest> {
        &mut self.connect_request
    }

    // .ListNeighboursRequest list_neighbours_request = 4;

    pub fn clear_list_neighbours_request(&mut self) {
        self.list_neighbours_request.clear();
    }

    pub fn has_list_neighbours_request(&self) -> bool {
        self.list_neighbours_request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_list_neighbours_request(&mut self, v: ListNeighboursRequest) {
        self.list_neighbours_request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_list_neighbours_request(&mut self) -> &mut ListNeighboursRequest {
        if self.list_neighbours_request.is_none() {
            self.list_neighbours_request.set_default();
        }
        self.list_neighbours_request.as_mut().unwrap()
    }

    // Take field
    pub fn take_list_neighbours_request(&mut self) -> ListNeighboursRequest {
        self.list_neighbours_request.take().unwrap_or_else(|| ListNeighboursRequest::new())
    }

    pub fn get_list_neighbours_request(&self) -> &ListNeighboursRequest {
        self.list_neighbours_request.as_ref().unwrap_or_else(|| ListNeighboursRequest::default_instance())
    }

    fn get_list_neighbours_request_for_reflect(&self) -> &::protobuf::SingularPtrField<ListNeighboursRequest> {
        &self.list_neighbours_request
    }

    fn mut_list_neighbours_request_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ListNeighboursRequest> {
        &mut self.list_neighbours_request
    }

    // .MessageSender sender = 5;

    pub fn clear_sender(&mut self) {
        self.sender.clear();
    }

    pub fn has_sender(&self) -> bool {
        self.sender.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sender(&mut self, v: MessageSender) {
        self.sender = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sender(&mut self) -> &mut MessageSender {
        if self.sender.is_none() {
            self.sender.set_default();
        }
        self.sender.as_mut().unwrap()
    }

    // Take field
    pub fn take_sender(&mut self) -> MessageSender {
        self.sender.take().unwrap_or_else(|| MessageSender::new())
    }

    pub fn get_sender(&self) -> &MessageSender {
        self.sender.as_ref().unwrap_or_else(|| MessageSender::default_instance())
    }

    fn get_sender_for_reflect(&self) -> &::protobuf::SingularPtrField<MessageSender> {
        &self.sender
    }

    fn mut_sender_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<MessageSender> {
        &mut self.sender
    }

    // uint32 id = 6;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = v;
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &u32 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.id
    }

    // string version = 7;

    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: ::std::string::String) {
        self.version = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version(&mut self) -> &mut ::std::string::String {
        &mut self.version
    }

    // Take field
    pub fn take_version(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.version, ::std::string::String::new())
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    fn get_version_for_reflect(&self) -> &::std::string::String {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.version
    }
}

impl ::protobuf::Message for GeneralRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.query_request {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.search_request {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.connect_request {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.list_neighbours_request {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.sender {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.query_request)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.search_request)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.connect_request)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.list_neighbours_request)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.sender)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.id = tmp;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.version)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.query_request.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.search_request.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.connect_request.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.list_neighbours_request.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.sender.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(6, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.version.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.version);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.query_request.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.search_request.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.connect_request.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.list_neighbours_request.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.sender.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.id != 0 {
            os.write_uint32(6, self.id)?;
        }
        if !self.version.is_empty() {
            os.write_string(7, &self.version)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GeneralRequest {
    fn new() -> GeneralRequest {
        GeneralRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GeneralRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<QueryRequest>>(
                    "query_request",
                    GeneralRequest::get_query_request_for_reflect,
                    GeneralRequest::mut_query_request_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SearchRequest>>(
                    "search_request",
                    GeneralRequest::get_search_request_for_reflect,
                    GeneralRequest::mut_search_request_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ConnectRequest>>(
                    "connect_request",
                    GeneralRequest::get_connect_request_for_reflect,
                    GeneralRequest::mut_connect_request_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ListNeighboursRequest>>(
                    "list_neighbours_request",
                    GeneralRequest::get_list_neighbours_request_for_reflect,
                    GeneralRequest::mut_list_neighbours_request_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MessageSender>>(
                    "sender",
                    GeneralRequest::get_sender_for_reflect,
                    GeneralRequest::mut_sender_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "id",
                    GeneralRequest::get_id_for_reflect,
                    GeneralRequest::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "version",
                    GeneralRequest::get_version_for_reflect,
                    GeneralRequest::mut_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GeneralRequest>(
                    "GeneralRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GeneralRequest {
    fn clear(&mut self) {
        self.clear_query_request();
        self.clear_search_request();
        self.clear_connect_request();
        self.clear_list_neighbours_request();
        self.clear_sender();
        self.clear_id();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GeneralRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GeneralRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GeneralResponse {
    // message fields
    pub query_response: ::protobuf::SingularPtrField<QueryResponse>,
    pub search_response: ::protobuf::SingularPtrField<SearchResponse>,
    pub connect_response: ::protobuf::SingularPtrField<ConnectResponse>,
    pub list_neighbours_response: ::protobuf::SingularPtrField<ListNeighboursResponse>,
    pub api_error: ::protobuf::SingularPtrField<ApiError>,
    pub sender: ::protobuf::SingularPtrField<MessageSender>,
    pub id: u32,
    pub version: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GeneralResponse {}

impl GeneralResponse {
    pub fn new() -> GeneralResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GeneralResponse {
        static mut instance: ::protobuf::lazy::Lazy<GeneralResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GeneralResponse,
        };
        unsafe {
            instance.get(GeneralResponse::new)
        }
    }

    // .QueryResponse query_response = 1;

    pub fn clear_query_response(&mut self) {
        self.query_response.clear();
    }

    pub fn has_query_response(&self) -> bool {
        self.query_response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_query_response(&mut self, v: QueryResponse) {
        self.query_response = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_query_response(&mut self) -> &mut QueryResponse {
        if self.query_response.is_none() {
            self.query_response.set_default();
        }
        self.query_response.as_mut().unwrap()
    }

    // Take field
    pub fn take_query_response(&mut self) -> QueryResponse {
        self.query_response.take().unwrap_or_else(|| QueryResponse::new())
    }

    pub fn get_query_response(&self) -> &QueryResponse {
        self.query_response.as_ref().unwrap_or_else(|| QueryResponse::default_instance())
    }

    fn get_query_response_for_reflect(&self) -> &::protobuf::SingularPtrField<QueryResponse> {
        &self.query_response
    }

    fn mut_query_response_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<QueryResponse> {
        &mut self.query_response
    }

    // .SearchResponse search_response = 2;

    pub fn clear_search_response(&mut self) {
        self.search_response.clear();
    }

    pub fn has_search_response(&self) -> bool {
        self.search_response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_search_response(&mut self, v: SearchResponse) {
        self.search_response = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_search_response(&mut self) -> &mut SearchResponse {
        if self.search_response.is_none() {
            self.search_response.set_default();
        }
        self.search_response.as_mut().unwrap()
    }

    // Take field
    pub fn take_search_response(&mut self) -> SearchResponse {
        self.search_response.take().unwrap_or_else(|| SearchResponse::new())
    }

    pub fn get_search_response(&self) -> &SearchResponse {
        self.search_response.as_ref().unwrap_or_else(|| SearchResponse::default_instance())
    }

    fn get_search_response_for_reflect(&self) -> &::protobuf::SingularPtrField<SearchResponse> {
        &self.search_response
    }

    fn mut_search_response_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SearchResponse> {
        &mut self.search_response
    }

    // .ConnectResponse connect_response = 3;

    pub fn clear_connect_response(&mut self) {
        self.connect_response.clear();
    }

    pub fn has_connect_response(&self) -> bool {
        self.connect_response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_connect_response(&mut self, v: ConnectResponse) {
        self.connect_response = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_connect_response(&mut self) -> &mut ConnectResponse {
        if self.connect_response.is_none() {
            self.connect_response.set_default();
        }
        self.connect_response.as_mut().unwrap()
    }

    // Take field
    pub fn take_connect_response(&mut self) -> ConnectResponse {
        self.connect_response.take().unwrap_or_else(|| ConnectResponse::new())
    }

    pub fn get_connect_response(&self) -> &ConnectResponse {
        self.connect_response.as_ref().unwrap_or_else(|| ConnectResponse::default_instance())
    }

    fn get_connect_response_for_reflect(&self) -> &::protobuf::SingularPtrField<ConnectResponse> {
        &self.connect_response
    }

    fn mut_connect_response_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ConnectResponse> {
        &mut self.connect_response
    }

    // .ListNeighboursResponse list_neighbours_response = 4;

    pub fn clear_list_neighbours_response(&mut self) {
        self.list_neighbours_response.clear();
    }

    pub fn has_list_neighbours_response(&self) -> bool {
        self.list_neighbours_response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_list_neighbours_response(&mut self, v: ListNeighboursResponse) {
        self.list_neighbours_response = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_list_neighbours_response(&mut self) -> &mut ListNeighboursResponse {
        if self.list_neighbours_response.is_none() {
            self.list_neighbours_response.set_default();
        }
        self.list_neighbours_response.as_mut().unwrap()
    }

    // Take field
    pub fn take_list_neighbours_response(&mut self) -> ListNeighboursResponse {
        self.list_neighbours_response.take().unwrap_or_else(|| ListNeighboursResponse::new())
    }

    pub fn get_list_neighbours_response(&self) -> &ListNeighboursResponse {
        self.list_neighbours_response.as_ref().unwrap_or_else(|| ListNeighboursResponse::default_instance())
    }

    fn get_list_neighbours_response_for_reflect(&self) -> &::protobuf::SingularPtrField<ListNeighboursResponse> {
        &self.list_neighbours_response
    }

    fn mut_list_neighbours_response_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ListNeighboursResponse> {
        &mut self.list_neighbours_response
    }

    // .ApiError api_error = 5;

    pub fn clear_api_error(&mut self) {
        self.api_error.clear();
    }

    pub fn has_api_error(&self) -> bool {
        self.api_error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_api_error(&mut self, v: ApiError) {
        self.api_error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_api_error(&mut self) -> &mut ApiError {
        if self.api_error.is_none() {
            self.api_error.set_default();
        }
        self.api_error.as_mut().unwrap()
    }

    // Take field
    pub fn take_api_error(&mut self) -> ApiError {
        self.api_error.take().unwrap_or_else(|| ApiError::new())
    }

    pub fn get_api_error(&self) -> &ApiError {
        self.api_error.as_ref().unwrap_or_else(|| ApiError::default_instance())
    }

    fn get_api_error_for_reflect(&self) -> &::protobuf::SingularPtrField<ApiError> {
        &self.api_error
    }

    fn mut_api_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ApiError> {
        &mut self.api_error
    }

    // .MessageSender sender = 6;

    pub fn clear_sender(&mut self) {
        self.sender.clear();
    }

    pub fn has_sender(&self) -> bool {
        self.sender.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sender(&mut self, v: MessageSender) {
        self.sender = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sender(&mut self) -> &mut MessageSender {
        if self.sender.is_none() {
            self.sender.set_default();
        }
        self.sender.as_mut().unwrap()
    }

    // Take field
    pub fn take_sender(&mut self) -> MessageSender {
        self.sender.take().unwrap_or_else(|| MessageSender::new())
    }

    pub fn get_sender(&self) -> &MessageSender {
        self.sender.as_ref().unwrap_or_else(|| MessageSender::default_instance())
    }

    fn get_sender_for_reflect(&self) -> &::protobuf::SingularPtrField<MessageSender> {
        &self.sender
    }

    fn mut_sender_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<MessageSender> {
        &mut self.sender
    }

    // uint32 id = 7;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = v;
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &u32 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.id
    }

    // string version = 8;

    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: ::std::string::String) {
        self.version = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version(&mut self) -> &mut ::std::string::String {
        &mut self.version
    }

    // Take field
    pub fn take_version(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.version, ::std::string::String::new())
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    fn get_version_for_reflect(&self) -> &::std::string::String {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.version
    }
}

impl ::protobuf::Message for GeneralResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.query_response {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.search_response {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.connect_response {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.list_neighbours_response {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.api_error {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.sender {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.query_response)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.search_response)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.connect_response)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.list_neighbours_response)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.api_error)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.sender)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.id = tmp;
                },
                8 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.version)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.query_response.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.search_response.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.connect_response.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.list_neighbours_response.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.api_error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.sender.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(7, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.version.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.version);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.query_response.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.search_response.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.connect_response.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.list_neighbours_response.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.api_error.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.sender.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.id != 0 {
            os.write_uint32(7, self.id)?;
        }
        if !self.version.is_empty() {
            os.write_string(8, &self.version)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GeneralResponse {
    fn new() -> GeneralResponse {
        GeneralResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GeneralResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<QueryResponse>>(
                    "query_response",
                    GeneralResponse::get_query_response_for_reflect,
                    GeneralResponse::mut_query_response_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SearchResponse>>(
                    "search_response",
                    GeneralResponse::get_search_response_for_reflect,
                    GeneralResponse::mut_search_response_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ConnectResponse>>(
                    "connect_response",
                    GeneralResponse::get_connect_response_for_reflect,
                    GeneralResponse::mut_connect_response_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ListNeighboursResponse>>(
                    "list_neighbours_response",
                    GeneralResponse::get_list_neighbours_response_for_reflect,
                    GeneralResponse::mut_list_neighbours_response_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ApiError>>(
                    "api_error",
                    GeneralResponse::get_api_error_for_reflect,
                    GeneralResponse::mut_api_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MessageSender>>(
                    "sender",
                    GeneralResponse::get_sender_for_reflect,
                    GeneralResponse::mut_sender_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "id",
                    GeneralResponse::get_id_for_reflect,
                    GeneralResponse::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "version",
                    GeneralResponse::get_version_for_reflect,
                    GeneralResponse::mut_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GeneralResponse>(
                    "GeneralResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GeneralResponse {
    fn clear(&mut self) {
        self.clear_query_response();
        self.clear_search_response();
        self.clear_connect_response();
        self.clear_list_neighbours_response();
        self.clear_api_error();
        self.clear_sender();
        self.clear_id();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GeneralResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GeneralResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct QueryRequest {
    // message fields
    pub key: ::protobuf::SingularPtrField<Key>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for QueryRequest {}

impl QueryRequest {
    pub fn new() -> QueryRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static QueryRequest {
        static mut instance: ::protobuf::lazy::Lazy<QueryRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const QueryRequest,
        };
        unsafe {
            instance.get(QueryRequest::new)
        }
    }

    // .Key key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: Key) {
        self.key = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut Key {
        if self.key.is_none() {
            self.key.set_default();
        }
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> Key {
        self.key.take().unwrap_or_else(|| Key::new())
    }

    pub fn get_key(&self) -> &Key {
        self.key.as_ref().unwrap_or_else(|| Key::default_instance())
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularPtrField<Key> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Key> {
        &mut self.key
    }
}

impl ::protobuf::Message for QueryRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.key {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.key)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.key.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.key.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for QueryRequest {
    fn new() -> QueryRequest {
        QueryRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<QueryRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Key>>(
                    "key",
                    QueryRequest::get_key_for_reflect,
                    QueryRequest::mut_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<QueryRequest>(
                    "QueryRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for QueryRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for QueryRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueryRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct QueryResponse {
    // message fields
    pub nodes: ::protobuf::RepeatedField<Node>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for QueryResponse {}

impl QueryResponse {
    pub fn new() -> QueryResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static QueryResponse {
        static mut instance: ::protobuf::lazy::Lazy<QueryResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const QueryResponse,
        };
        unsafe {
            instance.get(QueryResponse::new)
        }
    }

    // repeated .Node nodes = 1;

    pub fn clear_nodes(&mut self) {
        self.nodes.clear();
    }

    // Param is passed by value, moved
    pub fn set_nodes(&mut self, v: ::protobuf::RepeatedField<Node>) {
        self.nodes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_nodes(&mut self) -> &mut ::protobuf::RepeatedField<Node> {
        &mut self.nodes
    }

    // Take field
    pub fn take_nodes(&mut self) -> ::protobuf::RepeatedField<Node> {
        ::std::mem::replace(&mut self.nodes, ::protobuf::RepeatedField::new())
    }

    pub fn get_nodes(&self) -> &[Node] {
        &self.nodes
    }

    fn get_nodes_for_reflect(&self) -> &::protobuf::RepeatedField<Node> {
        &self.nodes
    }

    fn mut_nodes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Node> {
        &mut self.nodes
    }
}

impl ::protobuf::Message for QueryResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.nodes {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.nodes)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.nodes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.nodes {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for QueryResponse {
    fn new() -> QueryResponse {
        QueryResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<QueryResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Node>>(
                    "nodes",
                    QueryResponse::get_nodes_for_reflect,
                    QueryResponse::mut_nodes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<QueryResponse>(
                    "QueryResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for QueryResponse {
    fn clear(&mut self) {
        self.clear_nodes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for QueryResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueryResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SearchRequest {
    // message fields
    pub key: ::protobuf::SingularPtrField<Key>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SearchRequest {}

impl SearchRequest {
    pub fn new() -> SearchRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SearchRequest {
        static mut instance: ::protobuf::lazy::Lazy<SearchRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SearchRequest,
        };
        unsafe {
            instance.get(SearchRequest::new)
        }
    }

    // .Key key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: Key) {
        self.key = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut Key {
        if self.key.is_none() {
            self.key.set_default();
        }
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> Key {
        self.key.take().unwrap_or_else(|| Key::new())
    }

    pub fn get_key(&self) -> &Key {
        self.key.as_ref().unwrap_or_else(|| Key::default_instance())
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularPtrField<Key> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Key> {
        &mut self.key
    }
}

impl ::protobuf::Message for SearchRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.key {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.key)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.key.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.key.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SearchRequest {
    fn new() -> SearchRequest {
        SearchRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<SearchRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Key>>(
                    "key",
                    SearchRequest::get_key_for_reflect,
                    SearchRequest::mut_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SearchRequest>(
                    "SearchRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SearchRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SearchRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SearchRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SearchResponse {
    // message fields
    pub node: ::protobuf::SingularPtrField<Node>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SearchResponse {}

impl SearchResponse {
    pub fn new() -> SearchResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SearchResponse {
        static mut instance: ::protobuf::lazy::Lazy<SearchResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SearchResponse,
        };
        unsafe {
            instance.get(SearchResponse::new)
        }
    }

    // .Node node = 1;

    pub fn clear_node(&mut self) {
        self.node.clear();
    }

    pub fn has_node(&self) -> bool {
        self.node.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node(&mut self, v: Node) {
        self.node = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_node(&mut self) -> &mut Node {
        if self.node.is_none() {
            self.node.set_default();
        }
        self.node.as_mut().unwrap()
    }

    // Take field
    pub fn take_node(&mut self) -> Node {
        self.node.take().unwrap_or_else(|| Node::new())
    }

    pub fn get_node(&self) -> &Node {
        self.node.as_ref().unwrap_or_else(|| Node::default_instance())
    }

    fn get_node_for_reflect(&self) -> &::protobuf::SingularPtrField<Node> {
        &self.node
    }

    fn mut_node_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Node> {
        &mut self.node
    }
}

impl ::protobuf::Message for SearchResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.node {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.node)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.node.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.node.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SearchResponse {
    fn new() -> SearchResponse {
        SearchResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SearchResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Node>>(
                    "node",
                    SearchResponse::get_node_for_reflect,
                    SearchResponse::mut_node_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SearchResponse>(
                    "SearchResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SearchResponse {
    fn clear(&mut self) {
        self.clear_node();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SearchResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SearchResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ConnectRequest {
    // message fields
    pub node: ::protobuf::SingularPtrField<Node>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ConnectRequest {}

impl ConnectRequest {
    pub fn new() -> ConnectRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ConnectRequest {
        static mut instance: ::protobuf::lazy::Lazy<ConnectRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ConnectRequest,
        };
        unsafe {
            instance.get(ConnectRequest::new)
        }
    }

    // .Node node = 1;

    pub fn clear_node(&mut self) {
        self.node.clear();
    }

    pub fn has_node(&self) -> bool {
        self.node.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node(&mut self, v: Node) {
        self.node = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_node(&mut self) -> &mut Node {
        if self.node.is_none() {
            self.node.set_default();
        }
        self.node.as_mut().unwrap()
    }

    // Take field
    pub fn take_node(&mut self) -> Node {
        self.node.take().unwrap_or_else(|| Node::new())
    }

    pub fn get_node(&self) -> &Node {
        self.node.as_ref().unwrap_or_else(|| Node::default_instance())
    }

    fn get_node_for_reflect(&self) -> &::protobuf::SingularPtrField<Node> {
        &self.node
    }

    fn mut_node_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Node> {
        &mut self.node
    }
}

impl ::protobuf::Message for ConnectRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.node {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.node)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.node.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.node.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ConnectRequest {
    fn new() -> ConnectRequest {
        ConnectRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ConnectRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Node>>(
                    "node",
                    ConnectRequest::get_node_for_reflect,
                    ConnectRequest::mut_node_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ConnectRequest>(
                    "ConnectRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ConnectRequest {
    fn clear(&mut self) {
        self.clear_node();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ConnectRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConnectRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ConnectResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ConnectResponse {}

impl ConnectResponse {
    pub fn new() -> ConnectResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ConnectResponse {
        static mut instance: ::protobuf::lazy::Lazy<ConnectResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ConnectResponse,
        };
        unsafe {
            instance.get(ConnectResponse::new)
        }
    }
}

impl ::protobuf::Message for ConnectResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ConnectResponse {
    fn new() -> ConnectResponse {
        ConnectResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ConnectResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ConnectResponse>(
                    "ConnectResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ConnectResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ConnectResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConnectResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListNeighboursRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListNeighboursRequest {}

impl ListNeighboursRequest {
    pub fn new() -> ListNeighboursRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListNeighboursRequest {
        static mut instance: ::protobuf::lazy::Lazy<ListNeighboursRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListNeighboursRequest,
        };
        unsafe {
            instance.get(ListNeighboursRequest::new)
        }
    }
}

impl ::protobuf::Message for ListNeighboursRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ListNeighboursRequest {
    fn new() -> ListNeighboursRequest {
        ListNeighboursRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListNeighboursRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ListNeighboursRequest>(
                    "ListNeighboursRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListNeighboursRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListNeighboursRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListNeighboursRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListNeighboursResponse {
    // message fields
    pub nodes: ::protobuf::RepeatedField<Node>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListNeighboursResponse {}

impl ListNeighboursResponse {
    pub fn new() -> ListNeighboursResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListNeighboursResponse {
        static mut instance: ::protobuf::lazy::Lazy<ListNeighboursResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListNeighboursResponse,
        };
        unsafe {
            instance.get(ListNeighboursResponse::new)
        }
    }

    // repeated .Node nodes = 1;

    pub fn clear_nodes(&mut self) {
        self.nodes.clear();
    }

    // Param is passed by value, moved
    pub fn set_nodes(&mut self, v: ::protobuf::RepeatedField<Node>) {
        self.nodes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_nodes(&mut self) -> &mut ::protobuf::RepeatedField<Node> {
        &mut self.nodes
    }

    // Take field
    pub fn take_nodes(&mut self) -> ::protobuf::RepeatedField<Node> {
        ::std::mem::replace(&mut self.nodes, ::protobuf::RepeatedField::new())
    }

    pub fn get_nodes(&self) -> &[Node] {
        &self.nodes
    }

    fn get_nodes_for_reflect(&self) -> &::protobuf::RepeatedField<Node> {
        &self.nodes
    }

    fn mut_nodes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Node> {
        &mut self.nodes
    }
}

impl ::protobuf::Message for ListNeighboursResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.nodes {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.nodes)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.nodes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.nodes {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ListNeighboursResponse {
    fn new() -> ListNeighboursResponse {
        ListNeighboursResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListNeighboursResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Node>>(
                    "nodes",
                    ListNeighboursResponse::get_nodes_for_reflect,
                    ListNeighboursResponse::mut_nodes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListNeighboursResponse>(
                    "ListNeighboursResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListNeighboursResponse {
    fn clear(&mut self) {
        self.clear_nodes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListNeighboursResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListNeighboursResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ApiErrorType {
    NoError = 0,
    Parse = 1,
    Configuration = 2,
    External = 3,
    Internal = 4,
}

impl ::protobuf::ProtobufEnum for ApiErrorType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ApiErrorType> {
        match value {
            0 => ::std::option::Option::Some(ApiErrorType::NoError),
            1 => ::std::option::Option::Some(ApiErrorType::Parse),
            2 => ::std::option::Option::Some(ApiErrorType::Configuration),
            3 => ::std::option::Option::Some(ApiErrorType::External),
            4 => ::std::option::Option::Some(ApiErrorType::Internal),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ApiErrorType] = &[
            ApiErrorType::NoError,
            ApiErrorType::Parse,
            ApiErrorType::Configuration,
            ApiErrorType::External,
            ApiErrorType::Internal,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ApiErrorType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ApiErrorType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ApiErrorType {
}

impl ::std::default::Default for ApiErrorType {
    fn default() -> Self {
        ApiErrorType::NoError
    }
}

impl ::protobuf::reflect::ProtobufValue for ApiErrorType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fproto_api.proto\"0\n\x03Key\x12\x15\n\x06key_id\x18\x01\x20\x01(\t\
    R\x05keyId\x12\x12\n\x04data\x18\x02\x20\x01(\x0cR\x04data\"6\n\x07Addre\
    ss\x12\x17\n\x07ip_data\x18\x01\x20\x01(\x0cR\x06ipData\x12\x12\n\x04por\
    t\x18\x02\x20\x01(\rR\x04port\"B\n\x04Node\x12\x16\n\x03key\x18\x01\x20\
    \x01(\x0b2\x04.KeyR\x03key\x12\"\n\x07address\x18\x02\x20\x01(\x0b2\x08.\
    AddressR\x07address\"J\n\x08ApiError\x12\x10\n\x03msg\x18\x01\x20\x01(\t\
    R\x03msg\x12,\n\nerror_type\x18\x02\x20\x01(\x0e2\r.ApiErrorTypeR\terror\
    Type\";\n\rMessageSender\x12\x16\n\x03key\x18\x01\x20\x01(\x0b2\x04.KeyR\
    \x03key\x12\x12\n\x04port\x18\x02\x20\x01(\rR\x04port\"\xd7\x02\n\x0eGen\
    eralRequest\x122\n\rquery_request\x18\x01\x20\x01(\x0b2\r.QueryRequestR\
    \x0cqueryRequest\x125\n\x0esearch_request\x18\x02\x20\x01(\x0b2\x0e.Sear\
    chRequestR\rsearchRequest\x128\n\x0fconnect_request\x18\x03\x20\x01(\x0b\
    2\x0f.ConnectRequestR\x0econnectRequest\x12N\n\x17list_neighbours_reques\
    t\x18\x04\x20\x01(\x0b2\x16.ListNeighboursRequestR\x15listNeighboursRequ\
    est\x12&\n\x06sender\x18\x05\x20\x01(\x0b2\x0e.MessageSenderR\x06sender\
    \x12\x0e\n\x02id\x18\x06\x20\x01(\rR\x02id\x12\x18\n\x07version\x18\x07\
    \x20\x01(\tR\x07version\"\x8c\x03\n\x0fGeneralResponse\x125\n\x0equery_r\
    esponse\x18\x01\x20\x01(\x0b2\x0e.QueryResponseR\rqueryResponse\x128\n\
    \x0fsearch_response\x18\x02\x20\x01(\x0b2\x0f.SearchResponseR\x0esearchR\
    esponse\x12;\n\x10connect_response\x18\x03\x20\x01(\x0b2\x10.ConnectResp\
    onseR\x0fconnectResponse\x12Q\n\x18list_neighbours_response\x18\x04\x20\
    \x01(\x0b2\x17.ListNeighboursResponseR\x16listNeighboursResponse\x12&\n\
    \tapi_error\x18\x05\x20\x01(\x0b2\t.ApiErrorR\x08apiError\x12&\n\x06send\
    er\x18\x06\x20\x01(\x0b2\x0e.MessageSenderR\x06sender\x12\x0e\n\x02id\
    \x18\x07\x20\x01(\rR\x02id\x12\x18\n\x07version\x18\x08\x20\x01(\tR\x07v\
    ersion\"&\n\x0cQueryRequest\x12\x16\n\x03key\x18\x01\x20\x01(\x0b2\x04.K\
    eyR\x03key\",\n\rQueryResponse\x12\x1b\n\x05nodes\x18\x01\x20\x03(\x0b2\
    \x05.NodeR\x05nodes\"'\n\rSearchRequest\x12\x16\n\x03key\x18\x01\x20\x01\
    (\x0b2\x04.KeyR\x03key\"+\n\x0eSearchResponse\x12\x19\n\x04node\x18\x01\
    \x20\x01(\x0b2\x05.NodeR\x04node\"+\n\x0eConnectRequest\x12\x19\n\x04nod\
    e\x18\x01\x20\x01(\x0b2\x05.NodeR\x04node\"\x11\n\x0fConnectResponse\"\
    \x17\n\x15ListNeighboursRequest\"5\n\x16ListNeighboursResponse\x12\x1b\n\
    \x05nodes\x18\x01\x20\x03(\x0b2\x05.NodeR\x05nodes*U\n\x0cApiErrorType\
    \x12\x0b\n\x07NoError\x10\0\x12\t\n\x05Parse\x10\x01\x12\x11\n\rConfigur\
    ation\x10\x02\x12\x0c\n\x08External\x10\x03\x12\x0c\n\x08Internal\x10\
    \x04b\x06proto3\
";

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
