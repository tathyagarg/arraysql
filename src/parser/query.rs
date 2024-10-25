//                      FIELD ,      CONSTR,   OPTIONS
type Constraints = Vec<(String, Vec<(String, Vec<String>)>)>;

#[derive(Default, Debug, PartialEq, Eq)]
pub enum QueryType {
    #[default]
    None,
    DatabaseCreation,
    TableCreation,
}

#[derive(Default, Debug)]
pub struct Query {
    pub _type: QueryType,
    pub db_name: String,
    pub table_name: String,

    //               DTYPE ,    OPTIONS , IDTFR
    pub fields: Vec<(String, Vec<String>, String)>,
    pub modes: Vec<String>,
    pub constraints: Constraints,
}
