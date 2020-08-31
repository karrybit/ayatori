#[derive(Clone, PartialEq, Debug)]
pub(crate) enum TokenType {
    Literal(String),
    LBrace,
    RBrace,
    Equal,
    HearDoc(String),
    Illegal(char),
    EOF,
}
