use std::error::Error;
use super::host::{Host, HostCtx};
use crate::readline::{ReadLineContext, CommandAction, Action};

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

    fn host_search(&self, host_url: &str) -> Result<usize, usize> {
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

    //fn add_host(&mut self, host: Host) -> &mut Host {
    //    self.hosts.push(host);
    //    let h = self.hosts.last_mut().unwrap();
    //    self.hosts.sort_unstable();
    //    h
    //}

    pub fn add_host_by_url(&mut self, host_url: &str, token: &str) -> Result<(usize, Option<String>), Box<dyn Error>> {
        Ok(match self.host_search(host_url) {
            Ok(idx) => {
                (idx, Some(self.hosts.get_mut(idx).unwrap().set_token(token)))
            },
            Err(idx) => {
                let h = Host::new(host_url, token)?;
                self.hosts.insert(idx, h);
                (idx, None)
            },
        })
    }
}

pub struct RootCtx<'r> {
    root: &'r mut Root,
}

impl<'r> RootCtx<'r> {
    pub fn new(root: &'r mut Root) -> Self {
        Self {
            root,
        }
    }
}

impl<'s> ReadLineContext<'s> for RootCtx<'s> {
    fn command(&'s mut self, cmd: &str, args: &[&str]) -> CommandAction<'s> {
        use Action::*;

        match (cmd, args) {
            ("host", &[host_url]) => {
                let host = self.root.get_host(host_url);
                match host {
                    None => CommandAction::new(
                        Action::Failed("Host not found. If you want to add it, specify a token.".into())),
                    Some(h) => CommandAction::new(
                        Action::SetContext("Ok, found host.".into(), self)),
                }
            }
            ("host", &[host_url, token]) => {
                match self.root.add_host_by_url(host_url, token) {
                    Ok((_, prev_token)) => match prev_token {
                        Some(token) => CommandAction::new(
                            Action::SideEffect(format!("Replaced {}'s token {}.", host_url, token).into())),
                        None => CommandAction::new(
                            Action::SideEffect(format!("Host added: {}", host_url).into())),
                    },
                    Err(e) => CommandAction::new(
                        Action::Failed(format!("{:?}", e).into())),
                }
            },
            ("host", _) => CommandAction::new(
                Action::Usage("usage: host <3scale-system-host> [<token>]".into())),
            (_, _) => CommandAction::new(
                Action::NotFound),
        }
    }

    fn prompt(&self) -> &str {
        "(root)"
    }
}