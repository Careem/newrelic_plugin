use serde_json::value::Value;
use chrono::prelude::*;
use binding::component::Component;
use binding::request::Request;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Context{
    components: Vec<Component>,
    pub license_key: String,
    pub version: String,
    pub host: String,
    pub pid: u64,
    pub last_reported: Option<i64>
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Components: {}", self.display_components())
    }
}

impl Context{

    pub fn new(license_key: String, version: String, host: String,
        pid: u64) -> Self{
        Context{
            version: version,
            host: host,
            pid: pid,
            license_key: license_key,
            last_reported: None,
            components: vec![]
        }
    }

    fn display_components(&self) -> String{
        let mut components = String::new();
        for component in &self.components{
            components = format!("{}", component);
        }
        components
    }

    pub fn register_component(&mut self, component: Component){
        self.components.push(component);
    }

    pub fn report_metric(&mut self, component_guid: String, metric_name: String, value: f64,
        options: Option<(u64, f64, f64, f64)>) -> f64{
        let mut old_value = 0f64;
        for component in &mut self.components{
            if component.guid == component_guid{
                old_value = component.report_metric(metric_name, value, options);
                break;
            }
        }
        old_value
    }

    fn request_hash(&self) -> Value{
        let mut hash = json!({});
        hash["agent"] = json!({
            "host": self.host,
            "pid": self.pid,
            "version": self.version
        });
        let mut components = vec![];
        for component in &self.components{
            let mut metrics = json!({});
            for metric in &component.metrics{
                metrics[metric.to_hash().0] = json!(metric.to_hash().1);
            }

            components.push(json!({
                "name": component.name,
                "guid": component.guid,
                "duration": component.duration(),
                "metrics": metrics
            }));
        }
        hash["components"] = json!(components);
        hash
    }

    pub fn deliver(&mut self){
        let mut request = Request::new(self.request_hash(), self.license_key.clone());
        let success = request.send();
        if success{
            for component in &mut self.components{
                component.last_delivered_now();
            }
        }
        self.last_reported = Some(Utc::now().timestamp());
    }

}
