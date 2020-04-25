///
/// Algorithm implemented is as from Complete Inverted Files for Efficient Text Retrieval and Analysis
/// by A Blumer et. al.
///
use std::fmt::{Debug, Formatter};
use std::time::Duration;

use crate::cmp::Compare;
use crate::data::{SeaDHashMap, new_hashmap};
use crate::map::VecMapU32;
use crate::id_allocator::U32IdAllocator;
use crate::vec::sorted_u8::SortedVecU8;
use crate::vec::sorted::SortedVecU32;

use super::traversal::{
  TraversalExecutor, TraversalContextData, TraversalMode,
  FindPrefixTraverser, FindPrefixContext, TraversalResult,
  FindSuffixTraverser, FindSuffixContext,
  FindSuperStringTraverser, FindSuperStringContext,
};

pub type NodeId = u32;
pub type EdgeId = u32;
pub type Letter = u8;
pub type StrLength = u32;
pub type StrIdx = i16;
pub type NodeLength = i16;

const SOURCE_ID: NodeId = 0;

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
  pub (in crate) fn new_edge(&mut self, dest: &NodeId, label: &u8, primary: bool) -> (SeaEdge, EdgeId) {

    let edge_id = self.next_edge_id();
    let edge = SeaEdge {
      dest: *dest,
      label: *label,
      primary
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

impl Debug for SeaEdges {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SeaEdges")
      .field("edges", &self.inner)
      .finish()
  }
}

pub (in crate) struct SeaNodes {
  internal: VecMapU32<SeaNode>,
  _internal_id_allocator: U32IdAllocator,
  pub to_edges: SeaDHashMap<NodeId, SortedVecU8<(Letter, EdgeId)>>,
}

impl SeaNodes {

  pub fn new() -> Self {

    let mut nodes = SeaNodes {
      internal: VecMapU32::new(),
      _internal_id_allocator: U32IdAllocator::new_start_at(0),
      to_edges: new_hashmap(),
    };

    let (source_node, source_node_id) = nodes.new_inode(&SOURCE_ID);
    nodes.add_node(source_node_id, source_node);
    return nodes;
  }

  #[inline]
  pub fn new_inode(&mut self, suffix: &NodeId) -> (SeaNode, NodeId) {

    return (SeaNode::new(*suffix), self.next_inode_id());
  }

  fn next_inode_id(&mut self) -> NodeId {

    return self._internal_id_allocator.next_id();
  }

  #[inline]
  pub fn add_node(&mut self, node_id: NodeId, node: SeaNode) {
    self.internal.insert(node_id, node);
  }

  #[inline]
  pub fn get_internal(&self, id: &NodeId) -> Option<&SeaNode> {
    return self.internal.get(id);
  }

  #[inline]
  pub fn get_mut_internal(&mut self, id: &NodeId) -> Option<&mut SeaNode> {
    return self.internal.get_mut(id);
  }

  pub fn has_no_to_edges(&self, src_id: &NodeId) -> bool {

    let container_opt = self.to_edges.get(src_id);
    if container_opt.is_none() {
      return false;
    }

    return container_opt.unwrap().is_empty();
  }

  pub fn add_to(&mut self, src_id: &NodeId, letter: &Letter, id: &EdgeId) {

    let container_opt = self.to_edges.get_mut(src_id);
    if container_opt.is_none() {

      let mut container = SortedVecU8::new();
      container.insert_unique((*letter, *id));

      self.to_edges.insert(*src_id, container);
      return;
    }

    container_opt.unwrap().insert_unique((*letter, *id));
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

impl Debug for SeaNodes {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SeaNodes")
      .field("internal", &self.internal)
      .field("to_edges", &self.to_edges)
      .finish()
  }
}

#[repr(packed)]
#[derive(Debug)]
pub struct SeaEdge {
  pub label: u8,
  pub dest: NodeId,
  primary: bool,
}

#[repr(packed)]
#[derive(Debug)]
pub (in crate) struct SeaNode {
  suffix: NodeId,
  // freq: u32,
}

impl SeaNode {
  pub fn new(suffix: NodeId) -> Self {
    return Self {
      suffix,
      // freq: 0,
    };
  }
}

pub (in crate) struct NodeSinks<V> {
  sinks: VecMapU32<SeaSinkNode<V>>,
  pub ids_by_node: SeaDHashMap<NodeId, SortedVecU32<NodeId>>,
  _sink_id_allocator: U32IdAllocator,
}

impl <V> NodeSinks<V> {
  pub fn new() -> Self {
    return Self {
      sinks: VecMapU32::new(),
      ids_by_node: new_hashmap(),
      _sink_id_allocator: U32IdAllocator::new_start_at(0),
    };
  }

  pub fn len(&self) -> usize {
    return self.sinks.len();
  }

  fn next_sink_id(&mut self) -> NodeId {

    return self._sink_id_allocator.next_id();
  }

  #[inline]
  pub fn add_sink(&mut self, id: NodeId, node: SeaSinkNode<V>) {
    self.sinks.insert(id, node);
  }

  #[inline]
  pub fn get_sink(&self, id: &NodeId) -> Option<&SeaSinkNode<V>> {
    return self.sinks.get(id);
  }

  #[inline]
  pub fn get_mut_sink(&mut self, id: &NodeId) -> Option<&mut SeaSinkNode<V>> {
    return self.sinks.get_mut(id);
  }

  pub fn has_no_ids(&self, src_id: &NodeId) -> bool {

    let container_opt = self.ids_by_node.get(src_id);
    if container_opt.is_none() {
      return false;
    }

    return container_opt.unwrap().is_empty();
  }

  pub fn add(&mut self, src_id: &NodeId, id: &NodeId) {

    let container_opt = self.ids_by_node.get_mut(src_id);
    if container_opt.is_none() {

      let mut container = SortedVecU32::new();
      container.insert_unique(*id);

      self.ids_by_node.insert(*src_id, container);
      return;
    }

    container_opt.unwrap().insert_unique( *id);
  }

  pub fn remove(&mut self, src_id: &NodeId, id: &NodeId) {

    let container_opt = self.ids_by_node.get_mut(src_id);
    if container_opt.is_none() {
      return;
    }

    container_opt.unwrap().remove_item(id);
  }

  pub fn ids(&self, src_id: &NodeId) -> Vec<NodeId> {

    let container_opt = self.ids_by_node.get(src_id);
    if container_opt.is_none() {
      return vec![];
    }

    return container_opt.unwrap().iter().cloned().collect();
  }

  fn copy_sinks(&mut self, src_id: &NodeId, dest: &NodeId) {

    let sink_container_opt = self.ids_by_node.get(src_id);

    if sink_container_opt.is_none() {
      return;
    }

    let sink_ids: *const SortedVecU32<NodeId> = sink_container_opt.unwrap();

    unsafe {
      for sink_id in (*sink_ids).iter() {
        self.add(dest, sink_id);
      }
    }
  }
}

impl<V: Debug> Debug for NodeSinks<V> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SeaSinkNodes")
      .field("sinks", &self.sinks)
      .field("ids_by_node", &self.ids_by_node)
      .finish()
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

#[derive(Debug)]
pub struct SeaDawgCore<V = ()> {
  pub (in crate) nodes: SeaNodes,
  pub (in crate) edges: SeaEdges,
  pub (in crate) sinks: NodeSinks<V>,
  source_id: u32,
}

impl <V> SeaDawgCore<V> {

  pub fn new() -> Self {

    let nodes = SeaNodes::new();
    let edges = SeaEdges::new();

    return SeaDawgCore {
      source_id: SOURCE_ID,
      nodes,
      edges,
      sinks: NodeSinks::new(),
    };
  }

  #[inline]
  pub fn size(&self) -> usize {
    return self.sinks.len();
  }

  #[inline]
  pub fn inodes_count(&self) -> usize {
    return self.nodes.internal.len();
  }

  #[inline]
  pub fn snodes_count(&self) -> usize {
    return self.sinks.len();
  }

  #[inline]
  pub fn edges_count(&self) -> usize {
    return self.edges.inner.len();
  }

  #[inline]
  pub fn get_sink(&self, node_id: &NodeId) -> Option<&SeaSinkNode<V>> {

    return self.sinks.get_sink(node_id);
  }

  #[inline]
  pub fn get_mut_sink(&mut self, node_id: &NodeId) -> Option<&mut SeaSinkNode<V>> {

    return self.sinks.get_mut_sink(node_id);
  }

  pub fn add(&mut self, mut sink: SeaSinkNode<V>) {

    let word = sink.word.clone();
    let word_bytes = word.as_bytes();
    let sink_id = self.sinks.next_sink_id();
    self.sinks.add_sink(sink_id, sink);

    let mut active_node_id = self.source_id;

    for word_byte in word_bytes {
      active_node_id = self.update(&active_node_id, word_byte);
    }

    self.add_sinks_to_suffixes(&active_node_id, &sink_id);
  }

  fn add_sinks_to_suffixes(&mut self, start_node: &NodeId, id: &NodeId) {

    let mut cur_node = *start_node;

    while cur_node != SOURCE_ID {
      self.sinks.add(&cur_node, id);
      let node = self.nodes.get_mut_internal(&cur_node).unwrap();
      cur_node = node.suffix;
    }
  }

  fn update(&mut self, active_node_id: &NodeId, letter: &Letter) -> NodeId {

    let active_node_edge_opt = self.get_to_edge(active_node_id, letter);

    if active_node_edge_opt.is_some() {

      let edge = active_node_edge_opt.unwrap();
      let new_active_node_id = edge.dest;
      if edge.primary {
        return new_active_node_id;
      }

      return self.split(active_node_id, &new_active_node_id, letter);
    }

    let (mut new_active_node, new_active_node_id) = self.nodes.new_inode(&SOURCE_ID);
    self.nodes.add_node(new_active_node_id, new_active_node);

    let (new_edge, new_edge_id) = self.edges.new_edge(&new_active_node_id, letter, true);
    self.edges.add(new_edge_id, new_edge);

    self.nodes.add_to(&active_node_id, letter, &new_edge_id);

    let mut cur_node_id = *active_node_id;
    let mut suffix_node_id_opt: Option<NodeId> = None;

    while cur_node_id != self.source_id && suffix_node_id_opt.is_none() {

      let cur_node = self.nodes.get_internal(&cur_node_id).unwrap();
      cur_node_id = cur_node.suffix;

      let cur_node_edge_opt = self.get_to_edge(&cur_node_id, letter);
      if cur_node_edge_opt.is_some() {

        let edge = cur_node_edge_opt.unwrap();
        let is_primary = edge.primary;

        if is_primary {

          suffix_node_id_opt = Some(edge.dest);
        } else {

          let child_node = edge.dest;
          let split_node = self.split(&cur_node_id, &child_node, letter);
          suffix_node_id_opt = Some(split_node);
        }
      } else {

        let (edge, edge_id) = self.edges.new_edge(&new_active_node_id, letter, false);
        self.edges.add(edge_id, edge);

        self.nodes.add_to(&cur_node_id, letter, &edge_id);
      }
    }

    if suffix_node_id_opt.is_none() {
      suffix_node_id_opt = Some(SOURCE_ID);
    }

    let new_active_node = self.nodes.get_mut_internal(&new_active_node_id).unwrap();
    new_active_node.suffix = suffix_node_id_opt.unwrap();

    return new_active_node_id;
  }

  fn split(&mut self, parent_node_id: &NodeId, child_node_id: &NodeId, letter: &Letter) -> NodeId {
    let child_suffix = self.nodes.get_internal(&child_node_id).unwrap().suffix;
    let (new_child_node, new_child_node_id) = self.nodes.new_inode(&child_suffix);
    self.nodes.add_node(new_child_node_id, new_child_node);

    self.promote_secondary_edge(letter, parent_node_id, child_node_id, &new_child_node_id);

    let child_node_edge_ids = self.nodes.get_to_edges(child_node_id);
    for edge_id in child_node_edge_ids {

      let edge = self.edges.get(&edge_id).unwrap();
      let edge_dest = edge.dest;
      let edge_label = edge.label;
      let (cloned_edge, cloned_edge_id) = self.edges.new_edge(&edge_dest, &edge_label, false);

      self.edges.add(cloned_edge_id, cloned_edge);

      self.nodes.add_to(&new_child_node_id, &edge_label, &cloned_edge_id);
    }

    self.sinks.copy_sinks(&child_node_id, &new_child_node_id);

    self.nodes.get_mut_internal(&child_node_id).unwrap().suffix = new_child_node_id;

    let mut cur_node_id = *parent_node_id;

    while cur_node_id != self.source_id {

      cur_node_id = self.nodes.get_internal(&cur_node_id).unwrap().suffix;

      let edge_opt = self.get_to_edge(&cur_node_id, letter);
      if edge_opt.is_some() && !edge_opt.unwrap().primary && &edge_opt.unwrap().dest == child_node_id {

        self.redirect_edge(&cur_node_id, &letter, &new_child_node_id);
        continue;
      }

      break;
    }

    return new_child_node_id;
  }

  pub (in crate) fn get_to_edge(&mut self, src_id: &NodeId, letter: &Letter) -> Option<&SeaEdge> {

    let edge_id = self.nodes.get_to(src_id, letter)?;
    let edge = self.edges.get(edge_id);

    return edge;
  }

  pub (in crate)  fn get_mut_to_edge(&mut self, src_id: &NodeId, letter: &Letter) -> Option<&mut SeaEdge> {

    let edge_id = self.nodes.get_to(src_id, letter).unwrap();
    let edge = self.edges.get_mut(edge_id);

    return edge;
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

      let partial_len = 1usize;
      let needle_substring_len = word_idx + partial_len;

      if needle_substring_len <= needle_len {

        if needle_len == needle_substring_len {

          let sinks = self.sinks.ids(&matching_edge.dest);

          for sink_node_id in sinks.iter() {

            let sink_node = self.sinks.get_sink(sink_node_id).unwrap();

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

  fn redirect_edge(&mut self, src_id: &NodeId, letter: &Letter, new_dest: &NodeId) {

    let edge = self.get_mut_to_edge(src_id, letter).unwrap();
    edge.dest = *new_dest;
  }

  fn promote_secondary_edge(&mut self, letter: &Letter, src_id: &NodeId, old_dest: &NodeId, new_dest: &NodeId) {

    let edge = self.get_mut_to_edge(src_id, letter).unwrap();

    if &edge.dest != old_dest {
      panic!("Dest was not equal to old dest");
    }

    edge.dest = *new_dest;
    edge.primary = true;
  }
}