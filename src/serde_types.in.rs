//use std::collections::HashMap;
//use super::reql::Error;
//use super::ql2::proto;

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
     pub success: bool,
     pub min_protocol_version: Option<usize>,
     pub max_protocol_version: Option<usize>,
     pub server_version: Option<String>,
     pub error_code: Option<usize>,
     pub error: Option<String>,
}

/*
#[derive(Serialize, Deserialize, Debug)]
pub struct Term {
    pub name: String,
    pub raw_query: bool,
    pub root_term: bool,
    pub term_type: proto::Term_TermType,
    pub data: String,
    pub args: Vec<Term>,
    pub opt_args: HashMap<String, String>,
    pub last_err: Error,
}
*/
