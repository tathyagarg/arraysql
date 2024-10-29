pub const DATABASE: &str = "DATABASE";
pub const TABLE: &str = "TABLE";
pub const INSERT: &str = "INSERT";
pub const READ: &str = "READ";

pub const ON: &str = "ON";
pub const STRUCTURED: &str = "STRUCTURED";
pub const MODE: &str = "MODE";
pub const FADD: &str = "FADD";
pub const FREAD: &str = "FREAD";
pub const FDELETE: &str = "FDELETE";
pub const LMEM: &str = "LMEM";
pub const CONSTRAINED: &str = "CONSTRAINED";
pub const WHERE: &str = "WHERE";

pub const EXISTS: &str = "EXISTS";
pub const UNIQUE: &str = "UNIQUE";
pub const PKEY: &str = "PKEY";
pub const FKEY: &str = "FKEY";
pub const SUCHTHAT: &str = "SUCHTHAT";
pub const DEFAULT: &str = "DEFAULT";
pub const INC: &str = "INC";

pub const OPEN_PAREN: &str = "(";
pub const CLOSE_PAREN: &str = ")";
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";

pub const EQ: &str = "=";
pub const NE: &str = "!=";
pub const GE: &str = ">=";
pub const LE: &str = "<=";
pub const GT: &str = ">";
pub const LT: &str = "<";

pub const AND: &str = "AND";
pub const OR: &str = "OR";

pub const ADD: &str = "+";
pub const SUB: &str = "-";
pub const MUL: &str = "*";
pub const DIV: &str = "/";

pub const ABS: &str = "ABS";
pub const NEG: &str = "NEG";
pub const NOT: &str = "NOT";
pub const BWNOT: &str = "~";

pub const KEYWORDS: &[&str] = &[
    OPEN_PAREN,
    CLOSE_PAREN,
    EQ,
    NE,
    GE,
    LE,
    GT,
    LT,
    COMMA,
    SEMICOLON,
    DATABASE,
    TABLE,
    INSERT,
    READ,
    ON,
    STRUCTURED,
    MODE,
    FADD,
    FREAD,
    FDELETE,
    LMEM,
    CONSTRAINED,
    WHERE,
    EXISTS,
    UNIQUE,
    PKEY,
    FKEY,
    SUCHTHAT,
    DEFAULT,
    INC,
    AND,
    OR,
    NOT,
    ABS,
    NEG,
    BWNOT,
    ADD,
    SUB,
    MUL,
    DIV,
];

pub const DT_STRING: &str = "STRING";
pub const DT_OPTIONS: &str = "OPTIONS";
pub const DT_CHAR: &str = "CHAR";
pub const DT_BYTES: &str = "BYTES";
pub const DT_UINT: &str = "UINT";
pub const DT_INT: &str = "INT";
pub const DT_FLOAT: &str = "FLOAT";
pub const DT_TIMESTAMP: &str = "TIMESTAMP";

pub const DATATYPES: &[&str] = &[
    DT_STRING,
    DT_OPTIONS,
    DT_CHAR,
    DT_BYTES,
    DT_UINT,
    DT_INT,
    DT_FLOAT,
    DT_TIMESTAMP,
];

pub const OPERATORS: &[&str] = &[
    ABS, NEG, NOT, BWNOT, EXISTS, ADD, SUB, MUL, DIV, EQ, NE, GE, LE, GT, LT, AND, OR,
];

pub const BINOPS: &[&str] = &[ADD, SUB, MUL, DIV, EQ, NE, GE, LE, GT, LT];
pub const CONNECTORS: &[&str] = &[AND, OR];

pub const UNOPS: &[&str] = &[ABS, NEG, NOT, BWNOT, EXISTS];
