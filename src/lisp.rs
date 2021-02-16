use std::fmt;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub raw: String,
    pub line_number: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
    Root,
    CallExpression,
    Paren,
    NumberLiteral,
    StringLiteral,
    Name,
    Error,
}

#[derive(Debug)]
pub struct AstNode {
    pub kind: TokenKind,
    pub raw: String,
    pub children: Option<Vec<usize>>, // index into AST nodelist
    pub line_number: usize,
}

#[derive(Debug)]
pub struct Ast {
    pub nodelist: Vec<AstNode>,
}

impl Ast {
    pub fn new() -> Ast {
        Ast {
            nodelist: vec![AstNode {
                kind: TokenKind::Root,
                raw: "".into(),
                children: Some(vec![]),
                line_number: 0,
            }],
        }
    }
    /*pub fn add_node(&mut self, node: AstNode) -> usize {
        self.nodelist.push(node);
        self.nodelist.len() - 1
    }*/
}

impl fmt::Display for Ast {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::TokenKind::*;

        fn format_node(nodelist: &Vec<AstNode>, index: usize, s: String) -> String {
            let node = &nodelist[index];
            match node.kind {
                Name => format!("{:?}:{}, ", node.kind, node.raw),
                NumberLiteral => format!("{}, ", node.raw),
                StringLiteral => format!("{:?}, ", node.raw),
                _ => format!(
                    "{{ {:?} {} }} ",
                    node.kind,
                    match node.children {
                        Some(ref v) => {
                            let mut body = String::new();
                            for &i in v.iter() {
                                body += &format_node(nodelist, i, s.clone())
                            }
                            format!("{} {}", s, body)
                        }
                        None => s,
                    },
                ),
            }
        }

        // Write into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", format_node(&self.nodelist, 0, String::new()))
    }
}

impl AstNode {
    pub fn push_param(&mut self, node_id: usize) {
        match self.children {
            Some(ref mut v) => {
                v.push(node_id);
                return;
            }
            None => (),
        }
        self.children = Some(vec![node_id]);
    }
}

/*struct Parser {
    tokenlist: Vec<Token>,
    current_token: i64,
}*/

pub fn tokenize(source: &str) -> Vec<Token> {
    use self::TokenKind::*;

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
            b'0'..=b'9' => {
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
            b'a'..=b'z' => {
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
    use self::TokenKind::*;

    let mut ast = Ast::new();

    let mut current_token = 0;

    let mut stack = vec![0];

    println!("{}", ast);
    println!();

    fn add_node(nodelist: &mut Vec<AstNode>, node: AstNode, stack: &Vec<usize>) -> usize {
        use self::TokenKind::*;
        nodelist.push(node);
        let node_id = nodelist.len() - 1;
        let &prev_id = stack.last().unwrap();
        let parent_node = &mut nodelist[prev_id];
        match parent_node.kind {
            Root | CallExpression => {
                parent_node.push_param(node_id);
            }
            _ => panic!(),
        }
        node_id
    }

    while current_token < tokens.len() {
        let token = &tokens[current_token];
        println!("{:?}   stack: {:?}", token, stack);

        match token.kind {
            Error => panic!(),
            NumberLiteral | StringLiteral | Name => {
                add_node(
                    &mut ast.nodelist,
                    AstNode {
                        kind: token.kind,
                        raw: token.raw.clone(),
                        children: None,
                        line_number: token.line_number,
                    },
                    &stack,
                );
            }
            Paren => {
                match token.raw.as_ref() {
                    "(" => {
                        let node_id = add_node(
                            &mut ast.nodelist,
                            AstNode {
                                kind: CallExpression,
                                raw: String::new(),
                                children: None,
                                line_number: token.line_number,
                            },
                            &stack,
                        );
                        stack.push(node_id);
                    }
                    ")" => match stack.pop() {
                        None => panic!(),
                        _ => (), //current_token += 1,
                    },
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }

        current_token += 1;
    }

    println!();
    println!();
    println!("{}", ast);
    println!();

    {
        let mut i = 0;
        for v in ast.nodelist.iter() {
            println!("{}: {:?}", i, v);
            i += 1;
        }
    }

    println!();

    ast
}
