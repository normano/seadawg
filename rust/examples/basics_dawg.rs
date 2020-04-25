use std::time::{Instant, Duration};
use rand::Rng;
use rand::distributions::Alphanumeric;

// use seadawg::bdawg::core::{SeaDawgCore, SeaSinkNode};
use seadawg::bt::core::{SeaDawgCore, SeaSinkNode};

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
  cocoa();
  find_exact();
  find_prefix();
  find_suffix();
  find_with_substring();

  println!("-------------------- End --------------------");

  println!("Finished, waiting 10secs");
  std::thread::sleep(Duration::from_secs(10));
}

fn cocoa() {
  let mut seadawg = SeaDawgCore::new();

  let sink = SeaSinkNode::new_empty("cocoacoal");
  seadawg.add(sink);

  let sink = SeaSinkNode::new_empty("cocoa");
  seadawg.add(sink);

  let sink = SeaSinkNode::new_empty("cola");
  seadawg.add(sink);

  let sink = SeaSinkNode::new_empty("coa");
  seadawg.add(sink);

  println!("{:?}", seadawg);

  let find_start = Instant::now();
  let result = seadawg.find_exact("coa").unwrap() == 3;
  let find_duration = find_start.elapsed();
  println!("Did find coa: {}, took {:?}", result, find_duration);

  let inodes_count = seadawg.inodes_count();
  let snodes_count = seadawg.snodes_count();
  let edges_count = seadawg.edges_count();
  println!("There are {} internal nodes, {} sink nodes, {} edges", inodes_count, snodes_count, edges_count);
}

fn find_exact() {
  println!("-------------------- Find Exact --------------------");
  let mut seadawg = SeaDawgCore::<()>::new();

  let sink = SeaSinkNode::new_empty("lol");
  seadawg.add(sink);
  println!("Inserted lol");

  let find_start = Instant::now();
  let result = seadawg.find_exact("lol").is_some();
  let find_duration = find_start.elapsed();

  println!("Did find lol: {}, took {:?}", result, find_duration);

  let find_start = Instant::now();
  let result = seadawg.find_exact("l").is_none();
  let find_duration = find_start.elapsed();

  println!("Did not find l: {}, took {:?}", result, find_duration);

  let find_start = Instant::now();
  let result = seadawg.find_exact("lo").is_none();
  let find_duration = find_start.elapsed();

  println!("Did not find lo: {}, took {:?}", result, find_duration);

  let find_start = Instant::now();
  let result = seadawg.find_exact("ol").is_none();
  let find_duration = find_start.elapsed();

  println!("Did not find ol: {}, took {:?}", result, find_duration);

  let mut seadawg = SeaDawgCore::<()>::new();

  let sink = SeaSinkNode::new_empty("I_am_not_a_test_dood48902");
  seadawg.add(sink);
  println!("Inserted I_am_not_a_test_dood48902");

  let sink = SeaSinkNode::new_empty("I_am_not_a_test_dood48663");
  seadawg.add(sink);
  println!("Inserted I_am_not_a_test_dood48663");

  let sink = SeaSinkNode::new_empty("I_am_not_a_test_dood4866");
  seadawg.add(sink);
  println!("Inserted I_am_not_a_test_dood4866");

  let result = seadawg.find_exact("I_am_not_a_test_dood48902").is_some();
  println!("Did find I_am_not_a_test_dood48902: {}, took {:?}", result, find_duration);

  let result = seadawg.find_exact("I_am_not_a_test_dood48663").is_some();
  println!("Did find I_am_not_a_test_dood48663: {}, took {:?}", result, find_duration);

  let result = seadawg.find_exact("I_am_not_a_test_dood4866").is_some();
  println!("Did find I_am_not_a_test_dood4866: {}, took {:?}", result, find_duration);
}

fn find_prefix() {
  println!("-------------------- Find Prefix --------------------");
  let mut seadawg = SeaDawgCore::<()>::new();

  println!("Loading Items");
  let mut rng = rand::thread_rng();
  let max = 300;
  let load_start = Instant::now();

  let sink = SeaSinkNode::new_empty("lol");
  seadawg.add(sink);
  println!("Inserted lol");

  for _ in 1..=max {
    let mut name_string: String = rng.sample_iter(&Alphanumeric).take(32).collect();

    let sink = SeaSinkNode::new_empty(name_string.as_str());
    seadawg.add(sink);
  }

  let load_duration = load_start.elapsed();
  println!("Finished loading {} items {:?}", seadawg.size(), load_duration);
 // println!("There are {} internal nodes, {} sink nodes, {} term nodes, {} edges", seadawg.inodes_count(), seadawg.snodes_count(), seadawg.tnodes_count(), seadawg.edges_count());
  println!("There are {} internal nodes, {} sink nodes, {} edges", seadawg.inodes_count(), seadawg.snodes_count(), seadawg.edges_count());

  let find_start = Instant::now();

  let result = seadawg.find_with_prefix("l");

  println!("Words starting with 'l': {:?}", result);
  let find_duration = find_start.elapsed();

  println!("Took {:?}", find_duration);
}

fn find_suffix() {
  println!("-------------------- Find Suffix --------------------");
  let mut seadawg = SeaDawgCore::<()>::new();

  println!("Loading Items");
  let mut rng = rand::thread_rng();
  let max = 300;
  let load_start = Instant::now();

  let sink = SeaSinkNode::new_empty("lol");
  seadawg.add(sink);
  println!("Inserted lol");

  let sink = SeaSinkNode::new_empty("ol1");
  seadawg.add(sink);
  println!("Inserted ol1");

  let sink = SeaSinkNode::new_empty("lo2");
  seadawg.add(sink);
  println!("Inserted lo2");


  for _ in 1..=max {
    let mut name_string: String = rng.sample_iter(&Alphanumeric).take(32).collect();

    let sink = SeaSinkNode::new_empty(name_string.as_str());
    seadawg.add(sink);
  }

  let load_duration = load_start.elapsed();
  println!("Finished loading {} items {:?}", seadawg.size(), load_duration);
  // println!("There are {} internal nodes, {} sink nodes, {} term nodes, {} edges", seadawg.inodes_count(), seadawg.snodes_count(), seadawg.tnodes_count(), seadawg.edges_count());
  println!("There are {} internal nodes, {} sink nodes, {} edges", seadawg.inodes_count(), seadawg.snodes_count(), seadawg.edges_count());

  let find_start = Instant::now();

  let result = seadawg.find_with_suffix("l");

  println!("Words ending with 'l': {:?}", result);
  let find_duration = find_start.elapsed();

  println!("Took {:?}", find_duration);

  let find_start = Instant::now();

  let result = seadawg.find_with_suffix("ol");

  println!("Words ending with 'ol': {:?}", result);
  let find_duration = find_start.elapsed();

  println!("Took {:?}", find_duration);

  let result = seadawg.find_with_suffix("o2");

  println!("Words ending with 'o2': {:?}", result);
  let find_duration = find_start.elapsed();

  println!("Took {:?}", find_duration);

  let result = seadawg.find_with_suffix("l1");

  println!("Words ending with 'l1': {:?}", result);
  let find_duration = find_start.elapsed();

  println!("Took {:?}", find_duration);

  let result = seadawg.find_with_suffix("ol1");

  println!("Words ending with 'ol1': {:?}", result);
  let find_duration = find_start.elapsed();

  println!("Took {:?}", find_duration);
}

fn find_with_substring() {
  println!("-------------------- Find Superstrings --------------------");
  let mut seadawg = SeaDawgCore::<()>::new();

  println!("Loading Items");
  let mut rng = rand::thread_rng();
  let max = 100;
  let load_start = Instant::now();

  let sink = SeaSinkNode::new_empty("lol");
  seadawg.add(sink);
  println!("Inserted 'lol'");

  let sink = SeaSinkNode::new_empty("ole");
  seadawg.add(sink);
  println!("Inserted 'ole'");

  let sink = SeaSinkNode::new_empty("ol");
  seadawg.add(sink);
  println!("Inserted 'ol'");

  let sink = SeaSinkNode::new_empty("black beard");
  seadawg.add(sink);
  println!("Inserted 'black beard'");

  for _ in 1..=max {
    let mut name_string: String = rng.sample_iter(&Alphanumeric).take(32).collect();

    let sink = SeaSinkNode::new_empty(name_string.as_str());
    seadawg.add(sink);
  }

  let load_duration = load_start.elapsed();
  println!("Finished loading {} items {:?}", seadawg.size(), load_duration);
  //println!("There are {} internal nodes, {} sink nodes, {} term nodes, {} edges", seadawg.inodes_count(), seadawg.snodes_count(), seadawg.tnodes_count(), seadawg.edges_count());
  println!("There are {} internal nodes, {} sink nodes, {} edges", seadawg.inodes_count(), seadawg.snodes_count(), seadawg.edges_count());


  let find_start = Instant::now();

  let result = seadawg.find_with_substring("l");

  let find_duration = find_start.elapsed();

  println!("Words containing 'l': {:?}", result);

  println!("Took {:?}", find_duration);
}
