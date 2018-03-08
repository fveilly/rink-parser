#[macro_use]
extern crate nom;

extern crate memchr;

#[macro_use]
pub mod macros;

mod ast;
mod tokens;
mod rules;
mod internal;
mod span;

pub use self::internal::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
