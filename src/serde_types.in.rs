// "{\"max_protocol_version\":0,\"min_protocol_version\":0,\"server_version\":\"2.3.5\",\"success\":true}"
#[derive(Serialize, Deserialize, Debug)]
struct Info {
    max_protocol_version: usize,
    min_protocol_version: usize,
    server_version: String,
    success: bool,
}
