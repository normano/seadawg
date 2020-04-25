use crate::data::{new_hashmap, SeaDHashMap, SeaDHashSet, new_hashset};
use crate::id_allocator::U32IdAllocator;
use crate::map::VecMapU32;
use super::traversal::{
  TraversalExecutor, TraversalContextData, TraversalMode,
  FindPrefixTraverser, FindPrefixContext,
  FindSuffixTraverser, FindSuffixContext,
  FindSuperStringTraverser, FindSuperStringContext,
};
use crate::utils::{get_codepoint_at};
use crate::vec::sorted::SortedVecU32;
use crate::bt::traversal::TraversalResult;
use std::collections::HashSet;
use std::time::Duration;
use crate::vec::sorted_u8::SortedVecU8;
use crate::cmp::Compare;
use std::fmt::{Debug, Formatter, Error};
use std::borrow::Borrow;

pub type NodeId = u32;
pub type EdgeId = u32;
pub type Letter = u8;
pub type StrLength = u32;
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
  pub fn new_edge(&mut self, dest: NodeId, sink_id: NodeId, start_idx: StrIdx, end_idx: StrIdx) -> (SeaEdge, EdgeId) {

    let edge_id = self.next_edge_id();
    let edge = SeaEdge {
      dest,
      sink_id,
      start_idx,
      end_idx,
    };

    return (edge, edge_id as EdgeId);
  }

  fn next_edge_id(&mut self) -> NodeId {

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

impl Debug for SeaEdges {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SeaEdges")
      .field("edges", self.inner.borrow())
      .finish()
  }
}

/// Remark: Because of the structure, there will be a lot of leaves, so "to_edges" exists to lazily instantiate
/// a vector rather than waste memory on the leaf if it has no edges. Hashmap has longer term costs of course.
pub (in crate) struct SeaNodes<V> {
  internal: VecMapU32<SeaINode>,
  sinks: VecMapU32<SeaSinkNode<V>>,
  pub to_edges: SeaDHashMap<NodeId, SortedVecU8<(Letter, EdgeId)>>,
  _internal_id_allocator: U32IdAllocator,
  _sink_id_allocator: U32IdAllocator,
}

impl <V> SeaNodes<V> {

  pub fn new() -> Self {

    let mut nodes = SeaNodes {
      internal: VecMapU32::new(),
      sinks: VecMapU32::new(),
      to_edges: new_hashmap(),
      _internal_id_allocator: U32IdAllocator::new_start_at(2),
      _sink_id_allocator: U32IdAllocator::new_start_at(0),
    };

    let root_node = SeaINode::new(-1, 0);

    let mut source_node = SeaINode::new(0, ROOT_ID);

    nodes.add_node(ROOT_ID, root_node);
    nodes.add_node(SOURCE_ID, source_node);

    return nodes;
  }

  #[inline]
  pub fn new_inode(&mut self, length: NodeLength, suffix: NodeId) -> (SeaINode, NodeId) {

    return (SeaINode::new(length, suffix), self.next_inode_id());
  }

  fn next_inode_id(&mut self) -> NodeId {

    return self._internal_id_allocator.next_id();
  }

  fn next_sink_id(&mut self) -> NodeId {

    return self._sink_id_allocator.next_id();
  }

  #[inline]
  pub fn add_node(&mut self, node_id: u32, node: SeaINode) {
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

  pub fn has_no_to_edges(&self, src_id: &NodeId) -> bool {

    let container_opt = self.to_edges.get(src_id);
    if container_opt.is_none() {
      return false;
    }

    return container_opt.unwrap().is_empty();
  }

  pub fn add_to(&mut self, src_id: NodeId, letter: Letter, id: EdgeId) {

    let container_opt = self.to_edges.get_mut(&src_id);
    if container_opt.is_none() {

      let mut container = SortedVecU8::new();
      container.insert_unique((letter, id));

      self.to_edges.insert(src_id, container);
      return;
    }

    container_opt.unwrap().insert_unique((letter, id));
  }

  pub fn remove_to(&mut self, src_id: &NodeId, letter: &Letter) {

    let container_opt = self.to_edges.get_mut(src_id);
    if container_opt.is_none() {
      return;
    }

    container_opt.unwrap().remove_by_key(letter, |item| item.0);
  }

  #[inline]
  pub fn get_to(&self, src_id: &NodeId, letter: &Letter) -> Option<&EdgeId> {

    let container_opt = self.to_edges.get(src_id);
    if container_opt.is_none() {
      return None;
    }

    return container_opt.unwrap().find(letter, |item| item.0).map(|item| &item.1);
  }

  pub fn get_to_edges(&self, src_id: &NodeId) -> Vec<EdgeId> {

    let container_opt = self.to_edges.get(src_id);
    if container_opt.is_none() {
      return vec![];
    }

    return container_opt.unwrap().iter().map(|item| item.1).collect();
  }

  #[inline]
  pub fn contains_to(&self, src_id: &NodeId, letter: &Letter) -> bool {
    return self.get_to(src_id, letter).is_some();
  }
}

impl<V: Debug> Debug for SeaNodes<V> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SeaNodes")
      .field("sinks", &self.sinks)
      .field("internal", &self.internal)
      .field("to_edges", &self.to_edges)
      .finish()
  }
}

#[derive(Debug)]
#[repr(packed)]
pub struct SeaEdge {
  pub dest: NodeId,
  pub sink_id: NodeId,
  pub start_idx: NodeLength,
  pub end_idx: NodeLength,
}

#[derive(Debug)]
#[repr(packed)]
pub struct SeaINode {
  _length: NodeLength,
  suffix: NodeId,
  pub sink_nodes: SortedVecU32<NodeId>,
}

impl SeaINode {
  pub fn new(length: NodeLength, suffix: NodeId) -> Self {
    return Self {
      suffix,
      sink_nodes: SortedVecU32::new(),
      _length: length,
    };
  }

  #[inline]
  pub fn length(&self) -> NodeLength {
    return self._length;
  }

  pub fn add_sink(&mut self, sink_id: NodeId) {
    self.sink_nodes.insert_unique(sink_id);
  }

  pub fn remove_sink(&mut self, sink_id: &NodeId) {
    self.sink_nodes.remove_item(&sink_id);
  }

  pub fn has_sink(&self, sink_id: &NodeId) -> bool {
    return self.sink_nodes.contains(&sink_id);
  }

  pub fn sink_ids(&self) -> Vec<NodeId> {
    return self.sink_nodes.iter().cloned().collect();
  }
}

#[derive(Debug)]
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
  pub fn length(&self) -> NodeLength {
    return self.word.len() as NodeLength;
  }
}

///
/// Online Multi String CDAWG extended with the property of no terminator.
///
#[derive(Debug)]
pub struct SeaDawgCore<V = ()> {
  pub (in crate) edges: SeaEdges,
  pub (in crate) nodes: SeaNodes<V>,
  sink_id: NodeId,
  _size: u32,
  _debug: bool,
  _lite: bool,
}

impl <V> SeaDawgCore <V> {

  pub fn new() -> Self {

    return Self {
      edges: SeaEdges::new(),
      nodes: SeaNodes::new(),
      sink_id: std::u32::MAX,
      _size: 0,
      _debug: false,
      _lite: false,
    };
  }

  #[inline]
  pub fn enable_lite(&mut self) {
    panic!("Not supported");
    // self._lite = true;
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

    let word = sink.word.clone();
    let word_bytes = word.as_bytes();
    self.sink_id = self.nodes.next_sink_id();
    self.nodes.add_sink(self.sink_id.clone(), sink);

    let mut update_data = (SOURCE_ID, 0);
    let mut word_idx: usize = 0;
    while word_idx < word_bytes.len() {

      let letter = word_bytes[word_idx];

      if !self.nodes.contains_to(&ROOT_ID, &letter) {
        self.set_edge(ROOT_ID, self.sink_id, word_idx as i16, word_idx as i16, SOURCE_ID);
      }

      update_data = self.update(&*word_bytes, letter, update_data, word_idx as i16);

      word_idx += 1;
    }

    if update_data.1 < word_bytes.len() as i16 {
      //TODO Should find the best way to merge this into the update algorithm
      // This section exists to create splits in the graph since the initial run does not create all the nodes
      // If one adds lol, the CDAWG should create this structure without terminators.
      // Sinks are referenced in the nodes instead of having explicit edges point to them.
      // SRC ------> OL
      //     -> L -> OL
      let mut prev_node_id: Option<NodeId> = None;
      let word_bytes_len = word_bytes.len();
      for word_start_idx in 0..word_bytes_len {
        let mut sub_node_id: NodeId = SOURCE_ID;
        let mut sub_node_opt: Option<&mut SeaINode> = None;
        let mut edge_id: EdgeId = 0;
        let mut edge_src_id = 0;
        let mut matching_edge_start_idx = 0usize;
        let mut matching_edge_end_idx = 0usize;

        let needle_len = word_bytes_len - word_start_idx;
        let mut word_idx = word_start_idx; // current check
        let mut prev_idx = 0; // Last eq success

        while word_idx < word_bytes_len {
          let edge_letter = &word_bytes[word_idx as usize];
          let edge_id_opt = self.nodes.get_to(&sub_node_id, edge_letter);

          if edge_id_opt.is_none() {
            break;
          }

          let matching_edge_id = edge_id_opt.unwrap();
          let matching_edge_opt = self.edges.get(matching_edge_id);
          let matching_edge = matching_edge_opt.unwrap();
          matching_edge_start_idx = matching_edge.start_idx as usize;
          matching_edge_end_idx = matching_edge.end_idx as usize;
          let sink = self.get_sink(&matching_edge.sink_id);
          let edge_word = sink.unwrap().word.as_bytes();

          let partial_len: usize = (self.get_edge_idx_diff(matching_edge) + 1) as usize;
          let needle_substring_len = word_idx + partial_len;
          let word_bytes_remaining = needle_len as isize - (needle_substring_len - word_start_idx) as isize;

          if word_bytes_remaining >= 0 && edge_word[matching_edge_start_idx..matching_edge_end_idx + 1].feq(&word_bytes[word_idx..needle_substring_len]) {
            edge_src_id = sub_node_id;
            edge_id = *edge_id_opt.unwrap();
            let edge = self.edges.get(&edge_id).unwrap();
            prev_idx = word_idx;

            word_idx += partial_len;
            sub_node_id = edge.dest;
          } else {
            edge_src_id = sub_node_id;
            edge_id = *edge_id_opt.unwrap();
            let edge = self.edges.get(&edge_id).unwrap();

            sub_node_id = edge.dest;

            break;
          }
        }

        if word_idx > word_bytes_len {
          let edge = self.edges.get(&edge_id).unwrap();
          let sink = self.get_sink(&edge.sink_id);

          unsafe {
            let edge_word_ptr: *const [u8] = &sink.unwrap().word.as_bytes()[(edge.start_idx as usize)..(edge.end_idx as usize + 1)];
            let edge_word: &[u8] = &*edge_word_ptr;

            let mut split_idx = 0;

            for temp_split_idx in 0..(word_bytes.len() - prev_idx) {
              if !(&*edge_word)[temp_split_idx..temp_split_idx + 1].feq(&word_bytes[(prev_idx + temp_split_idx)..(prev_idx + temp_split_idx + 1)]) {
                split_idx = temp_split_idx;
                break;
              }
            }

            if split_idx == 0 && ((word_bytes.len() - prev_idx) < (&*edge_word).len()) {
              // Reaching here means that the partial word is shorter than the corresponding edge partial and is the same up to where it ends, so a split needs to occur at the effectively at the last letter so the sink can be linked.
              split_idx = word_bytes.len() - prev_idx - 1;
            }

            sub_node_id = self.split_edge(
              &edge_src_id,
              0,
              split_idx as i16,
              &*edge_word
            );
          }

          sub_node_opt = self.nodes.get_mut_internal(&sub_node_id);
        } else if word_idx < word_bytes_len {
          let edge = self.edges.get(&edge_id).unwrap();
          let sink = self.get_sink(&edge.sink_id);

          let edge_dest = edge.dest;
          let edge_dest_suffix = self.nodes.get_internal(&edge_dest).unwrap().suffix;
          let mut split_idx = 0;
          let edge_word_len = (matching_edge_end_idx - matching_edge_start_idx + 1) as usize;
          let word_bytes_remaining = word_bytes_len - word_idx;
          let mut should_add_edge = false;

          if edge_word_len > 1 {
            unsafe {
              let edge_word_ptr: *const [u8] = &sink.unwrap().word.as_bytes()[(matching_edge_start_idx as usize)..(matching_edge_end_idx as usize + 1)];
              let edge_word: &[u8] = &*edge_word_ptr;

              let diff;
              if word_bytes_remaining > edge_word_len {
                diff = edge_word_len;
              } else {
                diff = word_bytes_remaining;
              }

              for temp_split_idx in 0..(diff) {
                // First idx should match should the branch was taken
                if !(&*edge_word)[temp_split_idx..(temp_split_idx + 1)].feq(&word_bytes[(word_idx + temp_split_idx)..(word_idx + temp_split_idx + 1)]) {
                  split_idx = temp_split_idx - 1;
                  should_add_edge = true;
                  break;
                }
              }

              if split_idx == 0 && (word_bytes_remaining < edge_word_len) {
                // Reaching here means that the partial word is shorter than the corresponding edge partial and is the same up to where it ends, so a split needs to occur at the effectively at the last letter so the sink can be linked.
                split_idx = word_bytes_remaining - 1;
              }

              let parent = edge_src_id;
              edge_src_id = self.split_edge(
                &edge_src_id,
                0,
                split_idx as i16,
                &*edge_word
              );

              //let mut edge_parent_opt = self.nodes.get_mut_internal(&parent);
              // edge_parent_opt.as_mut().unwrap().suffix = edge_dest_suffix_suffix;
              //edge_parent_opt.unwrap().add_sink(self.sink_id);

              // let edge_dest_suffix_suffix = self.get_suffix_id(&edge_dest_suffix);
              let mut edge_src_opt = self.nodes.get_mut_internal(&edge_src_id);
              edge_src_opt.as_mut().unwrap().suffix = edge_dest_suffix;
              edge_src_opt.unwrap().add_sink(self.sink_id);

              // if !should_add_edge {
              //   edge_src_id = parent;
              // }
            }
          } else {

            should_add_edge = true;
            //split_idx += 1;
          }

          if should_add_edge {
            let update_node_next = self.nodes.get_internal(&edge_src_id).unwrap();
            let sub_node_length = update_node_next._length + split_idx as NodeLength + 1;
            let new_sub_node = self.nodes.new_inode(sub_node_length, edge_dest_suffix);
            let mut sub_node = new_sub_node.0;
            sub_node_id = new_sub_node.1;

            self.nodes.add_node(sub_node_id, sub_node);

            self.set_edge(
              edge_src_id,
              self.sink_id,
              (word_idx + split_idx - 1) as i16,
              (word_bytes_len - 1) as i16,
              sub_node_id,
            );
          } else {
            sub_node_id = edge_src_id;
          }

          sub_node_opt = self.nodes.get_mut_internal(&sub_node_id);
        } else {
          sub_node_opt = self.nodes.get_mut_internal(&sub_node_id);
        }

        if prev_node_id.is_some() {

          // It would be correct to keep a previous sub node id since iteration happens from left to right
          // when previous node is not null, set its suffix to the sub node. Should probably only set this
          // if the suffix is not already set though.
          // Suffix link works like [abba] -> [bba] -> [ba] -> [a]
          let mut prev_node_opt = self.nodes.get_mut_internal(&prev_node_id.unwrap());
          let prev_node = prev_node_opt.as_mut().unwrap();

          if !self._lite {
            prev_node.suffix = sub_node_id;
          } else {
            prev_node.suffix = prev_node_id.unwrap();
          }
        }

        sub_node_opt = self.nodes.get_mut_internal(&sub_node_id);

        sub_node_opt.unwrap().add_sink(self.sink_id);
        prev_node_id = Some(sub_node_id);
      }

      if prev_node_id.is_some() && prev_node_id.unwrap() != SOURCE_ID {

        // It would be correct to keep a previous sub node id since iteration happens from left to right
        // when previous node is not null, set its suffix to the sub node. Should probably only set this
        // if the suffix is not already set though.
        // Suffix link works like [abba] -> [bba] -> [ba] -> [a]
        let mut prev_node_opt = self.nodes.get_mut_internal(&prev_node_id.unwrap());
        let prev_node = prev_node_opt.as_mut().unwrap();
        if !self._lite {
          prev_node.suffix = SOURCE_ID;
        } else {
          prev_node.suffix = prev_node_id.unwrap();
        }
      }
    } else {

      // Suffix is already entered in, so add sinks
      let mut cur_node = update_data.0;

      while cur_node != SOURCE_ID {
        let node = self.nodes.get_mut_internal(&cur_node).unwrap();
        node.add_sink(self.sink_id);
        let suffix_node_id = self.get_suffix_id(&cur_node);
        cur_node = suffix_node_id;
      }
    }

    self.sink_id = NONE_SINK_ID;
    self._size += 1;
  }

  /// Not Implemented yet
  pub fn remove(&mut self, word: &str) -> Option<SeaSinkNode<V>> {
    unimplemented!()
  }

  // Returns edge src, edge, dest
  fn _find(&self, word_bytes: &[u8]) -> Option<(NodeId, EdgeId, NodeId)> {

    let word_bytes_len = word_bytes.len();
    let mut sub_node_id: NodeId = SOURCE_ID;
    let mut edge_id: EdgeId = 0;
    let mut edge_src_id = 0;
    let mut matching_edge_start_idx = 0usize;
    let mut matching_edge_end_idx = 0usize;

    let needle_len = word_bytes_len;
    let mut word_idx = 0;
    let mut prev_idx = 0;

    while word_idx < word_bytes_len {

      let edge_letter = &word_bytes[word_idx as usize];
      let edge_id_opt = self.nodes.get_to(&sub_node_id, edge_letter);

      if edge_id_opt.is_none() {
        break;
      }

      let matching_edge_id = edge_id_opt.unwrap();
      let matching_edge_opt = self.edges.get(matching_edge_id);
      let matching_edge = matching_edge_opt.unwrap();
      matching_edge_start_idx = matching_edge.start_idx as usize;
      matching_edge_end_idx = matching_edge.end_idx as usize;
      let sink = self.get_sink(&matching_edge.sink_id);
      let edge_word = sink.unwrap().word.as_bytes();

      let partial_len: usize = (self.get_edge_idx_diff(matching_edge) + 1) as usize;
      let needle_substring_len = word_idx + partial_len;

      if needle_substring_len <= needle_len && edge_word[matching_edge_start_idx..matching_edge_end_idx + 1].feq(&word_bytes[word_idx..needle_substring_len]) {

        edge_src_id = sub_node_id;
        edge_id = *edge_id_opt.unwrap();
        let edge = self.edges.get(&edge_id).unwrap();
        prev_idx = word_idx;

        word_idx += partial_len;
        sub_node_id = edge.dest;

      } else {

        return None;
      }
    }

    return Some((edge_src_id, edge_id, sub_node_id));
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
      let matching_edge_id_option = self.nodes.get_to(&current_node_id, &word_cp);

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

      let partial_len: usize = (self.get_edge_idx_diff(matching_edge) + 1) as usize;
      let needle_substring_len = word_idx + partial_len;

      if needle_substring_len <= needle_len && edge_word[matching_edge_start_idx..matching_edge_end_idx + 1].feq(&needle_bytes[word_idx..needle_substring_len]) {

        if needle_len == needle_substring_len {

          let dest = self.nodes.get_internal(&matching_edge.dest).unwrap();
          for sink_node_id in dest.sink_nodes.iter() {

            let sink_node = self.nodes.get_sink(sink_node_id).unwrap();

            if needle_bytes.feq(sink_node.word.as_bytes()) {
              target_node_id = Some(*sink_node_id);
              break;
            }
          }
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

  pub fn find_with_prefix(&self, prefix: &str) -> Vec<TraversalResult> {

    let mut traverser = FindPrefixTraverser::new(prefix);
    let prefix_inner = TraversalContextData::new(
      TraversalMode::Traversal,
      Some(SOURCE_ID),
      vec![],
      vec![],
      0,
      None,
    );
    let base_context = FindPrefixContext::new(prefix_inner);

    let executor = TraversalExecutor::new();

    return executor.execute_traversal(self, &mut traverser, base_context);
  }

  pub fn find_with_suffix(&self, needle: &str) -> Vec<TraversalResult> {

    let mut traverser = FindSuffixTraverser::new(needle);
    let context_inner = TraversalContextData::new(
      TraversalMode::Traversal,
      Some(SOURCE_ID),
      vec![],
      vec![],
      0,
      None,
    );
    let base_context = FindSuffixContext::new(context_inner);

    let executor = TraversalExecutor::new();

    return executor.execute_traversal(self, &mut traverser, base_context);
  }

  pub fn find_with_substring(&self, needle: &str) -> Vec<TraversalResult> {

    let mut traverser = FindSuperStringTraverser::new(needle);
    let context_inner = TraversalContextData::new(
      TraversalMode::Traversal,
      Some(SOURCE_ID),
      vec![],
      vec![],
      0,
      None,
    );
    let base_context = FindSuperStringContext::new(context_inner, false);

    let executor = TraversalExecutor::new();
    let results = executor.execute_traversal(self, &mut traverser, base_context);

    return results;
  }

  fn update(&mut self, word: &[u8], letter: Letter, (mut update_node_id, mut start_idx): (NodeId, StrIdx), end_idx: StrIdx) -> (NodeId, StrIdx) {

    let mut prev_node_id_option: Option<NodeId> = None;
    let mut update_node_prime_option: Option<NodeId> = None;
    let mut update_node_next_id: Option<NodeId> = None;
    let prev_end_idx = end_idx - 1;

    while !self.check_endpoint(&update_node_id, start_idx, prev_end_idx, letter, word) {

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

      //TODO Maybe use a flag
      assert!(!self.nodes.contains_to(&update_node_next_id.unwrap(), &letter), "Edge Clobbering detected");

      let update_node_next = self.nodes.get_internal(&update_node_next_id.unwrap()).unwrap();
      let sub_node_length = update_node_next._length + word.len() as NodeLength - end_idx as NodeLength;

      let mut new_sub_node = self.nodes.new_inode(sub_node_length, SOURCE_ID);
      let mut sub_node = new_sub_node.0;
      let sub_node_id = new_sub_node.1;

      self.nodes.add_node(sub_node_id, sub_node);
      self.set_edge(
        update_node_next_id.unwrap(),
        self.sink_id,
        end_idx,
        (word.len() - 1) as StrIdx,
        sub_node_id
      );
      let sub_node = self.nodes.get_mut_internal(&sub_node_id).unwrap();
      sub_node.add_sink(self.sink_id);
      update_node_next_id = Some(sub_node_id);

      if prev_node_id_option.is_some() {
        //   let suffix = self.get_suffix_id(&prev_node_id_option.unwrap());
        self.nodes.get_mut_internal(&prev_node_id_option.unwrap()).unwrap().suffix = update_node_next_id.unwrap();

        if update_node_next_id.unwrap() != update_node_id {
          // update_node_next_id = Some(sub_node_id);
        }
      }

      prev_node_id_option = update_node_next_id.clone();

      let canonized_data = self.canonize(
        self.get_suffix_id(&update_node_id),
        start_idx,
        prev_end_idx,
        word
      );
      update_node_id = canonized_data.0;
      start_idx = canonized_data.1;
    }

    if prev_node_id_option.is_some() {

      let suffix = self.get_suffix_id(&prev_node_id_option.unwrap());
      if !self._lite {
        self.nodes.get_mut_internal(&prev_node_id_option.unwrap()).unwrap().suffix = update_node_id;
      } else {
        self.nodes.get_mut_internal(&prev_node_id_option.unwrap()).unwrap().suffix = prev_node_id_option.unwrap();
      }
    }

    return self.separate_node(update_node_id, start_idx, end_idx, word);
  }

  fn check_endpoint(&self, node_id: &NodeId, start_idx: StrIdx, end_idx: StrIdx, letter: Letter, word: &[u8]) -> bool {

    if start_idx <= end_idx {
      let word_letter = word[start_idx as usize];
      let edge_id = self.nodes.get_to(node_id, &word_letter).unwrap();
      let edge = self.edges.get(edge_id).unwrap();

      let sink = self.get_sink(&edge.sink_id);
      let word = &*sink.unwrap().word;
      let partial_letter = get_codepoint_at(word, (edge.start_idx + end_idx - start_idx + 1) as usize);

      return letter == partial_letter;
    }

    return self.nodes.contains_to(node_id, &letter);
  }

  fn canonize(&mut self, mut node_id: NodeId, mut start_idx: StrIdx, end_idx: StrIdx, word: &[u8]) -> (NodeId, StrIdx) {

    if start_idx > end_idx {
      return (node_id, start_idx);
    }

    let edge_letter = word[start_idx as usize];

    let mut edge_id = self.nodes.get_to(&node_id, &edge_letter).unwrap();

    let mut edge = self.edges.get(edge_id).unwrap();
    let mut edge_src = node_id;

    let mut edge_idx_diff = self.get_edge_idx_diff(edge) as StrIdx;
    while edge_idx_diff <= end_idx - start_idx {

      start_idx += edge_idx_diff + 1;

      node_id = edge.dest;

      if start_idx <= end_idx {
        let word_letter = &word[start_idx as usize];

        edge_id = self.nodes.get_to(&node_id, word_letter).unwrap();
        edge = self.edges.get(edge_id).unwrap();
        edge_src = node_id;
      }

      edge_idx_diff = self.get_edge_idx_diff(edge) as StrIdx;
    }

    return (node_id, start_idx);
  }

  fn extension(&self, node_id: NodeId, start_idx: StrIdx, end_idx: StrIdx, word: &[u8]) -> NodeId {

    if start_idx > end_idx {
      return node_id;
    }

    let letter = word[start_idx as usize];
    let edge_id = self.nodes.get_to(&node_id, &letter).unwrap();
    let edge = self.edges.get(edge_id).unwrap();

    return edge.dest;
  }

  fn redirect_edge(&mut self, src_node_id: NodeId, start_idx: StrIdx, end_idx: StrIdx, dest: NodeId, word: &[u8]) {

    let letter = word[start_idx as usize];
    let edge_id = self.nodes.get_to(&src_node_id, &letter).unwrap();
    let edge = self.edges.get(edge_id).unwrap();
    let edge_start_idx = edge.start_idx;
    let edge_sink_id = edge.sink_id;

    let substring_idx_diff = end_idx - start_idx;
    let edge_end_idx = edge_start_idx + substring_idx_diff;

    self.set_edge(src_node_id, edge_sink_id, edge_start_idx, edge_end_idx, dest);
  }

  fn split_edge(&mut self, src_node_id: &NodeId, start_idx: StrIdx, end_idx: StrIdx, word: &[u8]) -> u32 {

    if start_idx > end_idx {
      panic!("Split edge cannot have start less than end");
    }

    let letter = word[start_idx as usize];
    let src_node = self.nodes.get_internal(&src_node_id).unwrap();
    let node_length = src_node.length();
    let edge_id = self.nodes.get_to(&src_node_id, &letter).unwrap();
    let edge = self.edges.get(edge_id).unwrap();
    let edge_dest = edge.dest;
    let edge_start_idx = edge.start_idx;
    let edge_end_idx = edge.end_idx;
    let edge_sink_id = edge.sink_id;

    let left_substring_idx_diff = end_idx - start_idx;
    let left_substring_length = left_substring_idx_diff + 1;

    let new_new_node = self.nodes.new_inode(node_length + left_substring_length as NodeLength,SOURCE_ID);
    let mut new_node = new_new_node.0;
    let new_node_id = new_new_node.1;

    /*TODO Need to think about how splits should distribute nodes  */
    // Copy sinks
    // let mut sinks_to_remove = vec![];
    // let src_node = self.nodes.get_internal(&src_node_id).unwrap();
    // for sink_id in src_node.sink_nodes.iter() {
    //   new_node.add_sink(*sink_id);
    //
    //   if src_node_id == &SOURCE_ID {
    //     let sink = self.nodes.get_sink(sink_id).unwrap();
    //     let sink_word = &*sink.word;
    //     let edge_partial = &sink_word.as_bytes()[(edge_start_idx as usize)..(edge_start_idx + left_substring_idx_diff) as usize];
    //     let target_word = &word[(start_idx as usize)..(start_idx + left_substring_idx_diff) as usize];
    //
    //     if edge_partial.ends_with(target_word) {
    //       sinks_to_remove.push(*sink_id);
    //     }
    //   }
    // }
    //
    // if !sinks_to_remove.is_empty() {
    //   let src_node = self.nodes.get_mut_internal(&src_node_id).unwrap();
    //   for sink in sinks_to_remove {
    //     src_node.sink_nodes.remove_item(&sink);
    //   }
    // }

    self.nodes.add_node(new_node_id, new_node);

    self.set_edge(
      new_node_id,
      edge_sink_id,
      edge_start_idx + left_substring_length,
      edge_end_idx,
      edge_dest,
    );

    self.set_edge(
      *src_node_id,
      edge_sink_id,
      edge_start_idx,
      edge_start_idx + left_substring_idx_diff,
      new_node_id,
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

    // unsafe {
    //   // Copy sinks
    //   let mut sep_node: *mut SeaINode = self.nodes.get_mut_internal(&sep_node_id).unwrap();
    //   let canon_node = self.nodes.get_internal(&canon_node_id).unwrap();
    //   for sink in canon_node.sink_nodes.iter() {
    //     sep_node.as_mut().unwrap().add_sink(*sink);
    //   }
    // }

    loop {


      let letter = word[start_idx as usize];
      let edge_id = self.nodes.get_to(&src_node_id, &letter).unwrap();
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
      );

      src_node = self.nodes.get_mut_internal(&src_node_id).unwrap();
      let src_node_suffix = src_node.suffix;

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

  pub (in crate) fn get_edge_idx_diff(&self, edge: &SeaEdge) -> NodeLength {

    return edge.end_idx - edge.start_idx;
  }

  fn clone_node(&mut self, node_id: &NodeId) -> u32 {

    let src_node = self.nodes.get_internal(node_id).unwrap();

    let suffix = src_node.suffix;
    let length = src_node.length();

    let new_cloned_node = self.nodes.new_inode(length, suffix);
    let cloned_node = new_cloned_node.0;
    let cloned_node_id = new_cloned_node.1;

    self.nodes.add_node(cloned_node_id ,cloned_node);

    // REMARK: Rust is stupidly weird. If I have a MUT lock, then I must obviously have exclusive access to internal
    // data. WTF is this annoying error around not being able to take a non exclusive READ (immutable) lock where
    // I already have an exclusive WRITE (mutable) lock. I resort to unsafe then.
    let to_edges = self.nodes.get_to_edges(node_id);
    for edge_id in to_edges {
      let edge = self.edges.get(&edge_id).unwrap();

      let sink_id = edge.sink_id;
      let start_idx = edge.start_idx;
      let end_idx = edge.end_idx;
      let edge_dest = edge.dest;

      self.set_edge(
        cloned_node_id,
        sink_id,
        start_idx,
        end_idx,
        edge_dest,
      );
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
  ) -> EdgeId {

    if start_idx > end_idx {
      panic!("start idx cannot be greater than end");
    }

    let sink = self.get_sink(&sink_id);
    let word = &*sink.unwrap().word;
    let letter = get_codepoint_at(word, start_idx as usize);

    let existing_edit_id_option = self.nodes.get_to(&src_node_id, &letter);

    if existing_edit_id_option.is_some() {

      let existing_edge_id = *existing_edit_id_option.unwrap();

      if let Some(existing_edge) = self.edges.get_mut(&existing_edge_id) {

        existing_edge.sink_id = sink_id;
        existing_edge.start_idx = start_idx;
        existing_edge.end_idx = end_idx;
        existing_edge.dest = dest;
      }
      return existing_edge_id;
    } else {

      let (new_edge, new_edge_id) = self.edges.new_edge(
        dest,
        sink_id,
        start_idx,
        end_idx,
      );

      self.nodes.add_to(src_node_id, letter, new_edge_id);

      self.edges.add(new_edge_id, new_edge);
      return new_edge_id;
    }
  }

  fn remove_edge(&mut self, src_node_id: NodeId, letter: Letter) -> Option<u32> {

    let existing_edit_id_option = self.nodes.get_to(&src_node_id, &letter);

    if existing_edit_id_option.is_none() {
      return None;
    }

    let existing_edge_id = *existing_edit_id_option.unwrap();

    if let Some(existing_edge) = self.edges.get_mut(&existing_edge_id) {

      self.nodes.remove_to(&src_node_id, &letter);

      return Some(existing_edge_id);
    }

    return None;
  }
}