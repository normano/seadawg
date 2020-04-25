use std::time::Duration;

use seadawg::bt::core::{SeaDawgCore, SeaSinkNode};

fn main() {
  let expected = 8;
  let words = vec![
    "mPxVpn0JVJ4orUievpJX8l7Uq9T8DC5NYVFTAEvzQ4DI3ECUOwBfGVa8KYQ",
    "M4HgKUvD7L1Ec9ETsCzAfJqg0HY3eBw0S8yTQc9EQt1cfRx3hQNaZCkgPbTL5LIMFpsF90G1Ueuwkn8rzjgQYPweeXePV5GSzKfTpPLrmCcu7Do2908Woyx0Sut",
    "fGGFDCZ7cvEU1LcsV6xcbciLzjuT5gPDMPpspziXnehSWYImO",
    "72DewQLAaA4qkFbFPdAKMWRJMnXvQcDWsulRy1AomO0wf9TTTtYb3lupi1BKpvAJzU6RYL8PfevXZsUg",
    "qpEjdcSKcKEf",
    "pQgBN7qawChBxntLeCgkz8jz9AWyBDdQVq6Y2RfH8HIDoiWQ8s7qCXMNkYmsZp5h1Tt6OZk2ATGkEmSeqnR61xsKrIhImRzhLYQxjm2",
    "i2Mnwma8iU191lg5vOzOXMY8yE4dbKJSka2MWJvFPQz0Op6dXQwB60VBkq",
    "SoUBSrwZQa1t16tNndocVpwfAG6mTIMQIKSYT4Cz8oCphW4OB5rD380DNK6xkC5NzQSlvPVTXshWbSCbx0xPUX4PDq",
    "umkIOfdwRLe2fJ6NPkNCuhWC2tEVtY4gFwl95",
    "V2HmPl66RYoKsDjwCDdDx1FTEKR0yjd5QhNKSKSlaZiXfEA"];

  let mut seadawg = SeaDawgCore::new();

  for word in words {
    seadawg.add(SeaSinkNode::new((), word));
  }
/*
  let results = seadawg.find_with_substring("p");

  if results.len() != expected {
    println!("{} {}", results.len(), expected);
    println!("{:?}", results);
    panic!("Results count mismatch");
  }
  */
  println!("Finished waiting 10secs");
  std::thread::sleep(Duration::from_secs(10));
}