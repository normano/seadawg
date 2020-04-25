use std::time::{Instant, Duration};
use rand::Rng;
use rand::distributions::Alphanumeric;

use seadawg::bdawg::core::{SeaDawgCore, SeaSinkNode};

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
  rand_128();
  //rand_128_test_dood();
}

fn rand_128() {
  let mut seadawg = SeaDawgCore::new();

  println!("Loading Items");
  let mut rng = rand::thread_rng();
  let max = 300000;
  let load_start = Instant::now();

  for idx in 1..=max {
    //let mut name_string: String = rng.sample_iter(&Alphanumeric).take(rng.gen_range(10,32)).collect();
    let mut name_string: String = rng.sample_iter(&Alphanumeric).take(128).collect();

    //println!("Adding {}", name_string);

    let sink = SeaSinkNode::new_empty(name_string.as_str());
    seadawg.add(sink);
  }

  let load_duration = load_start.elapsed();
  println!("Finished loading in {:?}", load_duration);

  let inodes_count = seadawg.inodes_count();
  let snodes_count = seadawg.snodes_count();
  let edges_count = seadawg.edges_count();
  println!("There are {} internal nodes, {} sink nodes, {} edges", inodes_count, snodes_count, edges_count);

  println!("Finished, waiting 10secs");
  std::thread::sleep(Duration::from_secs(10));
}


fn rand_128_test_dood() {
  let mut seadawg = SeaDawgCore::<()>::new();

  println!("Loading Items");
  let mut rng = rand::thread_rng();
  let max = 200000;
  let load_start = Instant::now();

  for idx in 1..=max {
    // let mut name_string: String = rng.sample_iter(&Alphanumeric).take(rng.gen_range(10,128)).collect();
    let mut name_string: String = rng.sample_iter(&Alphanumeric).take(128).collect();

    let sink = SeaSinkNode::new_empty(name_string.as_str());
    seadawg.add(sink);
  }

  for idx in 1..=max {

    let mut name_string = String::from("I_am_not_a_test_dood");
    name_string.push_str(rng.gen_range(1i32, 10000000i32).to_string().as_str());

    let sink = SeaSinkNode::new_empty(name_string.as_str());
    seadawg.add(sink);
  }

  for idx in 1..=max {

    let mut name_string = String::from("test_dood");
    name_string.push_str(rng.gen_range(1i32, 10000000i32).to_string().as_str());

    let sink = SeaSinkNode::new_empty(name_string.as_str());
    seadawg.add(sink);
  }

  let load_duration = load_start.elapsed();
  println!("Finished loading {} items {:?}", seadawg.size(), load_duration);
  println!("There are {} internal nodes, {} sink nodes, {} edges", seadawg.inodes_count(), seadawg.snodes_count(), seadawg.edges_count());

  println!("Finished, waiting 10secs");
  std::thread::sleep(Duration::from_secs(10));
}