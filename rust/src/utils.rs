
/// Utilities
pub (in crate) fn get_char_at(word: &str, start_idx: u32) -> char {
  let word_letter = word.chars().nth(start_idx as usize).unwrap();
  word_letter
}

pub (in crate) fn get_codepoint_at(word: &str, start_idx: usize) -> u8 {
  let codepoint = word.as_bytes()[start_idx];
  codepoint
}

pub (in crate) fn slice_concat_byte(byte_vec: &[u8], right: &u8) -> Vec<u8> {
  let mut new_vec = byte_vec.to_vec();
  new_vec.push(*right);

  return new_vec;
}

pub (in crate) fn slice_concat_bytes(byte_vec: &[u8], right: &[u8]) -> Vec<u8> {
  let mut new_vec = byte_vec.to_vec();
  new_vec.extend_from_slice(right);

  return new_vec;
}

pub (in crate) fn vec_concat_bytes(byte_vec: &Vec<u8>, right: &[u8]) -> Vec<u8> {
  let mut new_vec = byte_vec.clone();
  new_vec.extend_from_slice(right);

  return new_vec;
}