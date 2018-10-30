use std::fmt;

#[derive(Debug, Clone)]
pub struct Metric{
    pub name: String,
    pub value: f64,
    pub prev: f64,
    count: u64,
    min: f64,
    max: f64,
    sum_of_squares: f64
}

impl fmt::Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}, Value: {}, Count: {}, Min.: {}, Max.: {}, Sum of sqaures: {}",
            self.name, self.value, self.count, self.min, self.max, self.sum_of_squares)
    }
}

impl Metric{

    pub fn new(name: String) -> Self{
        Metric{
            name: name,
            value: 0f64,
            prev: 0f64,
            count: 0,
            min: 0f64,
            max: 0f64,
            sum_of_squares: 0f64
        }
    }

    pub fn new_valued(name: String, value: f64,
        options: Option<(u64, f64, f64, f64)>) -> Self{
        if options.is_some(){
            let options = options.unwrap();
            Metric{
                name: name,
                value: value,
                prev: 0f64,
                count: options.0,
                min: options.1,
                max: options.2,
                sum_of_squares: options.3
            }
        }else{
            Metric{
                name: name,
                value: value,
                prev: 0f64,
                count: 1,
                min: value,
                max: value,
                sum_of_squares: value * value
            }
        }
    }

    pub fn aggregate(&mut self, metric: &Metric) -> f64{
        let prev = self.prev.clone();
        self.prev = metric.value;
        self.value += metric.value;
        if self.count == 0{
            self.min = metric.value;
        }else{
            self.min = self.min.min(metric.min);
        }
        self.count += metric.count;
        self.max = self.min.max(metric.max);
        self.sum_of_squares += metric.sum_of_squares;
        prev
    }

    pub fn to_hash(&self) -> (String, Vec<f64>){
        (
            self.name.clone(),
            vec![self.value, self.count as f64, self.min, self.max, self.sum_of_squares]
        )
    }

    pub fn reset(&mut self){
        // self.prev = self.value;
        self.value = 0f64;
        self.count = 0;
        self.min = 0f64;
        self.max = 0f64;
        self.sum_of_squares = 0f64;
    }
}
