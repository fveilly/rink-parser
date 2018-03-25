use rules::identifier::identifier;
use span::Span;

use ast::ast::{
    Variable,
    Expression
};

named_attr!(
    #[doc="
         Recognize a variable.
    "],
    pub variable<Span, Variable>,
    map_res!(
        identifier,
        variable_mapper
    )
);

#[inline]
fn variable_mapper(span: Span) -> Result<Variable, ()> {
    Ok(Variable(span))
}

#[cfg(test)]
mod tests {
    use super::variable;

    use ast::ast::{
        Variable,
        Expression
    };

    use internal::{
        Context,
        Error,
        ErrorKind
    };

    use span::Span;

    #[test]
    fn case_variable() {
        let input  = Span::new("foo\n");
        let output = Ok((
            Span::new_at("\n", 3, 1, 4),
            Variable(Span::new_at("foo", 0, 1, 1))
        ));

        assert_eq!(variable(input), output);
    }
}