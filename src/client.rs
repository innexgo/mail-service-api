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
    pub async fn mail(
        &self,
        request_id: i64,
        topic: String,
        destination: String,
        title: String,
        content: String,
    ) -> Result<response::Mail, response::MailError> {
        let mnr = request::MailNew {
            topic,
            destination,
            title,
            content,
        };

        self.client
            .post(format!("{}/mail/new", self.mail_service_url))
            .json(&mnr)
            .send()
            .await
            .map_err(|_| response::MailError::Unknown)?
            .json()
            .await
            .map_err(|_| response::MailError::Unknown)?
    }
}
