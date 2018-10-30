use binding::connection::Connection;
use serde_json::Value;

#[derive(Debug)]
pub struct Request{
    data: Value,
    license_key: String,
    delivered: bool
}

impl Request{
    pub fn new(data: Value, license_key: String) -> Self{
        Request{
            data: data,
            license_key: license_key,
            delivered: false
        }
    }

    pub fn send(&mut self) -> bool{
        let connection = Connection::new(self.data.clone(), self.license_key.clone());
        if connection.send_request(){
            self.delivered = true;
            return true;
        }
        false
    }
}

