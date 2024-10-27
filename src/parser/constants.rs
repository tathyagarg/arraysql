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

pub const KEYWORDS: &[&str] = &[
    OPEN_PAREN,
    CLOSE_PAREN,
    "=",
    "<=",
    ">=",
    "!=",
    ">",
    "<",
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
];

pub const DT_STRING: &str = "STRING";
pub const DT_OPTIONS: &str = "OPTIONS";
pub const DT_CHAR: &str = "CHAR";
pub const DT_BYTES: &str = "BYTES";
pub const DT_UINT: &str = "UINT";
pub const DT_INT: &str = "INT";
pub const DT_FLOAT: &str = "FLOAT";
pub const DT_TIMESTAMP: &str = "TIMESTAMP";

pub const DATATYPES: [&str; 8] = [
    DT_STRING,
    DT_OPTIONS,
    DT_CHAR,
    DT_BYTES,
    DT_UINT,
    DT_INT,
    DT_FLOAT,
    DT_TIMESTAMP,
];
