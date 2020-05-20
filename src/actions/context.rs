use std::error::Error;
use std::time::Duration;
use super::straitjacket::StraitJacket;

pub struct Context {
    straitjacket: StraitJacket,
}

impl Context {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            straitjacket: StraitJacket::new(Some(Duration::from_secs(10)))?,
        })
    }

    pub fn prompt(&self) -> String {
        use console::style;

        let mut s = self.straitjacket().host_url()
            .map_or_else(|| String::with_capacity(3), |url| {
                style(url).bold().dim().to_string()
            });
        s.push_str(">> ");
        s
    }

    pub fn straitjacket_mut(&mut self) -> &mut StraitJacket {
        &mut self.straitjacket
    }

    pub fn straitjacket(&self) -> &StraitJacket {
        &self.straitjacket
    }

    pub fn set_host<'h, H: Into<Option<&'h str>>>(&mut self, host: H) -> Result<(), Box<dyn Error>> {
        self.straitjacket.set_host(host)
    }

    pub fn set_token<T: Into<Option<String>>>(&mut self, token: T) -> Result<(), Box<dyn Error>> {
        self.straitjacket.set_token(token.into())
    }
}