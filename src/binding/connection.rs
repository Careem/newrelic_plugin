use serde_json::{Value, from_str as unjson};
use binding::config::Config;
use std::io::Read;
use curl::easy::{Easy, List};

#[derive(Debug)]
pub struct Connection{
    data: Value,
    license_key: String,
    url: String
}

impl Connection {
    pub fn new(data: Value, license_key: String) -> Self{
        Connection{
            data: data,
            license_key: license_key,
            url: Config::new().get_endpoint()
        }
    }

    pub fn send_request(&self) -> bool{
        info!(target: "agent", "\tJSON Payload: {}", self.data.to_string());
        let body = self.data.clone();
        let body = body.to_string();
        let mut body = body.as_bytes();
        let mut response_body = Vec::new(); 
        let mut status_code = 0;
        let mut easy = Easy::new();
        let response;
        easy.url(&self.url).unwrap();
        easy.post(true).unwrap();
        easy.post_field_size(body.len() as u64).unwrap();

        let mut list = List::new();
        list.append(&format!("X-License-Key: {}", self.license_key)).unwrap();
        list.append("Content-Type: application/json").unwrap();
        list.append("Accept: application/json").unwrap();
        easy.http_headers(list).unwrap();
        {

            let mut transfer = easy.transfer();
            transfer.read_function(|buf| {
                Ok(body.read(buf).unwrap_or(0))
            }).unwrap();
            transfer.write_function(|data| {
                response_body.extend_from_slice(data);
                Ok(data.len())
            }).unwrap();
            let _ = transfer.header_function(|hh| {
                let header = String::from_utf8(hh.to_vec()).unwrap();
                if header.contains("HTTP/1.1"){
                    status_code = usize::from_str_radix(header.split(" ").nth(1).unwrap(), 10).unwrap();
                }
                true
            });
            response = transfer.perform();
        }
        let response_body = String::from_utf8(response_body).unwrap();
        info!(target: "agent", "Response: {:?}", response);
        debug!(target: "agent", "Status code: {}", status_code);
        debug!(target: "agent", "Response body: {}", response_body);

        if response.is_err(){
            error!(target: "agent", "Connection Error: {}", response.err().unwrap());
            return false;
        }else{
            return self.evaluate_response(status_code, response_body);
        }
    }

    fn evaluate_response(&self, response_code: usize, response_body: String) -> bool{
        let mut return_status = None;
        let last_result: Value;
        match response_code{
            200 => {
                last_result = unjson(&response_body).unwrap();
                if last_result["status"] != "ok"{
                    return_status = Some(format!("FAILED {}, {}", response_code, last_result["error"]));
                }
            },
            403 => {
                error!(target: "agent", "Forbidden request. Will Panic. Response: {}", response_body);
                panic!("Forbidden request. Response: {}", response_body);
            },
            503 => {
                error!(target: "agent", "Collector temporarily unavailable. Continuing.");
            },
            _ => {
                if response_body.len() > 0 {
                    last_result = unjson(&response_body).unwrap();
                }else{
                    last_result = json!({"error": "no data returned"});
                }
                return_status = Some(format!("FAILED {}, {}", response_code, last_result["error"]));
            }
        }
        if return_status.is_some(){
            error!(target: "agent", "{}", return_status.as_ref().unwrap());
        }
        return_status.is_none()
    }
}

