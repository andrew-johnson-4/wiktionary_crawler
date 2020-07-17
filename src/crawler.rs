use std::collections::HashSet;

pub const MAX_REQUEST_INTERVAL: f64 = 60.0;
pub const MIN_REQUEST_INTERVAL: f64 = 0.2;

pub struct Crawler {
   pub request_interval: f64,
   crawl_visited_urls: HashSet<String>,
   crawl_visited_words: HashSet<String>,
   crawl_queue: HashSet<String>
}
impl Crawler {
   pub fn new() -> std::io::Result<Crawler> {
      let mut cr = Crawler {
         request_interval: MIN_REQUEST_INTERVAL,
         crawl_visited_urls: HashSet::new(),
         crawl_visited_words: HashSet::new(),
         crawl_queue: HashSet::new(),
      };
      cr.resume_queue_from_file("crawl_visited.partial.txt")?;
      cr.resume_queue_from_file("crawl_data.partial.txt")?;
      Ok(cr)
   }
   pub fn add_to_queue(&mut self, url: &str) {
      if !self.crawl_visited_urls.contains(url) {
         self.crawl_queue.insert(url.to_string());
      }
   }
   pub fn resume_queue_from_file(&mut self, fp: &str) -> std::io::Result<()> {
      if !std::path::Path::new(fp).exists() {
         std::fs::File::create(fp)?;
      }
      Ok(())
   }
   pub fn resume_data_from_file(&mut self, fp: &str) -> std::io::Result<()> {
      if !std::path::Path::new(fp).exists() {
         std::fs::File::create(fp)?;
      }
      Ok(())
   }
}
