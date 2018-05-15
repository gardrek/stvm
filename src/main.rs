mod bf {
    use std::io::{self, Read};
    //use std::ops::Index;

    #[derive(Debug, Clone)]
    pub struct Tape<T: Copy> {
        data: Vec<T>,
        pub cursor: usize,
    }

    #[derive(Debug)]
    pub struct BFVM {
        sourcecode: String,
        tape: Tape<i8>,
        bytecode: Tape<Command>,
        //input: Vec<i8>,
        //output: Vec<i8>,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    enum Command {
        //Nop,
        Inc,
        Dec,
        IncP,
        DecP,
        Output,
        Input,
        StartLoop,
        EndLoop,
        JumpIfZero(usize),
        JumpIfNonzero(usize),
        Add(i8),
        MoveTape(isize),
    }

    /*impl<T: Copy> Index<usize> for Tape<T> {
        type Output = usize;

        fn index(&self, i: usize) -> T {
            self.data[i]
        }
    }*/

    impl<T: Copy> Tape<T> {
        fn get_cursor(&self) -> usize {
            self.cursor
        }

        fn jump(&mut self, target: usize) {
            if target >= self.data.len() {panic!("Tape pointer jump target is outside right bound")};
            self.cursor = target;
        }

        fn read(&self) -> T {
            self.get(self.cursor)
        }

        fn write(&mut self, value: T) {
            //self.data[self.cursor] = value;
            let index = self.cursor;
            self.set(index, value);
        }

        fn get(&self, index: usize) -> T {
            self.data[index]
        }

        fn set(&mut self, index: usize, value: T) {
            self.data[index] = value;
        }

        fn len(&mut self) -> usize {
            self.data.len()
        }

        fn push(&mut self, n: T) {
            self.data.push(n);
        }

        /*fn pop(&mut self) -> Option<T> {
            self.data.pop()
        }*/
    }

    impl Tape<Command>{
        fn move_cursor(&mut self, change: isize) {
            let m = self.cursor as isize + change;
            if m < 0 {panic!("Tape pointer outside left bound")};
            if m >= self.data.len() as isize {println!("[End of Program]")};
            self.cursor = m as usize;
        }
    }

    impl Tape<i8>{
        /*fn inc(&mut self) {
            self.add(1);
        }

        fn dec(&mut self) {
            self.add(-1);
        }*/

        fn move_cursor(&mut self, change: isize) {
            let m = self.cursor as isize + change;
            if m < 0 {panic!("Tape pointer outside left bound")};
            if m >= self.data.len() as isize {
                self.data.resize((m + 1) as usize, 0i8)
            };
            self.cursor = m as usize;
        }

        fn add(&mut self, n: i8) {
            let m = self.data[self.cursor];
            self.data[self.cursor] = m.wrapping_add(n)
        }
    }

    impl BFVM {
        pub fn new() -> BFVM {
            BFVM {
                tape: Tape {
                    data: vec![0],
                    cursor: 0,
                },
                sourcecode: String::from(""),
                bytecode: Tape {
                    data: vec![],
                    cursor: 0,
                },
            }
        }

        pub fn from_code(code: &str) -> BFVM {
            let mut bfvm = Self::new();
            bfvm.compile(code);
            bfvm
        }

        pub fn from_file(filename: &str) -> BFVM {
            use std::fs::File;
            let mut f = File::open(filename).expect("file not found");
            let mut contents = String::new();
            f.read_to_string(&mut contents)
                .expect("something went wrong reading the file");

            //println!("With text:\n{}", contents);
            BFVM::from_code(&contents)
        }

        pub fn compile(&mut self, code: &str) {
            use self::Command::*;
            use std::collections::HashMap;
            self.sourcecode = String::from(code);

            let mut ops = HashMap::new();
            ops.insert('+', Inc);
            ops.insert('-', Dec);
            ops.insert('>', IncP);
            ops.insert('<', DecP);
            ops.insert('.', Output);
            ops.insert(',', Input);
            ops.insert('[', StartLoop);
            ops.insert(']', EndLoop);

            for c in self.sourcecode.chars() {
                if let Some(&com) = ops.get(&c) {
                    self.bytecode.push(com);
                }
            };

            self.optim();

            let mut stack = Vec::new();
            for index in 0..self.bytecode.len() {
                let com = self.bytecode.get(index);
                match com {
                    StartLoop => {
                        stack.push(index);
                    },
                    EndLoop => {
                        let f = stack.pop().unwrap();
                        self.bytecode.set(index, JumpIfNonzero(f));
                        self.bytecode.set(f, JumpIfZero(index));
                    },
                    _ => {
                        //
                    },
                    
                };
            };
            if stack.len() != 0 {panic!("unmatched bracket!")};
        }

        fn optim(&mut self) {
            use std::mem::swap;
            use self::Command::*;
            //debug!
            println!("Optimizing");
            //println!("{:?}", self.bytecode);
            println!("length: {} commands", self.bytecode.len());
            let old_length = self.bytecode.len();
            if old_length == 0 {
                println!("command list is empty, skipping optimize");
                return
            };

            let mut old = Tape {
                data: vec![],
                cursor: 0,
            };
            swap(&mut old, &mut self.bytecode);

            let mut count;
            let mut index = 0;
            while index < old.len() {
                let current = old.get(index);
                match current {
                    Inc | Dec  | IncP | DecP => {
                        count = 1;
                        while
                            index < old.len() - 1 && // next is within bounds
                            old.get(index + 1) == current && // same commannd
                            count < 127 // less than an i8, as Add takes an i8
                        {
                            count += 1;
                            index += 1;
                        };
                        match current {
                            Inc => self.bytecode.push(Add(count as i8)),
                            Dec => self.bytecode.push(Add(-count as i8)),
                            IncP => self.bytecode.push(MoveTape(count as isize)),
                            DecP => self.bytecode.push(MoveTape(-(count as isize))),
                            _ => panic!(),
                        };
                    },
                    _ => self.bytecode.push(current),
                };
                index += 1;
            }

            println!("\nOptimized");
            //println!("{:?}", self.bytecode);
            println!("length after optimize: {} commands", self.bytecode.len());
            println!("ratio: {}%\n", (self.bytecode.len() * 100) / old_length);
        }

        fn read(&self) -> i8 {
            self.tape.read()
        }

        pub fn step(&mut self) -> bool { // probably want to return a Result or something like it
            use self::Command::*;
            if self.bytecode.get_cursor() < self.bytecode.len() {
                // don't forget iterator .next()
                //println!("at {}", self.bytecode.get_cursor());
                let com = self.bytecode.read();
                match com {
                    Add(n) => self.tape.add(n),
                    MoveTape(n) => self.tape.move_cursor(n),
                    JumpIfZero(target) => {
                        //let value = self.read();
                        if self.read() == 0 {
                            self.bytecode.jump(target);
                        }
                    },
                    JumpIfNonzero(target) => {
                        //let value = self.read();
                        if self.read() != 0 {
                            self.bytecode.jump(target);
                        }
                    },
                    Input => {
                        let mut buffer = [0u8; 1];
                        let mut stdin = io::stdin();
                        stdin.lock();
                        match stdin.read(&mut buffer) {
                        //match io::stdin().read(&mut buffer) {
                            Err(e) => panic!(e),
                            Ok(n) => if n == 1 {
                                self.tape.write(buffer[0] as i8);
                            } else {
                                panic!("wrong numstepber of bytes read! {} bytes", n)
                            },
                        }
                    },
                    Output => print!("{}", self.read() as u8 as char),
                    _ => panic!("unexpected command {:?} in compiled code", com),
                }
                //println!("{:?} executed!", com);
                self.bytecode.move_cursor(1);
                //println!("going to {}", self.bytecode.cursor);
                //println!("");
                true
            } else {
                //println!("no more code");
                //println!("");
                false
            }
        }
    }

    /*impl Command {
        fn from_c(c: char) -> Command{
            use self::Command::*;
            match c {
                '+' => Inc,
                '-' => Dec,
                '>' => Right,
                '<' => Left,
                '.' => Output,
                ',' => Input,
                '[' => StartLoop,
                ']' => EndLoop,
                _ => panic!(),
            }
        }
    }*/

    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() {
            let mut test_vm = super::BFVM::new();
            assert_eq!(test_vm.read(), 0);
            test_vm.step();
        }
    }
}

fn main() -> () {
    let zero = 0u8;
    let neg_two = -2i8;
    assert_eq!(neg_two as u8, zero.wrapping_add(neg_two as u8));

    //let mut main_vm = bf::BFVM::new();
    //let mut main_vm = bf::BFVM::from_file("tictactoe.bf");
    //let mut main_vm = bf::BFVM::from_code(">");
    let mut main_vm = bf::BFVM::from_code("+++++>++<[->>+<<]");

    /*let mut main_vm = bf::BFVM::from_code(
        "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>."
    );*/

    /*let mut main_vm = bf::BFVM::from_code(
        ">++++++++[-<+++++++++>]<.>>+>-[+]++>++>+++[>[->+++<<+++>]<<]>-----.>->
        +++..+++.>-.<<+[>[+>+]>>]<--------------.>>.+++.------.--------.>+.>+."
    );*/

    /*let mut main_vm = bf::BFVM::from_code(
        "++++++++++[>+++>++++>+++++++++++>++++++++<<<<-]>++>++++>++>+++.<.--------.+.+++++.++++++++++.<<.>>>>++[<<---------.>>-]<<<<.>>----.++++++++++.-----------.++.++++++++.<<.>>++++++.++++.>>++++[<<----->>-]<<.>>++++[<<++++>>-]<<+.++.++++++.<.<.>>>>++++[<<---->>-]<<.+++++++++++.>>++++[<<---->>-]<<-.+++.--.<<.>>++++++++.++++++++++++.<<.>>---.-------.++++++++.<++.>>[-]<[-]<[-]<[-]<"
    );*/

    /*let mut main_vm = bf::BFVM::from_code(
        ">>>>--<-<<+[+[<+>--->->->-<<<]>]<<--.<++++++.<<-..<<.<+.>>.>>.<<<.+++.>>.>>-.<<<+."
    );*/

   //println!("{:?}\n--------\n", main_vm);
    while main_vm.step() {};
    println!("\n\n--------\n{:?}", main_vm);
}
