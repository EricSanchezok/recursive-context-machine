/// A token in the `.rcm` language.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Token {
    Ident(String),
    StringLit(String),
    LexError(String),
    LBrace,
    RBrace,
    Arrow,
    Dot,
    Semicolon,
    Equals,
    Eof,
}

/// Tokenize a `.rcm` source file into tokens.
pub(crate) fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = source.chars().collect();
    let mut pos = 0usize;

    while pos < chars.len() {
        let ch = chars[pos];

        if ch.is_whitespace() {
            pos += 1;
            continue;
        }

        // Line comments — skipped entirely.
        if ch == '/' && pos + 1 < chars.len() && chars[pos + 1] == '/' {
            while pos < chars.len() && chars[pos] != '\n' {
                pos += 1;
            }
            continue;
        }

        if ch == ';' && pos + 1 < chars.len() && chars[pos + 1] == ';' {
            while pos < chars.len() && chars[pos] != '\n' {
                pos += 1;
            }
            continue;
        }

        // String literals.
        if ch == '"' {
            pos += 1;
            let start = pos;
            while pos < chars.len() && chars[pos] != '"' {
                if chars[pos] == '\\' {
                    pos += 1;
                }
                pos += 1;
            }
            if pos >= chars.len() {
                tokens.push(Token::LexError("unclosed string literal".to_string()));
                continue;
            }
            let raw: String = chars[start..pos].iter().collect();
            tokens.push(Token::StringLit(raw));
            pos += 1;
            continue;
        }

        // Bracket and comma (for arrays).
        if ch == '[' || ch == ']' || ch == ',' {
            tokens.push(Token::Ident(ch.to_string()));
            pos += 1;
            continue;
        }

        if ch == '{' {
            tokens.push(Token::LBrace);
            pos += 1;
            continue;
        }
        if ch == '}' {
            tokens.push(Token::RBrace);
            pos += 1;
            continue;
        }
        if ch == '.' {
            tokens.push(Token::Dot);
            pos += 1;
            continue;
        }
        if ch == ';' {
            tokens.push(Token::Semicolon);
            pos += 1;
            continue;
        }
        if ch == '=' {
            tokens.push(Token::Equals);
            pos += 1;
            continue;
        }

        if ch == '-' && pos + 1 < chars.len() && chars[pos + 1] == '>' {
            tokens.push(Token::Arrow);
            pos += 2;
            continue;
        }

        if ch.is_ascii_alphabetic() || ch == '_' {
            let start = pos;
            while pos < chars.len()
                && (chars[pos].is_ascii_alphanumeric() || chars[pos] == '_' || chars[pos] == '-')
            {
                pos += 1;
            }
            let ident: String = chars[start..pos].iter().collect();
            tokens.push(Token::Ident(ident));
            continue;
        }

        if ch.is_ascii_digit() {
            let start = pos;
            while pos < chars.len() && chars[pos].is_ascii_digit() {
                pos += 1;
            }
            tokens.push(Token::Ident(chars[start..pos].iter().collect()));
            continue;
        }

        pos += 1;
    }

    tokens.push(Token::Eof);
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_agent_block() {
        let source = r#"
            agent research {
                purpose = "hello world"
                model = "deepseek-v4-flash"
                tools = ["shell", "fs"]
            }
        "#;
        let tokens = tokenize(source);
        let kinds: Vec<&str> = tokens
            .iter()
            .filter_map(|t| match t {
                Token::Ident(i) => Some(i.as_str()),
                Token::StringLit(s) => Some(s.as_str()),
                Token::LBrace => Some("{"),
                Token::RBrace => Some("}"),
                Token::Arrow => Some("->"),
                Token::Dot => Some("."),
                Token::Semicolon => Some(";"),
                Token::Equals => Some("="),
                _ => None,
            })
            .collect();

        assert_eq!(
            kinds,
            vec![
                "agent",
                "research",
                "{",
                "purpose",
                "=",
                "hello world",
                "model",
                "=",
                "deepseek-v4-flash",
                "tools",
                "=",
                "[",
                "shell",
                ",",
                "fs",
                "]",
                "}",
            ]
        );
    }

    #[test]
    fn lex_error_on_unclosed_string() {
        let tokens = tokenize(r#"agent x { purpose = "unclosed"#);
        assert!(tokens.iter().any(|t| matches!(t, Token::LexError(_))));
    }

    #[test]
    fn comments_are_skipped() {
        let tokens = tokenize("// this is a comment\nagent x { }");
        assert!(
            !tokens
                .iter()
                .any(|t| matches!(t, Token::Ident(i) if i == "//"))
        );
        assert!(
            tokens
                .iter()
                .any(|t| matches!(t, Token::Ident(i) if i == "agent"))
        );
    }
}
