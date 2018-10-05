/*!
 * Stack-Tape Virtual Machine
 */

mod prng;
use prng::Prng;

mod tape;
use tape::Tape;

mod command;

mod lisp;

use std::io::{self, Read, Write};
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
    bytecode: Tape<i8>,
}

#[derive(Debug)]
struct RegisterSet {
    pub acc: i16,
    pub zero: bool,
    pub overflow: bool,
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
pub enum VMError {
    Halt,
    IO(&'static str),
    InvalidOperation(usize, i8),
    UnexpectedEOF,
    UnknownTapeError,
    UnexpectedCommand(command::Opcode),
    //Other,
}

impl From<tape::TapeError> for VMError {
    fn from(e: tape::TapeError) -> VMError {
        match e {
            tape::TapeError::EOF => VMError::UnexpectedEOF,
            _  => VMError::UnknownTapeError,
        }
    }
}

impl RegisterSet {
    pub fn new() -> Self {
        Self {
            acc: 0,
            zero: false,
            overflow: false,
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
        match self.lang {
            Lang::BF => (),
            _ => panic!("tried to compile wrong language"),
        }

        println!("--------");
        println!("Compiling bytecode from BF");
        println!();

        use command::Opcode::*;
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

        let mut tmp = Tape::new(vec![]);

        for c in self.sourcecode.chars() {
            if let Some(&com) = ops.get(&c) {
                tmp.push(com);
            }
        }

        // This way the run loop can start with inc_read even
        // tho the index is a usize (so no negative)
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
                        self.bytecode.push(Inc.to_i8());
                    } else if count == -1 {
                        self.bytecode.push(Dec.to_i8());
                    } else {
                        self.bytecode.push(SubImmediate.to_i8());
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
                        self.bytecode.push(IncTape.to_i8());
                    } else if count == -1 {
                        self.bytecode.push(DecTape.to_i8());
                    } else if count.abs() <= 127 {
                        self.bytecode.push(MoveTapeShort.to_i8());
                        self.bytecode.push(count as i8);
                    } else {
                        self.bytecode.push(MoveTapeLong.to_i8());
                        self.bytecode.push_int(2, count as i16 as u32);
                    }
                }
                OutputByte | InputByte => self.bytecode.push(current as i8),
                StartLoop => {
                    self.bytecode.push(JumpAbsoluteIfZero.to_i8());
                    for _i in 0..4 {
                        self.bytecode.push(0);
                    }
                    loop_stack.push(self.bytecode.len());
                }
                EndLoop => {
                    // TODO: line number for this error?
                    let target = loop_stack.pop().expect("unmatched bracket while compiling BF");
                    self.bytecode.push(JumpAbsoluteIfNonzero.to_i8());
                    self.bytecode.push_int(4, target as u32);
                    let here = self.bytecode.len();
                    self.bytecode.write_int_at(target - 4, 4, here as u32)
                }
                _ => unreachable!(), // in theory
            };
            index += 1;
        }

        self.bytecode.push(HaltAlways.to_i8());

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
        
        println!("Finished");
        println!();
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

    pub fn new_test() -> STVM {
        use command::Opcode::*;
        let mut vm = STVM::new();
        vm.program.bytecode = Tape::new(vec![
            Nop.to_i8(),
            Set.to_i8(), 2,
            IncTape.to_i8(),
            Set.to_i8(), 32,
            SubImmediate.to_i8(), 10,
            //SubRelativeLong.to_i8(), -128, -1,
            IncTape.to_i8(),
            Set.to_i8(), 20,
            IncTape.to_i8(),
            IncTape.to_i8(),
            Push.to_i8(),
            Set.to_i8(), 40,
            Push.to_i8(),
            Set.to_i8(), 50,
            Push.to_i8(),
            Set.to_i8(), 1,
            JumpRelativeShortIfZero.to_i8(), 3,
            IncTape.to_i8(),
            Pop.to_i8(),
            JumpRelativeShortIfNonzero.to_i8(), -3,
            Set.to_i8(), 100,
            JumpRelativeShortIfZero.to_i8(), 6,
            IncTape.to_i8(),
            PushRand.to_i8(),
            Pop.to_i8(),
            OutputByte.to_i8(),
            DecTape.to_i8(),
            Dec.to_i8(),
            JumpRelativeShortIfNonzero.to_i8(), -6,
            //JumpRelativeShortIfNonzero(-6),
            HaltAlways.to_i8(),
        ]);
        vm
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

    pub fn step(&mut self) -> Result<(), VMError> {
        // probably want to return a Result or something like it
        use command::Opcode::*;

        //let index = self.program.bytecode.get_cursor();

        //self.program.bytecode.inc_cursor();
        //let op = self.program.bytecode.peek();

        let op = self.program.bytecode.peek();
        let com = command::Opcode::from_i8(op).ok_or(VMError::InvalidOperation(self.program.bytecode.get_cursor(), op))?;

        self.program.bytecode.inc_cursor();

        //println!("   {:08x}: {:?} ({}, {:02x})", index, com, op, op);

        //let index = self.program.bytecode.get_cursor();
        //println!("{:08x}: {:?} ({}, {:02x})", index, com, op, op);

        /*
        let com;
        if let Some(com0) = command::Opcode::from_i8(self.program.bytecode.peek()) {
            com = com0;
        } else {
            return Err(VMError::Other);VMError::Other
        }
        // */

        match com {
            Nop => (),
            Inc => self.registers.overflow = self.tape.sub(-1),
            Dec => self.registers.overflow = self.tape.sub(1),
            IncTape => self.registers.tape_outside_right_bound = self.tape.move_cursor(1),
            DecTape => self.registers.tape_outside_right_bound = self.tape.move_cursor(-1),
            Set => {
                let (n, _) = self.program.bytecode.read_inc();
                self.tape.write(n);
            }
            SubImmediate => {
                let (n, _) = self.program.bytecode.read_inc();
                self.registers.overflow = self.tape.sub(n);
            }
            SubRelativeLong => {
                let n = self.program.bytecode.read_int(2)?;
                let m = self.tape.peek_relative(n as i32 as isize);
                self.registers.overflow = self.tape.sub(m);
            }
            MoveTapeShort => {
                let n = self.program.bytecode.read_int(1)?;
                self.registers.tape_outside_right_bound = self.tape.move_cursor(n as i8 as isize)
            }
            MoveTapeLong => {
                let n = self.program.bytecode.read_int(2)?;
                self.registers.tape_outside_right_bound = self.tape.move_cursor(n as i16 as isize)
            }
            /*SeekRight => while self.tape.peek() != 0 {
                // TODO: Optimize?
                self.tape.move_cursor(1);
            }
            SeekLeft => while self.tape.peek() != 0 {
                // TODO: Optimize?
                self.tape.move_cursor(-1);
            }
            JumpRelativeShortIfZero(target) => if self.tape.peek() == 0 {
                self.program.bytecode.jump_relative(target as isize);
            }
            JumpRelativeShortIfNonzero(target) => if self.tape.peek() != 0 {
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
                    Ok(n) => if n == 1 {
                        self.tape.write(buffer[0] as i8);
                    } else if n == 0 {
                        return Err(VMError::IO("no bytes read from input"));
                    } else {
                        //return Err(&format!("wrong number of bytes read! {} bytes", n));
                        return Err(VMError::IO("wrong number of bytes read!"));
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
                    return Err(VMError::Halt);
                }
            }
            Push => {
                self.stack.push(self.tape.peek())
            }
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
            HaltAlways => return Err(VMError::Halt),
            _ => {
                return Err(VMError::UnexpectedCommand(com));
            }
        }
        //Err(VMError::Halt)
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), VMError> {
        loop {
            let err = self.step();
            match err {
                Ok(_) => continue,
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
    fn it_works() {
        let mut test_vm = super::STVM::from_code(
            super::Lang::BF,
            "+++++[>+++<-]>",
        );
        test_vm.run();
        assert_eq!(test_vm.peek(), 15);
    }
}
