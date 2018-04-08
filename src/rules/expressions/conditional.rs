named_attr!(
    #[doc="
        Recognize all conditional expressions.
    "],
    pub conditional<Span, Expression>,
    map_res!(
        logical,
        nary_expression_mapper
    )
);

#[inline]
fn conditional_mapper<'a>(nary_operation: NAryOperation<'a>) -> Result<Expression<'a>, ()> {
    Ok(Expression::NAryOperation(nary_operation))
}