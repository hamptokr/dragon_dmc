//! dragon_dmc
//!
//! dragon_dmc is an implementation of the DMC v2 protocol for the Dragonframe
//! stop motion software

#![deny(missing_debug_implementations, missing_docs)]

use std::ffi::CString;

use libc::c_char;

#[repr(C)]
#[derive(Debug)]
/// A DMC header
pub struct DmcMessage {
    marker: *mut c_char,
    id: u32,
    message_type: MessageType,
    length: u16,
    data: *mut u8,
    checksum: u16,
}

impl DmcMessage {
    /// Initialize a new DmcMessage
    pub fn new(id: u32, message_type: MessageType) -> Self {
        Self {
            marker: CString::new("DF").unwrap().into_raw(),
            id,
            message_type,
            length: 0,
            data: [0u8; 2048].as_mut_ptr(),
            checksum: 0,
        }
    }
}

#[repr(u16)]
#[derive(Debug)]
/// Message types determine the request response data
pub enum MessageType {
    /// The device must respond to every message it receives. Some requests have
    /// specific responses that they expect. Some just expect a response with a
    /// MSG_FLAG_ACK set and a response code.
    MsgFlagAck = 0x8000,
    /// Request basic information about device. Dragonfram always starts with
    /// this request. The device should also issue a MSG_HI whenever it starts
    /// up.
    MsgHi = 0x0001,
    /// Set DMX light values
    MsgDmx = 0x0020,
}

#[repr(u8)]
#[derive(Debug)]
/// Response codes are sent with MSG_FLAG_ACK [0x8000]
pub enum ResponseCode {
    /// Received, understood, and acted on message
    Ok = 0x0010,
    /// Checksum didn't match
    ErrChecksum = 0x0011,
    /// Can't handle this command while the rig is moving
    ErrMoving = 0x0012,
    /// The message type is not known or supported
    ErrUnsupported = 0x0013,
    /// A parameter was out of range
    ErrRange = 0x0014,
    /// General Error
    ErrGeneral = 0x0015,
    /// Software upper limit hit
    ErrSoftUp = 0x0020,
    /// Software lower limit hit
    ErrSoftLow = 0x0021,
    /// Hardware upper limit hit
    ErrHardUp = 0x0022,
    /// Hardware lower limit hit
    ErrHardLow = 0x0023,
}

#[repr(C)]
#[derive(Debug)]
/// Response data for MSG_HI[0x0001]
pub struct HiResponseData<'a> {
    name: &'a [u8; 32],
    /// from protocol, '2' for now
    fw_major: u8,
    /// from protocol, '2' for now
    fw_minor: u8,
    /// device-defined
    fw_rev: u8,
    motor_count: u8,
    dmx_count: u16,
    gio_out_count: u8,
    gio_input_count: u8,
    hw_limit_count: u8,
    upload_frame_count: u32,
    capabilities: u32,
}

#[repr(C)]
#[derive(Debug)]
/// The request specifies the initial channel to change, and then one or more
/// light values. If RAMP is set to 1, the device should ramp the light value of
/// each channel up or down to reach the target channel, rather than switching
/// to it immediately
///
/// Response: MSG_FLAG_ACK + Response code
pub struct DmxRequestData {
    ramp: u8,
    start_channel: u16,
    light_value: u8,
    light_value_2: u8,
    light_value_3: u8,
    light_value_4: u8,
}

#[repr(C)]
#[derive(Debug)]
/// Set the state for general I/O output triggers
pub struct GuiOutData {
    triggers: u32,
}

#[no_mangle]
/// Create a new DmcMessage object
pub extern "C" fn dragon_dmc_header_new(id: u32, message_type: MessageType) -> *mut DmcMessage {
    Box::into_raw(Box::new(DmcMessage::new(id, message_type)))
}
