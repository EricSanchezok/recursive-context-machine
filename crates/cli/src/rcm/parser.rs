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
        let mut uses = Vec::new();
        let mut models = Vec::new();
        let mut mcps = Vec::new();
        let mut body = None;

        while !self.is_eof() {
            match self.peek_ident() {
                Some("use") => uses.push(self.use_def()?),
                Some("model") => models.push(self.model()?),
                Some("mcp") => mcps.push(self.mcp()?),
                Some("accelerator") => {
                    if body.is_some() {
                        return Err("rcm file can only export one accelerator".to_string());
                    }
                    body = Some(AcceleratorBodyDef::Primitive(self.top_accelerator()?));
                }
                Some("graph") => {
                    if body.is_some() {
                        return Err("rcm file can only export one accelerator".to_string());
                    }
                    body = Some(AcceleratorBodyDef::Graph(self.graph()?));
                }
                _ => return Err(format!("unexpected token: {:?}", self.peek())),
            }
        }

        Ok(RcmFile {
            name,
            uses,
            models,
            mcps,
            body: body.ok_or_else(|| "rcm file requires accelerator or graph body".to_string())?,
        })
    }

    fn parse_name(&mut self) -> Result<String, String> {
        self.expect_ident("name")?;
        self.expect(Token::Equals)?;
        self.expect_string()
    }

    fn use_def(&mut self) -> Result<UseDef, String> {
        self.expect_ident("use")?;
        let path = self.expect_string()?;
        self.expect_ident("as")?;
        let alias = self.expect_ident_any()?;
        Ok(UseDef { path, alias })
    }

    fn top_accelerator(&mut self) -> Result<PrimitiveDef, String> {
        self.expect_ident("accelerator")?;
        self.expect(Token::LBrace)?;
        self.primitive_fields()
    }

    fn graph(&mut self) -> Result<GraphDef, String> {
        self.expect_ident("graph")?;
        self.expect(Token::LBrace)?;
        let mut accelerators = Vec::new();
        let mut fluxes = Vec::new();
        let mut conditions = Vec::new();
        let mut wires = Vec::new();

        while !self.eat(Token::RBrace) {
            match self.peek_ident() {
                Some("accelerator") => accelerators.push(self.graph_accelerator()?),
                Some("flux") => fluxes.push(self.flux()?),
                Some("condition") => conditions.push(self.condition()?),
                _ => wires.push(self.wire()?),
            }
        }

        Ok(GraphDef {
            accelerators,
            fluxes,
            conditions,
            wires,
        })
    }

    fn graph_accelerator(&mut self) -> Result<GraphAcceleratorDef, String> {
        self.expect_ident("accelerator")?;
        let id = self.expect_ident_any()?;
        let source = if self.eat(Token::Equals) {
            let alias = self.expect_ident_any()?;
            let overrides = if self.eat(Token::LBrace) {
                self.primitive_fields()?
            } else {
                PrimitiveDef::default()
            };
            AcceleratorSourceDef::Import { alias, overrides }
        } else {
            self.expect(Token::LBrace)?;
            AcceleratorSourceDef::Inline(self.primitive_fields()?)
        };
        Ok(GraphAcceleratorDef { id, source })
    }

    fn primitive_fields(&mut self) -> Result<PrimitiveDef, String> {
        let mut def = PrimitiveDef::default();
        while !self.eat(Token::RBrace) {
            let key = self.expect_ident_any()?;
            self.expect(Token::Equals)?;
            match key.as_str() {
                "purpose" => def.purpose = Some(self.expect_string()?),
                "models" => def.models = self.expect_string_array()?,
                "policy" => def.policy = Some(self.expect_string()?),
                "environment" => def.environment = Some(self.expect_string()?),
                "prompts" => def.prompts = Some(self.prompt_sources()?),
                "tools" => def.tools = Some(self.expect_string_array()?),
                "mcps" => def.mcps = Some(self.expect_string_array()?),
                "spawns" => def.spawns = self.expect_string_array()?,
                _ => return Err(format!("unknown accelerator field: {}", key)),
            }
        }
        Ok(def)
    }

    fn prompt_sources(
        &mut self,
    ) -> Result<std::collections::HashMap<String, PromptSourceDef>, String> {
        self.expect(Token::LBrace)?;
        let mut prompts = std::collections::HashMap::new();
        while !self.eat(Token::RBrace) {
            let name = self.expect_ident_any()?;
            self.expect(Token::Equals)?;
            let source = if self.eat_ident("file") {
                PromptSourceDef::File(self.expect_string()?)
            } else {
                PromptSourceDef::Inline(self.expect_string()?)
            };
            if prompts.insert(name.clone(), source).is_some() {
                return Err(format!("duplicate prompt: {}", name));
            }
            self.eat(Token::Semicolon);
        }
        Ok(prompts)
    }

    fn flux(&mut self) -> Result<FluxDef, String> {
        self.expect_ident("flux")?;
        let id = self.expect_ident_any()?;
        self.expect(Token::LBrace)?;
        let mut name = None;
        let mut channel = None;
        let mut mode = None;
        let mut from = None;
        let mut to = None;
        let mut arity = None;
        while !self.eat(Token::RBrace) {
            let key = self.expect_ident_any()?;
            self.expect(Token::Equals)?;
            match key.as_str() {
                "name" => name = Some(self.expect_string()?),
                "channel" => channel = Some(self.expect_ident_any()?),
                "mode" => mode = Some(self.expect_ident_any()?),
                "from" => from = Some(self.expect_ident_any()?),
                "to" => to = Some(self.expect_ident_any()?),
                "arity" => arity = Some(self.expect_usize()?),
                _ => return Err(format!("unknown flux field: {}", key)),
            }
        }
        Ok(FluxDef {
            id,
            name,
            channel: channel.ok_or_else(|| "flux requires channel".to_string())?,
            mode: mode.ok_or_else(|| "flux requires mode".to_string())?,
            from,
            to,
            arity: arity.ok_or_else(|| "flux requires arity".to_string())?,
        })
    }

    fn condition(&mut self) -> Result<ConditionDef, String> {
        self.expect_ident("condition")?;
        let id = self.expect_ident_any()?;
        self.expect(Token::LBrace)?;
        let mut name = None;
        while self.peek_ident() == Some("name") {
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
        self.expect(Token::LBrace)?;

        let mut transport = None;
        let mut command = None;
        let mut args = Vec::new();
        let mut env = std::collections::HashMap::new();
        let mut cwd = None;
        let mut url = None;
        let mut headers = std::collections::HashMap::new();

        while !self.eat(Token::RBrace) {
            let key = self.expect_ident_any()?;
            self.expect(Token::Equals)?;
            match key.as_str() {
                "transport" => transport = Some(self.expect_ident_any()?),
                "command" => command = Some(self.expect_string()?),
                "args" => args = self.expect_string_array()?,
                "env" => env = self.mcp_values()?,
                "cwd" => cwd = Some(self.expect_string()?),
                "url" => url = Some(self.expect_string()?),
                "headers" => headers = self.mcp_values()?,
                other => return Err(format!("unknown mcp field: {}", other)),
            }
            self.eat(Token::Semicolon);
        }

        let transport = match transport.as_deref() {
            Some("stdio") => McpTransportDef::Stdio {
                command: command.ok_or_else(|| "stdio mcp requires command".to_string())?,
                args,
                env,
                cwd,
            },
            Some("http") => McpTransportDef::Http {
                url: url.ok_or_else(|| "http mcp requires url".to_string())?,
                headers,
            },
            Some("sse") => McpTransportDef::Sse {
                url: url.ok_or_else(|| "sse mcp requires url".to_string())?,
                headers,
            },
            Some(other) => return Err(format!("unknown mcp transport: {}", other)),
            None => return Err("mcp requires transport".to_string()),
        };

        Ok(McpDef { label, transport })
    }

    fn mcp_values(&mut self) -> Result<std::collections::HashMap<String, McpValueDef>, String> {
        self.expect(Token::LBrace)?;
        let mut values = std::collections::HashMap::new();
        while !self.eat(Token::RBrace) {
            let name = self.expect_ident_any()?;
            self.expect(Token::Equals)?;
            let value = if self.eat_ident("env") {
                McpValueDef::Env(self.expect_string()?)
            } else {
                McpValueDef::Literal(self.expect_string()?)
            };
            if values.insert(name.clone(), value).is_some() {
                return Err(format!("duplicate mcp value: {}", name));
            }
            self.eat(Token::Semicolon);
        }
        Ok(values)
    }

    fn model(&mut self) -> Result<ModelDef, String> {
        self.expect_ident("model")?;
        let id = self.expect_ident_any()?;
        self.expect(Token::LBrace)?;

        let mut protocol = String::new();
        let mut endpoint = None;
        let mut credentials_env = None;
        let mut credentials_key = None;
        let mut limit_context = None;
        let mut limit_input = None;
        let mut limit_output = 0;
        let mut modalities_input = Vec::new();
        let mut modalities_output = Vec::new();
        let mut headers = std::collections::HashMap::new();
        let mut thinking = false;
        let mut timeout = None;

        while !self.eat(Token::RBrace) {
            let key = self.expect_ident_any()?;
            self.expect(Token::Equals)?;
            if self.eat(Token::LBrace) {
                match key.as_str() {
                    "credentials" => {
                        self.model_credentials(&mut credentials_env, &mut credentials_key)?
                    }
                    "limit" => {
                        self.model_limit(&mut limit_context, &mut limit_input, &mut limit_output)?
                    }
                    "modalities" => {
                        self.model_modalities(&mut modalities_input, &mut modalities_output)?
                    }
                    "headers" => headers = self.model_headers()?,
                    other => return Err(format!("unknown model block: {}", other)),
                }
            } else {
                match key.as_str() {
                    "protocol" => protocol = self.expect_string()?,
                    "endpoint" => endpoint = Some(self.expect_string()?),
                    "thinking" => {
                        let value = self.expect_string()?;
                        thinking = value.parse().map_err(|_| {
                            format!("thinking must be \"true\" or \"false\", got: {}", value)
                        })?;
                    }
                    "timeout" => {
                        let value = self.expect_string()?;
                        timeout = Some(value.parse().map_err(|_| {
                            format!("timeout must be a number of seconds, got: {}", value)
                        })?);
                    }
                    other => return Err(format!("unknown model field: {}", other)),
                }
            }
        }

        if protocol.is_empty() {
            return Err("model requires a protocol".to_string());
        }

        Ok(ModelDef {
            id,
            protocol,
            endpoint,
            credentials_env,
            credentials_key,
            limit_context,
            limit_input,
            limit_output,
            modalities_input,
            modalities_output,
            headers,
            thinking,
            timeout,
        })
    }

    fn model_credentials(
        &mut self,
        env: &mut Option<String>,
        key: &mut Option<String>,
    ) -> Result<(), String> {
        while !self.eat(Token::RBrace) {
            let field = self.expect_ident_any()?;
            self.expect(Token::Equals)?;
            match field.as_str() {
                "env" => *env = Some(self.expect_string()?),
                "key" => *key = Some(self.expect_string()?),
                other => return Err(format!("unknown credentials field: {}", other)),
            }
            self.eat_ident(",");
        }
        Ok(())
    }

    fn model_limit(
        &mut self,
        context: &mut Option<u64>,
        input: &mut Option<u64>,
        output: &mut u64,
    ) -> Result<(), String> {
        while !self.eat(Token::RBrace) {
            let field = self.expect_ident_any()?;
            self.expect(Token::Equals)?;
            let value = self.expect_string()?;
            let number = value
                .parse()
                .map_err(|_| format!("invalid limit value: {}", value))?;
            match field.as_str() {
                "context" => *context = Some(number),
                "input" => *input = Some(number),
                "output" => *output = number,
                other => return Err(format!("unknown limit field: {}", other)),
            }
            self.eat_ident(",");
        }
        Ok(())
    }

    fn model_modalities(
        &mut self,
        input: &mut Vec<String>,
        output: &mut Vec<String>,
    ) -> Result<(), String> {
        while !self.eat(Token::RBrace) {
            let field = self.expect_ident_any()?;
            self.expect(Token::Equals)?;
            match field.as_str() {
                "input" => *input = self.expect_string_array()?,
                "output" => *output = self.expect_string_array()?,
                other => return Err(format!("unknown modalities field: {}", other)),
            }
            self.eat_ident(",");
        }
        Ok(())
    }

    fn model_headers(&mut self) -> Result<std::collections::HashMap<String, String>, String> {
        let mut headers = std::collections::HashMap::new();
        while !self.eat(Token::RBrace) {
            let name = self.expect_ident_any()?;
            self.expect(Token::Equals)?;
            let value = self.expect_string()?;
            if headers.insert(name.clone(), value).is_some() {
                return Err(format!("duplicate header: {}", name));
            }
            self.eat_ident(",");
        }
        Ok(headers)
    }

    fn wire(&mut self) -> Result<WireDef, String> {
        let from = self.port_ref()?;
        self.expect(Token::Arrow)?;
        let to = self.port_ref()?;
        Ok(WireDef { from, to })
    }

    fn port_ref(&mut self) -> Result<PortDef, String> {
        let owner_name = self.expect_ident_any()?;
        self.expect(Token::Dot)?;
        let endpoint = self.endpoint()?;
        let owner = match owner_name.as_str() {
            "input" => PortOwnerDef::Input,
            "output" => PortOwnerDef::Output,
            _ => PortOwnerDef::Component(owner_name),
        };
        Ok(PortDef { owner, endpoint })
    }

    fn endpoint(&mut self) -> Result<EndpointDef, String> {
        let name = self.expect_ident_any()?;
        match name.as_str() {
            "trigger" => Ok(EndpointDef::Trigger),
            "done" => Ok(EndpointDef::Done),
            "purpose" | "context" | "environment" | "policy" | "resources" => {
                Ok(EndpointDef::State(name))
            }
            "out" => Ok(EndpointDef::FluxOut),
            "slot" => {
                self.expect(Token::LParen)?;
                let slot = self.expect_usize()?;
                self.expect(Token::RParen)?;
                Ok(EndpointDef::FluxSlot(slot))
            }
            "true" => Ok(EndpointDef::ConditionTrue),
            "false" => Ok(EndpointDef::ConditionFalse),
            _ => Err(format!("unknown endpoint: {}", name)),
        }
    }

    fn predicate_block(&mut self) -> Result<Predicate, String> {
        let key = self.expect_ident_any()?;
        match key.as_str() {
            "all" => {
                self.expect(Token::LBrace)?;
                let mut predicates = Vec::new();
                while !self.eat(Token::RBrace) {
                    predicates.push(self.predicate_block()?);
                    self.eat(Token::Semicolon);
                }
                Ok(Predicate::All(predicates))
            }
            "any" => {
                self.expect(Token::LBrace)?;
                let mut predicates = Vec::new();
                while !self.eat(Token::RBrace) {
                    predicates.push(self.predicate_block()?);
                    self.eat(Token::Semicolon);
                }
                Ok(Predicate::Any(predicates))
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
                    _ => Err(format!("unknown env var predicate action: {}", action)),
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

    fn peek(&self) -> &Token {
        static EOF: Token = Token::Eof;
        self.tokens.get(self.pos).unwrap_or(&EOF)
    }

    fn peek_ident(&self) -> Option<&str> {
        match self.peek() {
            Token::Ident(ident) => Some(ident.as_str()),
            _ => None,
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
                let value = ident.clone();
                self.advance();
                Ok(value)
            }
            Token::LexError(error) => Err(error.clone()),
            other => Err(format!("expected identifier, found {:?}", other)),
        }
    }

    fn expect_string(&mut self) -> Result<String, String> {
        match self.peek() {
            Token::StringLit(value) => {
                let value = value.clone();
                self.advance();
                Ok(value)
            }
            Token::LexError(error) => Err(error.clone()),
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

    fn expect_usize(&mut self) -> Result<usize, String> {
        let value = self.expect_ident_any()?;
        value
            .parse()
            .map_err(|_| format!("expected number, found {}", value))
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
