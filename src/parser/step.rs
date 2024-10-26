#[derive(Default, Debug, PartialEq, Eq)]
pub enum Step {
    #[default]
    Start,
    DefineDatabaseName,
    // ============ Table Creation ============
    DefineTableName,
    DefineTableDatabase,
    DefineFieldDatatype,
    DefineFieldDatatypeOption,
    DefineFieldIdentifier,
    DefineTableStructureCloseParen,
    DefineTableMode,
    DefineConstraintOn,
    DefineConstraintIdentifier,
    DefineConstraint,
    DefineConstraintOption,
    DefineConstraintOptionCloseParen,
    DefineConstraintCloseParen,
    // ============ Insertions ============
    InsertValueIdentifier,
    InsertTable,
    InsertFieldIdentifier,
    InsertDatabase,
    End,
}
