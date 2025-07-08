use std::collections::HashMap;
use std::str::FromStr;
use handle_errors::Error;
use handle_errors::Error::ParseError;

#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}

fn param_value_of(key: &str, params: &HashMap<String, String>) -> Result<usize, Error> {
    params.get(key)
        .unwrap()
        .parse::<usize>()
        .map_err(Error::ParseError)
}


pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            start: param_value_of("start", &params)?,
            end: param_value_of("end", &params)?,
        });
    }

    Err(Error::MissingParameters)
}