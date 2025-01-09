use std::collections::HashMap;
use crate::types::value::OrmValue;

pub struct OrmRow{
    column: Vec<String>,
    values:HashMap<String, OrmValue>
}