use std::collections::HashMap;
use hyper::server::Response;
use hyper::header::{self, ContentLength};
use serde_json;
use std::str;

use transport::tlv;

pub mod server;
pub mod handlers;
pub mod encrypted_stream;

pub enum Status {
    Success,
    InsufficientPrivileges,
    ServiceCommunicationFailure,
    ResourceBusy,
    ReadOnlyCharacteristic,
    WriteOnlyCharacteristic,
    NotificationNotSupported,
    OutOfResource,
    OperationTimedOut,
    ResourceDoesNotExist,
    InvalidValueInRequest,
}

impl Status {
    pub fn as_i32(&self) -> i32 {
        match self {
            &Status::Success => 0,
            &Status::InsufficientPrivileges => -70401,
            &Status::ServiceCommunicationFailure => -70402,
            &Status::ResourceBusy => -70403,
            &Status::ReadOnlyCharacteristic => -70404,
            &Status::WriteOnlyCharacteristic => -70405,
            &Status::NotificationNotSupported => -70406,
            &Status::OutOfResource => -70407,
            &Status::OperationTimedOut => -70408,
            &Status::ResourceDoesNotExist => -70409,
            &Status::InvalidValueInRequest => -70410,
        }
    }
}

enum ContentType {
    PairingTLV8,
    HapJson,
}

impl ContentType {
    pub fn for_hyper(&self) -> header::ContentType {
        match self {
            &ContentType::PairingTLV8 => header::ContentType("application/pairing+tlv8".parse().unwrap()),
            &ContentType::HapJson => header::ContentType("application/hap+json".parse().unwrap()),
        }
    }
}

pub fn tlv_response(answer: HashMap<u8, Vec<u8>>) -> Response {
    let body = tlv::encode(answer);
    response(body, ContentType::PairingTLV8)
}

pub fn json_response(answer: serde_json::Value) -> Response {
    let body = serde_json::to_vec(&answer).unwrap();
    println!("response: {:?}", str::from_utf8(&body).unwrap());

    response(body, ContentType::HapJson)
}

fn response(body: Vec<u8>, content_type: ContentType) -> Response {
    Response::new()
        .with_header(ContentLength(body.len() as u64))
        .with_header(content_type.for_hyper())
        .with_body(body)
}
