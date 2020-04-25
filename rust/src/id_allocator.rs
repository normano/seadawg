use croaring::Bitmap;

//TODO: Should look at creating an interval version based on sorted vectors and see how much space is needed
// Idea is that a sorted vector of intervals is a relatively cheap way to keep track of freed ids and binary search
// would not be bad. With intervals, there could be a "context" object that can be passed around pointing to the free intervals
// for fast id allocation before reverting to increment based allocation.
pub (in crate) struct U32IdAllocator {
  _next_id: u32,
  _unused_ids: Bitmap,
}

impl U32IdAllocator {

  pub (in crate) fn new() -> Self {

    return Self {
      _next_id: 0,
      _unused_ids: Bitmap::create(),
    };
  }

  pub (in crate) fn new_start_at(start_id: u32) -> Self {
    return Self {
      _next_id: start_id,
      _unused_ids: Bitmap::create(),
    };
  }

  pub fn next_id(&mut self) -> u32 {

    if self._unused_ids.is_empty() {

      if self._next_id + 1 == std::u32::MAX {
        panic!("ID Space was exhausted");
      }

      let next_id = self._next_id;
      self._next_id += 1;
      return next_id;
    }

    let reuse_id = self._unused_ids.minimum().unwrap();
    self._unused_ids.remove(reuse_id);

    return reuse_id;
  }

  pub fn free_id(&mut self, id: u32) {

    self._unused_ids.add(id);
  }
}