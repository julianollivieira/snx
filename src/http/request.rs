use std::collections::HashMap;

pub struct Request {
    pub path: String,
    pub headers: HashMap<String, String>,
}
