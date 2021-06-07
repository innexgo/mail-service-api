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
      .post(format!("{}/mail/new", self.mail_service_url))
      .json(&props)
      .send()
      .await
      .map_err(|_| response::MailError::NetworkError)?
      .json::<Result<response::Mail, response::MailError>>()
      .await
      .map_err(|_| response::MailError::DecodeError)?
  }

  pub async fn mail_view(
    &self,
    props: request::MailViewProps,
  ) -> Result<Vec<response::Mail>, response::MailError> {
    self
      .client
      .post(format!("{}/mail/view", self.mail_service_url))
      .json(&props)
      .send()
      .await
      .map_err(|_| response::MailError::NetworkError)?
      .json()
      .await
      .map_err(|_| response::MailError::DecodeError)?
  }
}
