//! Module for parsing tokens.

use std::fmt::Display;

use thiserror::Error;

use crate::ensure;

/// Ensure that the next tokens in the list match the given tokens.
/// This will remove the tokens from the list.
/// Tokens are checked in reverse order.
#[macro_export]
macro_rules! ensure_tokens {
    ($tokens:ident, $($token:expr),+) => {
        $(
            $crate::ensure!(
                $tokens.last().ok_or($crate::parse::ParseError::EndOfFile)? == $token,
                $crate::parse::ParseError::InvalidToken {
                    token: $tokens.last().unwrap().clone(),
                    error: format!("Expected {}", $token),
                }
            );
            $tokens.pop();
        )+
    };
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ParseError {
    #[error("Unexpected EOF")]
    EndOfFile,

    #[error("Invalid token: {error}: {token}")]
    InvalidToken { token: String, error: String },
}

pub trait Parse
where
    Self: Sized,
{
    /// Parse a list of tokens into an object, consuming the tokens as needed.
    /// The token list is consumed in reverse order.
    /// If this fails, it is **not** guaranteed that no tokens have been consumed.
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError>;
}

/// An identifier.
/// The identifier must not be empty or start with a number, and can only contain letters, numbers, and underscores.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(pub String);
impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Parse for Identifier {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        let value: String = tokens.pop().ok_or(ParseError::EndOfFile)?;

        ensure!(
            !value.is_empty(),
            ParseError::InvalidToken {
                token: value,
                error: "Empty identifier".to_string()
            }
        );

        let mut chars: std::str::Chars = value.chars();
        ensure!(
            chars
                .next()
                .map(|c| c.is_ascii_alphabetic() || c == '_')
                .unwrap(),
            ParseError::InvalidToken {
                token: value,
                error: "Identifiers must not start with a number and can only contain letters, numbers, and underscores".to_string()
            }
        );
        ensure!(
            chars.all(|c| c.is_ascii_alphanumeric() || c == '_'),
            ParseError::InvalidToken {
                token: value,
                error: "Identifiers can only contain letters, numbers, and underscores".to_string()
            }
        );

        Ok(Identifier(value))
    }
}

/// A literal value.
/// This can be a boolean, integer, float, or string.
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Boolean(bool),
    Integer(isize),
    Float(f64),
    String(String),
}
impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Literal::*;
        match self {
            Boolean(value) => write!(f, "{value}"),
            Integer(value) => write!(f, "{value}"),
            Float(value) => write!(f, "{value}"),
            String(value) => write!(f, "\"{value}\""),
        }
    }
}
impl Parse for Literal {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        match tokens.pop().ok_or(ParseError::EndOfFile)?.as_str() {
            "true" => Ok(Literal::Boolean(true)),
            "false" => Ok(Literal::Boolean(false)),
            "\"" => {
                let mut string: String = String::new();
                while tokens.last().ok_or(ParseError::EndOfFile)? != "\"" {
                    string += &tokens.pop().unwrap();
                }
                ensure_tokens!(tokens, "\"");
                Ok(Literal::String(string))
            }
            token => {
                let mut token: String = token.to_string();
                if let "+" | "-" = token.as_str() {
                    token += &tokens.pop().ok_or(ParseError::EndOfFile)?;
                }

                if let Ok(int) = strtoint::strtoint(&token) {
                    Ok(Literal::Integer(int))
                } else if let Ok(float) = token.parse::<f64>() {
                    Ok(Literal::Float(float))
                } else {
                    tokens.push(token.clone());
                    Err(ParseError::InvalidToken {
                        token: token.clone(),
                        error: "Invalid literal".to_string(),
                    })
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_parse, tokenize};

    use super::*;

    #[test]
    fn test_identifier() {
        let mut tokens: Vec<String> = tokenize!("cool_identifier");

        test_parse!(
            tokens,
            Identifier,
            Ok(Identifier("cool_identifier".to_string()))
        );

        assert!(tokens.is_empty());
        test_parse!(tokens, Identifier, Err(ParseError::EndOfFile));
    }

    #[test]
    fn test_literal() {
        let mut tokens: Vec<String> = tokenize!("true false 0 +42 -5 123.0 +8.5 -11.4 \"string\"");

        test_parse!(tokens, Literal, Ok(Literal::Boolean(true)));
        test_parse!(tokens, Literal, Ok(Literal::Boolean(false)));
        test_parse!(tokens, Literal, Ok(Literal::Integer(0)));
        test_parse!(tokens, Literal, Ok(Literal::Integer(42)));
        test_parse!(tokens, Literal, Ok(Literal::Integer(-5)));
        test_parse!(tokens, Literal, Ok(Literal::Float(123.0)));
        test_parse!(tokens, Literal, Ok(Literal::Float(8.5)));
        test_parse!(tokens, Literal, Ok(Literal::Float(-11.4)));
        test_parse!(tokens, Literal, Ok(Literal::String("string".to_string())));

        assert!(tokens.is_empty());
        test_parse!(tokens, Literal, Err(ParseError::EndOfFile));
    }
}
