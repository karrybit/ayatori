#[derive(Clone, PartialEq, Debug)]
pub(crate) enum TokenType {
    Literal(String),
    LBrace,
    RBrace,
    Equal,
    Illegal(char),
    EOF,
}
