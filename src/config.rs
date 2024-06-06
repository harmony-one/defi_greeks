use std::env;
use lazy_static::lazy_static;

lazy_static! {
  pub static ref ENVIRONMENT: String = env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
}