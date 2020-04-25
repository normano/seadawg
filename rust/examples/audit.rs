use std::time::{Instant, Duration};
use rand::Rng;
use rand::distributions::Alphanumeric;
use rand::prelude::ThreadRng;

use seadawg::bt::core::{SeaDawgCore, SeaSinkNode};

//#[global_allocator]
//static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
  rand_128();
}

fn rand_128() {

  let mut rng = rand::thread_rng();
  let test_count = 100000;
  let verbose = false;

  for test_idx in 0..test_count {

    let mut seadawg = SeaDawgCore::<()>::new();
    println!("Test #{}", test_idx + 1);

    let max = 10;
    let load_start = Instant::now();
    let mut words = vec![];

    if verbose {
      println!("Loading Items");
    }

    for idx in 1..=max {
      let mut name_string: String = rng.sample_iter(&Alphanumeric).take(rng.gen_range(10, 128)).collect();
      name_string.push_str(format!("\0", idx).as_str());

      println!("Adding {}", name_string);

      let sink = SeaSinkNode::new_empty(name_string.as_str());
      seadawg.add(sink);
      words.push(name_string);
    }

    if verbose {
      let load_duration = load_start.elapsed();
      //println!("Finished loading {} items {:?}", seadawg.size(), load_duration);
      println!("Finished loading in {:?}", load_duration);
      //println!("There are {} internal nodes, {} sink nodes, {} term nodes, {} edges", seadawg.inodes_count(), seadawg.snodes_count(), seadawg.tnodes_count(), seadawg.edges_count());
      let inodes_count = seadawg.inodes_count();
      let snodes_count = seadawg.snodes_count();
      let edges_count = seadawg.edges_count();
      println!("There are {} internal nodes, {} sink nodes, {} edges", inodes_count, snodes_count, edges_count);
    }

    verify_find_exact(&seadawg, &words);
    /*
    verify_find_prefix(&mut rng, &seadawg, &words);
    verify_find_suffix(&mut rng, &seadawg, &words);
    verify_find_superstring(&mut rng, &seadawg, &words);
    */
  }

/*
  let sink = SeaSinkNode::default();
  seadawg.add("lol", sink);
  println!("Inserted lol");

  let find_start = Instant::now();

  println!("Did not find lo: {}, took {:?}", result, find_duration);

  /*
    let result = seadawg.find_with_prefix("l");

    println!("Prefix of l: {:?}", result);
  */
*/
  println!("Finished, waiting 10secs");
  std::thread::sleep(Duration::from_secs(10));
}

fn verify_find_exact(seadawg: &SeaDawgCore<()>, words: &Vec<String>) {
  println!("---------- Verifying fn: find exact ----------");
  let find_start = Instant::now();
  for word in words.iter() {
    let result = seadawg.find_exact(word.as_str());
    if result.is_some() == false {
      println!("Did not find {}", word);
      panic!("");
    }
  }
  let find_duration = find_start.elapsed();
  println!("Total find exact time: {:?}", find_duration);
}

/*
fn verify_find_prefix(rng: &mut ThreadRng, seadawg: &SeaDawgCore<()>, words: &Vec<String>) {

  println!("----------  Verifying fn: find_with_prefix ----------");
  let find_start = Instant::now();
  for word in words.iter() {

    let prefix = &word.as_str()[0..(rng.gen_range(2, word.len()))];
    let mut expected_prefix_count = 0;

    for word in words.iter() {
      if word.starts_with(prefix) {
        expected_prefix_count += 1;
      }
    }

    let result = seadawg.find_with_prefix(prefix);
    let found_count = result.len();

    if expected_prefix_count != found_count {
      println!("Actual prefix count {} did not match expected {}.", found_count, expected_prefix_count);
      println!("Searched Prefix {}", prefix);
      println!("{:?}", result);
      panic!("");
    }
  }
  let find_duration = find_start.elapsed();
  println!("Total find time: {:?}", find_duration);
}

fn verify_find_suffix(rng: &mut ThreadRng, seadawg: &SeaDawgCore<()>, words: &Vec<String>) {

  println!("---------- Verifying fn: find_with_suffix ---------- ");
  let find_start = Instant::now();
  for word in words.iter() {

    let suffix = &word.as_str()[(rng.gen_range(0, word.len()-1))..word.len()];

    let mut expected_suffix_count = 0;
    for word in words.iter() {
      if word.ends_with(suffix) {
        expected_suffix_count += 1;
      }
    }

    let result = seadawg.find_with_suffix(suffix);
    let found_count = result.len();

    if expected_suffix_count != found_count {
      println!("Actual suffix count {} did not match expected {}.", found_count, expected_suffix_count);
      println!("Searched suffix {}", suffix);
      println!("result: {:?}", result);
      println!("words: {:?}", words);
      panic!();
    }
  }
  let find_duration = find_start.elapsed();
  println!("Total find time: {:?}", find_duration);
}

fn verify_find_superstring(rng: &mut ThreadRng, seadawg: &SeaDawgCore<()>, words: &Vec<String>) {

  println!("---------- Verifying fn: find_with_substring ----------");
  let find_start = Instant::now();
  for word in words.iter() {

    let start_idx = rng.gen_range(0, word.len()-1);
    let max_end_len = rng.gen_range(start_idx + 1, word.len());
    let end_len;
    if max_end_len > start_idx + 1 {
      end_len = rng.gen_range(start_idx + 1, max_end_len);
    } else {
      end_len = start_idx + 1;
    }

    let substring = &word.as_str()[start_idx..end_len];

    let mut expected_superstring_count = 0;
    for word in words.iter() {
      if word.contains(substring) {
        expected_superstring_count += 1;
      }
    }

    let result = seadawg.find_with_substring(substring);
    let found_count = result.len();

    if expected_superstring_count != found_count {
      println!("Actual superstring count {} did not match expected {}.", found_count, expected_superstring_count);
      println!("Searched substring {}", substring);
      println!("result: {:?}", result);
      println!("words: {:?}", words);
      panic!("GG");
    }
  }
  let find_duration = find_start.elapsed();
  println!("Total find time: {:?}", find_duration);
}
*/