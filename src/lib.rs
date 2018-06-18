/*!
 * Stack-Tape Virtual Machine
 */

mod lisp;

use std::io::{self, Read, Write};
use std::ops::{Index, IndexMut};
use lisp::*;

/// Supported languages for compiling
#[derive(Debug)]
pub enum Lang {
    Raw,
    BF,
    LISP,
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
struct RegisterSet {
    pub acc: i16,
    pub zero: bool,
    pub overflow: bool,
    pub stack_underflow: bool,
    pub tape_outside_right_bound: bool,
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
    //bytecode: Tape<i8>,
    stack: Tape<i8>,
    registers: RegisterSet
    //input: Vec<i8>, // should be a FIFO tho
    //output: Vec<i8>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Command {
    //Nop,
    OutputByte,
    InputByte,
    StartLoop,
    EndLoop,
    JumpAbsoluteIfZero(usize),
    JumpAbsoluteIfNonzero(usize),
    JumpRelativeLongIfZero(i16),
    JumpRelativeLongIfNonzero(i16),
    JumpRelativeShortIfZero(i8),
    JumpRelativeShortIfNonzero(i8),
    Sub(i8),
    MoveTape(i16),
    Set(i8),
    // Move the tape until a non-zero cell is found
    SeekRight,
    SeekLeft,
    HaltAlways,
    HaltIfNotEqual(i8),
    Push,
    Pop,
}

/*impl Command {
    pub fn len(&self) -> usize {
        use Command::*;
        match self {
            OutputByte |
            InputByte |
            StartLoop |
            EndLoop |
            Push |
            Pop |
            SeekRight |
            SeekLeft |
            HaltAlways => 1,
            Sub(_i8) |
            Set(_i8) |
            HaltIfNotEqual(_i8) => 2,
            JumpRelativeLongIfZero(_) |
            JumpRelativeLongIfNonzero(_) |
            MoveTape(_)  => 3,
        }
    }
}*/

impl RegisterSet {
    pub fn new() -> Self {
        Self {
            acc: 0,
            zero: false,
            overflow: false,
            stack_underflow: false,
            tape_outside_right_bound: true,
        }
    }
}

impl Program {
    pub fn new(lang: Lang, sourcecode: &str) -> Program {
        Program {
            lang,
            sourcecode: sourcecode.to_string(),
            commandlist: Tape::new(vec![]),
        }
    }

    pub fn from_file(filename: &str) -> Program {
        // TODO: choose source language from file name extension
        let lang = Lang::BF;

        use std::fs::File;
        let mut f = File::open(filename).expect("file not found");
        let mut sourcecode = String::new();
        f.read_to_string(&mut sourcecode)
            .expect("something went wrong reading the file");

        Program::new(lang, &sourcecode)
    }

    fn compile(&mut self) {
        match self.lang {
            Lang::BF => self.compile_bf(),
            Lang::LISP => self.compile_lisp(),
            _ => unimplemented!(),
        }
    }

    fn compile_lisp(&mut self) {
        let tokens = lisp::tokenize(&self.sourcecode);
        let _ast = lisp::parse(tokens);
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
        ops.insert('+', Sub(-1));
        ops.insert('-', Sub(1));
        ops.insert('>', MoveTape(1));
        ops.insert('<', MoveTape(-1));
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
        if self.commandlist.len() > std::u32::MAX as usize {
            panic!("length of command list exceeds 32-bit address max index")
        }
        for index in 0..self.commandlist.len() {
            let com = self.commandlist[index];
            match com {
                StartLoop => {
                    stack.push(index);
                }
                EndLoop => {
                    let start_index = match stack.pop() {
                        None => panic!("Unmatched bracket!"),
                        Some(x) => x,
                    };
                    let jump_distance = index - start_index;
                    if jump_distance <= std::i8::MAX as usize {
                        self.commandlist[start_index] = JumpRelativeShortIfZero(jump_distance as i8);
                        self.commandlist[index] = JumpRelativeShortIfNonzero(-(jump_distance as i8));
                    } else if jump_distance <= std::i16::MAX as usize {
                        self.commandlist[start_index] = JumpRelativeLongIfZero(jump_distance as i16);
                        self.commandlist[index] = JumpRelativeLongIfNonzero(-(jump_distance as i16));
                    } else {
                        if index > std::u32::MAX as usize || start_index > std::u32::MAX as usize {
                            panic!("jump target exceeds 32-bit address max index")
                        }
                        self.commandlist[start_index] = JumpAbsoluteIfZero(index);
                        self.commandlist[index] = JumpAbsoluteIfNonzero(start_index);
                    }
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

        let mut old = Tape::new(vec![]);

        std::mem::swap(&mut old, &mut self.commandlist);
        let mut count: isize;
        let mut next_command;
        let mut index = 0;
        while index < old.len() {
            let current = old[index];
            match current {
                Sub(argument) => {
                    count = argument as isize;
                    while index + 1 < old.len() {
                        next_command = old[index + 1];
                        match next_command {
                            Sub(n) => {
                                if count.abs() >= std::i8::MAX as isize {break}
                                count += n as isize;
                            },
                            _ => break,
                        }
                        index = index + 1;
                    }
                    self.commandlist.push(Sub(count as i8));
                },
                MoveTape(argument) => {
                    count = argument as isize;
                    while index + 1 < old.len() {
                        next_command = old[index + 1];
                        match next_command {
                            MoveTape(n) => {
                                // NOTE: the following line limits moves to ±32767
                                if count.abs() >= std::i16::MAX as isize {break}
                                count += n as isize;
                            },
                            _ => break,
                        }
                        index = index + 1;
                    }
                    self.commandlist.push(MoveTape(count as i16));
                },
                StartLoop => {
                    if index < old.len() - 1 && old[index + 1] == EndLoop {
                        index += 1;
                        self.commandlist.push(HaltIfNotEqual(0))
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

        old = Tape::new(vec![]);
        std::mem::swap(&mut old, &mut self.commandlist);

        let mut index = 0;
        while index < old.len() {
            if index < old.len() - 2 {
                match &old.data[index..index + 3] {
                    &[StartLoop, middle, EndLoop] => {
                        match middle {
                            Sub(1) | Sub(-1) => {
                                self.commandlist.push(Set(0));
                                index += 2;
                            },
                            MoveTape(x) => {
                                if x == 1 {
                                    self.commandlist.push(SeekRight);
                                    index += 2;
                                } else if x == -1 {
                                    self.commandlist.push(SeekLeft);
                                    index += 2;
                                } else {
                                    self.commandlist.push(old[index]);
                                }
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
    fn new(data: Vec<T>) -> Tape<T> {
        Tape {
            data,
            cursor: 0,
        }
    }

    fn get_cursor(&self) -> usize {
        self.cursor
    }

    fn jump_relative(&mut self, target: isize) {
        let c = self.get_cursor() as isize;
        self.jump((c + target) as usize);
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

    fn pop(&mut self) -> (T, bool) {
        /*if self.len() == 0 or self.cursor < 0 {
            // what to do if tape is empty
        } else*/ if self.cursor == self.len() - 1 {
            (self.read(), true)
        } else {
            // note that this will not panic unless cursor points to outside
            // the tape, which should not normally happen, unless len == 0
            (self.data.pop().unwrap(), false)
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> { self.data.iter() }
}

impl Tape<Command> {
    fn move_cursor(&mut self, change: isize) -> bool {
        let m = self.cursor as isize + change;
        if m < 0 {
            panic!("Tape pointer outside left bound")
        };
        let outside_right_bound = m >= self.data.len() as isize;
        /*if outside_right_bound {
            println!("[End of Program]")
        };*/
        self.cursor = m as usize;
        outside_right_bound
    }
}

impl Tape<i8> {
   fn move_cursor(&mut self, change: isize) -> bool {
        let m = self.cursor as isize + change;
        if m < 0 {
            panic!("Tape pointer outside left bound")
        };
        let outside_right_bound = m >= self.data.len() as isize;
        if outside_right_bound {
            self.data.resize((m + 1) as usize, 0i8)
        };
        self.cursor = m as usize;
        outside_right_bound
    }

    fn sub(&mut self, n: i8) -> bool {
        let m = self.data[self.cursor];
        let (v, overflow) = m.overflowing_add(-n);
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
                lang: Lang::Raw,
                sourcecode: String::from(""),
                commandlist: Tape::new(vec![]),
                //bytecode: Tape::new(vec![]),
            },
            tape: Tape::new(vec![0]),
            stack: Tape::new(vec![0]),
            //bytecode: Tape Tape::new(vec![]),
            registers: RegisterSet::new(),
        }
    }

    pub fn new_test() -> STVM {
        use self::Command::*;
        let mut vm = STVM::new();
        vm.program.commandlist = Tape::new(vec![
        /*
            JumpIfNonzero(usize),
            Sub(i8),
            MoveTape(isize),
            Set(i8),
            Seek(isize),
            HaltAlways,
            HaltIfNotEqual(i8),
        */
            Set(2),
            MoveTape(1),
            Set(32),
            MoveTape(1),
            Set(20),
            MoveTape(2),
            Set(40),
            Push,
            Set(50),
            Push,
            Set(1),
            JumpAbsoluteIfZero(14),
            MoveTape(1),
            Pop,
            JumpAbsoluteIfNonzero(11),
            HaltAlways,
        ]);
        vm
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
                Sub(n) => self.registers.overflow = self.tape.sub(n),
                Set(n) => self.tape.write(n),
                MoveTape(n) => self.registers.tape_outside_right_bound = self.tape.move_cursor(n as isize),
                SeekRight => while self.read() != 0 { // TODO: Optimize?
                    self.tape.move_cursor(1);
                },
                SeekLeft => while self.read() != 0 { // TODO: Optimize?
                    self.tape.move_cursor(-1);
                },
                JumpRelativeShortIfZero(target) => if self.tape.read() == 0 {
                    self.program.commandlist.jump_relative(target as isize);
                },
                JumpRelativeShortIfNonzero(target) => if self.tape.read() != 0 {
                    self.program.commandlist.jump_relative(target as isize);
                },
                JumpRelativeLongIfZero(target) => if self.tape.read() == 0 {
                    self.program.commandlist.jump_relative(target as isize);
                },
                JumpRelativeLongIfNonzero(target) => if self.tape.read() != 0 {
                    self.program.commandlist.jump_relative(target as isize);
                },
                JumpAbsoluteIfZero(target) => if self.tape.read() == 0 {
                    self.program.commandlist.jump(target);
                },
                JumpAbsoluteIfNonzero(target) => if self.tape.read() != 0 {
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
                HaltIfNotEqual(n) => {
                    if self.tape.read() != n {halt = true;};
                },
                HaltAlways => halt = true,
                Push => self.stack.push(self.tape.read()),
                Pop => {
                    let (n, underflow) = self.stack.pop();
                    self.registers.stack_underflow = underflow;
                    self.tape.write(n);
                },
                _ => {
                    //halt = true;
                    panic!("unexpected command {:?} in compiled code", com);
                },
            }
            //println!("{:?} executed!", com);
            self.program.commandlist.move_cursor(1);
            //println!("going to {}", self.program.commandlist.cursor);
            //println!("");
        };
        !halt
    }

    pub fn run(&mut self) {
        while self.step() {}
    }

    pub fn read(&self) -> i8 {
        self.tape.read()
    }

    pub fn get_cursor(&self) -> usize { self.tape.cursor }

    pub fn each_cell(&self) -> std::slice::Iter<'_, i8> { self.tape.iter() }

    pub fn debug_print(&self) {
        println!("{:?}", self.tape);
        //let l = self.program.commandlist.len();
        //if l < 2 {return};
        //println!("{:?}", self.program.commandlist[l - 2]);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut test_vm = super::STVM::new();
        test_vm.set_program(Program {
            lang: Lang::bf,
            sourecode: "+++++[>+++<-]>",
        });
        while test_vm.step() {}
        assert_eq!(test_vm.read(), 15);
    }
}
