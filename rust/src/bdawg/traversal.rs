use std::cmp::Ordering;

use scalable_cuckoo_filter::ScalableCuckooFilter;

use crate::cmp::Compare;
use crate::data::{SeaDHashMap, SeaDHashSet, new_hashset};
use crate::foundation::*;
use crate::utils::{get_codepoint_at, slice_concat_byte, slice_concat_bytes};
use crate::vec::sorted::SortedVecU32;
use super::core::{SeaDawgCore, SeaEdge};

#[derive(Clone)]
pub enum TraversalMode {
  Traversal,
  Collection,
  Sink,
}

#[derive(Clone, Debug)]
pub struct TraversalResult {
  pub traversed_word: Box<str>,
  pub sink_id: NodeId,
}

impl Ord for TraversalResult {
  fn cmp(&self, other: &Self) -> Ordering {
    return self.sink_id.cmp(&other.sink_id);
  }
}

impl PartialOrd for TraversalResult {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    return self.sink_id.partial_cmp(&other.sink_id);
  }
}

impl PartialEq for TraversalResult {
  fn eq(&self, other: &Self) -> bool {
    return self.sink_id.eq(&other.sink_id);
  }
}

impl Eq for TraversalResult {
}

pub (in crate) struct TraversalExecutor {}

impl TraversalExecutor {

  pub fn new() -> Self {

    return Self {};
  }

  pub fn execute_traversal<Context: TraversalContext, Value>(
    &self,
    seadawg: &SeaDawgCore<Value>,
    mut traverser: &mut impl Traverser<Context, Value>,
    mut base_context: Context,
  ) -> Vec<TraversalResult> {

    let mut result: SortedVecU32<TraversalResult> = SortedVecU32::new();
    let mut traversal_contexts = vec![];
    let mut collected_traversal_contexts = vec![];
    traverser.setup(&mut base_context, &mut collected_traversal_contexts, seadawg);

    Self::finish_traversal_iteration(base_context, &mut traversal_contexts, &mut collected_traversal_contexts);

    while !traversal_contexts.is_empty() {
      let mut context = traversal_contexts.pop().unwrap();

      let should_traverse = !context.get_mut_edges_to_scan().is_empty();
      if !should_traverse {
        continue;
      }

      let edge_id = context.get_mut_edges_to_scan().pop().unwrap();
      let edge = seadawg.edges.get(&edge_id).unwrap();

      match context.mode() {
        TraversalMode::Traversal => {

          traverser.traverse(
            &edge_id,
            edge,
            &mut context,
            &mut collected_traversal_contexts,
            seadawg,
          );
        },
        TraversalMode::Collection => {

          traverser.collect(
            &edge_id,
            edge,
            &mut context,
            &mut collected_traversal_contexts,
            seadawg
          );
        },
        TraversalMode::Sink => {

          let traversed_word = std::str::from_utf8(context.traversed_word()).unwrap();

          if traverser.should_accept_sink_node(&context.sink_node(), context.word_idx(), traversed_word, seadawg) {
            let traversed_word = Box::from(traversed_word);
            result.insert_unique(TraversalResult {
              traversed_word,
              sink_id: context.sink_node(),
            });
          }
        },
      }

      Self::finish_traversal_iteration(context, &mut traversal_contexts, &mut collected_traversal_contexts);
    }

    return result.into_vec();
  }

  fn finish_traversal_iteration<Context: TraversalContext>(
    context: Context,
    traversal_contexts: &mut Vec<Context>,
    next_traversal_contexts: &mut Vec<Context>,
  ) {

    traversal_contexts.push(context);

    next_traversal_contexts.reverse();
    while let Some(context) = next_traversal_contexts.pop() {
      traversal_contexts.push(context);
    }
  }
}

pub trait Traverser<Context: TraversalContext, Value> {

  /// Initial selection of edges
  fn setup(&mut self, context: &mut Context, traversal_contexts: &mut Vec<Context>, seadawg: &SeaDawgCore<Value>);

  /// Moves down the graph and initiates further traversals or collections
  fn traverse(&mut self, edge_id: &EdgeId, edge: &SeaEdge, context: &mut Context, traversal_contexts: &mut Vec<Context>, seadawg: &SeaDawgCore<Value>);

  /// After pruning select sink edges that will be considered
  fn collect(&mut self, edge_id: &EdgeId, edge: &SeaEdge, context: &Context, traversal_contexts: &mut Vec<Context>, seadawg: &SeaDawgCore<Value>);

  /// Should sink node be added to the result set
  fn should_accept_sink_node(&mut self, sink_node_id: &NodeId, current_word_idx: StrLength, word: &str, seadawg: &SeaDawgCore<Value>) -> bool;
}

pub trait TraversalContext {
  fn mode(&self) -> TraversalMode;

  fn node(&self) -> NodeId;

  fn get_edges_to_scan(&self) -> &Vec<u32>;

  fn get_mut_edges_to_scan(&mut self) -> &mut Vec<EdgeId>;

  fn word_idx(&self) -> StrLength;

  fn traversed_word(&self) -> &[u8];

  fn sink_node(&self) -> NodeId;
}

pub struct TraversalContextData {
  mode: TraversalMode,
  node: Option<NodeId>,
  edges_to_scan: Vec<u32>,
  traversed_word: Vec<u8>,
  word_idx: StrLength,
  sink_node: Option<NodeId>,
}

impl TraversalContextData {

  pub fn new(
    mode: TraversalMode,
    node: Option<NodeId>,
    edges_to_scan: Vec<u32>,
    traversed_word: Vec<u8>,
    word_idx: StrLength,
    sink_node: Option<NodeId>,
  ) -> Self {

    return Self {
      mode,
      node,
      edges_to_scan,
      traversed_word,
      word_idx,
      sink_node,
    };
  }
}

pub struct FindPrefixContext {
  inner_data: TraversalContextData,
}

impl FindPrefixContext {
  pub fn new(inner_data: TraversalContextData) -> Self {
    return Self {
      inner_data,
    };
  }
}

impl TraversalContext for FindPrefixContext {
  fn mode(&self) -> TraversalMode {
    return self.inner_data.mode.clone();
  }

  fn node(&self) -> NodeId {
    return self.inner_data.node.unwrap();
  }

  fn get_edges_to_scan(&self) -> &Vec<u32> {
    return & self.inner_data.edges_to_scan;
  }

  fn get_mut_edges_to_scan(&mut self) -> &mut Vec<u32> {
    return &mut self.inner_data.edges_to_scan;
  }

  fn word_idx(&self) -> StrLength {
    return self.inner_data.word_idx;
  }

  fn traversed_word(&self) -> &[u8] {
    return self.inner_data.traversed_word.as_slice();
  }

  fn sink_node(&self) -> u32 {
    return self.inner_data.sink_node.unwrap();
  }
}

pub (in crate) struct FindPrefixTraverser<'a> {
  prefix_word: &'a str,
  dup_filter: ScalableCuckooFilter<u32>,
}

impl <'a> FindPrefixTraverser<'a> {
  pub fn new(prefix_word: &'a str) -> Self {

    return Self {
      prefix_word,
      dup_filter: ScalableCuckooFilter::new(10, 0.0000000000001),
    };
  }
}

impl <'a, Value> Traverser<FindPrefixContext, Value> for FindPrefixTraverser<'a> {

  fn setup(&mut self, context: &mut FindPrefixContext, traversal_contexts: &mut Vec<FindPrefixContext>, seadawg: &SeaDawgCore<Value>) {

    let word_idx = &context.word_idx();

    let word_first_char = get_codepoint_at(self.prefix_word, *word_idx as usize);
    let matching_edge_id_option = seadawg.nodes.get_to(&context.node(), &word_first_char);

    if matching_edge_id_option.is_none() {
      return;
    }

    let matching_edge_id = matching_edge_id_option.unwrap();

    let proposed_context_inner = TraversalContextData {
      mode: TraversalMode::Traversal,
      node: Some(context.node()),
      word_idx: word_idx.clone(),
      traversed_word: context.traversed_word().to_vec(),
      edges_to_scan: vec![matching_edge_id.clone()],
      sink_node: None,
    };

    let proposed_context = FindPrefixContext::new(proposed_context_inner);
    traversal_contexts.push(proposed_context);
  }

  fn traverse(
    &mut self,
    edge_id: &EdgeId,
    edge: &SeaEdge,
    context: &mut FindPrefixContext,
    traversal_contexts: &mut Vec<FindPrefixContext>,
    seadawg: &SeaDawgCore<Value>,
  ) {
    let word_idx = context.word_idx() as usize + 1;
    let prefix_bytes = self.prefix_word.as_bytes();
    let prefix_bytes_len = prefix_bytes.len();
    let mut traversed_word = slice_concat_byte(context.traversed_word(), &edge.label);
    let traversed_word_len = traversed_word.len();

    if traversed_word_len > prefix_bytes_len {

      if traversed_word[0..prefix_bytes_len].feq(&prefix_bytes[0..prefix_bytes_len]) {
        let proposed_context_inner = TraversalContextData {
          mode: TraversalMode::Collection,
          node: None,
          word_idx: word_idx as u32,
          traversed_word: context.traversed_word().to_vec(),
          edges_to_scan: vec![edge_id.clone()],
          sink_node: None,
        };

        let proposed_context = FindPrefixContext::new(proposed_context_inner);
        traversal_contexts.push(proposed_context);
      }
    } else if traversed_word_len == prefix_bytes_len {

      if traversed_word.feq(&prefix_bytes[0..traversed_word_len]) {

        let proposed_context_inner = TraversalContextData {
          mode: TraversalMode::Collection,
          node: None,
          word_idx: word_idx as u32,
          traversed_word: context.traversed_word().to_vec(),
          edges_to_scan: vec![edge_id.clone()],
          sink_node: None,
        };

        let proposed_context = FindPrefixContext::new(proposed_context_inner);
        traversal_contexts.push(proposed_context);

      }
    } else if traversed_word_len < prefix_bytes_len {

      let edge_dest = edge.dest;
      let word_first_char = prefix_bytes[word_idx as usize];
      let matching_edge_id_option = seadawg.nodes.get_to(&edge_dest, &word_first_char);

      if matching_edge_id_option.is_none() {
        return;
      }

      let matching_edge_id = matching_edge_id_option.unwrap();

      let proposed_context_inner = TraversalContextData {
        mode: TraversalMode::Traversal,
        node: Some(edge.dest),
        word_idx: word_idx as u32,
        traversed_word,
        edges_to_scan: vec![matching_edge_id.clone()],
        sink_node: None,
      };

      let proposed_context = FindPrefixContext::new(proposed_context_inner);
      traversal_contexts.push(proposed_context);
    }
  }

  fn collect(&mut self, edge_id: &EdgeId, edge: &SeaEdge, context: &FindPrefixContext, traversal_contexts: &mut Vec<FindPrefixContext>, seadawg: &SeaDawgCore<Value>) {

    let traversed_word = slice_concat_byte(context.traversed_word(), &edge.label);

    let node_id = &edge.dest;

    if !seadawg.sinks.has_no_ids(node_id) {

      for sink_id in seadawg.sinks.ids(node_id).iter() {
        if !self.dup_filter.contains(&sink_id) {

          self.dup_filter.insert(&sink_id);
        }

        let proposed_context_inner = TraversalContextData {
          mode: TraversalMode::Sink,
          node: None,
          word_idx: context.word_idx(), // This does not advance
          traversed_word: traversed_word.clone(),
          edges_to_scan: vec![*edge_id],
          sink_node: Some(*sink_id),
        };

        let proposed_context = FindPrefixContext::new(proposed_context_inner);
        traversal_contexts.push(proposed_context);
      }
    }

    let proposed_context_inner = TraversalContextData {
      mode: TraversalMode::Collection,
      node: Some(*node_id),
      word_idx: context.word_idx() + 1,
      traversed_word,
      edges_to_scan: seadawg.nodes.get_to_edges(node_id),
      sink_node: None,
    };

    let proposed_context = FindPrefixContext::new(proposed_context_inner);
    traversal_contexts.push(proposed_context);
  }

  fn should_accept_sink_node(&mut self, sink_node_id: &NodeId, current_word_idx: u32, word: &str, seadawg: &SeaDawgCore<Value>) -> bool {

    let sink_node = seadawg.sinks.get_sink(sink_node_id).unwrap();

    return word.len() == sink_node.length() as usize;
  }
}

pub struct FindSuperStringContext {
  inner_data: TraversalContextData,
  fall_through: bool,
}

impl FindSuperStringContext {
  pub fn new(inner_data: TraversalContextData, fall_through: bool) -> Self {
    return Self {
      inner_data,
      fall_through,
    };
  }
}

impl TraversalContext for FindSuperStringContext {
  fn mode(&self) -> TraversalMode {
    return self.inner_data.mode.clone();
  }

  fn node(&self) -> NodeId {
    return self.inner_data.node.unwrap();
  }

  fn get_edges_to_scan(&self) -> &Vec<u32> {
    return &self.inner_data.edges_to_scan;
  }

  fn get_mut_edges_to_scan(&mut self) -> &mut Vec<u32> {
    return &mut self.inner_data.edges_to_scan;
  }

  fn word_idx(&self) -> StrLength {
    return self.inner_data.word_idx;
  }

  fn traversed_word(&self) -> &[u8] {
    return self.inner_data.traversed_word.as_slice();
  }

  fn sink_node(&self) -> u32 {
    return self.inner_data.sink_node.unwrap();
  }
}

///
/// Super strings are strings that contain a substring.
/// In order for this to work, we need to traverse until getting to a sink node.
/// Once we have a sink node associated with the substring, we can backtrack using
/// the source node to reconstruct the original string.
pub (in crate) struct FindSuperStringTraverser<'a> {
  dup_filter: SeaDHashSet<u32>,
  needle: &'a str,
}

impl <'a> FindSuperStringTraverser<'a> {
  pub fn new(needle: &'a str) -> Self {
    return Self {
      dup_filter: new_hashset(),
      needle,
    };
  }
}

impl <'a, Value> Traverser<FindSuperStringContext, Value> for FindSuperStringTraverser<'a> {
  fn setup(&mut self, context: &mut FindSuperStringContext, traversal_contexts: &mut Vec<FindSuperStringContext>, seadawg: &SeaDawgCore<Value>) {

    let word_idx = &context.word_idx();
    let node_id = &context.node();

    let word_first_char = get_codepoint_at(self.needle, *word_idx as usize);
    let matching_edge_id_option = seadawg.nodes.get_to(node_id, &word_first_char);

    if matching_edge_id_option.is_none() {
      return;
    }

    let matching_edge_id = matching_edge_id_option.unwrap();

    let proposed_context_inner = TraversalContextData {
      mode: TraversalMode::Traversal,
      node: Some(context.node()),
      word_idx: word_idx.clone(),
      traversed_word: context.traversed_word().to_vec(),
      edges_to_scan: vec![matching_edge_id.clone()],
      sink_node: None,
    };

    let proposed_context = FindSuperStringContext::new(proposed_context_inner, false);
    traversal_contexts.push(proposed_context);
  }

  /// The idea is to then traverse to the sink.
  /// Once we have a sink, then initiate collection.
  fn traverse(&mut self, edge_id: &u32, edge: &SeaEdge, context: &mut FindSuperStringContext, traversal_contexts: &mut Vec<FindSuperStringContext>, seadawg: &SeaDawgCore<Value>) {

    let dest_node_id = &edge.dest;
    if context.fall_through {
      if !seadawg.sinks.has_no_ids(dest_node_id) {

        let proposed_context_inner = TraversalContextData {
          mode: TraversalMode::Collection,
          node: Some(edge.dest),
          word_idx: 0,
          traversed_word: vec![],
          edges_to_scan: vec![*edge_id], // Ignored
          sink_node: None,
        };

        let mut proposed_context = FindSuperStringContext::new(proposed_context_inner, context.fall_through);
        traversal_contexts.push(proposed_context);
      }

      if !seadawg.nodes.has_no_to_edges(dest_node_id) {
        let edge_partial = &edge.label;
        let word_idx = context.word_idx() + 1;

        let traversed_word = slice_concat_byte(context.traversed_word(), edge_partial);
        let proposed_context_inner = TraversalContextData {
          mode: TraversalMode::Traversal,
          node: Some(*dest_node_id),
          word_idx,
          traversed_word,
          edges_to_scan: seadawg.nodes.get_to_edges(dest_node_id),
          sink_node: None,
        };

        let proposed_context = FindSuperStringContext::new(proposed_context_inner, context.fall_through);
        traversal_contexts.push(proposed_context);
      }

      return;
    }

    let word_idx = context.word_idx() as usize + 1;
    let needle_bytes = self.needle.as_bytes();
    let needle_bytes_len = needle_bytes.len();
    let mut traversed_word = slice_concat_byte(context.traversed_word(), &edge.label);
    let traversed_word_len = traversed_word.len();

    if traversed_word_len > needle_bytes_len {

      if traversed_word[0..needle_bytes_len].feq(&needle_bytes[0..needle_bytes_len]) {

        let proposed_context_inner = TraversalContextData {
          mode: TraversalMode::Traversal,
          node: Some(context.node()),
          word_idx: word_idx as u32,
          traversed_word: context.traversed_word().to_vec(),
          edges_to_scan:  vec![*edge_id],
          sink_node: None,
        };

        let proposed_context = FindSuperStringContext::new(proposed_context_inner, true);
        traversal_contexts.push(proposed_context);

      }

    } else if traversed_word_len == needle_bytes_len {

      if traversed_word.feq(&needle_bytes[0..traversed_word_len]) {

        let proposed_context_inner = TraversalContextData {
          mode: TraversalMode::Traversal,
          node: Some(context.node()),
          word_idx: context.word_idx(),
          traversed_word,
          edges_to_scan: vec![*edge_id],
          sink_node: None,
        };

        let proposed_context = FindSuperStringContext::new(proposed_context_inner, true);
        traversal_contexts.push(proposed_context);

      }
    } else {

      let next_letter = &get_codepoint_at(self.needle, word_idx as usize);
      let to_edge_opt = seadawg.nodes.get_to(dest_node_id, next_letter);

      if to_edge_opt.is_none() {
        return;
      }

      let proposed_context_inner = TraversalContextData {
        mode: TraversalMode::Traversal,
        node: Some(edge.dest),
        word_idx: word_idx as u32,
        traversed_word,
        edges_to_scan: vec![*to_edge_opt.unwrap()],
        sink_node: None,
      };

      let proposed_context = FindSuperStringContext::new(proposed_context_inner, false);
      traversal_contexts.push(proposed_context);
    }
  }

  fn collect(&mut self, edge_id: &u32, edge: &SeaEdge, context: &FindSuperStringContext, traversal_contexts: &mut Vec<FindSuperStringContext>, seadawg: &SeaDawgCore<Value>) {

    let node_id = &edge.dest;

    for sink_id in seadawg.sinks.ids(node_id).iter() {

      if self.dup_filter.contains(sink_id) {
        continue;
      }

      self.dup_filter.insert(*sink_id);

      let edges_to_scan = vec![*edge_id];

      let sink = seadawg.sinks.get_sink(sink_id).unwrap();
      let proposed_context_inner = TraversalContextData {
        mode: TraversalMode::Sink,
        node: None,
        word_idx: 0,
        traversed_word: sink.word.as_bytes().to_vec(),
        edges_to_scan,
        sink_node: Some(*sink_id),
      };

      let mut proposed_context = FindSuperStringContext::new(proposed_context_inner, false);
      traversal_contexts.push(proposed_context);
    }
  }

  fn should_accept_sink_node(&mut self, sink_node_id: &NodeId, current_word_idx: u32, word: &str, seadawg: &SeaDawgCore<Value>) -> bool {

    return true;
    //return word.contains(self.needle);
  }
}

pub struct FindSuffixContext {
  inner_data: TraversalContextData,
}

impl FindSuffixContext {
  pub fn new(inner_data: TraversalContextData) -> Self {
    return Self {
      inner_data,
    };
  }
}

impl TraversalContext for FindSuffixContext {
  fn mode(&self) -> TraversalMode {
    return self.inner_data.mode.clone();
  }

  fn node(&self) -> NodeId {
    return self.inner_data.node.unwrap();
  }

  fn get_edges_to_scan(&self) -> &Vec<u32> {
    return & self.inner_data.edges_to_scan;
  }

  fn get_mut_edges_to_scan(&mut self) -> &mut Vec<u32> {
    return &mut self.inner_data.edges_to_scan;
  }

  fn word_idx(&self) -> StrLength {
    return self.inner_data.word_idx;
  }

  fn traversed_word(&self) -> &[u8] {
    return self.inner_data.traversed_word.as_slice();
  }

  fn sink_node(&self) -> u32 {
    return self.inner_data.sink_node.unwrap();
  }
}

///
/// Super strings are strings that contain a substring.
/// In order for this to work, we need to traverse until getting to a sink node.
/// Once we have a sink node associated with the substring, we can backtrack using
/// the source node to reconstruct the original string.
pub (in crate) struct FindSuffixTraverser<'a> {
  dup_filter: ScalableCuckooFilter<u32>,
  needle: &'a str,
}

impl <'a> FindSuffixTraverser<'a> {
  pub fn new(needle: &'a str) -> Self {
    return Self {
      dup_filter: ScalableCuckooFilter::new(10, 0.0000000000001),
      needle,
    };
  }
}

impl <'a, Value> Traverser<FindSuffixContext, Value> for FindSuffixTraverser<'a> {
  fn setup(&mut self, context: &mut FindSuffixContext, traversal_contexts: &mut Vec<FindSuffixContext>, seadawg: &SeaDawgCore<Value>) {

    let word_idx = &context.word_idx();
    let node_id = &context.node();

    let word_first_char = get_codepoint_at(self.needle, *word_idx as usize);
    let matching_edge_id_option = seadawg.nodes.get_to(node_id, &word_first_char);

    if matching_edge_id_option.is_none() {
      return;
    }

    let matching_edge_id = matching_edge_id_option.unwrap();

    let proposed_context_inner = TraversalContextData {
      mode: TraversalMode::Traversal,
      node: Some(context.node()),
      word_idx: word_idx.clone(),
      traversed_word: context.traversed_word().to_vec(),
      edges_to_scan: vec![matching_edge_id.clone()],
      sink_node: None,
    };

    let proposed_context = FindSuffixContext::new(proposed_context_inner);
    traversal_contexts.push(proposed_context);
  }

  /// This function never returns true because the initial forward edge selection process is done once
  /// since we will have a sufficient suffix. that prunes the search space.
  /// The idea is to then traverse to the sink.
  /// Once we have a sink, then initiate collection.
  fn traverse(&mut self, edge_id: &u32, edge: &SeaEdge, context: &mut FindSuffixContext, traversal_contexts: &mut Vec<FindSuffixContext>, seadawg: &SeaDawgCore<Value>) {

    let word_idx = context.word_idx() as usize + 1;
    let needle_bytes = self.needle.as_bytes();
    let needle_bytes_len = needle_bytes.len();
    let mut traversed_word = slice_concat_byte(context.traversed_word(), &edge.label);
    let traversed_word_len = traversed_word.len();

    if traversed_word_len > needle_bytes_len {

    } else if traversed_word_len == needle_bytes_len {

      if traversed_word.feq(&needle_bytes[0..traversed_word_len]) {

        let proposed_context_inner = TraversalContextData {
          mode: TraversalMode::Collection,
          node: None,
          word_idx: word_idx as u32,
          traversed_word,
          edges_to_scan: vec![edge_id.clone()],
          sink_node: None,
        };

        let proposed_context = FindSuffixContext::new(proposed_context_inner);
        traversal_contexts.push(proposed_context);

      }
    } else if traversed_word_len < needle_bytes_len {

      let edge_dest = edge.dest;
      let word_first_char = needle_bytes[word_idx as usize];
      let matching_edge_id_option = seadawg.nodes.get_to(&edge_dest, &word_first_char);

      if matching_edge_id_option.is_none() {
        return;
      }

      let matching_edge_id = matching_edge_id_option.unwrap();

      let proposed_context_inner = TraversalContextData {
        mode: TraversalMode::Traversal,
        node: Some(edge.dest),
        word_idx: word_idx as u32,
        traversed_word,
        edges_to_scan: vec![matching_edge_id.clone()],
        sink_node: None,
      };

      let proposed_context = FindSuffixContext::new(proposed_context_inner);
      traversal_contexts.push(proposed_context);
    }
  }

  fn collect(&mut self, edge_id: &u32, edge: &SeaEdge, context: &FindSuffixContext, traversal_contexts: &mut Vec<FindSuffixContext>, seadawg: &SeaDawgCore<Value>) {

    let node_id = &edge.dest;
    if seadawg.sinks.has_no_ids(node_id) {
      return;
    }

    let sinks = seadawg.sinks.ids(node_id);

    for sink_id in sinks.iter() {
      if !self.dup_filter.contains(&sink_id) {

        self.dup_filter.insert(&sink_id);
      }
      let edges_to_scan = vec![*edge_id];

      let sink = seadawg.sinks.get_sink(sink_id).unwrap();
      let proposed_context_inner = TraversalContextData {
        mode: TraversalMode::Sink,
        node: None,
        word_idx: 0,
        traversed_word: sink.word.as_bytes().to_vec(),
        edges_to_scan,
        sink_node: Some(*sink_id),
      };

      let mut proposed_context = FindSuffixContext::new(proposed_context_inner);
      traversal_contexts.push(proposed_context);
    }
  }

  fn should_accept_sink_node(&mut self, sink_node_id: &NodeId, current_word_idx: u32, word: &str, seadawg: &SeaDawgCore<Value>) -> bool {

    return true;
  }
}