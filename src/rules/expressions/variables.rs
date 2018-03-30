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
    Ok(Variable{ name: span, scope: VariableScope::Local, constant: false })
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
    Ok(Variable{ name: span, scope: VariableScope::Global, constant: false })
}

named_attr!(
    #[doc="
         Recognize a global constant.
    "],
    pub global_constant<Span, Variable>,
    map_res!(
        preceded!(
            tag!(tokens::CONSTANT),
            first!(identifier)
        ),
        global_constant_mapper
    )
);

#[inline]
fn global_constant_mapper(span: Span) -> Result<Variable, ()> {
    Ok(Variable{ name: span, scope: VariableScope::Global, constant: true })
}

#[cfg(test)]
mod tests {
    use super::{
        variable,
        global_variable,
        global_constant
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
            Variable{ name: Span::new_at("foo", 0, 1, 1), scope: VariableScope::Local, constant: false }
        ));

        assert_eq!(variable(input), output);
    }

    #[test]
    fn case_global_variable() {
        let input  = Span::new("VAR foo\n");
        let output = Ok((
            Span::new_at("\n", 7, 1, 8),
            Variable{ name: Span::new_at("foo", 0, 1, 1), scope: VariableScope::Global, constant: false }
        ));

        assert_eq!(global_variable(input), output);
    }

    #[test]
    fn case_global_variable_with_whitespace() {
        let input  = Span::new("VAR       foo\n");
        let output = Ok((
            Span::new_at("\n", 13, 1, 14),
            Variable{ name: Span::new_at("foo", 0, 1, 1), scope: VariableScope::Global, constant: false }
        ));

        assert_eq!(global_variable(input), output);
    }

    #[test]
    fn case_global_constant() {
        let input  = Span::new("CONST foo\n");
        let output = Ok((
            Span::new_at("\n", 9, 1, 10),
            Variable{ name: Span::new_at("foo", 0, 1, 1), scope: VariableScope::Global, constant: true }
        ));

        assert_eq!(global_constant(input), output);
    }

    #[test]
    fn case_global_constant_with_whitespace() {
        let input  = Span::new("CONST       foo\n");
        let output = Ok((
            Span::new_at("\n", 15, 1, 16),
            Variable{ name: Span::new_at("foo", 0, 1, 1), scope: VariableScope::Global, constant: true }
        ));

        assert_eq!(global_constant(input), output);
    }
}