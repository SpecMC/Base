//! A library with common code for parsing Minecraft specification.

pub mod parse;
pub mod tokenize;

/// Ensure that the given condition is true, otherwise return the given value.
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $ret:expr) => {
        if !$cond {
            return Err($ret);
        }
    };
}

#[macro_export]
macro_rules! tokenize {
    ($input:expr) => {
        $crate::tokenize::tokenize($input)
            .into_iter()
            .rev()
            .collect()
    };
}

#[cfg(test)]
mod tests {
    #[macro_export]
    macro_rules! test_parse {
        ($tokens:ident, $ty:ty, $value:expr) => {
            assert_eq!(<$ty>::parse(&mut $tokens), $value);
        };
    }
}
