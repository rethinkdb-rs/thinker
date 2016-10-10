#[derive(Serialize, Deserialize, Debug)]
struct Info {
     success: bool,
     min_protocol_version: Option<usize>,
     max_protocol_version: Option<usize>,
     server_version: Option<String>,
     error_code: Option<usize>,
     error: Option<String>,
}
