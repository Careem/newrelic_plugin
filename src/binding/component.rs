use chrono::prelude::*;
use binding::config::Config;
use binding::metric::Metric;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Component{
    pub name: String,
    pub guid: String,
    pub metrics: Vec<Metric>,
    last_delivered_at: Option<i64>
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}, GUID: {}, Metrics: {}, Last delivered at: {}",
            self.name, self.guid, self.display_metrics(), self.last_delivered_at.unwrap_or(-1))
    }
}

impl Component{
    pub fn new(name: String, guid: String) -> Self{
        Component{
            name: name,
            guid: guid,
            metrics: vec![],
            last_delivered_at: None
        }
    }

    fn display_metrics(&self) -> String{
        let mut metrics = String::new();
        for metric in &self.metrics{
            metrics = format!("{}", metric);
        }
        metrics
    }

    pub fn key(&self) -> String{
        format!("{}{}", self.name, self.guid)
    }

    pub fn duration(&self) -> i64{
        if self.last_delivered_at.is_none(){
            let config = Config::new();
            return config.deliver_cycle();
        }else{
            let now: DateTime<Utc> = Utc::now();
            return now.timestamp() - self.last_delivered_at.unwrap();
        }

    }

    pub fn add_metric(&mut self, name: String){
        let metric = Metric::new(name);
        self.metrics.push(metric);
    }

    pub fn report_metric(&mut self, metric_name: String, value: f64,
        options: Option<(u64, f64, f64, f64)>) -> f64{
        let mut old_value = 0f64;
        for metric in &mut self.metrics{
            if metric.name == metric_name{
                let m = Metric::new_valued(metric_name, value, options);
                old_value = metric.aggregate(&m);
                break;
            }
        }
        old_value
    }

    pub fn get_metric(&self, metric_name: String) -> Option<&Metric>{
        for metric in &self.metrics{
            if metric.name == metric_name{
                return Some(metric);
            }
        }
        None
    }

    pub fn last_delivered_now(&mut self){
        self.last_delivered_at = Some(Utc::now().timestamp());
        for metric in &mut self.metrics{
            metric.reset();
        }
    }
}

