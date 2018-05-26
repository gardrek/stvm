/*!
 * Stack-Tape Virtual Machine
 */

mod lex;

use std::io::{self, Read, Write};
use std::ops::{Index, IndexMut};
use lex::*;

/// Supported languages for compiling
#[derive(Debug)]
pub enum Lang {
    BF,
    LISP,
    _LANG,
}

/// A program's source code and compiled bytecode
#[derive(Debug)]
pub struct Program {
    lang: Lang,
    sourcecode: String,
    //tokenlist, ast, etc?
    commandlist: Tape<Command>,
    //bytecode: Tape<i8>,
}

#[derive(Debug)]
struct Flags {
    zero: bool,
    overflow: bool,
}

#[derive(Debug)]
struct RegisterSet {
    flags: Flags,
}

#[derive(Debug, Clone)]
struct Tape<T: Copy> {
    data: Vec<T>,
    cursor: usize,
}

#[derive(Debug)]
pub struct STVM {
    program: Program,
    tape: Tape<i8>,
    bytecode: Tape<Command>,
    stack: Tape<i8>,
    registerset: RegisterSet
    //input: Vec<i8>, // should be a FIFO tho
    //output: Vec<i8>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Command {
    //Nop,
    Inc,
    Dec,
    IncTape,
    DecTape,
    OutputByte,
    InputByte,
    StartLoop,
    EndLoop,
    JumpIfZero(usize),
    JumpIfNonzero(usize),
    Add(i8),
    MoveTape(isize),
    Set(i8),
    Seek(isize),
    HaltAlways,
    HaltIfNonzero,
    //Push(i8),
    //pPop,
}

impl RegisterSet {
    pub fn new() -> Self {
        Self {
            flags: Flags {
                zero: false,
                overflow: false,
            },
        }
    }
}

impl Program {
    pub fn new(lang: Lang, sourcecode: &str) -> Self {
        Self {
            lang,
            sourcecode: sourcecode.to_string(),
            commandlist: Tape { data: vec![], cursor: 0 },
        }
    }

    pub fn from_file(filename: &str) -> Self {
        // FIXME: choose source language from file name extension
        let lang = Lang::BF;

        use std::fs::File;
        let mut f = File::open(filename).expect("file not found");
        let mut sourcecode = String::new();
        f.read_to_string(&mut sourcecode).expect("something went wrong reading the file");

        Self::new(lang, &sourcecode)
    }

    fn compile(&mut self) {
        match self.lang {
            Lang::BF => self.compile_bf(),
            Lang::LISP => self.compile_lisp(),
            _ => unimplemented!(),
        }

    }

    // BF specific stuff
    fn compile_bf(&mut self) {
        //if self.lang != Lang::BF {panic!("")};
        match self.lang {
            Lang::BF => (),
            _ => panic!("tried to compile wrong language"),
        }

        use self::Command::*;
        use std::collections::HashMap;

        let mut ops = HashMap::new();
        ops.insert('+', Inc);
        ops.insert('-', Dec);
        ops.insert('>', IncTape);
        ops.insert('<', DecTape);
        ops.insert('.', OutputByte);
        ops.insert(',', InputByte);
        ops.insert('[', StartLoop);
        ops.insert(']', EndLoop);

        for c in self.sourcecode.chars() {
            if let Some(&com) = ops.get(&c) {
                self.commandlist.push(com);
            }
        }
        self.commandlist.push(HaltAlways);

        self.optim_bf();

        let mut stack = vec![];
        for index in 0..self.commandlist.len() {
            let com = self.commandlist[index];
            match com {
                StartLoop => {
                    stack.push(index);
                }
                EndLoop => {
                    let f = match stack.pop() {
                        None => panic!("Unmatched bracket!"),
                        Some(x) => x,
                    };
                    self.commandlist[index] = JumpIfNonzero(f);
                    self.commandlist[f] = JumpIfZero(index);
                }
                _ => {
                    //
                }
            };
        }
        if stack.len() != 0 {
            panic!("unmatched bracket!")
        };
    }

    fn optim_bf(&mut self) {
        use self::Command::*;

        println!("Optimizing");
        //println!("{:?}", self.commandlist);
        let old_length = self.commandlist.len();
        if self.commandlist.len() == 0 {
            println!("command list is empty, skipping optimize");
            return;
        };
        println!("length: {} commands\n", self.commandlist.len());

        let mut old = Tape { data: vec![], cursor: 0 };

        std::mem::swap(&mut old, &mut self.commandlist);
        let mut count:isize;
        let mut index = 0;
        while index < old.len() {
            let current = old[index];
            match current {
                Inc | Dec | IncTape | DecTape => {
                    count = 1;
                    while index < old.len() - 1 // next is within bounds
                        && old[index + 1] == current // same commannd
                        //count < 127 // less than i8 max, as Add takes an i8
                        // TODO: since MoveTape takes an isize, perhaps it
                        // would make sense to support more than ±127 for it
                    {
                        match current {
                            Inc | Dec => if count >= std::i8::MAX as isize {break},
                            IncTape | DecTape => if count >= std::isize::MAX {break},
                            _ => panic!(),
                        };
                        count += 1;
                        index += 1;
                    }
                    match current {
                        Inc => self.commandlist.push(Add(count as i8)),
                        Dec => self.commandlist.push(Add(-count as i8)),
                        IncTape => self.commandlist.push(MoveTape(count)),
                        DecTape => self.commandlist.push(MoveTape(-count)),
                        _ => panic!(),
                    };
                },
                StartLoop => {
                    if index < old.len() - 1 && old[index + 1] == EndLoop {
                        index += 1;
                        self.commandlist.push(HaltIfNonzero)
                    } else {
                        self.commandlist.push(current)
                    }
                },
                _ => self.commandlist.push(current),
            };
            index += 1;
        }

        println!("First pass");
        //println!("{:?}", self.commandlist);
        println!("length after first pass: {} commands", self.commandlist.len());
        println!("ratio: {}%\n", (self.commandlist.len() * 100) / old_length);

        old = Tape { data: vec![], cursor: 0 };
        std::mem::swap(&mut old, &mut self.commandlist);

        let mut index = 0;
        while index < old.len() {
            if index < old.len() - 2 {
                match &old.data[index..index + 3] {
                &[StartLoop, middle, EndLoop] => {
                    match middle {
                        Add(1) | Add(-1) => {
                            self.commandlist.push(Set(0));
                            index += 2;
                        },
                        MoveTape(x) => {
                            self.commandlist.push(Seek(x));
                            index += 2;
                        },
                        _ => self.commandlist.push(old[index]),
                    };
                },
                  _ => self.commandlist.push(old[index]),
                }
            } else {
                self.commandlist.push(old[index]);
            }
            index += 1;
        }

        println!("Second Pass");
        //println!("{:?}", self.commandlist);
        println!("length after second pass: {} commands", self.commandlist.len());
        println!("ratio: {}%\n", (self.commandlist.len() * 100) / old_length);
    }

    // LISP
    fn tokenize_lisp(&mut self) -> Vec<Token> {
        use TokenKind::*;

        let mut cursor = 0;
        let mut tokens = Vec::new();
        let bytevec = self.sourcecode.clone().into_bytes();
        let mut chr;
        
        while cursor < bytevec.len() {
            chr = bytevec[cursor];
            match chr {
                 b'(' | b')' => tokens.push(Token {
                    kind: Paren,
                    raw: (chr as char).to_string(),
                }),
                b'0' ... b'9' => {
                    let mut raw = "".to_string();

                    while chr >= b'0' && chr <= b'9' {
                        raw.push(chr as char);
                        cursor += 1;
                        if cursor >= bytevec.len() {break}
                        chr = bytevec[cursor];
                    }

                    tokens.push(Token { kind: Number, raw });
                    cursor -= 1;
                },
                b'a' ... b'z' => {
                    let mut raw = "".to_string();

                    while chr >= b'a' && chr <= b'z' {
                        raw.push(chr as char);
                        cursor += 1;
                        if cursor >= bytevec.len() {break}
                        chr = bytevec[cursor];
                    }

                    tokens.push(Token { kind: Name, raw });
                },
                b'"' => {
                    let mut raw = "".to_string();

                    cursor += 1;
                    if cursor >= bytevec.len() {break}
                    chr = bytevec[cursor];

                    while chr != b'"' {
                        raw.push(chr as char);
                        cursor += 1;
                        if cursor >= bytevec.len() {break}
                        chr = bytevec[cursor];
                    }

                    tokens.push(Token { kind: String, raw });
                },
                b' ' | b'\t' | b'\n' | b'\r' => (),
                _ => (),
            }
            cursor += 1;
        }

        //println!("{:?}", tokens);

        for v in tokens.iter() {
            println!("{:?}", v);
        }

        println!("\n");

        tokens
    }

    fn parse_lisp(&self, tokens: Vec<Token>) -> AST {
        use TokenKind::*;
        use ASTNodeKind::*;

        let mut ast = AST {
            nodelist: vec![],
        };

        ast.nodelist.push(ASTNode {
            kind: Root,
            raw: "".to_string(),
            params: Some(vec![]),
        });

        {
            // this scode is to make sure ast is not borrowed from when we return
            // idk if it is a good way to do it or not tho

            let mut cursor = 0;

            let mut current_node = &ast.nodelist[0];

            let mut node_stack = vec![];

            node_stack.push(current_node);

            while cursor < tokens.len() {
                let token = &tokens[cursor];
                let node = match token.kind {
                    Number => {
                        cursor += 1;
                        ASTNode {
                            kind: NumberLiteral,
                            raw: token.raw.clone(),
                            params: None,
                        }
                    },
                    _ => unimplemented!(),
                };
                cursor += 1;
            };
        };

        ast
    }

    fn compile_lisp(&mut self) {
        let tokens = self.tokenize_lisp();
        let _ast = self.parse_lisp(tokens);
    }
}

impl<T: Copy> Index<usize> for Tape<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        &self.data[i]
    }
}

impl<T: Copy> IndexMut<usize> for Tape<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        &mut self.data[i]
    }
}

impl<T: Copy> Tape<T> {
    fn get_cursor(&self) -> usize {
        self.cursor
    }

    fn jump(&mut self, target: usize) {
        if target >= self.data.len() {
            panic!("Tape pointer jump target is outside right bound")
        };
        self.cursor = target;
    }

    fn read(&self) -> T {
        self[self.cursor]
    }

    fn write(&mut self, value: T) {
        let index = self.cursor;
        self[index] = value;
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn push(&mut self, n: T) {
        self.data.push(n);
    }

    //fn pop(&mut self) -> Option<T> {
        //self.data.pop()
    //}

    pub fn iter(&self) -> std::slice::Iter<'_, T> { self.data.iter() }
}

impl Tape<Command> {
    fn move_cursor(&mut self, change: isize) {
        let m = self.cursor as isize + change;
        if m < 0 {
            panic!("Tape pointer outside left bound")
        };
        /*if m >= self.data.len() as isize {
            println!("[End of Program]")
        };*/
        self.cursor = m as usize;
    }
}

impl Tape<i8> {
   fn move_cursor(&mut self, change: isize) {
        let m = self.cursor as isize + change;
        if m < 0 {
            panic!("Tape pointer outside left bound")
        };
        if m >= self.data.len() as isize {
            self.data.resize((m + 1) as usize, 0i8)
        };
        self.cursor = m as usize;
    }

    fn add(&mut self, n: i8) -> bool {
        let m = self.data[self.cursor];
        let (v, overflow) = m.overflowing_add(n);
        self.data[self.cursor] = v;
        overflow
    }
}

/*impl<T: Copy> Iterator for Tape<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}*/

impl STVM {
    pub fn new() -> Self {
        Self {
            program: Program {
                lang: Lang::BF,
                sourcecode: String::from(""),
                commandlist: Tape { data: vec![], cursor: 0 },
                //bytecode: Tape { data: vec![], cursor: 0 },
            },
            tape: Tape { data: vec![0], cursor: 0 },
            stack: Tape { data: vec![0], cursor: 0 },
            bytecode: Tape { data: vec![], cursor: 0 },
            registerset: RegisterSet::new(),
        }
    }

    fn set_program(&mut self, program: Program) {
        self.program = program;
    }

    pub fn from_code(lang: Lang, sourcecode: &str) -> Self {
        let mut vm = Self::new();
        vm.set_program(Program::new(lang, sourcecode));
        vm.compile();
        vm
    }

    pub fn from_file(filename: &str) -> Self {
        let mut vm = Self::new();
        vm.set_program(Program::from_file(filename));
        vm.compile();
        vm
    }

    pub fn compile(&mut self) {
        self.program.compile()
    }

    pub fn step(&mut self) -> bool {
        // probably want to return a Result or something like it
        use self::Command::*;
        let mut halt = true;
        if self.program.commandlist.get_cursor() < self.program.commandlist.len() {
            // don't forget iterator .next()
            halt = false;

            let com = self.program.commandlist.read();
            match com {
                Add(n) => if self.tape.add(n) {self.set_overflow_flag()},
                Set(n) => self.tape.write(n),
                MoveTape(n) => self.tape.move_cursor(n),
                Seek(n) => while self.read() != 0 { // TODO: Optimize?
                    self.tape.move_cursor(n)
                },
                JumpIfZero(target) => if self.tape.read() == 0 {
                    self.program.commandlist.jump(target);
                },
                JumpIfNonzero(target) => if self.tape.read() != 0 {
                    self.program.commandlist.jump(target);
                },
                InputByte => {
                    // TODO: fix it so it takes input
                    // immediately instead of waiting for line end
                    let mut buffer = [0u8; 1];
                    let mut stdin = io::stdin();
                    stdin.lock();
                    match stdin.read(&mut buffer) {
                        Err(e) => panic!(e),
                        Ok(n) => if n == 1 {
                            self.tape.write(buffer[0] as i8);
                        } else if n == 0 {
                            halt = true;
                            println!("no bytes read from input, halting")
                        } else {
                            panic!("wrong number of bytes read! {} bytes", n)
                        },
                    }
                },
                OutputByte => {
                    print!("{}", self.tape.read() as u8 as char);
                    io::stdout().flush().unwrap();
                },
                HaltIfNonzero => {
                    if self.tape.read() != 0 {halt = true;};
                },
                HaltAlways => halt = true,
                //Push(n) => self.stack.push(n),
                //Pop => match self.stack.pop() {
                    //Some(n) => self.tape.write(n),
                    //None => panic!("attempt to pop empty stack"),
                //},
                _ => panic!("unexpected command {:?} in compiled code", com),
            }
            //println!("{:?} executed!", com);
            self.program.commandlist.move_cursor(1);
            //println!("going to {}", self.program.commandlist.cursor);
            //println!("");
        };
        !halt
    }

    pub fn run(&mut self) {
        while self.step() {};
    }

    pub fn read(&self) -> i8 {
        self.tape.read()
    }

    pub fn get_cursor(&self) -> usize { self.tape.cursor }

    pub fn each_cell(&self) -> std::slice::Iter<'_, i8> { self.tape.iter() }

    pub fn debug_print(&self) {
        let l = self.program.commandlist.len();
        if l < 2 {return};
        println!("{:?}", self.program.commandlist[l - 2]);
    }

    fn set_overflow_flag(&mut self) -> () {
        //println!("overflow flag set!")
    }

    //fn clear_overflow_flag(&mut self) -> () {
        ////println!("overflow flag cleared!")
    //}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut test_vm = super::STVM::new();
        test_vm.set_program(Program {lang: Lang::bf, sourecode: "+++++[>+++<-]>"});
        while test_vm.step() {};
        assert_eq!(test_vm.read(), 15);
    }
}
