use std::str::FromStr;
use error::{Error, ErrorType};
use Result;


/// SPARQL 1.0 keywords.
pub enum SparqlKeyword {
  Base,
  Prefix,
  Select,
  Distinct,
  Reduced,
  Construct,
  Describe,
  Ask,
  From,
  Named,
  Order,
  By,
  Asc,
  Desc,
  Offset,
  Optional,
  Filter,
  Graph,
  Union,
  Regex
}


impl FromStr for SparqlKeyword {
  type Err = Error;

  fn from_str(s: &str) -> Result<SparqlKeyword> {
    match s {
      "BASE" => Ok(SparqlKeyword::Base),
      "PREFIX" => Ok(SparqlKeyword::Prefix),
      "SELECT" => Ok(SparqlKeyword::Select),
      "DISTINCT" => Ok(SparqlKeyword::Distinct),
      "REDUCED" => Ok(SparqlKeyword::Reduced),
      "CONSTRUCT" => Ok(SparqlKeyword::Construct),
      "DESCRIBE" => Ok(SparqlKeyword::Describe),
      "ASK" => Ok(SparqlKeyword::Ask),
      "FROM" => Ok(SparqlKeyword::From),
      "NAMED" => Ok(SparqlKeyword::Named),
      "ORDER" => Ok(SparqlKeyword::Order),
      "BY" => Ok(SparqlKeyword::By),
      "ASC" => Ok(SparqlKeyword::Asc),
      "DESC" => Ok(SparqlKeyword::Desc),
      "OFFSET" => Ok(SparqlKeyword::Offset),
      "OPTIONAL" => Ok(SparqlKeyword::Optional),
      "FILTER" => Ok(SparqlKeyword::Filter),
      "GRAPH" => Ok(SparqlKeyword::Graph),
      "UNION" => Ok(SparqlKeyword::Union),
      "REGEX" => Ok(SparqlKeyword::Regex),
      _ => Err(Error::new(ErrorType::InvalidSparqlInput, "Unknown SPARQL keyword")),
    }
  }
}

impl SparqlKeyword { }