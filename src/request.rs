// Types of arguments for auth handlers
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailNew {
  pub request_id:i64,
  pub destination:String,
  pub title:String,
  pub content:String,
}
