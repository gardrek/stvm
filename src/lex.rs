#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub raw: String,
}

#[derive(Debug)]
pub enum TokenKind {
    Paren,
    Number,
    String,
    Name,
    //Error,
}

pub struct AST {
    pub nodelist: Vec<ASTNode>,
}

pub struct ASTNode {
    pub kind: ASTNodeKind,
    pub raw: String,
    pub params: Option<Vec<usize>>, // index into AST nodelist
}

pub enum ASTNodeKind {
    Root,
    NumberLiteral,
    CallExpression,
}
