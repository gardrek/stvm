#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    Nop,
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

    // Move the tape until a non-zero cell is found
    SeekRight,
    SeekLeft,

    HaltAlways,
    HaltIfNotEqual,

    Push,
    Pop,
    PushRand,
}

impl Opcode {
    pub fn len(&self) -> usize {
        use self::Opcode::*;
        match self {
            Nop |
            Inc |
            Dec |
            IncTape |
            DecTape |
            OutputByte |
            InputByte |
            StartLoop |
            EndLoop |
            SeekRight |
            SeekLeft |
            HaltAlways |
            Push |
            Pop |
            PushRand => 1,

            HaltIfNotEqual |
            JumpRelativeShortIfZero |
            JumpRelativeShortIfNonzero |
            SubImmediate |
            MoveTapeShort |
            Set => 2,

            JumpRelativeLongIfZero |
            JumpRelativeLongIfNonzero |
            SubRelativeLong |
            MoveTapeLong => 3,

            JumpAbsoluteIfZero |
            JumpAbsoluteIfNonzero => 5,
        }
    }

    pub fn to_i8(&self) -> i8 {
        use self::Opcode::*;
        match self {
            Nop => 0,
            Inc => 1,
            Dec => 2,
            IncTape => 3,
            DecTape => 4,
            OutputByte => 5,
            InputByte => 6,
            StartLoop => 7,
            EndLoop => 8,

            JumpRelativeShortIfZero => 9,
            JumpRelativeShortIfNonzero => 10,
            JumpRelativeLongIfZero => 11,
            JumpRelativeLongIfNonzero => 12,
            JumpAbsoluteIfZero => 13,
            JumpAbsoluteIfNonzero => 14,

            SubImmediate => 15,
            SubRelativeLong => 16,
            Set => 17,

            MoveTapeShort => 18,
            MoveTapeLong => 19,

            // Move the tape until a non-zero cell is found
            SeekRight => 20,
            SeekLeft => 21,

            HaltAlways => 22,
            HaltIfNotEqual => 23,

            Push => 24,
            Pop => 25,

            PushRand => 26,
        }
    }

    pub fn from_i8(n: i8) -> Option<Opcode> {
        use self::Opcode::*;
        match n {
            0 => Some(Nop),
            1 => Some(Inc),
            2 => Some(Dec),
            3 => Some(IncTape),
            4 => Some(DecTape),
            5 => Some(OutputByte),
            6 => Some(InputByte),
            7 => Some(StartLoop),
            8 => Some(EndLoop),

            9 => Some(JumpRelativeShortIfZero),
            10 => Some(JumpRelativeShortIfNonzero),
            11 => Some(JumpRelativeLongIfZero),
            12 => Some(JumpRelativeLongIfNonzero),
            13 => Some(JumpAbsoluteIfZero),
            14 => Some(JumpAbsoluteIfNonzero),

            15 => Some(SubImmediate),
            16 => Some(SubRelativeLong),
            17 => Some(Set),

            18 => Some(MoveTapeShort),
            19 => Some(MoveTapeLong),

            // Move the tape until a non-zero cell is found
            20 => Some(SeekRight),
            21 => Some(SeekLeft),

            22 => Some(HaltAlways),
            23 => Some(HaltIfNotEqual),

            24 => Some(Push),
            25 => Some(Pop),
            26 => Some(PushRand),

            _ => None,
        }
    }
}
