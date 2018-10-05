use super::command;

enum IR1 {
    Add(isize),
    Move(isize),
    OutputByte,
    InputByte,
    StartLoop,
    EndLoop,
}

pub fn compile(sourcecode) -> Vec<i8> {
    let v = vec![]; 
    for c in self.sourcecode.chars() {
        v.push(match c {
            '+' => Add(1),
            '-' => Add(-1),
            '>' => Move(1),
            '<' => Move(-1),
            '.' => OutputByte,
            ',' => InputByte,
            '[' => StartLoop,
            ']' => EndLoop,
        });
    }

    let index = 0;
    while index < v.len() {
    }
}
