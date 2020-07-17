
pub const MIN_REQUEST_INTERVAL: f64 = 60.0;
pub const MAX_REQUEST_INTERVAL: f64 = 0.2;

pub struct Crawler {
   pub request_interval: f64,
}
impl Crawler {
   pub fn new() -> Crawler {
      Crawler {
         request_interval: MAX_REQUEST_INTERVAL,
      }
   }
}
