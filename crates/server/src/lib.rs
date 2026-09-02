// tonic::Status (176 B) is used as the error type throughout the server
// layer — it bundles gRPC status code, message, and details. Boxing it at
// every call site would trade ergonomics for a lint. The size is accepted.
#![allow(clippy::result_large_err)]

pub mod rcm {
    tonic::include_proto!("rcm");
}

pub mod action_space;
pub mod decode;
pub mod manager;
pub mod mcp;
pub mod service;
pub mod state;
