use reqwest::Client;

use super::request;
use super::response;

#[derive(Clone)]
pub struct MailService {
  client: Client,
  mail_service_url: String,
}

impl MailService {
  pub async fn new(mail_service_url: &str) -> Self {
    MailService {
      mail_service_url: String::from(mail_service_url),
      client: Client::new(),
    }
  }

  // sends a mail to the email specified in destination
  // request_id is the unique id of the request, for internal use
  // topic is the description of the email for internal use
  // title is the reciever visible title
  // content is the content of the message
  pub async fn mail_new(
    &self,
    props: request::MailNewProps,
  ) -> Result<response::Mail, response::MailError> {
    self
      .client
      .post(format!("{}/mail_new", self.mail_service_url))
      .json(&props)
      .send()
      .await
      .map_err(|_| response::MailError::NetworkError)?
      .json()
      .await
      .map_err(|_| response::MailError::DecodeError)?
  }

  pub async fn mail_view(
    &self,
    props: request::MailViewProps,
  ) -> Result<Vec<response::Mail>, response::MailError> {
    self
      .client
      .post(format!("{}/mail_view", self.mail_service_url))
      .json(&props)
      .send()
      .await
      .map_err(|_| response::MailError::NetworkError)?
      .json()
      .await
      .map_err(|_| response::MailError::DecodeError)?
  }

  // fetches api information
  pub async fn info(
    &self,
  ) -> Result<Vec<response::Info>, response::MailError> {
    self
      .client
      .post(format!("{}/public/info", self.mail_service_url))
      .send()
      .await
      .map_err(|_| response::MailError::NetworkError)?
      .json()
      .await
      .map_err(|_| response::MailError::DecodeError)?
  }
}
