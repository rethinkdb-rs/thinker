#[derive(Serialize, Deserialize, Debug)]
struct Info {
     success: bool,
     min_protocol_version: Option<usize>,
     max_protocol_version: Option<usize>,
     server_version: Option<String>,
     error_code: Option<usize>,
     error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Term {
    name: String,
    raw_query: bool,
    root_term: bool,
    term_type: Term_TermType,
    data: ReqlData,
    args: Vec<Term>,
    opt_args: HashMap,
    last_err: Error,
}
