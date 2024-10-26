#[derive(Default, Debug, PartialEq, Eq)]
pub enum Step {
    #[default]
    Start,
    DefineDatabaseName,
    // ============ Table Creation ============
    DefineTableName,
    DefineTableOn,
    DefineTableDatabase,
    DefineTableStructure,
    DefineTableStructureOpenParen,
    DefineFieldDatatype,
    DefineFieldDatatypeOpenParen,
    DefineFieldDatatypeOption,
    DefineFieldIdentifier,
    DefineTableStructureCloseParen,
    DefineTableMode,
    DefineConstraintOpenParen,
    DefineConstraintOn,
    DefineConstraintIdentifier,
    DefineConstraint,
    DefineConstraintOption,
    DefineConstraintOptionCloseParen,
    DefineConstraintCloseParen,
    // ============ Insertions ============
    InsertValueStructure,
    InsertValueStructureOpenParen,
    InsertValueIdentifier,
    InsertValueStructureCloseParen,
    End,
}
