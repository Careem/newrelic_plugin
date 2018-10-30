use log4rs::init_file;
use chrono::prelude::*;
use binding::context::Context;
use binding::component::Component;
use binding::config::Config;
use std::time::Duration;
use std::thread;
use std::i64::MAX;
use std::fmt;


/// 
/// Constructs a new "Newrelic Agent".
/// Agents are interfaces that give access to the Newrelic API bindings
/// and are used for setting up plugins' components and metrics and reporting metrics to NewRelic API.
/// 
/// # Examples
/// ```
/// // Plugin agent setup
/// let mut agent = Agent::new("<license_key>".into(), "1.0.0".into(), "host".into(), 1234);
/// let mut c1 = agent.create_component("Test Plugin".into(), "com.test_plugin.plugin_name".into());
/// agent.create_metric(&mut c1, "Component/Request/Rate/host1[requests/second]".into());
/// agent.create_metric(&mut c1, "Component/Request/Rate/host2[requests/second]".into());
/// agent.register_component(c1);
/// 
/// // Poll cycle function. This function is excuted every [poll_cycle] seconds.
/// fn cycle(agent: &mut Agent){
///     agent.report_metric(
///         "com.test_plugin.plugin_name".into(),
///         "Component/Request/Rate/host1[requests/second]".into(), 
///         (1000) as f64, None
///     );
///     agent.report_metric(
///         "com.test_plugin.plugin_name".into(),
///         "Component/Request/Rate/host2[requests/second]".into(), 
///         (1000) as f64, None
///     );
/// }
/// // Start the agent
/// agent.run(cycle);
/// ```
/// 
/// # state
/// An agent has the option to have a state to be able to preserve metric readings through cycles. States are passed to *Agent* instances via the ```set_state``` function. States are generic types, which means you can create a custom struct like in this example:
/// 
/// ```
/// struct State{
///     prev_file_size: i32
/// }
/// 
/// // Plugin agent setup
/// let mut agent = Agent::new("<license_key>".into(), "1.0.0".into(), "host".into(), 1234);
/// let mut c1 = agent.create_component("Test Plugin".into(), "com.test_plugin.plugin_name".into());
/// agent.create_metric(&mut c1, "Component/File/Size/host1[bytes]".into());
/// agent.register_component(c1);
/// 
/// // Poll cycle function. This function is excuted every [poll_cycle] seconds.
/// fn cycle(agent: &mut Agent){
///     let prev_size = agent.get_state().as_ref().unwrap().prev_file_size.clone();
///     let new_size = 2000;
///     if prev_size != 0{
///         agent.report_metric(
///             "com.test_plugin.plugin_name".into(),
///             "Component/File/Size/host1[bytes]".into(), 
///             new_size - prev_size as f64, None
///         );  
///     }
///     agent.set_state(State{prev_file_size: new_size});
/// }
/// // Start the agent
/// agent.run(cycle);
/// ```
/// 
/// # config
/// 
/// NewRelic plugin reads configuration from a ```config.yml``` file located in the current working directory. If no ```config.yml``` file is present, default values are used. Possible config keys and values:
/// 
/// | Config key | Description | Default |
/// | ------------- |:-------------:| ----- |
/// | endpoint | NewRelic custom plugin API endpoint| https://platform-api.newrelic.com/platform/v1/metrics |
/// | log4rs_file | log4rs config file | log4rs.yml |
/// | deliver_cycle | metric reporting frequency | 60 (seconds) |
/// | poll_cycle | poll cycle frequency | 20 (seconds) |
/// 
/// # logging
/// 
/// NewRelic plugin uses log4rs. All events logged from within the agent are sent to a logger named *"agent"*. You can add more loggers in log4rs config file to log messages from your plugin's code.
/// 
/// **Example ```log4rs.yml``` file:**
/// 
/// ```yml
/// refresh_rate: 60 seconds
/// 
/// appenders:
///   agent:
///     kind: file
///     path: "log/agent.log"
///     encoder:
///       pattern: "{d} - {l} - {f} - {m}{n}"
///   plugin:
///     kind: file
///     path: "log/plugin.log"
///     encoder:
///       pattern: "{d} - {l} - {f} - {m}{n}"
/// 
/// loggers:
///   agent:
///     level: error
///     appenders:
///       - agent
///   plugin:
///     level: info
///     appenders:
///       - plugin
/// ```


pub struct Agent<T>{
    context: Context,
    config: Config,
    state: Option<T>
}


impl<T> fmt::Display for Agent<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "License Key: {}, Version: {}, Host: {}, PID: {}",
            self.context.license_key, self.context.version,
            self.context.host, self.context.pid)
    }
}

impl<T> Agent<T>{
    pub fn new(license_key: String, version: String, host: String, pid: u64) -> Self{
        let config = Config::new();
        let _ = init_file(config.log4rs_file(), Default::default());
        return Agent{
            context: Context::new(license_key, version, host, pid),
            config: config,
            state: None
        }
    }

    pub fn set_state(&mut self, state: T){
        self.state = Some(state);
    }

    pub fn get_state(&self) -> &Option<T>{
        &self.state
    }

    pub fn create_component(&self, name: String, guid: String) -> Component{
        Component::new(name, guid)
    }

    pub fn create_metric(&self, component: &mut Component, name: String){
        component.add_metric(name);
    }

    pub fn register_component(&mut self, component: Component){
        self.context.register_component(component);
    }

    pub fn report_metric(&mut self, component_guid: String, metric_name: String, value: f64,
        options: Option<(u64, f64, f64, f64)>) -> f64{
        self.context.report_metric(component_guid, metric_name, value, options)
    }

    fn context_duration(&self) -> i64{
        if self.context.last_reported.is_some(){
            let now: DateTime<Utc> = Utc::now();
            return now.timestamp() - self.context.last_reported.unwrap();
        }else{
            MAX
        }
    }

    fn finish_cycle(&mut self){
        info!(target: "agent", "Finishing cycle. Elapsed: {}.", self.context_duration());
        if self.context_duration() >= self.config.deliver_cycle(){
            info!(target: "agent", "Sending metrics.");
            self.context.deliver();
        }
        info!(target: "agent", "Context now: {}", self.context);
    }

    pub fn run<F>(mut self, mut cycle_fn: F) where F: FnMut(&mut Agent<T>){
        loop{
            info!(target: "agent", "Starting cycle fn.");
            cycle_fn(&mut self);
            self.finish_cycle();
            thread::sleep(Duration::from_secs(self.config.poll_cycle() as u64));
        }
    }
}