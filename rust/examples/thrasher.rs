use std::time::{Instant, Duration};
use rand::Rng;
use rand::distributions::Alphanumeric;

use seadawg::tdawg::core::{SeaDawgCore, SeaSinkNode};

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
  let max = 10;
  let load_start = Instant::now();

  for idx in 1..=max {
    //let mut name_string: String = rng.sample_iter(&Alphanumeric).take(rng.gen_range(10,32)).collect();
    let mut name_string: String = rng.sample_iter(&Alphanumeric).take(10).collect();
    name_string.push_str(format!("${}", idx).as_str());

    println!("Adding {}", name_string);

    let sink = SeaSinkNode::new_empty(name_string.as_str());
    seadawg.add(sink);
  }

  let load_duration = load_start.elapsed();
  //println!("Finished loading {} items {:?}", seadawg.size(), load_duration);
  println!("Finished loading in {:?}", load_duration);
  //println!("There are {} internal nodes, {} sink nodes, {} term nodes, {} edges", seadawg.inodes_count(), seadawg.snodes_count(), seadawg.tnodes_count(), seadawg.edges_count());
  let inodes_count = seadawg.inodes_count();
  let snodes_count = seadawg.snodes_count();
  let edges_count = seadawg.edges_count();
  println!("There are {} internal nodes, {} sink nodes, {} edges", inodes_count, snodes_count, edges_count);
/*
  let sink = SeaSinkNode::default();
  seadawg.add("lol", sink);
  println!("Inserted lol");

  let find_start = Instant::now();
  let result = seadawg.find_exact("lol").is_some();
  let find_duration = find_start.elapsed();

  println!("Did find lol: {}, took {:?}", result, find_duration);

  let find_start = Instant::now();
  let result = seadawg.find_exact("lo").is_none();
  let find_duration = find_start.elapsed();

  println!("Did not find lo: {}, took {:?}", result, find_duration);

  /*
    let result = seadawg.find_with_prefix("l");

    println!("Prefix of l: {:?}", result);
  */
*/
  println!("Finished, waiting 10secs");
  std::thread::sleep(Duration::from_secs(10));
}


fn rand_128_test_dood() {
  let mut seadawg = SeaDawgCore::<()>::new();

  let sink = SeaSinkNode::new_empty("lol");
  seadawg.add(sink);
  println!("Inserted lol");

  /*
  let result = seadawg.find_exact("lol").is_some();

  println!("Did find lol: {}", result);

  let result = seadawg.find_exact("lo").is_some();

  println!("Did not find lo: {}", result);
  /*
    let result = seadawg.find_with_prefix("l");

    println!("Prefix of l: {:?}", result);
  */
*/
  println!("Loading Items");
  let mut rng = rand::thread_rng();
  let max = 200000;
  let load_start = Instant::now();

  for idx in 1..=max {
    let mut name_string: String = rng.sample_iter(&Alphanumeric).take(rng.gen_range(10,128)).collect();
    name_string.push_str(format!("${}", idx).as_str());

    let sink = SeaSinkNode::new_empty(name_string.as_str());
    seadawg.add(sink);
  }

  for idx in 1..=max {

    let mut name_string = String::from("I_am_not_a_test_dood");
    name_string.push_str(rng.gen_range(1i32, 10000000i32).to_string().as_str());
    name_string.push_str(format!("${}", idx).as_str());

    let sink = SeaSinkNode::new_empty(name_string.as_str());
    seadawg.add(sink);
  }

  for idx in 1..=max {

    let mut name_string = String::from("test_dood");
    //name_string.push_str(rng.gen_range(1i32, 10000000i32).to_string().as_str());
    name_string.push_str(format!("${}", idx).as_str());

    let sink = SeaSinkNode::new_empty(name_string.as_str());
    seadawg.add(sink);
  }

  let load_duration = load_start.elapsed();
  println!("Finished loading {} items {:?}", seadawg.size(), load_duration);
  println!("There are {} internal nodes, {} sink nodes, {} edges", seadawg.inodes_count(), seadawg.snodes_count(), seadawg.edges_count());


  println!("Finished, waiting 10secs");
  std::thread::sleep(Duration::from_secs(10));
}