use serde::{Deserialize, Serialize};
use strum::AsRefStr;

#[derive(Clone, Debug, Serialize, Deserialize, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MailError {
  DestinationProhibited,
  DestinationBounced,
  InternalServerError,
  NetworkError,
  DecodeError,
  NotFound,
  MethodNotAllowed,
  BadRequest,
  Unknown,
}

impl std::fmt::Display for MailError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_ref())
  }
}

impl std::error::Error for MailError { }

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mail {
  pub mail_id: i64,
  pub request_id: i64,
  pub creation_time: i64,
  pub topic: String,
  pub destination: String,
  pub title: String,
  pub content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
  pub service: String,
  pub version_major: i64,
  pub version_minor: i64,
  pub version_rev: i64,
}
