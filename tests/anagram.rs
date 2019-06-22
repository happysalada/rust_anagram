extern crate anagram;
use anagram::*;
use std::collections::HashMap;

#[test]
fn test_contains() {
  let anagram = hashmap::CharFreq::new("poultry outwits ants");
  let word = hashmap::CharFreq::new("poultry");

  assert!(anagram.contains(&word));

  let anagram = array::CharFreq::new("poultry outwits ants");
  let word = array::CharFreq::new("poultry");

  assert!(anagram.contains(&word))
}

#[test]
fn test_instantiation() {
  let word = hashmap::CharFreq::new("poultry");
  let mut expected_hashmap = HashMap::new();
  expected_hashmap.insert('p', 1);
  expected_hashmap.insert('o', 1);
  expected_hashmap.insert('u', 1);
  expected_hashmap.insert('l', 1);
  expected_hashmap.insert('t', 1);
  expected_hashmap.insert('r', 1);
  expected_hashmap.insert('y', 1);
  let expected = hashmap::CharFreq {
    hashmap: expected_hashmap,
  };

  assert!(expected == word);

  let word = array::CharFreq::new("poultry");

  let expected = array::CharFreq([
    //[a,b,c,d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z]
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 0, 0, 1, 0,
  ]);

  assert!(expected == word);
}
