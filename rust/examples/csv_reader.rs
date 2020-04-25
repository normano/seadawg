use std::time::{Instant, Duration};
use std::fs::File;
use std::io::BufReader;

use seadawg::tdawg::core::{SeaDawgCore, SeaSinkNode};

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
  read_doi_data();
}

fn read_doi_data() {
  let mut seadawg = SeaDawgCore::<()>::new();
  // seadawg.enable_lite();

  println!("Loading Items");
  let load_start = Instant::now();

  {
    // data sets: https://archive.org/details/doi-urls
    let f = File::open("/Users/norm/2011.csv").unwrap();
    let mut reader = BufReader::new(f);
    let mut rdr = csv::Reader::from_reader(reader);

    let mut idx = 0;
    for result in rdr.records() {
      let record = result.unwrap();
      let url_opt = record.get(record.len() - 1);

      //println!("{:?}", url_opt);
      if url_opt.is_some() {
        let mut name_string = url_opt.unwrap().to_string();
        name_string.push_str(format!("${}", idx).as_str());

        let sink = SeaSinkNode::new_empty(name_string.as_str());
        seadawg.add(sink);

        idx += 1;
      }
    }
  }

  let load_duration = load_start.elapsed();
  println!("Finished loading {} items {:?}", seadawg.size(), load_duration);
  //println!("There are {} internal nodes, {} sink nodes, {} term nodes, {} edges", seadawg.inodes_count(), seadawg.snodes_count(), seadawg.tnodes_count(), seadawg.edges_count());
  println!("There are {} internal nodes, {} sink nodes, {} edges", seadawg.inodes_count(), seadawg.snodes_count(), seadawg.edges_count());

  /*
    let result = seadawg.find_with_prefix("http://");

    println!("Prefix of l: {:?}", result);
  */

  println!("Finished, waiting 10secs");
  std::thread::sleep(Duration::from_secs(10));
}