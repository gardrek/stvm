/*!
 * Stack-Tape Virtual Machine
 */

mod prng;
use prng::Prng;

mod tape;
use tape::Tape;

mod command;
use command::Opcode;

mod lisp;

use std::io::{self, Read, Write};

use std::error::Error;
use std::fmt;

/// Supported languages for compiling
#[derive(Debug)]
pub enum Lang {
    Raw,
    Bf,
    Lisp,
}

/// A program's source code and compiled bytecode
#[derive(Debug)]
pub struct Program {
    lang: Lang,
    sourcecode: String,
    //tokenlist, ast, etc?
    bytecode: Tape<i8>,
}

#[derive(Debug)]
struct RegisterSet {
    pub acc: i16,
    pub zero: bool,
    pub arithmetic_overflow: bool,
    pub stack_underflow: bool,
    pub tape_outside_right_bound: bool,
}

#[derive(Debug)]
pub struct STVM {
    program: Program,
    tape: Tape<i8>,
    stack: Tape<i8>,
    registers: RegisterSet,
    //input: Vec<i8>, // should be a FIFO tho
    //output: Vec<i8>,
    prng: Prng,
}

#[derive(Debug)]
pub enum VmError {
    Halt,
    Io(&'static str),

    // byte op and location
    InvalidOperation(i8, usize),

    UnexpectedEof,
    TapeError(tape::TapeError),
    UnexpectedCommand(Opcode),
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::VmError::*;
        match self {
            Halt => write!(f, "Halt"),
            Io(s) => write!(f, "I/O Error: {:?}", s),
            InvalidOperation(op_byte, location) => {
                write!(f, "Invalid Operation {} at position {}", op_byte, location)
            }
            UnexpectedEof => write!(f, "Unexpected EOF"),
            TapeError(e) => write!(f, "Tape Error: {}", e),
            UnexpectedCommand(op) => write!(f, "Unexpected Coommand: {:?}", op),
        }
    }
}

impl Error for VmError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use self::VmError::*;
        match self {
            Halt | InvalidOperation(_, _) | UnexpectedCommand(_) | UnexpectedEof => None,
            TapeError(e) => Some(e),
            Io(_str) => None,
        }
    }
}

#[derive(Debug)]
pub enum VmState {
    Continue,
    Halt,
}

impl From<tape::TapeError> for VmError {
    fn from(e: tape::TapeError) -> VmError {
        match e {
            tape::TapeError::Eof => VmError::UnexpectedEof,
            _ => VmError::TapeError(e),
        }
    }
}

impl RegisterSet {
    pub fn new() -> Self {
        Self {
            acc: 0,
            zero: false,
            arithmetic_overflow: false,
            stack_underflow: false,
            tape_outside_right_bound: false,
        }
    }
}

impl Program {
    pub fn new(lang: Lang, sourcecode: &str) -> Program {
        Program {
            lang,
            sourcecode: sourcecode.to_string(),
            bytecode: Tape::new(vec![]),
        }
    }

    pub fn from_file(lang: Lang, filename: &str) -> Program {
        // TODO: choose source language from file name extension?

        use std::fs::File;
        let mut f = File::open(filename).expect("file not found");
        let mut sourcecode = String::new();
        f.read_to_string(&mut sourcecode)
            .expect("something went wrong reading the file");

        Program::new(lang, &sourcecode)
    }

    fn compile(&mut self) {
        match self.lang {
            Lang::Bf => self.compile_bf(),
            Lang::Lisp => self.compile_lisp(),
            _ => unimplemented!(),
        }
    }

    pub fn debug_inject_byte(&mut self, b: i8) {
        self.bytecode.push(b);
    }

    fn compile_lisp(&mut self) {
        let tokens = lisp::tokenize(&self.sourcecode);
        let _ast = lisp::parse(tokens);
    }

    // BF specific stuff

    fn compile_bf(&mut self) {
        match self.lang {
            Lang::Bf => (),
            _ => panic!("tried to compile wrong language"),
        }

        //println!("--------");
        //println!("Compiling bytecode from BF");
        //println!();

        use std::collections::HashMap;
        use Opcode::*;

        let mut ops = HashMap::new();
        ops.insert('+', Inc);
        ops.insert('-', Dec);
        ops.insert('>', IncTape);
        ops.insert('<', DecTape);
        ops.insert('.', OutputByte);
        ops.insert(',', InputByte);
        ops.insert('[', StartLoop);
        ops.insert(']', EndLoop);

        let mut tmp = Tape::new(vec![]);

        let mut size = 0;
        for c in self.sourcecode.chars() {
            if let Some(&com) = ops.get(&c) {
                tmp.push(com);
                size = size + 1;
            }
        }
        //println!("souce code size : {} byte(s)", size);

        // This way the run loop can start with inc_read even
        // tho the index is a usize (so no negative)
        // this is a hack
        self.bytecode.push(0);

        let mut loop_stack = vec![];
        let mut count: isize;
        let mut next_command;
        let mut index = 0;
        while index < tmp.len() {
            let current = tmp[index];
            match current {
                Inc | Dec => {
                    if current == Inc {
                        count = 1;
                    } else {
                        count = -1;
                    }
                    while index + 1 < tmp.len() {
                        next_command = tmp[index + 1];
                        match next_command {
                            Inc => {
                                if count.abs() >= std::i8::MAX as isize {
                                    break;
                                }
                                count += 1;
                            }
                            Dec => {
                                if count.abs() >= std::i8::MAX as isize {
                                    break;
                                }
                                count -= 1;
                            }
                            _ => break,
                        }
                        index = index + 1;
                    }
                    if count == 1 {
                        self.bytecode.push(Inc.into());
                    } else if count == -1 {
                        self.bytecode.push(Dec.into());
                    } else {
                        self.bytecode.push(SubImmediate.into());
                        self.bytecode.push(-count as i8);
                    }
                }
                IncTape | DecTape => {
                    if current == IncTape {
                        count = 1;
                    } else {
                        count = -1;
                    }
                    while index + 1 < tmp.len() {
                        next_command = tmp[index + 1];
                        match next_command {
                            IncTape => {
                                if count.abs() >= std::i8::MAX as isize {
                                    break;
                                }
                                count += 1;
                            }
                            DecTape => {
                                if count.abs() >= std::i8::MAX as isize {
                                    break;
                                }
                                count -= 1;
                            }
                            _ => break,
                        }
                        index = index + 1;
                    }
                    if count == 1 {
                        self.bytecode.push(IncTape.into());
                    } else if count == -1 {
                        self.bytecode.push(DecTape.into());
                    } else if count.abs() <= 127 {
                        self.bytecode.push(MoveTapeShort.into());
                        self.bytecode.push(count as i8);
                    } else {
                        self.bytecode.push(MoveTapeLong.into());
                        self.bytecode.push_int(2, count as i16 as u32);
                    }
                }
                OutputByte | InputByte => self.bytecode.push(current as i8),
                StartLoop => {
                    self.bytecode.push(JumpAbsoluteIfZero.into());
                    for _i in 0..4 {
                        self.bytecode.push(0);
                    }
                    loop_stack.push(self.bytecode.len());
                }
                EndLoop => {
                    // TODO: line number for this error?
                    let target = loop_stack
                        .pop()
                        .expect("unmatched bracket while compiling BF");
                    self.bytecode.push(JumpAbsoluteIfNonzero.into());
                    self.bytecode.push_int(4, target as u32);
                    let here = self.bytecode.len();
                    self.bytecode.write_int_at(target - 4, 4, here as u32)
                }
                _ => unreachable!(), // in theory
            };
            index += 1;
        }

        self.bytecode.push(HaltAlways.into());

        /*
        let mut count = 0;
        let mut prev_com = None;
        for c in self.sourcecode.chars() {
            if let Some(&com) = ops.get(&c) {
                match com {
                    Inc | Dec | IncTape | DecTape => {
                        if let Some(inside) = prev_com {
                            if com == inside {
                                count += 1;
                            } else if count > 1 {
                                match inside {
                                    Inc | Dec | IncTape | DecTape => {
                                        // end of a run
                                        println!("{:?} run of {}", inside, count);
                                        count = 1;
                                    }
                                    _ => count = 0,
                                }
                            }
                        } else {
                            // first command
                            println!("    First: {:?}", com);
                            count += 1;
                        }
                    }
                    OutputByte | InputByte | StartLoop | EndLoop => println!("{:?}, ", com),
                    _ => unreachable!(),
                }
                //self.bytecode.push(com as i8);
                prev_com = Some(com);
                println!("{}", count);
            }
        }
        //self.bytecode.push(HaltAlways);
        println!("\n");
        */

        //println!("compiled size : {} byte(s)", self.bytecode.len());

        //println!("Finished");
        //println!();
    }
}

impl STVM {
    pub fn new() -> STVM {
        STVM {
            program: Program {
                lang: Lang::Raw,
                sourcecode: String::from(""),
                bytecode: Tape::new(vec![]),
            },
            tape: Tape::new(vec![0]),
            stack: Tape::new(vec![0]),
            registers: RegisterSet::new(),
            prng: Prng::new_from_time(),
        }
    }

    fn set_program(&mut self, program: Program) {
        self.program = program;
    }

    pub fn from_code(lang: Lang, sourcecode: &str) -> STVM {
        let mut vm = STVM::new();
        vm.set_program(Program::new(lang, sourcecode));
        vm.compile();
        vm
    }

    pub fn from_file(lang: Lang, filename: &str) -> STVM {
        let mut vm = STVM::new();
        vm.set_program(Program::from_file(lang, filename));
        vm.compile();
        vm
    }

    pub fn compile(&mut self) {
        self.program.compile()
    }

    pub fn step(&mut self) -> Result<VmState, VmError> {
        use Opcode::*;

        //let index = self.program.bytecode.get_cursor();

        //self.program.bytecode.inc_cursor();
        //let op = self.program.bytecode.peek();

        let op = self.program.bytecode.peek();
        let com = Opcode::from_i8(op).ok_or(VmError::InvalidOperation(
            op,
            self.program.bytecode.get_cursor(),
        ))?;

        self.program.bytecode.inc_cursor();

        //println!("   {:08x}: {:?} ({}, {:02x})", index, com, op, op);

        //let index = self.program.bytecode.get_cursor();
        //println!("{:08x}: {:?} ({}, {:02x})", index, com, op, op);

        /*
        let com;
        if let Some(com0) = Opcode::from_i8(self.program.bytecode.peek()) {
            com = com0;
        } else {
            return Err(VmError::Other);VmError::Other
        }
        // */

        match com {
            Nop => (),
            Inc => self.registers.arithmetic_overflow = self.tape.sub(-1),
            Dec => self.registers.arithmetic_overflow = self.tape.sub(1),
            IncTape => self.registers.tape_outside_right_bound = self.tape.move_cursor(1),
            DecTape => self.registers.tape_outside_right_bound = self.tape.move_cursor(-1),
            Set => {
                let (n, _) = self.program.bytecode.read_inc();
                self.tape.write(n);
            }
            SubImmediate => {
                let (n, _) = self.program.bytecode.read_inc();
                self.registers.arithmetic_overflow = self.tape.sub(n);
            }
            SubRelativeLong => {
                let n = self.program.bytecode.read_int(2)?;
                let m = self.tape.peek_relative(n as i32 as isize);
                self.registers.arithmetic_overflow = self.tape.sub(m);
            }
            MoveTapeShort => {
                let n = self.program.bytecode.read_int(1)?;
                self.registers.tape_outside_right_bound = self.tape.move_cursor(n as i8 as isize)
            }
            MoveTapeLong => {
                let n = self.program.bytecode.read_int(2)?;
                self.registers.tape_outside_right_bound = self.tape.move_cursor(n as i16 as isize)
            }
            SeekRight => {
                while self.tape.peek() != 0 {
                    // TODO: Optimize?
                    self.tape.move_cursor(1);
                }
            }
            SeekLeft => {
                while self.tape.peek() != 0 {
                    // TODO: Optimize?
                    self.tape.move_cursor(-1);
                }
            }
            JumpRelativeShortIfZero => {
                let target = self.program.bytecode.read_int(1)?;
                self.program.bytecode.jump_relative(target as isize);
            }
            /*JumpRelativeShortIfNonzero(target) => if self.tape.peek() != 0 {
                self.program.bytecode.jump_relative(target as isize);
            }
            JumpRelativeLongIfZero(target) => if self.tape.peek() == 0 {
                self.program.bytecode.jump_relative(target as isize);
            }
            JumpRelativeLongIfNonzero(target) => if self.tape.peek() != 0 {
                self.program.bytecode.jump_relative(target as isize);
            }*/
            JumpAbsoluteIfZero => {
                let target = self.program.bytecode.read_int(4)?;
                if self.tape.peek() == 0 {
                    //println!("{}", target as usize - 1);
                    self.program.bytecode.jump(target as usize);
                }
            }
            JumpAbsoluteIfNonzero => {
                let target = self.program.bytecode.read_int(4)?;
                if self.tape.peek() != 0 {
                    //println!("{}, {}, {}", target, self.program.bytecode.len(), target as usize - 1);
                    self.program.bytecode.jump(target as usize);
                }
            }
            InputByte => {
                // TODO: fix it so it takes input
                // immediately instead of waiting for line end
                let mut buffer = [0u8; 1];
                let mut stdin = io::stdin();
                stdin.lock();
                match stdin.read(&mut buffer) {
                    Err(e) => panic!(e),
                    Ok(n) => {
                        if n == 1 {
                            self.tape.write(buffer[0] as i8);
                        } else if n == 0 {
                            return Err(VmError::Io("no bytes read from input"));
                        } else {
                            //return Err(&format!("wrong number of bytes read! {} bytes", n));
                            return Err(VmError::Io("wrong number of bytes read!"));
                        }
                    }
                }
            }
            OutputByte => {
                print!("{}", self.tape.peek() as u8 as char);
                io::stdout().flush().unwrap();
            }
            //OutputDebug => {
            //println!("{}", self.tape.peek());
            //io::stdout().flush().unwrap();
            //}
            HaltIfNotEqual => {
                let (n, _) = self.program.bytecode.read_inc();
                if self.tape.peek() != n {
                    return Ok(VmState::Halt);
                }
            }
            Push => self.stack.push(self.tape.peek()),
            Pop => {
                let (n, underflow) = self.stack.pop();
                self.registers.stack_underflow = underflow;
                self.tape.write(n);
            }
            PushRand => {
                let r = self.prng.gen_i8();
                self.stack.push(r);
                //let r = self.prng.gen();
                //self.stack.push((r >> 8) as i8);
                //self.stack.push((r & 0xff) as i8);
            }
            HaltAlways => return Ok(VmState::Halt),
            _ => {
                return Err(VmError::UnexpectedCommand(com));
            }
        }
        //Err(VmError::Halt)
        Ok(VmState::Continue)
    }

    pub fn run(&mut self) -> Result<(), VmError> {
        loop {
            match self.step() {
                Ok(s) => match s {
                    VmState::Continue => continue,
                    VmState::Halt => return Ok(()),
                },
                Err(e) => return Err(e),
            }
        }
    }

    pub fn get_cursor(&self) -> usize {
        self.tape.get_cursor()
    }

    pub fn each_cell(&self) -> std::slice::Iter<'_, i8> {
        self.tape.iter()
    }

    pub fn debug_new() -> STVM {
        let mut test_vm = STVM::new();

        test_vm.debug_inject_byte(0x00);

        test_vm.debug_inject_byte(Opcode::Nop.into());

        test_vm
    }

    pub fn debug_inject_byte(&mut self, b: i8) {
        self.program.debug_inject_byte(b);
    }

    pub fn debug_print(&self) {
        println!("{}", self.program.bytecode);
        //let l = self.program.commandlist.len();
        //if l < 2 {return};
        //println!("{:?}", self.program.commandlist[l - 2]);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn compiling_test() {
        let mut test_vm = super::STVM::from_code(super::Lang::Bf, "+++++[>+++<-]>");
        test_vm.run().expect("VM error");
        assert_eq!(test_vm.tape.peek(), 15);
    }
}
