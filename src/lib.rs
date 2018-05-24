use std::io::{self, Read, Write};
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub enum Lang {
    BF,
}

#[derive(Debug)]
pub struct Program {
    lang: Lang,
    sourcecode: String,
    //tokenlist, ast, etc?
    //commandlist: Tape<Command>,
    //bytecode: Tape<i8>,
}

#[derive(Debug)]
enum Flag {
    Zero,
    Overflow,
}

#[derive(Debug, Clone)]
struct Tape<T: Copy> {
    data: Vec<T>,
    cursor: usize,
}

#[derive(Debug)]
pub struct STVM {
    program: Program,
    sourcecode: String,
    tape: Tape<i8>,
    bytecode: Tape<Command>,
    stack: Tape<i8>,
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
    Push(i8),
    Pop,
}

impl Program {
    pub fn new(lang: Lang, sourcecode: &str) -> Self {
        Self {lang, sourcecode: sourcecode.to_string()}
    }

    pub fn from_file(filename: String) -> Self {
        // FIXME: choose source language from file name extension
        let lang = Lang::BF;

        use std::fs::File;
        let mut f = File::open(filename).expect("file not found");
        let mut sourcecode = String::new();
        f.read_to_string(&mut sourcecode).expect("something went wrong reading the file");

        Self::new(lang, &sourcecode)
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

    fn len(&mut self) -> usize {
        self.data.len()
    }

    fn push(&mut self, n: T) {
        self.data.push(n);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

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
                sourcecode: String::from(""),
                lang: Lang::BF,
                //commandlist: Tape { data: vec![], cursor: 0 },
                //bytecode: Tape { data: vec![], cursor: 0 },
            },
            tape: Tape { data: vec![0], cursor: 0 },
            stack: Tape { data: vec![0], cursor: 0 },
            sourcecode: String::from(""),
            bytecode: Tape { data: vec![], cursor: 0 },
        }
    }

    pub fn from_code(code: &str) -> Self {
        let mut vm = Self::new();
        vm.compile(code);
        vm
    }

    pub fn from_file(filename: &str) -> Self {
        use std::fs::File;
        let mut f = File::open(filename).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("something went wrong reading the file");

        Self::from_code(&contents)
    }

    pub fn compile(&mut self, code: &str) {
        use self::Command::*;
        use std::collections::HashMap;
        self.sourcecode = String::from(code);

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
                self.bytecode.push(com);
            }
        }
        self.bytecode.push(HaltAlways);

        self.optim();

        let mut stack = vec![];
        for index in 0..self.bytecode.len() {
            let com = self.bytecode[index];
            match com {
                StartLoop => {
                    stack.push(index);
                }
                EndLoop => {
                    let f = match stack.pop() {
                        None => panic!("Unmatched bracket!"),
                        Some(x) => x,
                    };
                    self.bytecode[index] = JumpIfNonzero(f);
                    self.bytecode[f] = JumpIfZero(index);
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

    fn optim(&mut self) {
        use self::Command::*;

        println!("Optimizing");
        //println!("{:?}", self.bytecode);
        let old_length = self.bytecode.len();
        if self.bytecode.len() == 0 {
            println!("command list is empty, skipping optimize");
            return;
        };
        println!("length: {} commands\n", self.bytecode.len());

        let mut old = Tape { data: vec![], cursor: 0 };

        std::mem::swap(&mut old, &mut self.bytecode);
        let mut count;
        let mut index = 0;
        while index < old.len() {
            let current = old[index];
            match current {
                Inc | Dec | IncTape | DecTape => {
                    count = 1;
                    while index < old.len() - 1 && // next is within bounds
                        old[index + 1] == current && // same commannd
                        count < 127 // less than an i8, as Add takes an i8
                    {
                        count += 1;
                        index += 1;
                    }
                    match current {
                        Inc => self.bytecode.push(Add(count as i8)),
                        Dec => self.bytecode.push(Add(-count as i8)),
                        IncTape => self.bytecode.push(MoveTape(count as isize)),
                        DecTape => self.bytecode.push(MoveTape(-(count as isize))),
                        _ => panic!(),
                    };
                },
                StartLoop => {
                    if index < old.len() - 1 && old[index + 1] == EndLoop {
                        index += 1;
                        self.bytecode.push(HaltIfNonzero)
                    } else {
                        self.bytecode.push(current)
                    }
                },
                _ => self.bytecode.push(current),
            };
            index += 1;
        }

        println!("First pass");
        //println!("{:?}", self.bytecode);
        println!("length after first pass: {} commands", self.bytecode.len());
        println!("ratio: {}%\n", (self.bytecode.len() * 100) / old_length);

        old = Tape { data: vec![], cursor: 0 };
        std::mem::swap(&mut old, &mut self.bytecode);

        let mut index = 0;
        while index < old.len() {
            if index < old.len() - 2 {
                match &old.data[index..index + 3] {
                &[StartLoop, middle, EndLoop] => {
                    match middle {
                        Add(1) | Add(-1) => {
                            self.bytecode.push(Set(0));
                            index += 2;
                        },
                        MoveTape(x) => {
                            self.bytecode.push(Seek(x));
                            index += 2;
                        },
                        _ => self.bytecode.push(old[index]),
                    };
                },
                  _ => self.bytecode.push(old[index]),
                }
            } else {
                self.bytecode.push(old[index]);
            }
            index += 1;
        }

        println!("Second Pass");
        //println!("{:?}", self.bytecode);
        println!("length after second pass: {} commands", self.bytecode.len());
        println!("ratio: {}%\n", (self.bytecode.len() * 100) / old_length);
    }

    pub fn step(&mut self) -> bool {
        // probably want to return a Result or something like it
        use self::Command::*;
        let mut halt = true;
        if self.bytecode.get_cursor() < self.bytecode.len() {
            // don't forget iterator .next()
            //println!("at {}", self.bytecode.get_cursor());

            halt = false;

            let com = self.bytecode.read();
            match com {
                Add(n) => if self.tape.add(n) {self.set_flag(Flag::Overflow)},
                Set(n) => self.tape.write(n),
                MoveTape(n) => self.tape.move_cursor(n),
                Seek(n) => while self.read() != 0 { // TODO: Optimize?
                    self.tape.move_cursor(n)
                },
                JumpIfZero(target) => if self.tape.read() == 0 {
                    self.bytecode.jump(target);
                },
                JumpIfNonzero(target) => if self.tape.read() != 0 {
                    self.bytecode.jump(target);
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
                Push(n) => self.stack.push(n),
                Pop => match self.stack.pop() {
                    Some(n) => self.tape.write(n),
                    None => panic!("attempt to pop empty stack"),
                },
                _ => panic!("unexpected command {:?} in compiled code", com),
            }
            //println!("{:?} executed!", com);
            self.bytecode.move_cursor(1);
            //println!("going to {}", self.bytecode.cursor);
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

    pub fn debug_print(&self) -> () {
        println!("{:?}", self.tape);
    }

    fn set_flag(&mut self, flag: Flag) -> () {
        println!("Flag {:?} set!", flag)
    }

    fn clear_flag(&mut self, flag: Flag) -> () {
        println!("Flag {:?} cleared!", flag)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut test_vm = super::STVM::new();
        test_vm.compile("+++++[>+++<-]>");
        while test_vm.step() {};
        assert_eq!(test_vm.read(), 15);
    }
}
