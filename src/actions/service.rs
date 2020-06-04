use crate::readline::{CommandAction, ReadLineContext};

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

#[derive(Clone, Debug)]
pub struct Service {
    service: straitjacket::api::v0::service::Service,
    mapping_rules: Option<Vec<MappingRule>>,
    metrics: Option<Vec<Metric>>,
    limits: Option<Vec<Limit>>,
    application_plans: Option<Vec<ApplicationPlan>>,
}