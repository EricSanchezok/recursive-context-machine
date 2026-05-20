mod ast;
pub mod compile;
mod lexer;
mod parser;

pub use ast::{
    AcceleratorBodyDef, AcceleratorSourceDef, ConditionDef, EndpointDef, FluxDef,
    GraphAcceleratorDef, GraphDef, McpDef, ModelDef, PortDef, PortOwnerDef, Predicate,
    PrimitiveDef, RcmFile, UseDef, WireDef,
};
pub(crate) use parser::Parser;

pub fn parse(source: &str) -> Result<RcmFile, String> {
    let tokens = lexer::tokenize(source);
    Parser::new(tokens).parse()
}
