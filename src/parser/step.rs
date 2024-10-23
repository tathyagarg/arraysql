use std::fmt;

#[derive(Default, Debug)]
pub enum Step {
    #[default]
    Start,
    DefineDatabaseName,
    DefineTableName,
    DefineTableOn,
    DefineTableDatabase,
    DefineTableStructure,
    DefineTableStructureOpenParen,
    DefineFieldDatatype,
    DefineFieldDatatypeOpenParen,
    DefineFieldDatatypeOption,
    DefineFieldDatatypeComma,
    DefineFieldDatatypeCloseParen,
    DefineFieldIdentifier,
    DefineFieldComma,
    DefineTableStructureCloseParen,
    End,
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Step::Start => write!(f, "Start"),
            Step::DefineDatabaseName => write!(f, "DefineDatabaseName"),
            Step::DefineTableName => write!(f, "DefineTableName"),
            Step::DefineTableOn => write!(f, "DefineTableOn"),
            Step::DefineTableDatabase => write!(f, "DefineTableDatabase"),
            Step::DefineTableStructure => write!(f, "DefineTableStructure"),
            Step::DefineTableStructureOpenParen => write!(f, "DefineTableStructureOpenParen"),
            Step::DefineFieldDatatype => write!(f, "DefineFieldDatatype"),
            Step::DefineFieldDatatypeOpenParen => write!(f, "DefineFieldDatatypeOpenParen"),
            Step::DefineFieldDatatypeOption => write!(f, "DefineFieldDatatypeOption"),
            Step::DefineFieldDatatypeComma => write!(f, "DefineFieldDatatypeComma"),
            Step::DefineFieldDatatypeCloseParen => write!(f, "DefineFieldDatatypeCloseParen"),
            Step::DefineFieldIdentifier => write!(f, "DefineFieldIdentifier"),
            Step::DefineFieldComma => write!(f, "DefineFieldComma"),
            Step::DefineTableStructureCloseParen => write!(f, "DefineTableStructureCloseParen"),
            Step::End => write!(f, "End"),
        }
    }
}
