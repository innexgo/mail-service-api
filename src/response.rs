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
  Unknown,
}

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
