use std::collections::HashMap;
use std::str::FromStr;
use handle_errors::Error;
use handle_errors::Error::ParseError;

#[derive(Debug, Clone, Default)]
pub struct Pagination {
    pub limit: Option<u32>,
    pub offset: u32,
}

fn param_value_of(key: &str, params: &HashMap<String, String>) -> Result<usize, Error> {
    params.get(key)
        .unwrap()
        .parse::<usize>()
        .map_err(Error::ParseError)
}


pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            limit: Some(param_value_of("limit", &params)? as u32),
            offset: param_value_of("offset", &params)? as u32,
        });
    }

    Err(Error::MissingParameters)
}