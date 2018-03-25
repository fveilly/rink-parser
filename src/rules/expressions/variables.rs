use rules::identifier::identifier;
use span::Span;
use tokens;

use ast::ast::{
    Variable,
    VariableScope,
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
    Ok(Variable{ name: span, scope: VariableScope::Local })
}

named_attr!(
    #[doc="
         Recognize a global variable.
    "],
    pub global_variable<Span, Variable>,
    map_res!(
        preceded!(
            tag!(tokens::GLOBAL_VARIABLE),
            first!(identifier)
        ),
        global_variable_mapper
    )
);

#[inline]
fn global_variable_mapper(span: Span) -> Result<Variable, ()> {
    Ok(Variable{ name: span, scope: VariableScope::Global })
}

#[cfg(test)]
mod tests {
    use super::{
        variable,
        global_variable
    };

    use ast::ast::{
        Variable,
        VariableScope,
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
            Variable{ name: Span::new_at("foo", 0, 1, 1), scope: VariableScope::Local }
        ));

        assert_eq!(variable(input), output);
    }

    #[test]
    fn case_global_variable() {
        let input  = Span::new("VAR foo\n");
        let output = Ok((
            Span::new_at("\n", 7, 1, 8),
            Variable{ name: Span::new_at("foo", 0, 1, 1), scope: VariableScope::Global }
        ));

        assert_eq!(global_variable(input), output);
    }

    #[test]
    fn case_global_variable_with_whitespace() {
        let input  = Span::new("VAR       foo\n");
        let output = Ok((
            Span::new_at("\n", 13, 1, 14),
            Variable{ name: Span::new_at("foo", 0, 1, 1), scope: VariableScope::Global }
        ));

        assert_eq!(global_variable(input), output);
    }
}