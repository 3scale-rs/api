use std::error::Error;
use std::borrow::Cow;
use std::time::Duration;

pub enum CommandAction<'s, T> {
    SetContext(Cow<'s, str>, T),
    Failed(Cow<'s, str>),
    SideEffect(Cow<'s, str>),
    NoProgress(Cow<'s, str>),
    Usage(Cow<'s, str>),
    NotFound,
}

pub trait ReadLineContext {
    fn prompt(&self) -> &str;
    fn command(&mut self, cmd: &str, args: &[&str]) -> CommandAction<'_, &dyn ReadLineContext>;
    fn set_parent(&mut self, parent: &dyn ReadLineContext);
    fn parent_mut(&self) -> &mut dyn ReadLineContext;
}

pub struct Limit {
    limit: straitjacket::api::v0::limit::Limit,
}

pub struct Metric {
    metric: straitjacket::api::v0::service::metric::Metric,
}

pub struct MappingRule {
    mapping_rule: straitjacket::api::v0::proxy::mapping_rules::MappingRule,
}

pub struct ApplicationPlan {
    application_plan: straitjacket::api::v0::service::plan::Plan,
}

pub struct Service {
    service: straitjacket::api::v0::service::Service,
    mapping_rules: Option<Vec<MappingRule>>,
    metrics: Option<Vec<Metric>>,
    limits: Option<Vec<Limit>>,
    application_plans: Option<Vec<ApplicationPlan>>,
}

pub struct Host {
    url: straitjacket::client::Url,
    token: String,
    // proxy: Proxy type from sj
    services: Vec<Service>,
}

impl ReadLineContext for Host {
    fn prompt(&self) -> &str {
        let prompt = self.url.to_string();
        prompt.push_str(">> ");
        prompt
    }

    fn command(&mut self, cmd: &str, args: &[&str]) -> CommandAction<'_, &dyn ReadLineContext> {
        todo!()
    }
    fn set_parent(&mut self, parent: &dyn ReadLineContext) {
        todo!()
    }
    fn parent_mut(&self) -> &mut dyn ReadLineContext {
        todo!()
    }
}

impl Host {
    pub fn new(host_url: &str, token: impl Into<String>) -> Result<Self, Box<dyn Error>> {
        let url = host_url.parse()?;
        Ok(Self {
            url,
            token: token.into(),
            services: Vec::new(),
        })
    }

    pub fn url(&self) -> &straitjacket::client::Url {
        &self.url
    }

    pub fn token(&self) -> &str {
        self.token.as_str()
    }

    pub fn set_token(&mut self, token: impl Into<String>) -> String {
        let old = self.token;
        self.token = token.into();
        old
    }
}

pub struct StraitJacket {
    client: straitjacket::client::Client,
    response: Option<straitjacket::client::Response>,
    body: Option<Vec<u8>>,
    hosts: Option<Vec<Host>>,
}

impl StraitJacket {
    pub fn new(timeout: Option<Duration>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            client: straitjacket::client::Client::new(timeout)?,
            response: None,
            body: None,
            hosts: None,
        })
    }

    pub fn client_mut(&mut self) -> &mut straitjacket::client::Client {
        &mut self.client
    }

    pub fn client(&self) -> &straitjacket::client::Client {
        &self.client
    }

    pub fn set_host<'h, H: Into<Option<&'h str>>>(
        &mut self,
        host: H,
    ) -> Result<(), Box<dyn Error>> {
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

    pub fn set_hosts(&mut self, hosts: Vec<Host>) {
        self.hosts = Some(hosts);
    }

    pub fn hosts(&self) -> Option<&Vec<Host>> {
        self.hosts.as_ref()
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
                let mut v = if let Some(capacity) = r.content_length().and_then(|len| {
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
            }
            None => return Err(From::from("no response")),
        };

        Ok(self.body.as_ref().unwrap())
    }

    pub fn fetch_body_as_str(&mut self) -> Result<&str, Box<dyn Error>> {
        std::str::from_utf8(self.fetch_body()?).map_err(|e| From::from(e))
    }

    pub fn response(&self) -> Option<&straitjacket::client::Response> {
        self.response.as_ref()
    }

    pub fn response_mut(&mut self) -> Option<&mut straitjacket::client::Response> {
        self.response.as_mut()
    }
}
