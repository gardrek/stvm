#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub raw: String,
    pub line_number: i64,
}

#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
    //Root,
    //CallExpression,
    Paren,
    NumberLiteral,
    StringLiteral,
    Name,
    Error,
}

pub struct Ast {
    pub nodelist: Vec<AstNode>,
}

impl Ast {
    /*pub fn add_node(&mut self, node: AstNode) -> usize {
        self.nodelist.push(node);
        self.nodelist.len() - 1
    }*/
}

pub struct AstNode {
    pub kind: TokenKind,
    pub raw: String,
    pub params: Option<Vec<usize>>, // index into AST nodelist
}

/*struct Parser {
    tokenlist: Vec<Token>,
    current_token: i64,
}*/

pub fn tokenize(source: &str) -> Vec<Token> {
    use TokenKind::*;

    let mut cursor = 0;
    let mut tokens = Vec::new();
    let bytevec = source.as_bytes();
    let mut chr;
    let mut has_error = false;
    let mut line_number = 1;

    while cursor < bytevec.len() {
        chr = bytevec[cursor];
        match chr {
            b'(' | b')' => tokens.push(Token {
                kind: Paren,
                raw: (chr as char).to_string(),
                line_number,
            }),
            b'0'...b'9' => {
                let mut raw = "".to_string();

                while chr >= b'0' && chr <= b'9' {
                    raw.push(chr as char);
                    cursor += 1;
                    if cursor >= bytevec.len() {
                        break;
                    }
                    chr = bytevec[cursor];
                }

                tokens.push(Token {
                    kind: NumberLiteral,
                    raw,
                    line_number,
                });
                cursor -= 1;
            }
            b'a'...b'z' => {
                let mut raw = "".to_string();

                while chr >= b'a' && chr <= b'z' {
                    raw.push(chr as char);
                    cursor += 1;
                    if cursor >= bytevec.len() {
                        break;
                    }
                    chr = bytevec[cursor];
                }

                tokens.push(Token {
                    kind: Name,
                    raw,
                    line_number,
                });
                cursor -= 1;
            }
            b'"' => {
                let mut raw = "".to_string();

                cursor += 1;
                if cursor >= bytevec.len() {
                    break;
                }
                chr = bytevec[cursor];

                while chr != b'"' {
                    raw.push(chr as char);
                    cursor += 1;
                    if cursor >= bytevec.len() {
                        break;
                    }
                    chr = bytevec[cursor];
                }

                tokens.push(Token {
                    kind: StringLiteral,
                    raw,
                    line_number,
                });
                // not doing cursor -= 1;
                // this way we skip the ending double-quote '"'
            }
            b' ' | b'\t' | b'\r' => (),
            b'\n' => line_number += 1,
            _ => {
                tokens.push(Token {
                    kind: Error,
                    raw: (chr as char).to_string(),
                    line_number,
                });
                has_error = true;
                //break; // Stop parsing tokens at the first error
            }
        }
        cursor += 1;
    }

    //println!("{:?}", tokens);

    if has_error {
        println!("E: Syntax error during lexing")
    }

    for v in tokens.iter() {
        println!("{:?}", v);
    }

    println!("\n");

    tokens
}

pub fn parse(tokens: Vec<Token>) -> Ast {
    println!("{}", tokens.len());
    unimplemented!()
    /*
    use TokenKind::*;

    fn walk(mut ast: &Ast, tokens: &Vec<Token>, mut current_token: usize, current_node: usize) -> (usize, AstNode) {
        let token = &tokens[current_token];

        //let current_node;

        match token.kind {
            Error => panic!(),
            NumberLiteral | StringLiteral => {
                current_token += 1;
                (current_token, AstNode {
                    kind: token.kind,
                    raw: token.raw.clone(),
                    params: None,
                })
            },
            Paren => {
                current_token += 1;
                token = &tokens[current_token];
                let node = AstNode {
                    kind: CallExpression,
                    raw: token.raw.clone(),
                    params: None,
                };
                current_token += 1;
                token = &tokens[current_token];
                while match token.kind {
                    Paren => token.raw != ")",
                    _ => true,
                } {
                    let (current_token, n) = walk(&ast, &tokens, current_token, current_node);
                }
                (current_token, ast)
            }
            _ => unimplemented!(),
        }
    }

    let mut ast = Ast {
        nodelist: vec![],
    };

    ast.nodelist.push(AstNode {
        kind: Root,
        raw: "".to_string(),
        params: Some(vec![]),
    });

    let (_, _node) = walk(&ast, &tokens, 0, 0);

    ast
    */
}
