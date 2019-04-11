#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    Nop = 0,
    Inc,
    Dec,
    IncTape,
    DecTape,
    OutputByte,
    InputByte,
    StartLoop,
    EndLoop,

    JumpRelativeShortIfZero,
    JumpRelativeShortIfNonzero,
    JumpRelativeLongIfZero,
    JumpRelativeLongIfNonzero,
    JumpAbsoluteIfZero,
    JumpAbsoluteIfNonzero,

    SubImmediate,
    SubRelativeLong,
    Set,

    MoveTapeShort,
    MoveTapeLong,

    // Move the tape until a non-zero cell is found.
    SeekRight,
    SeekLeft,

    HaltAlways,
    HaltIfNotEqual,

    Push,
    Pop,
    PushRand,

    // This opcode is always illegal to execute.
    // Due to the way conversion to the binary representation is implemented, no Opcode can be
    // listed after this one.
    Illegal,
}

impl Opcode {
    pub fn len(&self) -> usize {
        use self::Opcode::*;
        match self {
            Illegal => 1,

            Nop | Inc | Dec | IncTape | DecTape | OutputByte | InputByte | StartLoop | EndLoop
            | SeekRight | SeekLeft | HaltAlways | Push | Pop | PushRand => 1,

            HaltIfNotEqual
            | JumpRelativeShortIfZero
            | JumpRelativeShortIfNonzero
            | SubImmediate
            | MoveTapeShort
            | Set => 2,

            JumpRelativeLongIfZero | JumpRelativeLongIfNonzero | SubRelativeLong | MoveTapeLong => {
                3
            }

            JumpAbsoluteIfZero | JumpAbsoluteIfNonzero => 5,
        }
    }

    pub fn from_i8(n: i8) -> Option<Opcode> {
        let op = Opcode::from(n);
        match op {
            Opcode::Illegal => None,
            x => Some(x),
        }
    }
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        if v >= (Opcode::Illegal as u8) {
            Opcode::Illegal
        } else {
            unsafe { std::mem::transmute::<u8, Opcode>(v) }
        }
    }
}

impl From<Opcode> for u8 {
    fn from(op: Opcode) -> Self {
        op as u8
    }
}

impl From<i8> for Opcode {
    fn from(v: i8) -> Self {
        Opcode::from(v as u8)
    }
}

impl From<Opcode> for i8 {
    fn from(op: Opcode) -> Self {
        u8::from(op) as i8
    }
}
