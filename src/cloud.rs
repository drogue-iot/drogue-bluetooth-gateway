use log;
use reqwest;

pub struct Cloud {
    client: reqwest::blocking::Client,
    url: String,
    username: String,
    password: String,
}

impl Cloud {
    pub fn new(url: String, username: String, password: String) -> Cloud {
        Cloud {
            url: url,
            client: reqwest::blocking::Client::new(),
            username: username,
            password: password,
        }
    }

    pub fn publish(&self, data: String) -> Result<(), reqwest::Error> {
        let result = self
            .client
            .post(&format!("{}/publish/device_id/{}", self.url, self.username))
            .basic_auth(&self.username, Some(&self.password))
            .body(data)
            .send()?;
        log::debug!("Response: {:?}", result);
        let _ = result.error_for_status()?;
        Ok(())
    }
}
