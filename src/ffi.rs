#![allow(missing_docs)]
use std::os::raw;

/// A port is used to send or receive inter-isolate messages
pub type DartPort = i64;

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DartTypedDataType {
    ByteData = 0,
    Int8 = 1,
    Uint8 = 2,
    Uint8Clamped = 3,
    Int16 = 4,
    Uint16 = 5,
    Int32 = 6,
    Uint32 = 7,
    Int64 = 8,
    Uint64 = 9,
    Float32 = 10,
    Float64 = 11,
    Float32x4 = 12,
    Invalid = 13,
}

/// A Dart_CObject is used for representing Dart objects as native C
/// data outside the Dart heap. These objects are totally detached from
/// the Dart heap. Only a subset of the Dart objects have a
/// representation as a Dart_CObject.
///
/// The string encoding in the 'value.as_string' is UTF-8.
///
/// All the different types from dart:typed_data are exposed as type
/// kTypedData. The specific type from dart:typed_data is in the type
/// field of the as_typed_data structure. The length in the
/// as_typed_data structure is always in bytes.
///
/// The data for kTypedData is copied on message send and ownership remains with
/// the caller. The ownership of data for kExternalTyped is passed to the VM on
/// message send and returned when the VM invokes the
/// Dart_WeakPersistentHandleFinalizer callback; a non-NULL callback must be
/// provided.
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DartCObjectType {
    DartNull = 0,
    DartBool = 1,
    DartInt32 = 2,
    DartInt64 = 3,
    DartDouble = 4,
    DartString = 5,
    DartArray = 6,
    DartTypedData = 7,
    DartExternalTypedData = 8,
    DartSendPort = 9,
    DartCapability = 10,
    DartUnsupported = 11,
    DartNumberOfTypes = 12,
}

#[allow(missing_debug_implementations)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DartCObject {
    pub ty: DartCObjectType,
    pub value: DartCObjectValue,
}

#[allow(missing_debug_implementations)]
#[repr(C)]
#[derive(Copy, Clone)]
pub union DartCObjectValue {
    pub as_bool: bool,
    pub as_int32: i32,
    pub as_int64: i64,
    pub as_double: f64,
    pub as_string: *mut raw::c_char,
    pub as_send_port: DartNativeSendPort,
    pub as_capability: DartNativeCapability,
    pub as_array: DartNativeArray,
    pub as_typed_data: DartNativeTypedData,
    _bindgen_union_align: [u64; 5usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DartNativeSendPort {
    pub id: DartPort,
    pub origin_id: DartPort,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DartNativeCapability {
    pub id: i64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DartNativeArray {
    pub length: isize,
    pub values: *mut *mut DartCObject,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DartNativeTypedData {
    pub ty: DartTypedDataType,
    pub length: isize,
    pub values: *mut u8,
}

///  Posts a message on some port. The message will contain the
///  Dart_CObject object graph rooted in 'message'.
///
///  While the message is being sent the state of the graph of
///  Dart_CObject structures rooted in 'message' should not be accessed,
///  as the message generation will make temporary modifications to the
///  data. When the message has been sent the graph will be fully
///  restored.
///
///  `port_id` The destination port.
///  `message` The message to send.
///
///  return true if the message was posted.
pub type DartPostCObjectFnType =
    unsafe extern "C" fn(port_id: DartPort, message: *mut DartCObject) -> bool;