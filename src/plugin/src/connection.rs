use harana_common::hashbrown::HashMap;
use crate::authentication::*;

#[derive(Debug, Clone)]
struct Connection {
    authentication: Option<Authentication>,
    parameter_values: HashMap<String, String>,
}

impl Connection {
    fn new() -> Self {
        Connection {
            authentication_methods: Vec::new(),
            oauth: None,
            http_basic: None,
            token: None,
        }
    }
}