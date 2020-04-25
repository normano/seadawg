use std::convert::TryFrom;

use crate::foundation::*;
use crate::id_allocator::U32IdAllocator;
use crate::map::VecMapU32;
/*
use super::traversal::{TraversalExecutor, TraversalContextData, TraversalMode,
  FindPrefixTraverser, FindPrefixContext,
  FindSuperStringTraverser, FindSuperStringContext,
  FindSuffixTraverser, FindSuffixContext,
};*/
use crate::vec::sorted::SortedVecU32;
use super::traversal::TraversalResult;
use crate::vec::sorted_u8::SortedVecU8;
use crate::utils::{get_codepoint_at};
use crate::cmp::Compare;

pub type NodeId = u32;
pub type EdgeId = u32;
pub type Letter = u8;
pub type StrLength = u16;
pub type StrIdx = i16;
pub type NodeLength = i16;

pub const ROOT_ID: NodeId = 0;
pub const SOURCE_ID: NodeId = 1;
pub const NONE_SINK_ID: u32 = std::u32::MAX;

pub (in crate) struct SeaEdges {
  inner: VecMapU32<SeaEdge>,
  _next_edge_id_allocator: U32IdAllocator,
}

impl SeaEdges {

  pub fn new() -> Self {
    return SeaEdges {
      inner: VecMapU32::new(),
      _next_edge_id_allocator: U32IdAllocator::new(),
    };
  }

  /// This will only allocate an edge with an id, but it will not add the edge.
  pub fn new_edge(&mut self, dest: NodeId, sink_id: NodeId, start_idx: StrIdx, end_idx: StrIdx, is_sink: bool) -> (SeaEdge, EdgeId) {

    let edge_id = self.next_edge_id();
    let edge = SeaEdge {
      dest,
      sink_id,
      start_idx,
      end_idx,
      is_sink,
    };

    return (edge, edge_id as EdgeId);
  }

  fn next_edge_id(&mut self) -> EdgeId {

    return self._next_edge_id_allocator.next_id();
  }

  pub fn add(&mut self, id: EdgeId, edge: SeaEdge) {
    self.inner.insert(id, edge);
  }

  pub fn get(&self, id: &EdgeId) -> Option<&SeaEdge> {
    return self.inner.get(id);
  }

  pub fn get_mut(&mut self, id: &EdgeId) -> Option<&mut SeaEdge> {
    return self.inner.get_mut(id);
  }
}

pub (in crate) struct SeaNodes<V> {
  internal: VecMapU32<SeaINode>,
  sinks: VecMapU32<SeaSinkNode<V>>,
  _internal_id_allocator: U32IdAllocator,
  _sink_id_allocator: U32IdAllocator,
}

impl <V> SeaNodes<V> {

  pub fn new() -> Self {

    let mut nodes = SeaNodes {
      internal: VecMapU32::new(),
      sinks: VecMapU32::new(),
      _internal_id_allocator: U32IdAllocator::new_start_at(2),
      _sink_id_allocator: U32IdAllocator::new_start_at(0),
    };

    let mut root_node = SeaINode::new(0);
    root_node._length = -1;

    let mut source_node = SeaINode::new(ROOT_ID);

    nodes.add_node(ROOT_ID, root_node);
    nodes.add_node(SOURCE_ID, source_node);

    return nodes;
  }

  #[inline]
  pub fn new_inode(&mut self, suffix: NodeId) -> (SeaINode, NodeId) {

    return (SeaINode::new(suffix), self.next_inode_id());
  }

  fn next_inode_id(&mut self) -> NodeId {

    return self._internal_id_allocator.next_id();
  }

  fn next_sink_id(&mut self) -> NodeId {

    return self._sink_id_allocator.next_id();
  }

  #[inline]
  pub fn add_node(&mut self, node_id: NodeId, node: SeaINode) {
    self.internal.insert(node_id, node);
  }

  #[inline]
  pub fn add_sink(&mut self, id: NodeId, node: SeaSinkNode<V>) {
    self.sinks.insert(id, node);
  }

  #[inline]
  pub fn get_internal(&self, id: &NodeId) -> Option<&SeaINode> {
    return self.internal.get(id);
  }

  #[inline]
  pub fn get_mut_internal(&mut self, id: &NodeId) -> Option<&mut SeaINode> {
    return self.internal.get_mut(id);
  }

  #[inline]
  pub fn get_sink(&self, id: &NodeId) -> Option<&SeaSinkNode<V>> {
    return self.sinks.get(id);
  }

  #[inline]
  pub fn get_mut_sink(&mut self, id: &NodeId) -> Option<&mut SeaSinkNode<V>> {
    return self.sinks.get_mut(id);
  }
}

#[repr(packed)]
pub struct SeaEdge {
  pub dest: NodeId,
  pub sink_id: u32,
  pub start_idx: StrIdx,
  pub end_idx: StrIdx,
  pub is_sink: bool,
}

#[repr(packed)]
pub struct SeaINode {
  _length: NodeLength,
  suffix: NodeId,
  pub to_edges: SortedVecU8<(Letter, EdgeId)>,
}

impl SeaINode {
  pub fn new(suffix: NodeId) -> Self {
    return Self {
      suffix,
      to_edges: SortedVecU8::new(),
      _length: 0,
    };
  }

  #[inline]
  pub fn length(&self) -> NodeLength {
    return self._length;
  }

  pub fn has_no_to_edges(&self) -> bool {
    return self.to_edges.iter().next().is_none();
  }

  pub fn add_to(&mut self, letter: Letter, id: EdgeId) {
    self.to_edges.insert_unique((letter, id));
  }

  pub fn remove_to(&mut self, letter: &Letter) {
    self.to_edges.remove_by_key(&letter, |item| &item.0);
  }

  #[inline]
  pub fn get_to(&self, letter: &Letter) -> Option<&EdgeId> {
    return self.to_edges.find(letter, |item| item.0).map(|item| &item.1);
  }

  pub fn get_to_edges(&self) -> Vec<EdgeId> {
    return self.to_edges.iter().map(|item| item.1).collect();
  }

  #[inline]
  pub fn contains_to(&self, letter: &Letter) -> bool {
    return self.to_edges.find(letter, |item| item.0).is_some();
  }
}

#[repr(packed)]
pub struct SeaSinkNode<Val> {
  pub word: Box<str>,
  pub data: Val,
}

impl SeaSinkNode<()> {

  #[inline]
  pub fn new_empty(word: &str) -> Self {

    return Self::new((), word);
  }
}

impl <Val> SeaSinkNode<Val> {
  #[inline]
  pub fn new(data: Val, word: &str) -> Self {

    return SeaSinkNode {
      word: Box::from(word),
      data,
    };
  }

  #[inline]
  pub fn length(&self) -> StrIdx {
    return self.word.len() as i16;
  }
}

///
/// Original Online Multi String CDAWG from the paper.
///
/// Terminators must be supplied by the user on add.
///
pub struct SeaDawgCore<V = ()> {
  pub (in crate) edges: SeaEdges,
  pub (in crate) nodes: SeaNodes<V>,
  sink_id: NodeId,
  _size: u32,
  _debug: bool,
}

impl <V> SeaDawgCore <V> {

  pub fn new() -> Self {

    return Self {
      edges: SeaEdges::new(),
      nodes: SeaNodes::new(),
      sink_id: NONE_SINK_ID,
      _size: 0,
      _debug: false,
    };
  }

  #[inline]
  pub fn size(&self) -> u32 {
    return self._size;
  }

  #[inline]
  pub fn inodes_count(&self) -> usize {
    return self.nodes.internal.len();
  }

  #[inline]
  pub fn snodes_count(&self) -> usize {
    return self.nodes.sinks.len();
  }

  #[inline]
  pub fn edges_count(&self) -> usize {
    return self.edges.inner.len();
  }

  #[inline]
  pub fn get_sink(&self, node_id: &NodeId) -> Option<&SeaSinkNode<V>> {

    return self.nodes.get_sink(node_id);
  }

  #[inline]
  pub fn get_mut_sink(&mut self, node_id: &NodeId) -> Option<&mut SeaSinkNode<V>> {

    return self.nodes.get_mut_sink(node_id);
  }

  pub fn add(&mut self, mut sink: SeaSinkNode<V>) {

    //TODO find sink and set it if exists
    let word = sink.word.clone();
    let word_bytes = word.as_bytes();
    self.sink_id = self.nodes.next_sink_id();
    self.nodes.add_sink(self.sink_id.clone(), sink);

    // Cached
    let mut update_data = (SOURCE_ID, 0i16);
    let mut word_idx: usize = 0;
    let end_word_len = word_bytes.len();

    while word_idx < end_word_len {

      let letter = word_bytes[word_idx];
      let root = self.nodes.get_internal(&ROOT_ID).unwrap();

      if !root.contains_to(&letter) {
        self.set_edge(ROOT_ID, self.sink_id, word_idx as i16, word_idx as i16, SOURCE_ID, false);
      }

      update_data = self.update(word_bytes, letter, update_data, word_idx as i16);

      word_idx += 1;
    }

    self.sink_id = NONE_SINK_ID;
    self._size += 1;
  }

  /// Not Implemented yet
  pub fn remove(&mut self, word: &str) -> Option<SeaSinkNode<V>> {
    unimplemented!()
  }
  
  pub fn find_exact(&self, needle: &str) -> Option<NodeId> {

    if needle.is_empty() {
      return None;
    }

    let needle_bytes = needle.as_bytes();
    let mut target_node_id: Option<NodeId> = None;
    let needle_len = needle_bytes.len();
    let mut word_idx: usize = 0;
    let mut current_node_id = SOURCE_ID;

    loop {
      let word_cp = needle_bytes[word_idx as usize];
      let current_node = self.nodes.get_internal(&current_node_id).unwrap();
      let matching_edge_id_option = current_node.get_to(&word_cp);

      if matching_edge_id_option.is_none() {
        break;
      }

      let matching_edge_id = matching_edge_id_option.unwrap();
      let matching_edge_opt = self.edges.get(matching_edge_id);
      let matching_edge = matching_edge_opt.unwrap();
      let matching_edge_start_idx = matching_edge.start_idx as usize;
      let matching_edge_end_idx = matching_edge.end_idx as usize;
      let sink = self.get_sink(&matching_edge.sink_id);
      let edge_word = sink.unwrap().word.as_bytes();

      let partial_len: usize = (self.get_edge_idx_diff(current_node_id, matching_edge) + 1) as usize;
      let needle_substring_len = word_idx + partial_len;

      if matching_edge.is_sink {

        let length = self.nodes.get_sink(&matching_edge.dest).unwrap().length();
        if length == needle_len as i16 {
          target_node_id = Some(matching_edge.dest);
        }
        break;
      }

      if needle_substring_len <= needle_len && edge_word[matching_edge_start_idx..matching_edge_end_idx + 1].feq(&needle_bytes[word_idx..needle_substring_len]) {
        if needle_len == needle_substring_len {
          break;
        }

        current_node_id = matching_edge.dest;
        word_idx += partial_len;
        continue;
      }

      break;
    }

    return target_node_id;
  }

/*
  pub fn find_with_prefix(&self, prefix: &str) -> Vec<TraversalResult> {

    let traverser = FindPrefixTraverser::new(prefix);
    let prefix_inner = TraversalContextData::new(
      TraversalMode::Traversal,
      Some(SOURCE_ID),
      vec![],
      String::new(),
      0,
      None,
    );
    let base_context = FindPrefixContext::new(prefix_inner);

    let executor = TraversalExecutor::new();

    return executor.execute_traversal(self, traverser, base_context);
  }

  pub fn find_with_suffix(&self, needle: &str) -> Vec<TraversalResult> {

    let terminated_needle = self.terminated_word(needle);
    let traverser = FindSuffixTraverser::new(&*terminated_needle);
    let context_inner = TraversalContextData::new(
      TraversalMode::Traversal,
      Some(SOURCE_ID),
      vec![],
      String::new(),
      0,
      None,
    );
    let base_context = FindSuffixContext::new(context_inner);

    let executor = TraversalExecutor::new();

    return executor.execute_traversal(self, traverser, base_context);
  }

  pub fn find_with_substring(&self, needle: &str) -> Vec<TraversalResult> {

    let traverser = FindSuperStringTraverser::new(needle);
    let context_inner = TraversalContextData::new(
      TraversalMode::Traversal,
      Some(SOURCE_ID),
      vec![],
      String::new(),
      0,
      None,
    );
    let base_context = FindSuperStringContext::new(context_inner, false);

    let executor = TraversalExecutor::new();

    return executor.execute_traversal(self, traverser, base_context);
  }
*/
  fn update(&mut self, word: &[u8], letter: Letter, (initial_update_node_id, initial_start_idx): (NodeId, StrIdx), end_idx: StrIdx) -> (NodeId, StrIdx) {

    let mut start_idx = initial_start_idx;
    let mut update_node_id = initial_update_node_id;
    let mut prev_node_id_option: Option<NodeId> = None;
    let mut update_node_prime_option: Option<NodeId> = None;
    let mut update_node_next_id: Option<NodeId> = None;
    let prev_end_idx = end_idx - 1;

    while !self.check_endpoint(update_node_id, start_idx, prev_end_idx, letter, word) {

      if start_idx <= prev_end_idx {
        let possible_extension = self.extension(update_node_id, start_idx, prev_end_idx, word);

        if update_node_prime_option.is_some() && update_node_prime_option.unwrap() == possible_extension {

          self.redirect_edge(update_node_id, start_idx, prev_end_idx, update_node_next_id.unwrap(), word);
          let canonized_data = self.canonize(self.get_suffix_id(&update_node_id), start_idx, prev_end_idx, word);
          update_node_id = canonized_data.0;
          start_idx = canonized_data.1;
          continue;
        }

        update_node_prime_option = Some(possible_extension);
        update_node_next_id = Some(self.split_edge(&update_node_id, start_idx, prev_end_idx, word));
      } else {

        update_node_next_id = Some(update_node_id);
      }

      self.set_edge(
        update_node_next_id.unwrap(),
        self.sink_id,
        end_idx,
        0,
        self.sink_id,
        true
      );

      if prev_node_id_option.is_some() {

        self.nodes.get_mut_internal(&prev_node_id_option.unwrap()).unwrap().suffix = update_node_next_id.unwrap();
      }

      prev_node_id_option = update_node_next_id.clone();

      let canonized_data = self.canonize(
        self.get_suffix_id(&update_node_id),
        start_idx,
        prev_end_idx,
        word,
      );
      update_node_id = canonized_data.0;
      start_idx = canonized_data.1;
    }

    if prev_node_id_option.is_some() {

      self.nodes.get_mut_internal(&prev_node_id_option.unwrap()).unwrap().suffix = update_node_id;
    }

    return self.separate_node(update_node_id, start_idx, end_idx, word);
  }

  fn check_endpoint(&self, node_id: NodeId, start_idx: StrIdx, end_idx: StrIdx, letter: Letter, word: &[u8]) -> bool {

    let src_node = self.nodes.get_internal(&node_id).unwrap();

    if start_idx <= end_idx {
      let word_letter = word[start_idx as usize];
      let edge_id = src_node.get_to(&word_letter).unwrap();
      let edge = self.edges.get(edge_id).unwrap();

      let sink = self.get_sink(&edge.sink_id);
      let word = &*sink.unwrap().word;

      let partial_letter = get_codepoint_at(word, (edge.start_idx + end_idx - start_idx + 1) as usize);

      return letter == partial_letter;
    }

    return src_node.contains_to(&letter);
  }

  fn canonize(&mut self, mut node_id: NodeId, mut start_idx: StrIdx, end_idx: StrIdx, word: &[u8]) -> (NodeId, StrIdx) {

    if start_idx > end_idx {
      return (node_id, start_idx);
    }

    let mut node = self.nodes.get_internal(&node_id).unwrap();
    let edge_letter = &word[start_idx as usize];

    let mut edge_id = node.get_to(edge_letter).unwrap();

    let mut edge = self.edges.get(edge_id).unwrap();
    let mut edge_src = node_id;

    let mut edge_idx_diff = self.get_edge_idx_diff(edge_src, edge) as StrIdx;
    while edge_idx_diff <= end_idx - start_idx {

      start_idx += edge_idx_diff + 1;

      if edge.is_sink {
        panic!("Only SeaNodes should be returned by canonize");
      }
      node_id = edge.dest;

      if start_idx <= end_idx {
        node = self.nodes.get_internal(&node_id).unwrap();
        let word_letter = &word[start_idx as usize];

        edge_id = node.get_to(word_letter).unwrap();
        edge = self.edges.get(edge_id).unwrap();
        edge_src = node_id;
      }

      edge_idx_diff = self.get_edge_idx_diff(edge_src, edge) as StrIdx;
    }

    return (node_id, start_idx);
  }

  fn extension(&self, node_id: NodeId, start_idx: StrIdx, end_idx: StrIdx, word: &[u8]) -> NodeId {

    if start_idx > end_idx {
      return node_id;
    }

    let letter = &word[start_idx as usize];
    let node = self.nodes.get_internal(&node_id).unwrap();
    let edge_id = node.get_to(letter).unwrap();
    let edge = self.edges.get(edge_id).unwrap();

    return edge.dest;
  }

  fn redirect_edge(&mut self, src_node_id: NodeId, start_idx: StrIdx, end_idx: StrIdx, dest: NodeId, word: &[u8]) {

    let letter = &word[start_idx as usize];
    let node = self.nodes.get_internal(&src_node_id).unwrap();
    let edge_id = node.get_to(letter).unwrap();
    let edge = self.edges.get(edge_id).unwrap();
    let edge_start_idx = edge.start_idx;
    let edge_sink_id = edge.sink_id;

    let substring_idx_diff = end_idx - start_idx;
    let edge_end_idx = edge_start_idx + substring_idx_diff;

    self.set_edge(src_node_id, edge_sink_id, edge_start_idx, edge_end_idx, dest, false);
  }

  fn split_edge(&mut self, src_node_id: &NodeId, start_idx: StrIdx, end_idx: StrIdx, word: &[u8]) -> NodeId {

    let letter = &word[start_idx as usize];
    let src_node = self.nodes.get_internal(&src_node_id).unwrap();
    let node_length = src_node.length();
    let edge_id = src_node.get_to(letter).unwrap();
    let edge = self.edges.get(edge_id).unwrap();
    let edge_dest = edge.dest;
    let edge_is_sink= edge.is_sink;
    let edge_start_idx = edge.start_idx;
    let edge_end_idx = edge.end_idx;
    let edge_sink_id = edge.sink_id;

    let left_substring_idx_diff = end_idx - start_idx;
    let left_substring_length = left_substring_idx_diff + 1;

    let new_new_node = self.nodes.new_inode(SOURCE_ID);
    let mut new_node = new_new_node.0;
    let new_node_id = new_new_node.1;
    new_node._length = node_length + left_substring_length as NodeLength;

    self.nodes.add_node(new_node_id, new_node);

    self.set_edge(
      new_node_id,
      edge_sink_id,
      edge_start_idx + left_substring_length,
      edge_end_idx,
      edge_dest,
      edge_is_sink,
    );

    self.set_edge(
      *src_node_id,
      edge_sink_id,
      edge_start_idx,
      edge_start_idx + left_substring_idx_diff,
      new_node_id,
      false,
    );

    return new_node_id;
  }

  fn separate_node(&mut self, mut src_node_id: NodeId, mut start_idx: StrIdx, end_idx: StrIdx, word: &[u8]) -> (NodeId, StrIdx) {

    let canonized_data = self.canonize(src_node_id, start_idx, end_idx, word);

    if canonized_data.1 <= end_idx {
      return canonized_data;
    }

    let mut src_node = self.nodes.get_internal(&src_node_id).unwrap();
    let canon_node_id = canonized_data.0;
    let canon_node = self.nodes.get_internal(&canon_node_id).unwrap();
    let sep_length = src_node.length() as StrIdx + end_idx - start_idx + 1;

    if canon_node.length() == sep_length as NodeLength {
      return canonized_data;
    }

    let sep_node_id = self.clone_node(&canon_node_id);
    let mut sep_node = self.nodes.get_mut_internal(&sep_node_id).unwrap();
    sep_node._length = sep_length as NodeLength;

    let canon_node = self.nodes.get_mut_internal(&canon_node_id).unwrap();
    canon_node.suffix = sep_node_id;

    loop {

      src_node = self.nodes.get_mut_internal(&src_node_id).unwrap();
      let src_node_suffix = src_node.suffix;

      let letter = word[start_idx as usize];
      let edge_id = src_node.get_to(&letter).unwrap();
      let edge = self.edges.get(edge_id).unwrap();
      let edge_sink_id = edge.sink_id;
      let edge_start_idx = edge.start_idx;
      let edge_end_idx = edge.end_idx;

      self.set_edge(
        src_node_id,
        edge_sink_id,
        edge_start_idx,
        edge_end_idx,
        sep_node_id,
        false,
      );

      let suffix_canonized_data = self.canonize(src_node_suffix, start_idx, end_idx - 1, word);
      src_node_id = suffix_canonized_data.0;
      start_idx = suffix_canonized_data.1;
      let new_canonized_node_pair = self.canonize(src_node_id, start_idx, end_idx, word);

      if (canonized_data.0 != new_canonized_node_pair.0) || canonized_data.1 != new_canonized_node_pair.1 {
        break;
      }
    }

    return (sep_node_id, end_idx + 1);
  }

  fn get_suffix_id(&self, node_id: &NodeId) -> NodeId {

    let node = self.nodes.get_internal(node_id).unwrap();

    return node.suffix;
  }

  fn get_edge_idx_diff(&self, src_node_id: NodeId, edge: &SeaEdge) -> StrIdx {

    if !edge.is_sink && src_node_id == ROOT_ID {
      return 0;
    }

    return match edge.is_sink {
      true => self.nodes.get_sink(&edge.dest).unwrap().length(),
      false => edge.end_idx - edge.start_idx,
    };
  }

  fn clone_node(&mut self, node_id: &NodeId) -> NodeId {

    let src_node = self.nodes.get_internal(node_id).unwrap();

    let suffix = src_node.suffix;
    let length = src_node.length();

    let new_cloned_node = self.nodes.new_inode(suffix);
    let mut cloned_node = new_cloned_node.0;
    let cloned_node_id = new_cloned_node.1;
    cloned_node._length = length;

    self.nodes.add_node(cloned_node_id, cloned_node);

    // REMARK: Rust is stupidly weird. If I have a MUT lock, then I must obviously have exclusive access to internal
    // data. WTF is this annoying error around not being able to take a non exclusive READ (immutable) lock where
    // I already have an exclusive WRITE (mutable) lock. I resort to unsafe then.
    let src_node: *const SeaINode = self.nodes.get_internal(node_id).unwrap();
    unsafe {
      for (letter, edge_id) in src_node.as_ref().unwrap().to_edges.iter() {
        let edge = self.edges.get(&edge_id).unwrap();

        let sink_id = edge.sink_id;
        let start_idx = edge.start_idx;
        let end_idx = edge.end_idx;
        let edge_dest = edge.dest;
        let edge_is_sink = edge.is_sink;

        self.set_edge(
          cloned_node_id,
          sink_id,
          start_idx,
          end_idx,
          edge_dest,
          edge_is_sink,
        );
      }
    }

    return cloned_node_id;
  }

  fn set_edge(
    &mut self,
    src_node_id: NodeId,
    sink_id: NodeId,
    start_idx: StrIdx,
    end_idx: StrIdx,
    dest: NodeId,
    is_sink: bool,
  ) -> EdgeId {

    if !is_sink && start_idx > end_idx {
      panic!("start idx cannot be greater than end");
    }

    let sink = self.get_sink(&sink_id);
    let word = &*sink.unwrap().word;
    let letter = get_codepoint_at(word, start_idx as usize);
    let src_node: &SeaINode = self.nodes.get_internal(&src_node_id).unwrap();
    let existing_edit_id_option = src_node.get_to(&letter);

    if existing_edit_id_option.is_some() {
      let existing_edge_id = *existing_edit_id_option.unwrap();

      if let Some(existing_edge) = self.edges.get_mut(&existing_edge_id) {

        existing_edge.sink_id = sink_id;
        existing_edge.start_idx = start_idx;
        existing_edge.end_idx = end_idx;
        existing_edge.is_sink = is_sink;
        existing_edge.dest = dest;
      }

      return existing_edge_id;
    } else {

      let (mut new_edge, new_edge_id) = self.edges.new_edge(
        dest,
        sink_id,
        start_idx,
        end_idx,
        is_sink
      );

      let src_node: &mut SeaINode = self.nodes.get_mut_internal(&src_node_id).unwrap();
      src_node.add_to(letter, new_edge_id);

      self.edges.add(new_edge_id, new_edge);
      return new_edge_id;
    }
  }

  fn remove_edge(&mut self, src_node_id: NodeId, letter: Letter) -> Option<EdgeId> {

    let src_node: &mut SeaINode = self.nodes.get_mut_internal(&src_node_id).unwrap();
    let existing_edit_id_option = src_node.get_to(&letter);

    if existing_edit_id_option.is_none() {
      return None;
    }

    let existing_edge_id = *existing_edit_id_option.unwrap();

    if let Some(existing_edge) = self.edges.get_mut(&existing_edge_id) {

      src_node.remove_to(&letter);

      return Some(existing_edge_id);
    }

    return None;
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
