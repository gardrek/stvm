use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
struct Tape<T: Copy> {
    data: Vec<T>,
    cursor: usize,
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
    Sub(i8),
    MoveTape(isize),
    Set(i8),
    Seek(isize),
    HaltAlways,
    HaltIfNotEqual(i8),
    Push,
    Pop,
}

/*impl Command {
    pub fn len(&self) -> usize {
        use Command::*;
        match self {
            Inc |
            Dec |
            IncTape |
            DecTape |
            OutputByte |
            InputByte |
            StartLoop |
            EndLoop |
            Push |
            Pop |
            HaltAlways => 1,
            Sub(_i8) |
            Set(_i8) |
            HaltIfNotEqual(_i8) => 2,
            JumpIfZero(_) |
            JumpIfNonzero(_) |
            MoveTape(_) |
            Seek(_) => 3,
        }
    }
}*/

