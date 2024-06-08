//! Module for parsing tokens.

/// Ensure that the next tokens in the list match the given tokens.
/// This will remove the tokens from the list.
/// Tokens are checked in reverse order.
#[macro_export]
macro_rules! ensure_tokens {
    ($tokens:ident, $($token:expr),+) => {
        $(
            $crate::ensure!(!$tokens.is_empty(), $crate::parse::ParseError::EndOfFile);
            $crate::ensure!(
                $tokens.last().unwrap() == $token,
                $crate::parse::ParseError::InvalidToken {
                    token: $tokens.last().unwrap().clone(),
                    error: format!("Expected {}", $token),
                }
            );
            $tokens.pop();
        )+
    };
}

/// Match the next token in the list.
/// If the list is empty, an empty token will be matched.
/// Otherwise, the last token will be matched and removed from the list.
#[macro_export]
macro_rules! match_token {
    ($tokens:ident, $($token:expr => $body:expr),+) => {
        $crate::match_token!($tokens, $($token => $body),+; _token => {});
    };

    ($tokens:ident, $($token:expr => $body:expr),+; $default:pat => $default_body:expr) => {
        match $tokens.last().map(std::string::String::as_str).unwrap_or("") {
            $(
                $token => {
                    $tokens.pop();
                    $body
                },
            )+
            $default => $default_body,
        }
    };
}

#[derive(Debug, thiserror::Error)]
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
    /// If this fails, it is not guaranteed that any tokens haven't been consumed.
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError>;
}

/// An identifier.
/// The identifier must not be empty and can only contain letters, numbers, and underscores.
/// The identifier must not start with a number.
#[derive(Debug)]
pub struct Identifier {
    inner: String,
}
impl Parse for Identifier {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        crate::ensure!(!tokens.is_empty(), ParseError::EndOfFile);
        let value: String = tokens.pop().unwrap();

        crate::ensure!(
            !value.is_empty(),
            ParseError::InvalidToken {
                token: value,
                error: "Empty identifier".to_string()
            }
        );

        let mut chars: std::str::Chars = value.chars();
        crate::ensure!(
            chars
                .next()
                .map(|c| c.is_ascii_alphabetic() || c == '_')
                .unwrap(),
            ParseError::InvalidToken {
                token: value,
                error: "Identifiers must not start with a number and can only contain letters, numbers, and underscores".to_string()
            }
        );
        crate::ensure!(
            chars.all(|c| c.is_ascii_alphanumeric() || c == '_'),
            ParseError::InvalidToken {
                token: value,
                error: "Identifiers can only contain letters, numbers, and underscores".to_string()
            }
        );

        Ok(Identifier { inner: value })
    }
}

/// A literal value.
/// This can be a boolean, integer, float, or string.
#[derive(Debug)]
pub enum Literal {
    Boolean(bool),
    Integer(isize),
    Float(f64),
    String(String),
}
impl Parse for Literal {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        crate::ensure!(!tokens.is_empty(), ParseError::EndOfFile);
        match_token!(tokens,
            "true" => Ok(Literal::Boolean(true)),
            "false" => Ok(Literal::Boolean(false)),
            "\"" => {
                crate::ensure!(!tokens.is_empty(), ParseError::EndOfFile);
                let mut string: String = String::new();
                while tokens.last().unwrap() != "\"" {
                    string += tokens.last().unwrap();
                    tokens.pop();
                }
                ensure_tokens!(tokens, "\"");
                Ok(Literal::String(string))
            };
            token => {
                if let Ok(int) = strtoint::strtoint(token) {
                    Ok(Literal::Integer(int))
                } else if let Ok(float) = token.parse::<f64>() {
                    Ok(Literal::Float(float))
                } else {
                    Err(ParseError::InvalidToken {
                        token: token.to_string(),
                        error: "Invalid literal".to_string(),
                    })
                }
            }
        )
    }
}
