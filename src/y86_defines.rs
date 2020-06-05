// icode
pub const IHALT: u32   = 0x0;
pub const INOP: u32    = 0x1;
pub const IRRMOVQ: u32 = 0x2;
pub const IIRMOVQ: u32 = 0x3;
pub const IRMMOVQ: u32 = 0x4;
pub const IMRMOVQ: u32 = 0x5;
pub const IOPQ: u32    = 0x6;
pub const IJXX: u32    = 0x7;
pub const ICALL: u32   = 0x8;
pub const IRET: u32    = 0x9;
pub const IPUSHQ: u32  = 0xA;
pub const IPOPQ: u32   = 0xB;

// fcode
pub const FNONE: u32   = 0x0;
pub const FADDQ: u32   = 0x0;
pub const FSUBQ: u32   = 0x1;
pub const FANDQ: u32   = 0x2;
pub const FXORQ: u32   = 0x3;
pub const FJMP: u32    = 0x0;
pub const FJLE: u32    = 0x1;
pub const FJL: u32     = 0x2;
pub const FJE: u32     = 0x3;
pub const FJNE: u32    = 0x4;
pub const FJGE: u32    = 0x5;
pub const FJG: u32     = 0x6;
pub const FRRMOVQ: u32 = 0x0;
pub const FCMOVLE: u32 = 0x1;
pub const FCMOVL: u32  = 0x2;
pub const FCMOVE: u32  = 0x3;
pub const FCMOVNE: u32 = 0x4;
pub const FCMOVGE: u32 = 0x5;
pub const FCMOVG: u32  = 0x6;

// register
pub const RRSP: u32  = 0x4;
pub const RNONE: u32 = 0xF;

// status code
pub const SAOK: u32 = 0b1;
pub const SADR: u32 = 0x2;
pub const SINS: u32 = 0x3;
pub const SHLT: u32 = 0x4;

// sizes
pub const BIT: u32    = 1;
pub const NIBBLE: u32 = 4;
pub const BYTE: u32   = 8;
pub const WORD: u32   = 16;
pub const DWORD: u32  = 32;
pub const QWORD: u32  = 64;
pub const INST: u32   = 80;

// others
pub const ALUADD: u32  = 0x0;
pub const ENABLE: u32  = 0b1;
pub const DISABLE: u32 = 0b0;