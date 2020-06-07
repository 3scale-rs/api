use std::error::Error;

use crate::readline::{CommandAction, Action, ReadLineContext};
use super::service::Service;

#[derive(Clone, Debug)]
pub struct Host {
    url: straitjacket::client::Url,
    url_s: String,
    token: String,
    services: Vec<Service>,
}

impl PartialEq for Host {
    fn eq(&self, other: &Self) -> bool {
        self.url_s.eq(&other.url_s)
    }
}

impl PartialOrd for Host {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.url_s.partial_cmp(&other.url_s)
    }
}

// not autoderived because it'd require every field to be Eq, yet we only care about a subset
impl Eq for Host {}

impl Ord for Host {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.url_s.cmp(&other.url_s)
    }
}

impl Host {
    pub fn new(host_url: impl Into<String>, token: impl Into<String>) -> Result<Self, Box<dyn Error>> {
        let url_s = host_url.into();
        let url = url_s.parse()?;
        Ok(Self {
            url,
            url_s,
            token: token.into(),
            services: Vec::new(),
        })
    }

    pub fn url(&self) -> &straitjacket::client::Url {
        &self.url
    }

    pub fn url_str(&self) -> &str {
        self.url.as_str()
    }

    pub fn token(&self) -> &str {
        self.token.as_str()
    }

    pub fn set_token(&mut self, token: impl Into<String>) -> String {
        core::mem::replace(&mut self.token, token.into())
    }
}

pub struct HostCtx<'h> {
    host: &'h mut Host,
    //parent: &'ctx dyn crate::readline::ReadLineContext,
}

impl<'h> HostCtx<'h> {
    pub fn new(host: &'h mut Host) -> Self {
        Self {
            host,
        }
    }
}

impl<'s> ReadLineContext<'s> for HostCtx<'s> {
    fn prompt(&self) -> &str {
        self.host.url_str()
    }

    fn command(&'s mut self, cmd: &str, args: &[&str]) -> CommandAction<'s> {
        if true {
            CommandAction::new(Action::NotFound)
        } else {
            CommandAction::new(Action::SetContext("parent".into(), self))
        }
    }
}