use std::error::Error;
use super::host::Host;
use crate::readline::{ReadLineContext, CommandAction};

pub struct Root {
    hosts: Vec<Host>,
}

impl Root {
    pub fn new() -> Self {
        Self { hosts: Vec::new() }
    }

    pub fn hosts(&self) -> &Vec<Host> {
        self.hosts.as_ref()
    }

    pub fn get_host(&self, host_url: &str) -> Option<&Host> {
        self.hosts.binary_search_by(|h| h.url_str().cmp(host_url)).ok()
              .and_then(|idx| self.hosts.get(idx))
    }

    fn host_search(&mut self, host_url: &str) -> Result<usize, usize> {
        self.hosts.binary_search_by(|h| h.url_str().cmp(host_url))
    }

    pub fn find_host(&mut self, host_url: &str) -> Result<&mut Host, usize> {
        //let idx = self.hosts.binary_search_by(|h| h.url_str().cmp(host_url))?;
        let idx = self.host_search(host_url)?;
        Ok(self.hosts.get_mut(idx).unwrap())
    }

    pub fn remove_host(&mut self, host_url: &str) -> Option<Host> {
        let idx = self.host_search(host_url).ok()?;
        Some(self.hosts.remove(idx))
    }

    fn add_host(&mut self, host: Host) -> &mut Host {
        self.hosts.push(host);
        let h = self.hosts.last_mut().unwrap();
        self.hosts.sort_unstable();
        h
    }

    pub fn add_host_by_url(&mut self, host_url: &str, token: &str) -> Result<(&mut Host, Option<String>), Box<dyn Error>> {
        let result = match self.find_host(host_url) {
            Ok(h) => {
                let old_token = h.set_token(token);
                (h, Some(old_token))
            },
            Err(idx) => {
                let h = Host::new(host_url, token)?;
                self.hosts.insert(idx, h);
                (self.hosts.get_mut(idx).unwrap(), None)
            },
        };

        Ok(result)
    }
}

impl ReadLineContext for Root {
    fn command(&mut self, cmd: &str, args: &[&str]) -> CommandAction<Box<dyn ReadLineContext>> {
        use CommandAction::*;

        match (cmd, args) {
            ("host", &[host_url]) => {
                let host = self.get_host(host_url);
                match host {
                    None => Failed("Host not found. If you want to add it, specify a token.".into()),
                    Some(h) => SetContext("Ok, found host.".into(), Box::new(h)),
                }
            }
            ("host", &[host_url, token]) => {
                match self.add_host_by_url(host_url, token) {
                    Ok((h, prev_token)) => match prev_token {
                        Some(token) => SideEffect(format!("Replaced token {}.", token).into()),
                        None => SideEffect(format!("Host added: {}", h.url().to_string().as_str()).into()),
                    },
                    Err(e) => Failed(format!("{:?}", e).into()),
                }
            },
            ("host", _) => Usage("usage: host <3scale-system-host> [<token>]".into()),
            (_, _) => NotFound,
        }
    }

    fn prompt(&self) -> &str {
        "(root)"
    }

    fn set_parent(&mut self, parent: &dyn ReadLineContext) {
        todo!()
    }

    fn parent_mut(&self) -> &mut dyn ReadLineContext {
        todo!()
    }
}