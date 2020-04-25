use std::time::{Instant, Duration};
use std::fs::File;
use std::io::{BufReader, BufRead};

use seadawg::bdawg::core::{SeaDawgCore, SeaSinkNode};

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
  read_500k_freq_data();
}

fn read_500k_freq_data() {
  let mut seadawg = SeaDawgCore::<()>::new();
  // seadawg.enable_lite();

  println!("Loading Items");
  let load_start = Instant::now();

  {
    // data sets: https://archive.org/details/doi-urls
    let f = File::open("/Users/norm/frequency_dictionary_en_500_000.txt").unwrap();
    let mut reader = BufReader::new(f);

    for result in reader.lines() {
      let record = result.unwrap();
      let text = record.split(" ").nth(0).unwrap().trim_start_matches('\u{feff}');

      //println!("{:?}", text);
      let sink = SeaSinkNode::new((), text);
      seadawg.add(sink);
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