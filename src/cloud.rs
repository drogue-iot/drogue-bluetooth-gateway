use log;
use reqwest;

pub struct Cloud {
    client: reqwest::blocking::Client,
    url: String,
    login: String,
    password: String,
}

impl Cloud {
    pub fn new(url: String, application: String, username: String, password: String) -> Cloud {
        Cloud {
            url: url,
            client: reqwest::blocking::Client::new(),
            login: format!("{}@{}", username, application),
            password: password,
        }
    }

    // TODO publish as a gateway device
    pub fn publish(&self, data: String) -> Result<(), reqwest::Error> {
        let result = self
            .client
            .post(&format!("{}/v1/temp/", self.url))
            .basic_auth(&self.login, Some(&self.password))
            .body(data)
            .send()?;
        log::debug!("Response: {:?}", result);
        let _ = result.error_for_status()?;
        Ok(())
    }
}
