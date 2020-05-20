use std::error::Error;
use std::time::Duration;

pub struct StraitJacket {
    client: straitjacket::client::Client,
    response: Option<straitjacket::client::Response>,
    body: Option<Vec<u8>>,
}

impl StraitJacket {
    pub fn new(timeout: Option<Duration>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            client: straitjacket::client::Client::new(timeout)?,
            response: None,
            body: None,
        })
    }

    pub fn client_mut(&mut self) -> &mut straitjacket::client::Client {
        &mut self.client
    }

    pub fn client(&self) -> &straitjacket::client::Client {
        &self.client
    }

    pub fn set_host<'h, H: Into<Option<&'h str>>>(&mut self, host: H) -> Result<(), Box<dyn Error>> {
        self.client_mut().set_host(host)?;
        Ok(())
    }

    pub fn host_url(&self) -> Option<&straitjacket::client::Url> {
        self.client.host_url()
    }

    pub fn set_token<T: Into<Option<String>>>(&mut self, token: T) -> Result<(), Box<dyn Error>> {
        let c = self.client_mut();
        c.set_token(token.into());
        Ok(())
    }

    pub fn token(&self) -> Option<&str> {
        self.client().token()
    }

    pub fn set_response(&mut self, response: straitjacket::client::Response) {
        self.body = None;
        self.response = Some(response);
    }

    pub fn body(&self) -> Option<&Vec<u8>> {
        self.body.as_ref()
    }

    pub fn fetch_body(&mut self) -> Result<&Vec<u8>, Box<dyn Error>> {
        if let Some(ref body) = self.body {
            return Ok(body);
        }

        match self.response_mut() {
            Some(r) => {
                use std::convert::TryFrom;
                //let mut v = Vec::new();
                let mut v = if let Some(capacity) = r.content_length()
                    .and_then(|len| {
                        // do not just trust a header, preallocate reasonably
                        let maxlen = std::cmp::min(len, u16::MAX as u64);
                        usize::try_from(maxlen).ok()
                    }) {
                        Vec::with_capacity(capacity)
                    } else {
                        Vec::new()
                    };

                r.copy_to(&mut v)?;
                self.body = Some(v);
            },
            None => return Err(From::from("no response")),
        };

        Ok(self.body.as_ref().unwrap())
    }

    pub fn response(&self) -> Option<&straitjacket::client::Response> {
        self.response.as_ref()
    }

    pub fn response_mut(&mut self) -> Option<&mut straitjacket::client::Response> {
        self.response.as_mut()
    }
}