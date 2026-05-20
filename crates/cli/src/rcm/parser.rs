use super::ast::*;
use super::lexer::Token;

pub(crate) struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<RcmFile, String> {
        let name = self.parse_name()?;

        let mut agents = Vec::new();
        let mut fluxes = Vec::new();
        let mut conditions = Vec::new();
        let mut wires = Vec::new();
        let mut mcps = Vec::new();

        while !self.is_eof() {
            match self.peek_string() {
                "agent" => agents.push(self.agent()?),
                "flux" => fluxes.push(self.flux()?),
                "condition" => conditions.push(self.condition()?),
                "mcp" => mcps.push(self.mcp()?),
                _ => {
                    wires.push(self.wire()?);
                }
            }
        }

        Ok(RcmFile {
            name,
            agents,
            fluxes,
            conditions,
            wires,
            mcps,
        })
    }

    // ── top-level ──

    fn parse_name(&mut self) -> Result<String, String> {
        self.expect_ident("name")?;
        self.expect(Token::Equals)?;
        self.expect_string()
    }

    fn agent(&mut self) -> Result<AgentDef, String> {
        self.expect_ident("agent")?;
        let id = self.expect_ident_any()?;
        self.expect(Token::LBrace)?;

        let mut name = None;
        let mut purpose = None;
        let mut model = None;
        let mut tools = Vec::new();
        let mut policy = None;

        while !self.eat(Token::RBrace) {
            let key = self.expect_ident_any()?;
            self.expect(Token::Equals)?;

            match key.as_str() {
                "name" => name = Some(self.expect_string()?),
                "purpose" => purpose = Some(self.expect_string()?),
                "model" => model = Some(self.expect_string()?),
                "policy" => policy = Some(self.expect_string()?),
                "tools" => tools = self.expect_string_array()?,
                _ => return Err(format!("unknown agent field: {}", key)),
            }
        }

        Ok(AgentDef {
            id,
            name,
            purpose,
            model,
            tools,
            policy,
        })
    }

    fn flux(&mut self) -> Result<FluxDef, String> {
        self.expect_ident("flux")?;
        let id = self.expect_ident_any()?;
        self.expect(Token::LBrace)?;

        let mut name = None;
        let mut channel = String::new();
        let mut mode = String::new();

        while !self.eat(Token::RBrace) {
            let key = self.expect_ident_any()?;
            self.expect(Token::Equals)?;
            match key.as_str() {
                "name" => name = Some(self.expect_string()?),
                "channel" => channel = self.expect_ident_any()?,
                "mode" => mode = self.expect_ident_any()?,
                _ => return Err(format!("unknown flux field: {}", key)),
            }
        }

        Ok(FluxDef {
            id,
            name,
            channel,
            mode,
        })
    }

    fn condition(&mut self) -> Result<ConditionDef, String> {
        self.expect_ident("condition")?;
        let id = self.expect_ident_any()?;
        self.expect(Token::LBrace)?;

        let mut name = None;

        while self.peek_string() == "name" {
            self.advance();
            self.expect(Token::Equals)?;
            name = Some(self.expect_string()?);
            self.eat(Token::Semicolon);
        }

        let predicate = self.predicate_block()?;
        self.expect(Token::RBrace)?;

        Ok(ConditionDef {
            id,
            name,
            predicate,
        })
    }

    fn mcp(&mut self) -> Result<McpDef, String> {
        self.expect_ident("mcp")?;
        let label = self.expect_ident_any()?;

        if self.peek_string() == "{" {
            self.expect(Token::LBrace)?;
            let mut url = None;
            let mut command = None;

            while !self.eat(Token::RBrace) {
                let key = self.expect_ident_any()?;
                self.expect(Token::Equals)?;
                match key.as_str() {
                    "url" => url = Some(self.expect_string()?),
                    "command" => command = Some(self.expect_string()?),
                    other => return Err(format!("unknown mcp field: {}", other)),
                }
            }

            Ok(McpDef {
                label,
                url,
                command,
                headers: Vec::new(),
            })
        } else {
            Ok(McpDef {
                label,
                url: None,
                command: None,
                headers: Vec::new(),
            })
        }
    }

    // ── wires ──

    fn wire(&mut self) -> Result<WireDef, String> {
        let from = self.port_ref()?;
        self.expect(Token::Arrow)?;
        let to = self.port_ref()?;
        Ok(WireDef { from, to })
    }

    fn port_ref(&mut self) -> Result<PortDef, String> {
        let id = self.expect_ident_any()?;
        if !self.eat(Token::Dot) {
            return Err(format!("expected '.' after '{}'", id));
        }
        let port = self.expect_ident_any()?;

        Ok(if id.starts_with("flux_") {
            PortDef::Flux { id, port }
        } else if id.starts_with("cond_") {
            PortDef::Condition { id, port }
        } else {
            PortDef::Agent { id, port }
        })
    }

    // ── predicates ──

    fn predicate_block(&mut self) -> Result<Predicate, String> {
        let key = self.expect_ident_any()?;

        match key.as_str() {
            "all" => {
                self.expect(Token::LBrace)?;
                let mut preds = Vec::new();
                while !self.eat(Token::RBrace) {
                    preds.push(self.predicate_block()?);
                    self.eat(Token::Semicolon);
                }
                Ok(Predicate::All(preds))
            }
            "any" => {
                self.expect(Token::LBrace)?;
                let mut preds = Vec::new();
                while !self.eat(Token::RBrace) {
                    preds.push(self.predicate_block()?);
                    self.eat(Token::Semicolon);
                }
                Ok(Predicate::Any(preds))
            }
            "not" => {
                self.expect(Token::LBrace)?;
                let inner = self.predicate_block()?;
                self.expect(Token::RBrace)?;
                Ok(Predicate::Not(Box::new(inner)))
            }
            channel => {
                let op = self.expect_ident_any()?;
                self.channel_predicate(channel, &op)
            }
        }
    }

    fn channel_predicate(&mut self, channel: &str, op: &str) -> Result<Predicate, String> {
        match (channel, op) {
            ("purpose", "contains") => Ok(Predicate::PurposeContains(self.expect_string()?)),
            ("purpose", "equals") => Ok(Predicate::PurposeEquals(self.expect_string()?)),
            ("purpose", "starts_with") => Ok(Predicate::PurposeStartsWith(self.expect_string()?)),
            ("purpose", "ends_with") => Ok(Predicate::PurposeEndsWith(self.expect_string()?)),
            ("purpose", "is_empty") => Ok(Predicate::PurposeIsEmpty),
            ("context", "has_tag") => Ok(Predicate::ContextHasTag(self.expect_string()?)),
            ("context", "has_role") => Ok(Predicate::ContextHasRole(self.expect_string()?)),
            ("context", "contains") => Ok(Predicate::ContextContains(self.expect_string()?)),
            ("context", "is_empty") => Ok(Predicate::ContextIsEmpty),
            ("env", "var") => {
                let var_name = self.expect_string()?;
                let action = self.expect_ident_any()?;
                match action.as_str() {
                    "exists" => Ok(Predicate::EnvVarExists(var_name)),
                    _ => Err(format!(
                        "expected 'exists' or 'equals' after env var, got '{}'",
                        action
                    )),
                }
            }
            ("env", "var_equals") => {
                let var_name = self.expect_string()?;
                let value = self.expect_string()?;
                Ok(Predicate::EnvVarEquals(var_name, value))
            }
            ("env", "cwd_contains") => Ok(Predicate::EnvCwdContains(self.expect_string()?)),
            ("env", "platform_is") => Ok(Predicate::EnvPlatformIs(self.expect_string()?)),
            ("resources", "has_model") => Ok(Predicate::ResHasModel(self.expect_string()?)),
            ("resources", "active_model_is") => {
                Ok(Predicate::ResActiveModelIs(self.expect_string()?))
            }
            ("resources", "has_tool") => Ok(Predicate::ResHasTool(self.expect_string()?)),
            ("resources", "tool_enabled") => Ok(Predicate::ResToolEnabled(self.expect_string()?)),
            ("resources", "has_prompt") => Ok(Predicate::ResHasPrompt(self.expect_string()?)),
            _ => Err(format!("unknown predicate: {} {}", channel, op)),
        }
    }

    // ── primitives ──

    fn peek(&self) -> &Token {
        static EOF: Token = Token::Eof;
        self.tokens.get(self.pos).unwrap_or(&EOF)
    }

    fn peek_string(&self) -> &str {
        match self.peek() {
            Token::Ident(s) => s.as_str(),
            Token::LexError(e) => e.as_str(),
            _ => "",
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn eat(&mut self, expected: Token) -> bool {
        if self.peek() == &expected {
            self.advance();
            true
        } else {
            false
        }
    }

    fn is_eof(&self) -> bool {
        matches!(self.peek(), Token::Eof)
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if self.eat(expected.clone()) {
            Ok(())
        } else {
            Err(format!("expected {:?}, found {:?}", expected, self.peek()))
        }
    }

    fn expect_ident(&mut self, value: &str) -> Result<(), String> {
        match self.peek() {
            Token::Ident(ident) if ident == value => {
                self.advance();
                Ok(())
            }
            other => Err(format!("expected '{}', found {:?}", value, other)),
        }
    }

    fn expect_ident_any(&mut self) -> Result<String, String> {
        match self.peek() {
            Token::Ident(ident) => {
                let s = ident.clone();
                self.advance();
                Ok(s)
            }
            other => Err(format!("expected identifier, found {:?}", other)),
        }
    }

    fn expect_string(&mut self) -> Result<String, String> {
        match self.peek() {
            Token::StringLit(s) => {
                let s = s.clone();
                self.advance();
                Ok(s)
            }
            other => Err(format!("expected string, found {:?}", other)),
        }
    }

    fn expect_string_array(&mut self) -> Result<Vec<String>, String> {
        self.expect_ident("[")?;
        let mut items = Vec::new();
        loop {
            if self.eat_ident("]") {
                break;
            }
            items.push(self.expect_string()?);
            self.eat_ident(",");
        }
        Ok(items)
    }

    fn eat_ident(&mut self, expected: &str) -> bool {
        match self.peek() {
            Token::Ident(ident) if ident == expected => {
                self.advance();
                true
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rcm::lexer;

    fn parse(source: &str) -> RcmFile {
        let tokens = lexer::tokenize(source);
        Parser::new(tokens).parse().unwrap()
    }

    #[test]
    fn parse_simple_agent() {
        let file = parse(
            r#"
            name = "test"
            agent research {
                purpose = "study quantum computing"
                model = "deepseek-v4-flash"
                tools = ["websearch", "fs"]
            }
        "#,
        );
        assert_eq!(file.name, "test");
        assert_eq!(file.agents.len(), 1);
        assert_eq!(file.agents[0].id, "research");
        assert_eq!(
            file.agents[0].purpose.as_deref(),
            Some("study quantum computing")
        );
        assert_eq!(file.agents[0].model.as_deref(), Some("deepseek-v4-flash"));
        assert_eq!(file.agents[0].tools, vec!["websearch", "fs"]);
    }

    #[test]
    fn parse_condition() {
        let file = parse(
            r#"
            name = "test"
            condition check {
                name = "Quality Check"
                all {
                    purpose contains "done"
                    context has_tag "results"
                }
            }
        "#,
        );
        assert_eq!(file.conditions.len(), 1);
        assert_eq!(file.conditions[0].name.as_deref(), Some("Quality Check"));
        assert!(matches!(file.conditions[0].predicate, Predicate::All(_)));
    }

    #[test]
    fn parse_wires() {
        let file = parse(
            r#"
            name = "test"
            agent a {}
            agent b {}
            a.pulse -> b.pulse
        "#,
        );
        assert_eq!(file.wires.len(), 1);
    }
}
