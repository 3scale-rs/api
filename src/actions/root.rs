use super::host::{Host, HostCtx};
use crate::readline::{NextContext, ReadLineContext};
use std::error::Error;

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
        self.hosts
            .binary_search_by(|h| h.url_str().cmp(host_url))
            .ok()
            .and_then(|idx| self.hosts.get(idx))
    }

    pub fn host_search(&self, host_url: &str) -> Result<usize, usize> {
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

    pub fn add_host_by_url(
        &mut self,
        host_url: &str,
        token: &str,
    ) -> Result<(usize, Option<String>), Box<dyn Error>> {
        Ok(match self.host_search(host_url) {
            Ok(idx) => (idx, Some(self.hosts.get_mut(idx).unwrap().set_token(token))),
            Err(idx) => {
                let h = Host::new(host_url, token)?;
                self.hosts.insert(idx, h);
                (idx, None)
            }
        })
    }
}

pub struct RootCtx<'r> {
    root: &'r mut Root,
}

impl<'r> RootCtx<'r> {
    pub fn new(root: &'r mut Root) -> Self {
        Self { root }
    }
}

impl<'a> ReadLineContext for RootCtx<'a> {
    fn command(&mut self, cmd: &str, args: &[&str]) -> NextContext {
        match (cmd, args) {
            ("host", &[host_url]) => {
                let host = self.root.host_search(host_url);
                match host {
                    Ok(idx) => NextContext::New(Box::new(HostCtx::new(idx))),
                    _ => NextContext::Unchanged,
                    //Action::Failed("Host not found. If you want to add it, specify a token.".into())),
                }
            }
            ("host", &[host_url, token]) => {
                match self.root.add_host_by_url(host_url, token) {
                    Ok((_, prev_token)) => match prev_token {
                        Some(token) => NextContext::Unchanged,
                        //Action::SideEffect(format!("Replaced {}'s token {}.", host_url, token).into())),
                        None => NextContext::Unchanged,
                        //Action::SideEffect(format!("Host added: {}", host_url).into())),
                    },
                    Err(e) => NextContext::Unchanged,
                    //Action::Failed(format!("{:?}", e).into())),
                }
            }
            ("host", _) => NextContext::Unchanged,
            //Action::Usage("usage: host <3scale-system-host> [<token>]".into())),
            (_, _) => NextContext::Parent,
            //Action::NotFound),
        }
    }

    fn prompt(&self) -> &str {
        "(root)"
    }
}
