# New Relic Rust Plugin Agent SDK

This project represents a Rust binding of New Relic Plugin API. This crate can be used to build plugin agents.

# Examples
```rust
// Plugin agent setup
let mut agent = Agent::new("<license_key>".into(), "1.0.0".into(), "host".into(), 1234);
let mut c1 = agent.create_component("Test Plugin".into(), "com.test_plugin.plugin_name".into());
agent.create_metric(&mut c1, "Component/Request/Rate/host1[requests/second]".into());
agent.create_metric(&mut c1, "Component/Request/Rate/host2[requests/second]".into());
agent.register_component(c1);

// Poll cycle function. This function is excuted every [poll_cycle] seconds.
fn cycle(agent: &mut Agent){
    agent.report_metric(
        "com.test_plugin.plugin_name".into(),
        "Component/Request/Rate/host1[requests/second]".into(), 
        (1000) as f64, None
    );
    agent.report_metric(
        "com.test_plugin.plugin_name".into(),
        "Component/Request/Rate/host2[requests/second]".into(), 
        (1000) as f64, None
    );
}
// Start the agent
agent.run(cycle);
```

# state
An agent has the option to have a state to be able to preserve metric readings through cycles. States are passed to *Agent* instances via the ```set_state``` function. States are generic types, which means you can create a custom struct like in this example:

```rust
struct State{
    prev_file_size: i32
}

// Plugin agent setup
let mut agent = Agent::new("<license_key>".into(), "1.0.0".into(), "host".into(), 1234);
let mut c1 = agent.create_component("Test Plugin".into(), "com.test_plugin.plugin_name".into());
agent.create_metric(&mut c1, "Component/File/Size/host1[bytes]".into());
agent.register_component(c1);

// Poll cycle function. This function is excuted every [poll_cycle] seconds.
fn cycle(agent: &mut Agent){
    let prev_size = agent.get_state().as_ref().unwrap().prev_file_size.clone();
    let new_size = 2000;
    if prev_size != 0{
        agent.report_metric(
            "com.test_plugin.plugin_name".into(),
            "Component/File/Size/host1[bytes]".into(), 
            new_size - prev_size as f64, None
        );  
    }
    agent.set_state(State{prev_file_size: new_size});
}
// Start the agent
agent.run(cycle);
```

# config

NewRelic plugin reads configuration from a ```config.yml``` file located in the current working directory. If no ```config.yml``` file is present, default values are used. Possible config keys and values:

| Config key | Description | Default |
| ------------- |:-------------:| ----- |
| endpoint | NewRelic custom plugin API endpoint| https://platform-api.newrelic.com/platform/v1/metrics |
| log4rs_file | log4rs config file | log4rs.yml |
| deliver_cycle | metric reporting frequency | 60 (seconds) |
| poll_cycle | poll cycle frequency | 20 (seconds) |

# logging

NewRelic plugin uses log4rs. All events logged from within the agent are sent to a logger named *"agent"*. You can add more loggers in log4rs config file to log messages from your plugin's code.

**Example ```log4rs.yml``` file:**

```yml
refresh_rate: 60 seconds

appenders:
  agent:
    kind: file
    path: "log/agent.log"
    encoder:
      pattern: "{d} - {l} - {f} - {m}{n}"
  plugin:
    kind: file
    path: "log/plugin.log"
    encoder:
      pattern: "{d} - {l} - {f} - {m}{n}"

loggers:
  agent:
    level: error
    appenders:
      - agent
  plugin:
    level: info
    appenders:
      - plugin
```
