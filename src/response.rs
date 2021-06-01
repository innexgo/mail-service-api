use serde::{Deserialize, Serialize};
use strum::AsRefStr;

#[derive(Clone, Debug, Serialize, Deserialize, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MailError {
  DestinationProhibited,
  DestinationBounced,
  Unknown,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mail {
  mail_id: i64,
  creation_time: i64,
  destination: String,
  title: String,
  content: String,
}
