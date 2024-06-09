//! Module for parsing tokens.

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
/// The identifier must not be empty and can only contain letters, numbers, and underscores.
/// The identifier must not start with a number.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(pub String);
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
                if let Ok(int) = strtoint::strtoint(token) {
                    Ok(Literal::Integer(int))
                } else if let Ok(float) = token.parse::<f64>() {
                    Ok(Literal::Float(float))
                } else {
                    tokens.push(token.to_string());
                    Err(ParseError::InvalidToken {
                        token: token.to_string(),
                        error: "Invalid literal".to_string(),
                    })
                }
            }
        }
    }
}
