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
pub struct Null {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Null {}

impl Null {
    pub fn new() -> Null {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Null {
        static mut instance: ::protobuf::lazy::Lazy<Null> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Null,
        };
        unsafe {
            instance.get(Null::new)
        }
    }
}

impl ::protobuf::Message for Null {
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

impl ::protobuf::MessageStatic for Null {
    fn new() -> Null {
        Null::new()
    }

    fn descriptor_static(_: ::std::option::Option<Null>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Null>(
                    "Null",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Null {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Null {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Null {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Block {
    // message fields
    pub blockchain: ::std::string::String,
    pub hash: ::std::string::String,
    pub previous_hash: ::std::string::String,
    pub timestamp: u64,
    pub height: u64,
    pub nonce: ::std::string::String,
    pub merkle_root: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Block {}

impl Block {
    pub fn new() -> Block {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Block {
        static mut instance: ::protobuf::lazy::Lazy<Block> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Block,
        };
        unsafe {
            instance.get(Block::new)
        }
    }

    // string blockchain = 1;

    pub fn clear_blockchain(&mut self) {
        self.blockchain.clear();
    }

    // Param is passed by value, moved
    pub fn set_blockchain(&mut self, v: ::std::string::String) {
        self.blockchain = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blockchain(&mut self) -> &mut ::std::string::String {
        &mut self.blockchain
    }

    // Take field
    pub fn take_blockchain(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.blockchain, ::std::string::String::new())
    }

    pub fn get_blockchain(&self) -> &str {
        &self.blockchain
    }

    fn get_blockchain_for_reflect(&self) -> &::std::string::String {
        &self.blockchain
    }

    fn mut_blockchain_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.blockchain
    }

    // string hash = 2;

    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::string::String) {
        self.hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::string::String {
        &mut self.hash
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.hash, ::std::string::String::new())
    }

    pub fn get_hash(&self) -> &str {
        &self.hash
    }

    fn get_hash_for_reflect(&self) -> &::std::string::String {
        &self.hash
    }

    fn mut_hash_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.hash
    }

    // string previous_hash = 3;

    pub fn clear_previous_hash(&mut self) {
        self.previous_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_previous_hash(&mut self, v: ::std::string::String) {
        self.previous_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_previous_hash(&mut self) -> &mut ::std::string::String {
        &mut self.previous_hash
    }

    // Take field
    pub fn take_previous_hash(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.previous_hash, ::std::string::String::new())
    }

    pub fn get_previous_hash(&self) -> &str {
        &self.previous_hash
    }

    fn get_previous_hash_for_reflect(&self) -> &::std::string::String {
        &self.previous_hash
    }

    fn mut_previous_hash_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.previous_hash
    }

    // uint64 timestamp = 4;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = 0;
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u64) {
        self.timestamp = v;
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    fn get_timestamp_for_reflect(&self) -> &u64 {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut u64 {
        &mut self.timestamp
    }

    // uint64 height = 5;

    pub fn clear_height(&mut self) {
        self.height = 0;
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: u64) {
        self.height = v;
    }

    pub fn get_height(&self) -> u64 {
        self.height
    }

    fn get_height_for_reflect(&self) -> &u64 {
        &self.height
    }

    fn mut_height_for_reflect(&mut self) -> &mut u64 {
        &mut self.height
    }

    // string nonce = 6;

    pub fn clear_nonce(&mut self) {
        self.nonce.clear();
    }

    // Param is passed by value, moved
    pub fn set_nonce(&mut self, v: ::std::string::String) {
        self.nonce = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nonce(&mut self) -> &mut ::std::string::String {
        &mut self.nonce
    }

    // Take field
    pub fn take_nonce(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.nonce, ::std::string::String::new())
    }

    pub fn get_nonce(&self) -> &str {
        &self.nonce
    }

    fn get_nonce_for_reflect(&self) -> &::std::string::String {
        &self.nonce
    }

    fn mut_nonce_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.nonce
    }

    // string merkle_root = 7;

    pub fn clear_merkle_root(&mut self) {
        self.merkle_root.clear();
    }

    // Param is passed by value, moved
    pub fn set_merkle_root(&mut self, v: ::std::string::String) {
        self.merkle_root = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_merkle_root(&mut self) -> &mut ::std::string::String {
        &mut self.merkle_root
    }

    // Take field
    pub fn take_merkle_root(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.merkle_root, ::std::string::String::new())
    }

    pub fn get_merkle_root(&self) -> &str {
        &self.merkle_root
    }

    fn get_merkle_root_for_reflect(&self) -> &::std::string::String {
        &self.merkle_root
    }

    fn mut_merkle_root_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.merkle_root
    }
}

impl ::protobuf::Message for Block {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.blockchain)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.hash)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.previous_hash)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.timestamp = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.height = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.nonce)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.merkle_root)?;
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
        if !self.blockchain.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.blockchain);
        }
        if !self.hash.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.hash);
        }
        if !self.previous_hash.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.previous_hash);
        }
        if self.timestamp != 0 {
            my_size += ::protobuf::rt::value_size(4, self.timestamp, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(5, self.height, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.nonce.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.nonce);
        }
        if !self.merkle_root.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.merkle_root);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.blockchain.is_empty() {
            os.write_string(1, &self.blockchain)?;
        }
        if !self.hash.is_empty() {
            os.write_string(2, &self.hash)?;
        }
        if !self.previous_hash.is_empty() {
            os.write_string(3, &self.previous_hash)?;
        }
        if self.timestamp != 0 {
            os.write_uint64(4, self.timestamp)?;
        }
        if self.height != 0 {
            os.write_uint64(5, self.height)?;
        }
        if !self.nonce.is_empty() {
            os.write_string(6, &self.nonce)?;
        }
        if !self.merkle_root.is_empty() {
            os.write_string(7, &self.merkle_root)?;
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

impl ::protobuf::MessageStatic for Block {
    fn new() -> Block {
        Block::new()
    }

    fn descriptor_static(_: ::std::option::Option<Block>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "blockchain",
                    Block::get_blockchain_for_reflect,
                    Block::mut_blockchain_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "hash",
                    Block::get_hash_for_reflect,
                    Block::mut_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "previous_hash",
                    Block::get_previous_hash_for_reflect,
                    Block::mut_previous_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "timestamp",
                    Block::get_timestamp_for_reflect,
                    Block::mut_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "height",
                    Block::get_height_for_reflect,
                    Block::mut_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "nonce",
                    Block::get_nonce_for_reflect,
                    Block::mut_nonce_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "merkle_root",
                    Block::get_merkle_root_for_reflect,
                    Block::mut_merkle_root_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Block>(
                    "Block",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Block {
    fn clear(&mut self) {
        self.clear_blockchain();
        self.clear_hash();
        self.clear_previous_hash();
        self.clear_timestamp();
        self.clear_height();
        self.clear_nonce();
        self.clear_merkle_root();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Block {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Block {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BcBlock {
    // message fields
    pub nonce: ::std::string::String,
    pub hash: ::std::string::String,
    pub previous_hash: ::std::string::String,
    pub merkle_root: ::std::string::String,
    pub height: u64,
    pub chain_root: ::std::string::String,
    pub other_blocks: ::protobuf::RepeatedField<Block>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BcBlock {}

impl BcBlock {
    pub fn new() -> BcBlock {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BcBlock {
        static mut instance: ::protobuf::lazy::Lazy<BcBlock> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BcBlock,
        };
        unsafe {
            instance.get(BcBlock::new)
        }
    }

    // string nonce = 1;

    pub fn clear_nonce(&mut self) {
        self.nonce.clear();
    }

    // Param is passed by value, moved
    pub fn set_nonce(&mut self, v: ::std::string::String) {
        self.nonce = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nonce(&mut self) -> &mut ::std::string::String {
        &mut self.nonce
    }

    // Take field
    pub fn take_nonce(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.nonce, ::std::string::String::new())
    }

    pub fn get_nonce(&self) -> &str {
        &self.nonce
    }

    fn get_nonce_for_reflect(&self) -> &::std::string::String {
        &self.nonce
    }

    fn mut_nonce_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.nonce
    }

    // string hash = 2;

    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::string::String) {
        self.hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::string::String {
        &mut self.hash
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.hash, ::std::string::String::new())
    }

    pub fn get_hash(&self) -> &str {
        &self.hash
    }

    fn get_hash_for_reflect(&self) -> &::std::string::String {
        &self.hash
    }

    fn mut_hash_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.hash
    }

    // string previous_hash = 3;

    pub fn clear_previous_hash(&mut self) {
        self.previous_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_previous_hash(&mut self, v: ::std::string::String) {
        self.previous_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_previous_hash(&mut self) -> &mut ::std::string::String {
        &mut self.previous_hash
    }

    // Take field
    pub fn take_previous_hash(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.previous_hash, ::std::string::String::new())
    }

    pub fn get_previous_hash(&self) -> &str {
        &self.previous_hash
    }

    fn get_previous_hash_for_reflect(&self) -> &::std::string::String {
        &self.previous_hash
    }

    fn mut_previous_hash_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.previous_hash
    }

    // string merkle_root = 4;

    pub fn clear_merkle_root(&mut self) {
        self.merkle_root.clear();
    }

    // Param is passed by value, moved
    pub fn set_merkle_root(&mut self, v: ::std::string::String) {
        self.merkle_root = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_merkle_root(&mut self) -> &mut ::std::string::String {
        &mut self.merkle_root
    }

    // Take field
    pub fn take_merkle_root(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.merkle_root, ::std::string::String::new())
    }

    pub fn get_merkle_root(&self) -> &str {
        &self.merkle_root
    }

    fn get_merkle_root_for_reflect(&self) -> &::std::string::String {
        &self.merkle_root
    }

    fn mut_merkle_root_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.merkle_root
    }

    // uint64 height = 5;

    pub fn clear_height(&mut self) {
        self.height = 0;
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: u64) {
        self.height = v;
    }

    pub fn get_height(&self) -> u64 {
        self.height
    }

    fn get_height_for_reflect(&self) -> &u64 {
        &self.height
    }

    fn mut_height_for_reflect(&mut self) -> &mut u64 {
        &mut self.height
    }

    // string chain_root = 6;

    pub fn clear_chain_root(&mut self) {
        self.chain_root.clear();
    }

    // Param is passed by value, moved
    pub fn set_chain_root(&mut self, v: ::std::string::String) {
        self.chain_root = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_chain_root(&mut self) -> &mut ::std::string::String {
        &mut self.chain_root
    }

    // Take field
    pub fn take_chain_root(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.chain_root, ::std::string::String::new())
    }

    pub fn get_chain_root(&self) -> &str {
        &self.chain_root
    }

    fn get_chain_root_for_reflect(&self) -> &::std::string::String {
        &self.chain_root
    }

    fn mut_chain_root_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.chain_root
    }

    // repeated .bc.Block other_blocks = 7;

    pub fn clear_other_blocks(&mut self) {
        self.other_blocks.clear();
    }

    // Param is passed by value, moved
    pub fn set_other_blocks(&mut self, v: ::protobuf::RepeatedField<Block>) {
        self.other_blocks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_other_blocks(&mut self) -> &mut ::protobuf::RepeatedField<Block> {
        &mut self.other_blocks
    }

    // Take field
    pub fn take_other_blocks(&mut self) -> ::protobuf::RepeatedField<Block> {
        ::std::mem::replace(&mut self.other_blocks, ::protobuf::RepeatedField::new())
    }

    pub fn get_other_blocks(&self) -> &[Block] {
        &self.other_blocks
    }

    fn get_other_blocks_for_reflect(&self) -> &::protobuf::RepeatedField<Block> {
        &self.other_blocks
    }

    fn mut_other_blocks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Block> {
        &mut self.other_blocks
    }
}

impl ::protobuf::Message for BcBlock {
    fn is_initialized(&self) -> bool {
        for v in &self.other_blocks {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.nonce)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.hash)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.previous_hash)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.merkle_root)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.height = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.chain_root)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.other_blocks)?;
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
        if !self.nonce.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.nonce);
        }
        if !self.hash.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.hash);
        }
        if !self.previous_hash.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.previous_hash);
        }
        if !self.merkle_root.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.merkle_root);
        }
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(5, self.height, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.chain_root.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.chain_root);
        }
        for value in &self.other_blocks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.nonce.is_empty() {
            os.write_string(1, &self.nonce)?;
        }
        if !self.hash.is_empty() {
            os.write_string(2, &self.hash)?;
        }
        if !self.previous_hash.is_empty() {
            os.write_string(3, &self.previous_hash)?;
        }
        if !self.merkle_root.is_empty() {
            os.write_string(4, &self.merkle_root)?;
        }
        if self.height != 0 {
            os.write_uint64(5, self.height)?;
        }
        if !self.chain_root.is_empty() {
            os.write_string(6, &self.chain_root)?;
        }
        for v in &self.other_blocks {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for BcBlock {
    fn new() -> BcBlock {
        BcBlock::new()
    }

    fn descriptor_static(_: ::std::option::Option<BcBlock>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "nonce",
                    BcBlock::get_nonce_for_reflect,
                    BcBlock::mut_nonce_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "hash",
                    BcBlock::get_hash_for_reflect,
                    BcBlock::mut_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "previous_hash",
                    BcBlock::get_previous_hash_for_reflect,
                    BcBlock::mut_previous_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "merkle_root",
                    BcBlock::get_merkle_root_for_reflect,
                    BcBlock::mut_merkle_root_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "height",
                    BcBlock::get_height_for_reflect,
                    BcBlock::mut_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "chain_root",
                    BcBlock::get_chain_root_for_reflect,
                    BcBlock::mut_chain_root_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Block>>(
                    "other_blocks",
                    BcBlock::get_other_blocks_for_reflect,
                    BcBlock::mut_other_blocks_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BcBlock>(
                    "BcBlock",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BcBlock {
    fn clear(&mut self) {
        self.clear_nonce();
        self.clear_hash();
        self.clear_previous_hash();
        self.clear_merkle_root();
        self.clear_height();
        self.clear_chain_root();
        self.clear_other_blocks();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BcBlock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BcBlock {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\ncore.proto\x12\x02bc\"\x06\n\x04Null\"\xcd\x01\n\x05Block\x12\x1e\n\
    \nblockchain\x18\x01\x20\x01(\tR\nblockchain\x12\x12\n\x04hash\x18\x02\
    \x20\x01(\tR\x04hash\x12#\n\rprevious_hash\x18\x03\x20\x01(\tR\x0cprevio\
    usHash\x12\x1c\n\ttimestamp\x18\x04\x20\x01(\x04R\ttimestamp\x12\x16\n\
    \x06height\x18\x05\x20\x01(\x04R\x06height\x12\x14\n\x05nonce\x18\x06\
    \x20\x01(\tR\x05nonce\x12\x1f\n\x0bmerkle_root\x18\x07\x20\x01(\tR\nmerk\
    leRoot\"\xde\x01\n\x07BcBlock\x12\x14\n\x05nonce\x18\x01\x20\x01(\tR\x05\
    nonce\x12\x12\n\x04hash\x18\x02\x20\x01(\tR\x04hash\x12#\n\rprevious_has\
    h\x18\x03\x20\x01(\tR\x0cpreviousHash\x12\x1f\n\x0bmerkle_root\x18\x04\
    \x20\x01(\tR\nmerkleRoot\x12\x16\n\x06height\x18\x05\x20\x01(\x04R\x06he\
    ight\x12\x1d\n\nchain_root\x18\x06\x20\x01(\tR\tchainRoot\x12,\n\x0cothe\
    r_blocks\x18\x07\x20\x03(\x0b2\t.bc.BlockR\x0botherBlocksb\x06proto3\
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
