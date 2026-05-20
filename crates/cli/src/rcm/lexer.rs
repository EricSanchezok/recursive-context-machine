/// A token in the `.rcm` language.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Token {
    Ident(String),
    StringLit(String),
    LBrace,
    RBrace,
    Arrow,
    Dot,
    Semicolon,
    Equals,
    Comment(String),
    Eof,
}

/// Tokenize a `.rcm` source file into tokens.
///
/// Rules:
/// - `// ...` and `;; ...` → comment (skipped)
/// - `"..."` → string literal
/// - `{` `}` `->` `.` `;` `=` → punctuation
/// - `[a-zA-Z_][a-zA-Z0-9_-]*` → identifier
/// - Whitespace is skipped.
pub(crate) fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = source.chars().collect();
    let mut pos = 0usize;

    while pos < chars.len() {
        let ch = chars[pos];

        // Skip whitespace.
        if ch.is_whitespace() {
            pos += 1;
            continue;
        }

        // Line comments.
        if ch == '/' && pos + 1 < chars.len() && chars[pos + 1] == '/' {
            let start = pos;
            while pos < chars.len() && chars[pos] != '\n' {
                pos += 1;
            }
            tokens.push(Token::Comment(chars[start..pos].iter().collect()));
            continue;
        }

        if ch == ';' && pos + 1 < chars.len() && chars[pos + 1] == ';' {
            let start = pos;
            while pos < chars.len() && chars[pos] != '\n' {
                pos += 1;
            }
            tokens.push(Token::Comment(chars[start..pos].iter().collect()));
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
            let raw: String = chars[start..pos].iter().collect();
            tokens.push(Token::StringLit(raw));
            pos += 1; // skip closing "
            continue;
        }

        // Bracket and comma (for arrays).
        if ch == '[' || ch == ']' || ch == ',' {
            tokens.push(Token::Ident(ch.to_string()));
            pos += 1;
            continue;
        }

        // Single-character punctuation.
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

        // Arrow ->
        if ch == '-' && pos + 1 < chars.len() && chars[pos + 1] == '>' {
            tokens.push(Token::Arrow);
            pos += 2;
            continue;
        }

        // Identifier or keyword.
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

        // Numbers — only appear inside url/strings, but accept here for robustness.
        if ch.is_ascii_digit() {
            let start = pos;
            while pos < chars.len() && chars[pos].is_ascii_digit() {
                pos += 1;
            }
            let ident: String = chars[start..pos].iter().collect();
            tokens.push(Token::Ident(ident));
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
    fn tokenize_wire_and_predicate() {
        let source = r#"
            research.pulse -> writer.pulse
            quality.true -> writer.pulse
        "#;
        let tokens = tokenize(source);
        let kinds: Vec<&str> = tokens
            .iter()
            .filter_map(|t| match t {
                Token::Ident(i) => Some(i.as_str()),
                Token::Arrow => Some("->"),
                Token::Dot => Some("."),
                _ => None,
            })
            .collect();

        assert_eq!(
            kinds,
            vec![
                "research", ".", "pulse", "->", "writer", ".", "pulse", "quality", ".", "true",
                "->", "writer", ".", "pulse",
            ]
        );
    }

    #[test]
    fn tokenize_mcp() {
        let source = r#"mcp search { url = "https://api.anysearch.com/mcp" }"#;
        let tokens = tokenize(source);

        let string_values: Vec<&str> = tokens
            .iter()
            .filter_map(|t| match t {
                Token::StringLit(s) => Some(s.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(string_values, vec!["https://api.anysearch.com/mcp"]);
    }

    #[test]
    fn comments_are_skipped() {
        let source = "// this is a comment\nagent x { }";
        let tokens = tokenize(source);
        assert!(
            tokens
                .iter()
                .any(|t| matches!(t, Token::Ident(i) if i == "agent"))
        );
    }
}
