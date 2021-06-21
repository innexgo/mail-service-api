// Types of arguments for auth handlers
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailNewProps {
  pub request_id: i64,
  pub topic: String,
  pub destination: String,
  pub title: String,
  pub content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailViewProps {
  pub mail_id: Option<i64>,
  pub request_id: Option<i64>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub topic: Option<String>,
  pub destination: Option<String>,
  pub title: Option<String>,
  pub offset: Option<i64>,
  pub count: Option<i64>,
}
