// todo find a place to put this

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Sorting {
    #[default]
    Default,
    NameInc,   // a - z
    NameDec,   // z - a
    LengthInc, // 0.00 - 1.00
    LengthDec, // 1.00 - 0.00
    IdInc,     // 0 - 9
    IdDec,     // 9 - 0
    SizeInc,   // 0kb - 9kb
    SizeDec,   // 9kb - 0kb
}
