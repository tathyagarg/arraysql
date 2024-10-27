//                      FIELD ,      CONSTR,   OPTIONS
type Constraints = Vec<(String, Vec<(String, Vec<String>)>)>;

#[derive(Default, Debug, PartialEq, Eq)]
pub enum QueryType {
    #[default]
    None,
    DatabaseCreation,
    TableCreation,
    Insert,
    Read,
}

#[derive(Default, Debug)]
pub struct Query {
    pub _type: QueryType,
    pub db_name: String,
    pub table_name: String,

    // ============ Table Creation ============
    //               DTYPE ,    OPTIONS , IDTFR
    pub fields: Vec<(String, Vec<String>, String)>,
    pub modes: Vec<String>,
    pub constraints: Constraints,
    //
    // ============ Insertions ============
    pub inserted_value: Vec<String>,
    pub inserted_field: Vec<String>,
    //
    // ============ Reading ============
    pub read_fields: Vec<String>,
    pub conditions: Vec<String>,
}
