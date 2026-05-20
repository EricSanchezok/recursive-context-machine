mod ast;
pub(crate) mod compile;
mod lexer;
mod parser;

pub use ast::{
    AgentDef, ConditionDef, FluxDef, McpDef, ModelDef, PortDef, Predicate, RcmFile, WireDef,
};
pub(crate) use parser::Parser;

/// Parse a `.rcm` source string into an AST.
pub fn parse(source: &str) -> Result<RcmFile, String> {
    let tokens = lexer::tokenize(source);
    Parser::new(tokens).parse()
}
