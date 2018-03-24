/// Helper to declare a token.
macro_rules! token {
    ($name:ident: $value:expr; $documentation:expr) => (
        #[doc=$documentation]
        const $name: &'static str = $value;
    );

    (pub $name:ident: $value:expr; $documentation:expr) => (
        #[doc=$documentation]
        pub const $name: &'static str = $value;
    )
}

token!(
    pub ADD: "+";
    "The `ADD` token.\n\nRepresent the addition operator, e.g. `x + y`."
);

token!(
    pub ADD_AND_ASSIGN: "+=";
    "The `ADD_AND_ASSIGN` token.\n\nRepresent the addition assignment operator, e.g. `x += y;`."
);

token!(
    pub SUBSTRACT: "-";
    "The `SUBSTRACT` token.\n\nRepresent the subtraction operator, e.g. `x - y`."
);

token!(
    pub SUBSTRACT_AND_ASSIGN: "-=";
    "The `SUBSTRACT_AND_ASSIGN` token.\n\nRepresent the subtraction assignment operator, e.g. `x -= y;`."
);

token!(
    pub MULTIPLY: "*";
    "The `MULTIPLY` token.\n\nRepresent the multiplication operator, e.g. `x * y`."
);

token!(
    pub DIVIDE: "/";
    "The `DIVIDE` token.\n\nRepresent the division operator, e.g. `x / y`."
);

token!(
    pub MODULO: "%";
    "The `MODULO` token.\n\nRepresent the modulus operator, e.g. `x % y`."
);

token!(
    pub INCREMENT: "++";
    "The `INCREMENT` token.\n\nRepresent the increment operator, e.g. `number++`."
);

token!(
    pub DECREMENT: "--";
    "The `DECREMENT` token.\n\nRepresent the decrement operator, e.g. `number--`."
);

token!(
    pub LESS_THAN: "<";
    "The `LESS_THAN` token.\n\nRepresent the less than comparison operator, e.g. `x < y`."
);

token!(
    pub LESS_THAN_OR_EQUAL_TO: "<=";
    "The `LESS_THAN_OR_EQUAL_TO` token.\n\nRepresent the less than or equal to comparison operator, e.g. `x <= y`."
);

token!(
    pub GREATER_THAN: ">";
    "The `GREATER_THAN` token.\n\nRepresent the greater than comparison operator, e.g. `x > y`."
);

token!(
    pub GREATER_THAN_OR_EQUAL_TO: ">=";
    "The `GREATER_THAN_OR_EQUAL_TO` token.\n\nRepresent the greater than or equal to comparison operator, e.g. `x >= y`."
);

token!(
    pub IF: "if";
    "The `IF` token.\n\nRepresent the truly block of a condition control structure, e.g. `if (…) { … }`."
);

token!(
    pub ELSE: "else";
    "The `ELSE` token.\n\nRepresent the falsy block of a condition control structure, e.g. `if (…) { … } else { … }`."
);

token!(
    pub EQUAL: "==";
    "The `EQUAL` token.\n\nRepresent the equality comparison operator, e.g. `x == y`."
);

token!(
    pub NOT_EQUAL: "!=";
    "The `NOT_EQUAL` token.\n\nRepresent the not equal comparison operator, e.g. `x != y`."
);

token!(
    pub INLINE_COMMENT: "//";
    "THe `INLINE_COMMENT` token.\n\nRepresent an inline comment, e.g. `// comment`."
);

token!(
    pub BLOCK_COMMENT_OPEN: "/*";
    "The `BLOCK_COMMENT_OPEN` token.\n\nRepresent the beginning of a block comment, e.g. `/* comment */`."
);

token!(
    pub BLOCK_COMMENT_CLOSE: "*/";
    "The `BLOCK_COMMENT_CLOSE` token.\n\nRepresent the end of a block comment, e.g. `/* comment */`."
);

token!(
    pub BOOLEAN_AND: "&&";
    "The `BOOLEAN_AND` token.\n\nRepresent the boolean conjunction operator, e.g. `x && y`."
);

token!(
    pub BOOLEAN_NOT: "!";
    "The `NOT` token.\n\nRepresent the boolean negation operator, e.g. `!x`."
);

token!(
    pub BOOLEAN_OR: "||";
    "The `BOOLEAN_OR` token.\n\nRepresent the boolean disjunction operator, e.g. `x || y`."
);

token!(
    pub BITWISE_AND: "&";
    "The `BITWISE_AND` token.\n\nRepresent the bitwise conjunction operator, e.g. `x & y`."
);

token!(
    pub BITWISE_AND_AND_ASSIGN: "&=";
    "The `BITWISE_AND_AND_ASSIGN` token.\n\nRepresent the bitwise conjunction assignment operator, e.g. `x &= y;`."
);

token!(
    pub BITWISE_LEFT_SHIFT: "<<";
    "The `BITWISE_LEFT_SHIFT` token.\n\nRepresent the bitwise left shift operator, e.g. `x << y`."
);

token!(
    pub BITWISE_LEFT_SHIFT_AND_ASSIGN: "<<=";
    "The `BITWISE_LEFT_SHIFT_AND_ASSIGN` token.\n\nRepresent the bitwise left shift assignment operator, e.g. `x <<= y;`."
);

token!(
    pub BITWISE_NOT: "~";
    "The `BITWISE_NOT` token.\n\nRepresent the bitwise negation operator, e.g. `~x`."
);

token!(
    pub BITWISE_OR: "|";
    "The `BITWISE_OR` token.\n\nRepresent the inclusive bitwise disjunction operator, e.g. `x | y`."
);

token!(
    pub BITWISE_OR_AND_ASSIGN: "|=";
    "The `BITWISE_OR_AND_ASSIGN` token.\n\nRepresent the inclusive bitwise disjunction assignment operator, e.g. `x |= y;`."
);

token!(
    pub BITWISE_RIGHT_SHIFT: ">>";
    "The `BITWISE_RIGHT_SHIFT` token.\n\nRepresent the bitwise right shift operator, e.g. `x >> y`."
);

token!(
    pub BITWISE_RIGHT_SHIFT_AND_ASSIGN: ">>=";
    "The `BITWISE_RIGHT_SHIFT_AND_ASSIGN` token.\n\nRepresent the bitwise right shift assignment operator, e.g. `x >>= y;`."
);

token!(
    pub BITWISE_XOR: "^";
    "The `BITWISE_XOR` token.\n\nRepresent the exclusive bitwise disjunction operator, e.g. `x ^ y`."
);

token!(
    pub BITWISE_XOR_AND_ASSIGN: "^=";
    "The `BITWISE_XOR_AND_ASSIGN` token.\n\nRepresent the exclusive bitwise disjunction assignment operator, e.g. `x ^= y;`."
);

token!(
    pub TAG: "#";
    "The `TAG` token.\n\nRepresent a tag, e.g. `# tag`."
);

token!(
    pub CHOICE: "*";
    "The `CHOICE` token.\n\nRepresent a text choice, e.g. `* choice`."
);

token!(
    pub STICKY_CHOICE: "+";
    "The `STICKY_CHOICE` token.\n\nRepresent a sticky choice, e.g. `+ choice`."
);

token!(
    pub DIVERT: "->";
    "The `DIVERT` token.\n\nRepresent a divert, e.g. `-> knot`."
);

token!(
    pub GLUE: "<>";
    "The `GLUE` token.\n\nRepresent a glue, e.g. `We hurried home <>`."
);

token!(
    pub STITCH: "=";
    "The `STITCH` token.\n\nRepresent a stitch, e.g. `= in_first_class`."
);

token!(
    pub REFERENCE: "ref";
    "The `REFERENCE` token.\n\nRepresent the declaration operator, e.g. `ref x`."
);

token!(
    pub LEFT_PARENTHESIS: "(";
    "The `LEFT_PARENTHESIS` token.\n\nUsed to open a group, e.g. `(a, b)`."
);

token!(
    pub RIGHT_PARENTHESIS: ")";
    "The `RIGHT_PARENTHESIS` token.\n\nUsed to close a group, e.g. `(a, b)`."
);

token!(
    pub LEFT_SQUARE_BRACKET: "[";
    "The `LEFT_SQUARE_BRACKET` token.\n\nRepresent the beginning of a suppressing choice text, e.g. `* [choice]`."
);

token!(
    pub RIGHT_SQUARE_BRACKET: "[";
    "The `RIGHT_SQUARE_BRACKET` token.\n\nRepresent the end of a suppressing choice text, e.g. `* [choice]`."
);

token!(
    pub LEFT_CURLY_BRACKET: "{";
    "The `LEFT_CURLY_BRACKET` token.\n\nUsed to open a block, e.g. `{ … }`."
);

token!(
    pub RIGHT_CURLY_BRACKET: "}";
    "The `RIGHT_CURLY_BRACKET` token.\n\nUsed to close a block, e.g. `{ … }`."
);

token!(
    pub END: "END";
    "The `END` token.\n\nRepresent the end of the flow, e.g. `-> END`."
);

token!(
    pub DONE: "DONE";
    "The `DONE` token.\n\nRepresent the end of a thread, e.g. `-> DONE`."
);

token!(
    pub INCLUDE: "INCLUDE";
    "The `INCLUDE` token.\n\nRepresent an include, e.g. `INCLUDE newspaper.ink`."
);

token!(
    pub ASSIGN: "=";
    "The `ASSIGN` token.\n\nRepresent a binding of a value to a variable, e.g. `x = 42`."
);

token!(
    pub GLOBAL_VARIABLE: "VAR";
    "The `GLOBAL_VARIABLE` token.\n\nDefinition of a global variable, e.g. `VAR x = 42`."
);

token!(
    pub CONSTANT: "CONST";
    "The `CONSTANT` token.\n\nRepresent the constant declaration operator, e.g. `CONST PI = 3.14`."
);

token!(
    pub STATEMENT: "~";
    "The `STATEMENT` token.\n\nRepresent a statement, e.g. `~ x = (x * x) - (y * y) + c`."
);

token!(
    pub RETURN: "return";
    "The `RETURN` token.\n\nRepresent the return operator, e.g. `return x;`."
);

token!(
    pub TUNNEL_END: "->->";
    "The `TUNNEL_RETURN` token.\n\nRepresent the end of a tunnel, e.g. `->->`."
);

token!(
    pub THREAD: "<-";
    "The `THREAD` token.\n\nRepresent a thread, e.g. `<- walking`."
);

token!(
    pub LIST: "LIST";
    "The `LIST` token.\n\nRepresent a list, e.g. `LIST kettleState = cold, boiling, recently_boiled`."
);
