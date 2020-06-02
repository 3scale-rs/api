use std::error::Error;

use super::straitjacket::*;

pub struct RootContext {
    hosts: Vec<Host>,
}

impl RootContext {
    pub fn new() -> Self {
        Self { hosts: Vec::new() }
    }

    pub fn find_host(&self, host_url: &str) -> Option<&Host> {
        self.hosts.iter().find(|h| h.url().to_string().as_str() == host_url)
    }

    pub fn add_host(&mut self, host_url: &str, token: Option<&str>) -> Result<(&Host, Option<String>), Box<dyn Error>> {
        let existing = self.find_host(host_url);

        if let Some(h) = existing {
            Ok((h, token.map(|t| h.set_token(t))))
        } else {
            if let Some(token) = token {
                let host = Host::new(host_url, token)?;
                self.hosts.push(host);

                let Some(h) = self.hosts.last();
                Ok((h, None))
            } else {
                Err("no token specified".into())
            }
        }
    }
}

impl ReadLineContext for RootContext {
    fn command(&mut self, cmd: &str, args: &[&str]) -> CommandAction<&dyn ReadLineContext> {
        use CommandAction::*;

        match (cmd, args) {
            ("host", &[host_url]) => {
                let host = self.find_host(host_url);
                match host {
                    None => Failed("Host not found. If you want to add it, specify a token.".into()),
                    Some(h) => SetContext("Ok, found host.".into(), h),
                }
            }
            ("host", &[host_url, token]) => {
                match self.add_host(host_url, Some(token)) {
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
        ">>"
    }

    fn set_parent(&mut self, parent: &dyn ReadLineContext) {
        todo!()
    }

    fn parent_mut(&self) -> &mut dyn ReadLineContext {
        todo!()
    }
}