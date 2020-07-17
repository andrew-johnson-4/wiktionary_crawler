use std::collections::HashSet;
use std::io::prelude::*;
use crate::WiktionaryEntry;

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
      cr.resume_data_from_file("crawl_data.partial.txt")?;
      Ok(cr)
   }
   pub fn add_to_queue(&mut self, url: &str) {
      if !self.crawl_visited_urls.contains(url) {
         self.crawl_queue.insert(url.to_string());
      }
   }
   pub fn persist_queue_to_file(&mut self, fp: &str) -> std::io::Result<()> {
      let mut file = std::fs::File::create(fp)?;
      for url in self.crawl_visited_urls.iter() {
         file.write_all(format!("{} true", url).as_bytes())?;
      }
      for url in self.crawl_queue.iter() {
         file.write_all(format!("{} false", url).as_bytes())?;
      }
      Ok(())
   }
   pub fn resume_queue_from_file(&mut self, fp: &str) -> std::io::Result<()> {
      if !std::path::Path::new(fp).exists() {
         std::fs::File::create(fp)?;
      }
      {
         let file = std::fs::File::open(fp)?;
         for line in std::io::BufReader::new(file).lines() {
            let line = line?;
            let ls = line.split_whitespace().collect::<Vec<&str>>();
            if ls.len() != 2 { continue; }
            let url = ls[0].to_string();
            let visited = ls[1]=="true";
            if visited {
               self.crawl_visited_urls.insert(url);
            } else {
               self.crawl_queue.insert(url);
            }
         }
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
