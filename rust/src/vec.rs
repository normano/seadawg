use std::marker::PhantomData;
use std::ptr;
use std::ops::{Deref, DerefMut};
use std::iter::FromIterator;
use std::mem;

pub struct VecU32<T> {
  ptr: *mut T,
  len: u32,
  cap: u32,
}

unsafe impl <T> Send for VecU32<T> {}

impl<T> VecU32<T> {
  /// Get the number of elements in the vector
  pub fn len(&self) -> usize {
    self.len as usize
  }

  pub fn is_empty(&self) -> bool {
    self.len == 0
  }

  /// Create a new, empty vector
  pub fn new() -> VecU32<T> {
    VecU32 {
      ptr: ptr::null_mut(),
      len: 0,
      cap: 0
    }
  }

  /// Create a new, empty vector with a given capacity
  pub fn with_capacity(cap: usize) -> VecU32<T> {
    unsafe {
      let layout = Layout::from_size_align_unchecked(
        (cap as usize) * size_of::<T>(),
        align_of::<T>()
      );
      let mut vec = VecU32 {
        ptr: alloc(layout) as *mut T,
        len: 0,
        cap: cap as u32,
      };

      vec
    }
  }

  /// Create a new vector from raw parts
  pub unsafe fn from_raw_parts(ptr: *mut T, len: usize, cap: usize) -> VecU32<T> {
    VecU32 {
      ptr,
      len: len as u32,
      cap: cap as u32,
    }
  }

  /// Maximum to hold before expansion
  pub fn capacity(&self) -> usize {
    self.cap as usize
  }

  /// Double the capacity of the vector by spilling onto the heap
  fn double_buf(&mut self) {
    unsafe {

      let mut new_cap = 1;
      if self.cap == 0 {

        let layout = Layout::from_size_align_unchecked(
          (new_cap as usize) * size_of::<T>(),
          align_of::<T>()
        );
        self.ptr = alloc(layout) as *mut T;
      } else {
        new_cap = self.cap * 2;

        let layout = Layout::from_size_align_unchecked(
          (self.cap as usize) * size_of::<T>(),
          align_of::<T>()
        );
        self.ptr = realloc(self.ptr as *mut u8, layout, (new_cap as usize) * size_of::<T>()) as *mut T;
      };

      self.cap = new_cap;
    }
  }

  /// Push an item into the vector
  #[inline]
  pub fn push(&mut self, value: T) {
    if self.len == self.cap {
      self.double_buf();
    }

    unsafe {
      let end = self.as_mut_ptr().offset(self.len as isize);
      ptr::write(end, value);
      self.len += 1;
    }
  }

  /// push at position
  pub fn push_at(&mut self, _: usize, value: T) {
    if self.len == self.cap {
      self.double_buf();
    }

    unsafe {
      let end = self.as_mut_ptr().offset(self.len as isize);
      ptr::write(end, value);
      self.len += 1;
    }
  }

  /// Extend from a copyable slice
  pub fn extend_from_copy_slice(&mut self, other: &[T])
  where
    T: Copy,
  {
    while self.len + other.len() as u32 > self.cap {
      self.double_buf();
    }

    let old_len = self.len as usize;
    self.len += other.len() as u32;
    self[old_len..].copy_from_slice(other);
  }

  /// Pop and return the last element, if the vector wasn't empty
  #[inline]
  pub fn pop(&mut self) -> Option<T> {
    if self.len == 0 {
      None
    } else {
      unsafe {
        self.len -= 1;
        Some(ptr::read(self.get_unchecked(self.len())))
      }
    }
  }

  /// Insert a value at `index`, copying the elements after `index` upwards
  pub fn insert(&mut self, index: usize, value: T) {
    if self.len == self.cap {
      self.double_buf();
    }

    unsafe {

      let p = self.as_mut_ptr().add(index);
      // Shift everything over to make space. (Duplicating the
      // `index`th element into two consecutive places.)
      ptr::copy(p, p.offset(1), self.len as usize - index);
      // Write it in, overwriting the first copy of the `index`th
      // element.
      ptr::write(p, value);

      self.len += 1;
    }
  }

  /// Remove the element at `index`, copying the elements after `index` downwards
  pub fn remove(&mut self, index: usize) -> T {
    let len = self.len as usize;
    assert!(index < len as usize);
    unsafe {
      // infallible
      let ret;
      {
        // the place we are taking from.
        let ptr = self.as_mut_ptr().add(index);
        // copy it out, unsafely having a copy of the value on
        // the stack and in the vector at the same time.
        ret = ptr::read(ptr);

        // Shift everything down to fill in that spot.
        ptr::copy(ptr.offset(1), ptr, len - index - 1);
      }
      self.len -= 1;
      ret
    }
  }

  /// Removes an element from the vector and returns it.
  ///
  /// The removed element is replaced by the last element of the vector.
  ///
  /// This does not preserve ordering, but is O(1).
  #[inline]
  pub fn swap_remove(&mut self, index: usize) -> T {
    unsafe {
      let len = self.len as usize;
      let hole: *mut T = &mut self[index];
      let last = ptr::read(self.get_unchecked(len - 1));
      self.len -= 1;
      ptr::replace(hole, last)
    }
  }

  /// Take a function which returns whether an element should be kept,
  /// and mutably removes all elements from the vector which are not kept
  pub fn retain<F: FnMut(&T) -> bool>(&mut self, mut keep: F) {
    let mut del = 0;
    let len = self.len as usize;
    {
      let v = &mut **self;

      for i in 0..len {
        if !keep(&v[i]) {
          del += 1;
        } else {
          v.swap(i - del, i);
        }
      }
    }

    if del > 0 {
      self.truncate(len - del);
    }
  }

  /// Truncate the vector to the given length
  pub fn truncate(&mut self, desired_len: usize) {
    unsafe {
      while desired_len < self.len as usize {
        self.len -= 1;
        let len = self.len;
        ptr::drop_in_place(self.get_unchecked_mut(len as usize));
      }
    }
  }

  #[inline]
  pub fn append(&mut self, other: &mut Self) {
    unsafe {
      self.append_elements(&other[..] as _);

      other.len = 0;
    }
  }

  /// Appends elements to `Self` from other buffer.
  #[inline]
  unsafe fn append_elements(&mut self, other: *const [T]) {
    let count = (*other).len();

    while self.len + count as u32 > self.cap {
      self.double_buf();
    }

    let len = self.len();
    ptr::copy_nonoverlapping(other as *const T, self.as_mut_ptr().add(len), count);

    self.len += count as u32;
  }

  /// Clear the vector
  #[inline]
  pub fn clear(&mut self) {
    self.truncate(0);
  }
}

impl<T> From<Vec<T>> for VecU32<T> {
  /// Create a `VecU32` from a normal `Vec`,
  /// directly using the backing storage as free heap storage
  fn from(mut vec: Vec<T>) -> Self {
    let cvec = unsafe { Self::from_raw_parts(vec.as_mut_ptr(), vec.len(), vec.capacity()) };
    ::std::mem::forget(vec);
    cvec
  }
}

impl<T> Drop for VecU32<T> {
  /// Drop elements and deallocate free heap storage, if any is allocated
  fn drop(&mut self) {
    unsafe {
      ptr::drop_in_place(&mut self[..]);
    };
  }
}

impl<T> Deref for VecU32<T> {
  type Target = [T];

  fn deref(&self) -> &[T] {
    if unsafe { self.ptr.is_null() } {
      unsafe { ::std::slice::from_raw_parts(0x1 as *const T, 0) }
    } else {
      unsafe { ::std::slice::from_raw_parts(self.ptr, self.len as usize) }
    }
  }
}

impl<T> DerefMut for VecU32<T> {
  fn deref_mut(&mut self) -> &mut [T] {
    if unsafe { self.ptr.is_null() } {
      unsafe { ::std::slice::from_raw_parts_mut(0x1 as *mut T, 0) }
    } else {
      unsafe { ::std::slice::from_raw_parts_mut(self.ptr, self.len as usize) }
    }
  }
}

pub struct IntoIter<T> {
  ptr: *mut T,
  len: usize,
  cap: usize,
  index: usize
}

impl<T> Iterator for IntoIter<T> {
  type Item = T;

  fn next(&mut self) -> Option<T> {
    if self.index < self.len {
      let item = unsafe { ptr::read(self.ptr.offset(self.index as isize)) };
      self.index += 1;
      Some(item)
    } else {
      None
    }
  }
}

impl<T> Drop for IntoIter<T> {
  fn drop(&mut self) {
    // drop all remaining elements
    unsafe {
      ptr::drop_in_place(&mut ::std::slice::from_raw_parts(
        self.ptr.offset(self.index as isize),
        self.len,
      ));
    };
  }
}

impl<T> IntoIterator for VecU32<T> {
  type Item = T;
  type IntoIter = IntoIter<T>;

  fn into_iter(self) -> Self::IntoIter {
    let iter = IntoIter {
      ptr: unsafe { &mut ptr::read(self.ptr) },
      len: self.len as usize,
      cap: self.cap as usize,
      index: 0,
    };
    ::std::mem::forget(self);
    iter
  }
}

impl<'a, T> IntoIterator for &'a VecU32<T> {
  type Item = &'a T;
  type IntoIter = ::std::slice::Iter<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a, T> IntoIterator for &'a mut VecU32<T> {
  type Item = &'a mut T;
  type IntoIter = ::std::slice::IterMut<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

impl<T: Clone> Clone for VecU32<T> {
  fn clone(&self) -> VecU32<T> {
    VecU32::from(self.iter().cloned().collect::<Vec<_>>())
  }
}

impl<T> FromIterator<T> for VecU32<T> {
  fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
    let into_iter = iter.into_iter();
    let mut vec = VecU32::with_capacity(into_iter.size_hint().0);
    for item in into_iter {
      vec.push(item);
    }
    vec
  }
}

impl<T> Extend<T> for VecU32<T> {
  fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
    for item in iter {
      self.push(item);
    }
  }
}

impl<T> Default for VecU32<T> {
  fn default() -> VecU32<T> {
    VecU32::new()
  }
}

impl<T: ::std::fmt::Debug> ::std::fmt::Debug for VecU32<T> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    (self.deref()).fmt(f)
  }
}

#[cfg(feature = "serde-serialization")]
use ::serde::ser::SerializeSeq;
use std::ptr::NonNull;
use std::alloc::{alloc, Layout, realloc, dealloc};
use std::mem::{size_of, align_of};

#[cfg(feature = "serde-serialization")]
impl<T> ::serde::ser::Serialize for VecU32<T>
where
  T: ::serde::ser::Serialize
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: ::serde::ser::Serializer,
  {
    let mut seq = serializer.serialize_seq(Some(self.len()))?;
    for e in self {
      seq.serialize_element(e)?;
    }
    seq.end()
  }
}

#[cfg(feature = "serde-serialization")]
struct VecU32Visitor<T> {
  marker: PhantomData<fn() -> VecU32<T>>
}

#[cfg(feature = "serde-serialization")]
impl<T> VecU32Visitor<T> {
  fn new() -> Self {
    VecU32Visitor {
      marker: PhantomData
    }
  }
}

#[cfg(feature = "serde-serialization")]
impl<'de, T> ::serde::de::Visitor<'de> for VecU32Visitor<T>
where
  T: ::serde::de::Deserialize<'de>
{
  type Value = VecU32<T>;

  fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    formatter.write_str("A Compact Vector")
  }

  fn visit_seq<S>(self, mut access: S) -> Result<Self::Value, S::Error>
  where
    S: ::serde::de::SeqAccess<'de>,
  {
    let mut vector = VecU32::with_capacity(access.size_hint().unwrap_or(0));

    while let Some(element) = access.next_element()? {
      vector.push(element);
    }

    Ok(vector)
  }
}

#[cfg(feature = "serde-serialization")]
impl<'de, T> ::serde::de::Deserialize<'de> for VecU32<T>
where
  T: ::serde::de::Deserialize<'de>
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: ::serde::de::Deserializer<'de>,
  {
    deserializer.deserialize_map(VecU32Visitor::new())
  }
}

pub mod u8 {

  #[repr(packed)]
  pub struct VecU8<T> {
    ptr: *mut T,
    len: u8,
    cap: u8,
  }

  impl<T> VecU8<T> {
    /// Get the number of elements in the vector
    pub fn len(&self) -> usize {
      self.len as usize
    }

    pub fn is_empty(&self) -> bool {
      self.len == 0
    }

    /// Create a new, empty vector
    pub fn new() -> VecU8<T> {
      VecU8 {
        ptr: ptr::null_mut(),
        len: 0,
        cap: 0
      }
    }

    /// Create a new, empty vector with a given capacity
    pub fn with_capacity(cap: usize) -> VecU8<T> {
      unsafe {
        let layout = Layout::from_size_align_unchecked(
          (cap as usize) * size_of::<T>(),
          align_of::<T>()
        );
        let mut vec = VecU8 {
          ptr: alloc(layout) as *mut T,
          len: 0,
          cap: cap as u8,
        };

        vec
      }
    }

    /// Create a new vector from raw parts
    pub unsafe fn from_raw_parts(ptr: *mut T, len: usize, cap: usize) -> VecU8<T> {
      VecU8 {
        ptr,
        len: len as u8,
        cap: cap as u8,
      }
    }

    /// Maximum to hold before expansion
    pub fn capacity(&self) -> usize {
      self.cap as usize
    }

    /// Double the capacity of the vector by spilling onto the heap
    fn double_buf(&mut self) {
      unsafe {

        let mut new_cap = 1;
        if self.cap == 0 {

          let layout = Layout::from_size_align_unchecked(
            (new_cap as usize) * size_of::<T>(),
            align_of::<T>()
          );
          self.ptr = alloc(layout) as *mut T;
        } else {
          new_cap = self.cap * 2;

          let layout = Layout::from_size_align_unchecked(
            (self.cap as usize) * size_of::<T>(),
            align_of::<T>()
          );
          self.ptr = realloc(self.ptr as *mut u8, layout, (new_cap as usize) * size_of::<T>()) as *mut T;
        };

        self.cap = new_cap;
      }
    }

    /// Push an item into the vector
    #[inline]
    pub fn push(&mut self, value: T) {
      if self.len == self.cap {
        self.double_buf();
      }

      unsafe {
        let end = self.as_mut_ptr().offset(self.len as isize);
        ptr::write(end, value);
        self.len += 1;
      }
    }

    /// push at position
    pub fn push_at(&mut self, _: usize, value: T) {
      if self.len == self.cap {
        self.double_buf();
      }

      unsafe {
        let end = self.as_mut_ptr().offset(self.len as isize);
        ptr::write(end, value);
        self.len += 1;
      }
    }

    /// Extend from a copyable slice
    pub fn extend_from_copy_slice(&mut self, other: &[T])
    where
      T: Copy,
    {
      while self.len + other.len() as u8 > self.cap {
        self.double_buf();
      }

      let old_len = self.len as usize;
      self.len += other.len() as u8;
      self[old_len..].copy_from_slice(other);
    }

    /// Pop and return the last element, if the vector wasn't empty
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
      if self.len == 0 {
        None
      } else {
        unsafe {
          self.len -= 1;
          Some(ptr::read(self.get_unchecked(self.len())))
        }
      }
    }

    /// Insert a value at `index`, copying the elements after `index` upwards
    pub fn insert(&mut self, index: usize, value: T) {
      if self.len == self.cap {
        self.double_buf();
      }

      unsafe {

        let p = self.as_mut_ptr().add(index);
        // Shift everything over to make space. (Duplicating the
        // `index`th element into two consecutive places.)
        ptr::copy(p, p.offset(1), self.len as usize - index);
        // Write it in, overwriting the first copy of the `index`th
        // element.
        ptr::write(p, value);

        self.len += 1;
      }
    }

    /// Remove the element at `index`, copying the elements after `index` downwards
    pub fn remove(&mut self, index: usize) -> T {
      let len = self.len as usize;
      assert!(index < len as usize);
      unsafe {
        // infallible
        let ret;
        {
          // the place we are taking from.
          let ptr = self.as_mut_ptr().add(index);
          // copy it out, unsafely having a copy of the value on
          // the stack and in the vector at the same time.
          ret = ptr::read(ptr);

          // Shift everything down to fill in that spot.
          ptr::copy(ptr.offset(1), ptr, len - index - 1);
        }
        self.len -= 1;
        ret
      }
    }

    /// Removes an element from the vector and returns it.
    ///
    /// The removed element is replaced by the last element of the vector.
    ///
    /// This does not preserve ordering, but is O(1).
    #[inline]
    pub fn swap_remove(&mut self, index: usize) -> T {
      unsafe {
        let len = self.len as usize;
        let hole: *mut T = &mut self[index];
        let last = ptr::read(self.get_unchecked(len - 1));
        self.len -= 1;
        ptr::replace(hole, last)
      }
    }

    /// Take a function which returns whether an element should be kept,
    /// and mutably removes all elements from the vector which are not kept
    pub fn retain<F: FnMut(&T) -> bool>(&mut self, mut keep: F) {
      let mut del = 0;
      let len = self.len as usize;
      {
        let v = &mut **self;

        for i in 0..len {
          if !keep(&v[i]) {
            del += 1;
          } else {
            v.swap(i - del, i);
          }
        }
      }

      if del > 0 {
        self.truncate(len - del);
      }
    }

    /// Truncate the vector to the given length
    pub fn truncate(&mut self, desired_len: usize) {
      unsafe {
        while desired_len < self.len as usize {
          self.len -= 1;
          let len = self.len;
          ptr::drop_in_place(self.get_unchecked_mut(len as usize));
        }
      }
    }

    #[inline]
    pub fn append(&mut self, other: &mut Self) {
      unsafe {
        self.append_elements(&other[..] as _);

        other.len = 0;
      }
    }

    /// Appends elements to `Self` from other buffer.
    #[inline]
    unsafe fn append_elements(&mut self, other: *const [T]) {
      let count = (*other).len();

      while self.len + count as u8 > self.cap {
        self.double_buf();
      }

      let len = self.len();
      ptr::copy_nonoverlapping(other as *const T, self.as_mut_ptr().add(len), count);

      self.len += count as u8;
    }

    /// Clear the vector
    #[inline]
    pub fn clear(&mut self) {
      self.truncate(0);
    }
  }

  impl<T> From<Vec<T>> for VecU8<T> {
    /// Create a `VecU32` from a normal `Vec`,
    /// directly using the backing storage as free heap storage
    fn from(mut vec: Vec<T>) -> Self {
      let cvec = unsafe { Self::from_raw_parts(vec.as_mut_ptr(), vec.len(), vec.capacity()) };
      ::std::mem::forget(vec);
      cvec
    }
  }

  impl<T> Drop for VecU8<T> {
    /// Drop elements and deallocate free heap storage, if any is allocated
    fn drop(&mut self) {
      unsafe {
        ptr::drop_in_place(&mut self[..]);
      };
    }
  }

  impl<T> Deref for VecU8<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
      if unsafe { self.ptr.is_null() } {
        unsafe { ::std::slice::from_raw_parts(0x1 as *const T, 0) }
      } else {
        unsafe { ::std::slice::from_raw_parts(self.ptr, self.len as usize) }
      }
    }
  }

  impl<T> DerefMut for VecU8<T> {
    fn deref_mut(&mut self) -> &mut [T] {
      if unsafe { self.ptr.is_null() } {
        unsafe { ::std::slice::from_raw_parts_mut(0x1 as *mut T, 0) }
      } else {
        unsafe { ::std::slice::from_raw_parts_mut(self.ptr, self.len as usize) }
      }
    }
  }

  pub struct IntoIter<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
    index: usize
  }

  impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
      if self.index < self.len {
        let item = unsafe { ptr::read(self.ptr.offset(self.index as isize)) };
        self.index += 1;
        Some(item)
      } else {
        None
      }
    }
  }

  impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
      // drop all remaining elements
      unsafe {
        ptr::drop_in_place(&mut ::std::slice::from_raw_parts(
          self.ptr.offset(self.index as isize),
          self.len,
        ));
      };
    }
  }

  impl<T> IntoIterator for VecU8<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
      let iter = IntoIter {
        ptr: unsafe { &mut ptr::read(self.ptr) },
        len: self.len as usize,
        cap: self.cap as usize,
        index: 0,
      };
      ::std::mem::forget(self);
      iter
    }
  }

  impl<'a, T> IntoIterator for &'a VecU8<T> {
    type Item = &'a T;
    type IntoIter = ::std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
      self.iter()
    }
  }

  impl<'a, T> IntoIterator for &'a mut VecU8<T> {
    type Item = &'a mut T;
    type IntoIter = ::std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
      self.iter_mut()
    }
  }

  impl<T: Clone> Clone for VecU8<T> {
    fn clone(&self) -> VecU8<T> {
      VecU8::from(self.iter().cloned().collect::<Vec<_>>())
    }
  }

  impl<T> FromIterator<T> for VecU8<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
      let into_iter = iter.into_iter();
      let mut vec = VecU8::with_capacity(into_iter.size_hint().0);
      for item in into_iter {
        vec.push(item);
      }
      vec
    }
  }

  impl<T> Extend<T> for VecU8<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
      for item in iter {
        self.push(item);
      }
    }
  }

  impl<T> Default for VecU8<T> {
    fn default() -> VecU8<T> {
      VecU8::new()
    }
  }

  impl<T: ::std::fmt::Debug> ::std::fmt::Debug for VecU8<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      (self.deref()).fmt(f)
    }
  }

  #[cfg(feature = "serde-serialization")]
  use ::serde::ser::SerializeSeq;
  use std::ptr::NonNull;
  use std::alloc::{alloc, Layout, realloc, dealloc};
  use std::mem::{size_of, align_of};
  use core::ptr;
  use std::ops::{DerefMut, Deref};
  use std::iter::FromIterator;
  use std::marker::PhantomData;

  #[cfg(feature = "serde-serialization")]
  impl<T> ::serde::ser::Serialize for VecU8<T>
  where
    T: ::serde::ser::Serialize
  {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: ::serde::ser::Serializer,
    {
      let mut seq = serializer.serialize_seq(Some(self.len()))?;
      for e in self {
        seq.serialize_element(e)?;
      }
      seq.end()
    }
  }

  #[cfg(feature = "serde-serialization")]
  struct VecU32Visitor<T> {
    marker: PhantomData<fn() -> VecU8<T>>
  }

  #[cfg(feature = "serde-serialization")]
  impl<T> VecU32Visitor<T> {
    fn new() -> Self {
      VecU32Visitor {
        marker: PhantomData
      }
    }
  }

  #[cfg(feature = "serde-serialization")]
  impl<'de, T> ::serde::de::Visitor<'de> for VecU32Visitor<T>
  where
    T: ::serde::de::Deserialize<'de>
  {
    type Value = VecU8<T>;

    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      formatter.write_str("A Compact Vector")
    }

    fn visit_seq<S>(self, mut access: S) -> Result<Self::Value, S::Error>
    where
      S: ::serde::de::SeqAccess<'de>,
    {
      let mut vector = VecU8::with_capacity(access.size_hint().unwrap_or(0));

      while let Some(element) = access.next_element()? {
        vector.push(element);
      }

      Ok(vector)
    }
  }

  #[cfg(feature = "serde-serialization")]
  impl<'de, T> ::serde::de::Deserialize<'de> for VecU8<T>
  where
    T: ::serde::de::Deserialize<'de>
  {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: ::serde::de::Deserializer<'de>,
    {
      deserializer.deserialize_map(VecU32Visitor::new())
    }
  }
}

pub mod sorted {
  //! Sorted vectors.
  //!
  //! [Repository](https://gitlab.com/spearman/sorted-vec)
  //!
  //! - `SortedVec` -- sorted from least to greatest
  //! - `ReverseSortedVec` -- sorted from greatest to least
  //!
  //! The `partial` module provides sorted vectors of types that only implement
  //! `PartialOrd` where comparison of incomparable elements results in runtime
  //! panic.

  use super::VecU32;
  use std::ops::Deref;

  //#[cfg(feature = "serde")]
  //#[macro_use] extern crate serde;

  // pub mod partial;

  /// Forward sorted vector
  #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
  #[repr(packed)]
  #[derive(Clone, Debug)]
  pub struct SortedVecU32<T: Ord> {
    vec: VecU32<T>
  }

  /// Reverse sorted vector
  #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
  #[repr(packed)]
  #[derive(Clone, Debug)]
  pub struct ReverseSortedVecU32<T: Ord> {
    vec: VecU32<T>
  }

  impl<T: Ord> SortedVecU32<T> {

    #[inline]
    pub fn new() -> Self {
      SortedVecU32 {
        vec: VecU32::new(),
      }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
      SortedVecU32 {
        vec: VecU32::with_capacity(capacity),
      }
    }

    /// Uses `sort_unstable()` to sort in place.
    #[inline]
    pub fn from_unsorted(mut vec: Vec<T>) -> Self {
      vec.sort_unstable();
      SortedVecU32 {
        vec: VecU32::from(vec),
      }
    }

    /// Insert an element into sorted position, returning the order index at which
    /// it was placed.
    ///
    /// If the element was already present, the order index is returned as an
    /// `Err`, otherwise it is returned with `Ok`.
    pub fn insert(&mut self, element: T) -> Result<usize, usize> {
      match &self.vec[..].binary_search(&element) {
        Ok(insert_at) => {
          self.vec.insert(*insert_at, element);
          Err(*insert_at)
        }
        Err(insert_at) => {
          self.vec.insert(*insert_at, element);
          Ok(*insert_at)
        }
      }
    }

    pub fn insert_unique(&mut self, element: T) -> Result<usize, usize> {
      match &self.vec[..].binary_search(&element) {
        Ok(insert_at) => {
          Err(*insert_at)
        }
        Err(insert_at) => {
          self.vec.insert(*insert_at, element);
          Ok(*insert_at)
        }
      }
    }

    #[inline]
    pub fn remove_item(&mut self, item: &T) -> Option<T> {
      match self.vec.binary_search(item) {
        Ok(remove_at) => Some(self.vec.remove(remove_at)),
        Err(_) => None
      }
    }

    #[inline]
    pub fn find<'a, F, K: Ord>(&'a self, key: &K, key_extrqctor: F) -> Option<&T>
    where F: FnMut(&'a T) -> K,
          K: Ord, {
      match self.vec.binary_search_by_key(key, key_extrqctor) {
        Ok(idx) => self.vec.get(idx),
        Err(_) => None
      }
    }

    #[inline]
    pub fn find_mut<'a, F, K: Ord>(&'a mut self, key: &K, key_extrqctor: F) -> Option<&mut T>
    where F: FnMut(&'a T) -> K,
          K: Ord, {

      unsafe {
        // Rust is just complete shit here
        let ptr: *mut VecU32<T> = &mut self.vec;
        match (*ptr).binary_search_by_key(key, key_extrqctor) {
          Ok(idx) => self.vec.get_mut(idx),
          Err(_) => None
        }
      }
    }

    #[inline]
    pub fn remove_by_key<'a, F, K>(&'a mut self, key: &'a K, key_extractor: F) -> Option<T>
    where F: FnMut(&'a T) -> K,
          K: Ord + 'a, {

      unsafe {
        // Rust is just complete shit here
        let ptr: *mut VecU32<T> =  &mut self.vec;
        let remove_at_option = (*ptr).binary_search_by_key(
          key,
          key_extractor
        );

        if remove_at_option.is_err() {
          return None;
        }

        let remove_at = remove_at_option.unwrap();
        let value = self.vec.remove(remove_at);
        Some(value)
      }
    }

    /// Panics if index is out of bounds
    #[inline]
    pub fn remove_index(&mut self, index: usize) -> T {
      self.vec.remove(index)
    }
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
      self.vec.pop()
    }
    #[inline]
    pub fn clear(&mut self) {
      self.vec.clear()
    }
    // #[inline]
    // pub fn dedup (&mut self) {
    //   self.vec.dedup();
    // }
    // #[inline]
    // pub fn drain <R> (&mut self, range : R) -> std::vec::Drain <T> where
    //   R : std::ops::RangeBounds <usize>
    // {
    //   self.vec.drain (range)
    // }
    #[inline]
    pub fn into_vec(mut self) -> Vec<T> {
      unsafe {
        let v = Vec::from_raw_parts(self.vec.as_mut_ptr(), self.vec.len(), self.vec.capacity());
        std::mem::forget(self);
        v
      }
    }

    /// Apply a closure mutating the sorted vector and use `sort_unstable()`
    /// to re-sort the mutated vector
    pub fn mutate_vec<F, O>(&mut self, f: F) -> O where
      F: FnOnce(&mut VecU32<T>) -> O
    {
      let res = f(&mut self.vec);
      self.vec.sort_unstable();
      res
    }
  }

  impl<T: Ord> Default for SortedVecU32<T> {
    fn default() -> Self {
      Self::new()
    }
  }

  impl<T: Ord> std::ops::Deref for SortedVecU32<T> {
    type Target = VecU32<T>;
    fn deref(&self) -> &Self::Target {
      &self.vec
    }
  }

  impl<T: Ord> std::ops::DerefMut for SortedVecU32<T> {

    fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.vec
    }
  }

  impl<T: Ord> Extend<T> for SortedVecU32<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
      for t in iter {
        let _ = self.insert(t);
      }
    }
  }

  impl<T: Ord> ReverseSortedVecU32<T> {
    #[inline]
    pub fn new() -> Self {
      ReverseSortedVecU32 { vec: VecU32::new() }
    }
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
      ReverseSortedVecU32 { vec: VecU32::with_capacity(capacity) }
    }
    /// Uses `sort_unstable_by()` to sort in place.
    #[inline]
    pub fn from_unsorted(mut vec: Vec<T>) -> Self {
      vec.sort_unstable_by(|x, y| x.cmp(y).reverse());
      ReverseSortedVecU32 { vec: VecU32::from(vec) }
    }
    /// Insert an element into (reverse) sorted position, returning the order
  /// index at which it was placed.
  ///
  /// If the element was already present, the order index is returned as an
  /// `Err`, otherwise it is returned with `Ok`.
    pub fn insert(&mut self, element: T) -> Result<usize, usize> {
      match &self.vec[..].binary_search_by(
        |other_element| other_element.cmp(&element).reverse()
      ) {
        Ok(insert_at) => {
          self.vec.insert(*insert_at, element);
          Err(*insert_at)
        }
        Err(insert_at) => {
          self.vec.insert(*insert_at, element);
          Ok(*insert_at)
        }
      }
    }
    #[inline]
    pub fn remove_item(&mut self, item: &T) -> Option<T> {
      match self.vec.binary_search_by(
        |other_item| other_item.cmp(&item).reverse()
      ) {
        Ok(remove_at) => Some(self.vec.remove(remove_at)),
        Err(_) => None
      }
    }
    /// Panics if index is out of bounds
    #[inline]
    pub fn remove_index(&mut self, index: usize) -> T {
      self.vec.remove(index)
    }
    #[inline]
    pub fn binary_search(&self, x: &T) -> Result<usize, usize> {
      self.vec.binary_search_by(|y| y.cmp(&x).reverse())
    }
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
      self.vec.pop()
    }
    #[inline]
    pub fn clear(&mut self) {
      self.vec.clear()
    }
    // #[inline]
    // pub fn dedup (&mut self) {
    //   self.vec.dedup();
    // }
    // #[inline]
    // pub fn drain <R> (&mut self, range : R) -> std::vec::Drain <T> where
    //   R : std::ops::RangeBounds <usize>
    // {
    //   self.vec.drain (range)
    // }
    #[inline]
    pub fn into_vec(mut self) -> Vec<T> {
      unsafe {
        let v = Vec::from_raw_parts(self.vec.as_mut_ptr(), self.vec.len(), self.vec.capacity());
        std::mem::forget(self);
        v
      }
    }
    /// Apply a closure mutating the reverse-sorted vector and use
  /// `sort_unstable_by()` to re-sort the mutated vector
    pub fn mutate_vec<F, O>(&mut self, f: F) -> O where
      F: FnOnce(&mut VecU32<T>) -> O
    {
      let res = f(&mut self.vec);
      self.vec.sort_unstable_by(|x, y| x.cmp(y).reverse());
      res
    }
  }

  impl<T: Ord> Default for ReverseSortedVecU32<T> {
    fn default() -> Self {
      Self::new()
    }
  }

  impl<T: Ord> std::ops::Deref for ReverseSortedVecU32<T> {
    type Target = VecU32<T>;
    fn deref(&self) -> &VecU32<T> {
      &self.vec
    }
  }

  impl<T: Ord> std::ops::DerefMut for ReverseSortedVecU32<T> {

    fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.vec
    }
  }

  impl<T: Ord> Extend<T> for ReverseSortedVecU32<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
      for t in iter {
        let _ = self.insert(t);
      }
    }
  }
  /*
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_sorted_vec() {
    let mut v = SortedVec::new();
    assert_eq!(v.insert (5), Ok (0));
    assert_eq!(v.insert (3), Ok (0));
    assert_eq!(v.insert (4), Ok (1));
    assert_eq!(v.insert (4), Err (1));
    assert_eq!(v.len(), 4);
    v.dedup();
    assert_eq!(v.len(), 3);
    assert_eq!(v.binary_search (&3), Ok (0));
    assert_eq!(*SortedVec::from_unsorted (
      vec![5, -10, 99, -11, 2, 17, 10]),
      vec![-11, -10, 2, 5, 10, 17, 99]);
    let mut v = SortedVec::new();
    v.extend(vec![5, -10, 99, -11, 2, 17, 10].into_iter());
    assert_eq!(*v, vec![-11, -10, 2, 5, 10, 17, 99]);
    let _ = v.mutate_vec (|v|{
      v[0] = 11;
      v[3] = 1;
    });
    assert_eq!(
      v.drain(..).collect::<Vec <i32>>(),
      vec![-10, 1, 2, 10, 11, 17, 99]);
  }

  #[test]
  fn test_reverse_sorted_vec() {
    let mut v = ReverseSortedVec::new();
    assert_eq!(v.insert (5), Ok (0));
    assert_eq!(v.insert (3), Ok (1));
    assert_eq!(v.insert (4), Ok (1));
    assert_eq!(v.insert (6), Ok (0));
    assert_eq!(v.insert (4), Err (2));
    assert_eq!(v.len(), 5);
    v.dedup();
    assert_eq!(v.len(), 4);
    assert_eq!(v.binary_search (&3), Ok (3));
    assert_eq!(*ReverseSortedVec::from_unsorted (
      vec![5, -10, 99, -11, 2, 17, 10]),
      vec![99, 17, 10, 5, 2, -10, -11]);
    let mut v = ReverseSortedVec::new();
    v.extend(vec![5, -10, 99, -11, 2, 17, 10].into_iter());
    assert_eq!(*v, vec![99, 17, 10, 5, 2, -10, -11]);
    let _ = v.mutate_vec (|v|{
      v[6] = 11;
      v[3] = 1;
    });
    assert_eq!(
      v.drain(..).collect::<Vec <i32>>(),
      vec![99, 17, 11, 10, 2, 1, -10]);
  }
}
*/
}

pub mod sorted_u8 {
  //! Sorted vectors.
  //!
  //! [Repository](https://gitlab.com/spearman/sorted-vec)
  //!
  //! - `SortedVec` -- sorted from least to greatest
  //! - `ReverseSortedVec` -- sorted from greatest to least
  //!
  //! The `partial` module provides sorted vectors of types that only implement
  //! `PartialOrd` where comparison of incomparable elements results in runtime
  //! panic.

  use super::u8::VecU8;
  use std::ops::Deref;

  //#[cfg(feature = "serde")]
  //#[macro_use] extern crate serde;

  // pub mod partial;

  /// Forward sorted vector
  #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
  #[derive(Clone, Debug)]
  pub struct SortedVecU8<T: Ord> {
    vec: VecU8<T>
  }

  /// Reverse sorted vector
  #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
  #[derive(Clone, Debug)]
  pub struct ReverseSortedVecU8<T: Ord> {
    vec: VecU8<T>
  }

  impl<T: Ord> SortedVecU8<T> {

    #[inline]
    pub fn new() -> Self {
      SortedVecU8 {
        vec: VecU8::new(),
      }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
      SortedVecU8 {
        vec: VecU8::with_capacity(capacity),
      }
    }

    /// Uses `sort_unstable()` to sort in place.
    #[inline]
    pub fn from_unsorted(mut vec: Vec<T>) -> Self {
      vec.sort_unstable();
      SortedVecU8 {
        vec: VecU8::from(vec),
      }
    }

    /// Insert an element into sorted position, returning the order index at which
    /// it was placed.
    ///
    /// If the element was already present, the order index is returned as an
    /// `Err`, otherwise it is returned with `Ok`.
    pub fn insert(&mut self, element: T) -> Result<usize, usize> {
      match &self.vec[..].binary_search(&element) {
        Ok(insert_at) => {
          self.vec.insert(*insert_at, element);
          Err(*insert_at)
        }
        Err(insert_at) => {
          self.vec.insert(*insert_at, element);
          Ok(*insert_at)
        }
      }
    }

    pub fn insert_unique(&mut self, element: T) -> Result<usize, usize> {
      match &self.vec[..].binary_search(&element) {
        Ok(insert_at) => {
          Err(*insert_at)
        }
        Err(insert_at) => {
          self.vec.insert(*insert_at, element);
          Ok(*insert_at)
        }
      }
    }

    #[inline]
    pub fn remove_item(&mut self, item: &T) -> Option<T> {
      match self.vec.binary_search(item) {
        Ok(remove_at) => Some(self.vec.remove(remove_at)),
        Err(_) => None
      }
    }

    #[inline]
    pub fn find<'a, F, K: Ord>(&'a self, key: &K, key_extrqctor: F) -> Option<&T>
    where F: FnMut(&'a T) -> K,
          K: Ord, {
      match self.vec.binary_search_by_key(key, key_extrqctor) {
        Ok(idx) => self.vec.get(idx),
        Err(_) => None
      }
    }

    #[inline]
    pub fn find_mut<'a, F, K: Ord>(&'a mut self, key: &K, key_extrqctor: F) -> Option<&mut T>
    where F: FnMut(&'a T) -> K,
          K: Ord, {

      unsafe {
        // Rust is just complete shit here
        let ptr: *mut VecU8<T> = &mut self.vec;
        match (*ptr).binary_search_by_key(key, key_extrqctor) {
          Ok(idx) => self.vec.get_mut(idx),
          Err(_) => None
        }
      }
    }

    #[inline]
    pub fn remove_by_key<'a, F, K>(&'a mut self, key: &'a K, key_extractor: F) -> Option<T>
    where F: FnMut(&'a T) -> K,
          K: Ord + 'a, {

      unsafe {
        // Rust is just complete shit here
        let ptr: *mut VecU8<T> =  &mut self.vec;
        let remove_at_option = (*ptr).binary_search_by_key(
          key,
          key_extractor
        );

        if remove_at_option.is_err() {
          return None;
        }

        let remove_at = remove_at_option.unwrap();
        let value = self.vec.remove(remove_at);
        Some(value)
      }
    }

    /// Panics if index is out of bounds
    #[inline]
    pub fn remove_index(&mut self, index: usize) -> T {
      self.vec.remove(index)
    }
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
      self.vec.pop()
    }
    #[inline]
    pub fn clear(&mut self) {
      self.vec.clear()
    }
    // #[inline]
    // pub fn dedup (&mut self) {
    //   self.vec.dedup();
    // }
    // #[inline]
    // pub fn drain <R> (&mut self, range : R) -> std::vec::Drain <T> where
    //   R : std::ops::RangeBounds <usize>
    // {
    //   self.vec.drain (range)
    // }
    #[inline]
    pub fn into_vec(mut self) -> Vec<T> {
      unsafe {
        let v = Vec::from_raw_parts(self.vec.as_mut_ptr(), self.vec.len(), self.vec.capacity());
        std::mem::forget(self);
        v
      }
    }

    /// Apply a closure mutating the sorted vector and use `sort_unstable()`
    /// to re-sort the mutated vector
    pub fn mutate_vec<F, O>(&mut self, f: F) -> O where
      F: FnOnce(&mut VecU8<T>) -> O
    {
      let res = f(&mut self.vec);
      self.vec.sort_unstable();
      res
    }
  }

  impl<T: Ord> Default for SortedVecU8<T> {
    fn default() -> Self {
      Self::new()
    }
  }

  impl<T: Ord> std::ops::Deref for SortedVecU8<T> {
    type Target = VecU8<T>;
    fn deref(&self) -> &Self::Target {
      &self.vec
    }
  }

  impl<T: Ord> std::ops::DerefMut for SortedVecU8<T> {

    fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.vec
    }
  }

  impl<T: Ord> Extend<T> for SortedVecU8<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
      for t in iter {
        let _ = self.insert(t);
      }
    }
  }

  impl<T: Ord> ReverseSortedVecU8<T> {
    #[inline]
    pub fn new() -> Self {
      ReverseSortedVecU8 { vec: VecU8::new() }
    }
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
      ReverseSortedVecU8 { vec: VecU8::with_capacity(capacity) }
    }
    /// Uses `sort_unstable_by()` to sort in place.
    #[inline]
    pub fn from_unsorted(mut vec: Vec<T>) -> Self {
      vec.sort_unstable_by(|x, y| x.cmp(y).reverse());
      ReverseSortedVecU8 { vec: VecU8::from(vec) }
    }
    /// Insert an element into (reverse) sorted position, returning the order
  /// index at which it was placed.
  ///
  /// If the element was already present, the order index is returned as an
  /// `Err`, otherwise it is returned with `Ok`.
    pub fn insert(&mut self, element: T) -> Result<usize, usize> {
      match &self.vec[..].binary_search_by(
        |other_element| other_element.cmp(&element).reverse()
      ) {
        Ok(insert_at) => {
          self.vec.insert(*insert_at, element);
          Err(*insert_at)
        }
        Err(insert_at) => {
          self.vec.insert(*insert_at, element);
          Ok(*insert_at)
        }
      }
    }
    #[inline]
    pub fn remove_item(&mut self, item: &T) -> Option<T> {
      match self.vec.binary_search_by(
        |other_item| other_item.cmp(&item).reverse()
      ) {
        Ok(remove_at) => Some(self.vec.remove(remove_at)),
        Err(_) => None
      }
    }
    /// Panics if index is out of bounds
    #[inline]
    pub fn remove_index(&mut self, index: usize) -> T {
      self.vec.remove(index)
    }
    #[inline]
    pub fn binary_search(&self, x: &T) -> Result<usize, usize> {
      self.vec.binary_search_by(|y| y.cmp(&x).reverse())
    }
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
      self.vec.pop()
    }
    #[inline]
    pub fn clear(&mut self) {
      self.vec.clear()
    }
    // #[inline]
    // pub fn dedup (&mut self) {
    //   self.vec.dedup();
    // }
    // #[inline]
    // pub fn drain <R> (&mut self, range : R) -> std::vec::Drain <T> where
    //   R : std::ops::RangeBounds <usize>
    // {
    //   self.vec.drain (range)
    // }
    #[inline]
    pub fn into_vec(mut self) -> Vec<T> {
      unsafe {
        let v = Vec::from_raw_parts(self.vec.as_mut_ptr(), self.vec.len(), self.vec.capacity());
        std::mem::forget(self);
        v
      }
    }
    /// Apply a closure mutating the reverse-sorted vector and use
  /// `sort_unstable_by()` to re-sort the mutated vector
    pub fn mutate_vec<F, O>(&mut self, f: F) -> O where
      F: FnOnce(&mut VecU8<T>) -> O
    {
      let res = f(&mut self.vec);
      self.vec.sort_unstable_by(|x, y| x.cmp(y).reverse());
      res
    }
  }

  impl<T: Ord> Default for ReverseSortedVecU8<T> {
    fn default() -> Self {
      Self::new()
    }
  }

  impl<T: Ord> std::ops::Deref for ReverseSortedVecU8<T> {
    type Target = VecU8<T>;
    fn deref(&self) -> &VecU8<T> {
      &self.vec
    }
  }

  impl<T: Ord> std::ops::DerefMut for ReverseSortedVecU8<T> {

    fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.vec
    }
  }

  impl<T: Ord> Extend<T> for ReverseSortedVecU8<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
      for t in iter {
        let _ = self.insert(t);
      }
    }
  }
}