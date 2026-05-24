pub mod rcm {
    tonic::include_proto!("rcm");
}

pub mod action_space;
pub mod decode;
pub mod manager;
pub mod mcp;
pub mod service;
pub mod state;
