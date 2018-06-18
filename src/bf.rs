#[derive(Debug)]
pub enum Token {
    Inc,
    Dec,
    IncTape,
    DecTape,
    OutputByte,
    InputByte,
    StartLoop,
    EndLoop,
}
