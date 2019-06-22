extern crate md5;
extern crate permutate;
use anagram::array::CharFreq;
use permutate::Permutator;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::time::Instant;

fn main() {
  let anagram = CharFreq::new("poultry outwits ants");

  let wordlist = fs::read_to_string("./wordlist").expect("file not found");
  let now = Instant::now();
  let filtered_space = wordlist
    .split('\n')
    .filter(|word| anagram.contains(&CharFreq::new(word)))
    // just filterwing words not contained leaves 61852
    // filtering by same charfreq with array leaves 1383
    .fold(HashMap::new(), |mut hash_map, word| {
      let char_freq = CharFreq::new(word);
      let word_set = hash_map.entry(char_freq).or_insert(HashSet::new());
      (*word_set).insert(word);
      hash_map
    });
  let seed: Vec<_> = filtered_space.keys().cloned().collect();
  println!("initial seed size {:#?}", seed.iter().count());
  println!("seed computation time in ms: {}", now.elapsed().as_millis());
  let now = Instant::now();

  let mut solutions = Vec::new();
  // return all anagrams of 3 and 4 word combinations
  for i in 1..seed.len() - 2 {
    for j in i + 1..seed.len() - 1 {
      let solution = seed[i].add(&seed[j]);
      if !anagram.contains(&solution) {
        continue;
      }
      for k in j + 1..seed.len() {
        let solution = solution.add(&seed[k]);
        if !anagram.contains(&solution) {
          continue;
        }
        if solution.contains(&anagram) {
          solutions.push(vec![i, j, k])
        }
        for l in k + 1..seed.len() {
          let solution = solution.add(&seed[l]);
          if !anagram.contains(&solution) {
            continue;
          }
          if solution.contains(&anagram) {
            solutions.push(vec![i, j, k, l])
          }
        }
      }
    }
  }

  println!("solutions count {:#?}", solutions.iter().count());
  let computation_time = now.elapsed().as_millis();
  println!(
    "solution computation time in ms: {},{}",
    computation_time / 1000,
    computation_time % 1000
  );
  let now = Instant::now();
  for vec in solutions.iter() {
    let ref_vec = vec.iter().map(|x| x).collect::<Vec<&usize>>();
    let slice = &[ref_vec.as_slice()];
    let mut permutator = Permutator::new(slice);
    if let Some(mut permutation) = permutator.next() {
      if permutation.len() == 3 {
        check_3_word_combinations(&permutation, &filtered_space, &seed)
      } else {
        check_4_word_combinations(&permutation, &filtered_space, &seed);
      }
      while permutator.next_with_buffer(&mut permutation) {
        if permutation.len() == 3 {
          check_3_word_combinations(&permutation, &filtered_space, &seed)
        } else {
          check_4_word_combinations(&permutation, &filtered_space, &seed);
        }
      }
    }
  }
  let computation_time = now.elapsed().as_millis();
  println!(
    "hash computation time in ms: {},{}",
    computation_time / 1000,
    computation_time % 1000
  );
}

fn check_3_word_combinations(
  permutation: &[&usize],
  filtered_space: &HashMap<anagram::array::CharFreq, std::collections::HashSet<&str>>,
  seed: &[anagram::array::CharFreq],
) {
  let easy_hash = "e4820b45d2277f3844eac66c903e84be";
  let medium_hash = "23170acc097c24edb98fc5488ab033fe";
  let first = &filtered_space[&seed[*permutation[0]]];
  let second = &filtered_space[&seed[*permutation[1]]];
  let third = &filtered_space[&seed[*permutation[2]]];
  for w1 in first.iter() {
    for w2 in second.iter() {
      for w3 in third.iter() {
        let phrase = format!("{} {} {}", w1, w2, w3);
        let hash = format!("{:x}", md5::compute(&phrase));
        if hash == easy_hash {
          println!("the easy phrase is: {}", phrase);
        }
        if hash == medium_hash {
          println!("the medium phrase is: {}", phrase);
        }
      }
    }
  }
}

fn check_4_word_combinations(
  permutation: &[&usize],
  filtered_space: &HashMap<anagram::array::CharFreq, std::collections::HashSet<&str>>,
  seed: &[anagram::array::CharFreq],
) {
  let hard_hash = "665e5bcb0c20062fe8abaaf4628bb154";
  let first = &filtered_space[&seed[*permutation[0]]];
  let second = &filtered_space[&seed[*permutation[1]]];
  let third = &filtered_space[&seed[*permutation[2]]];
  let fourth = &filtered_space[&seed[*permutation[3]]];
  for w1 in first.iter() {
    for w2 in second.iter() {
      for w3 in third.iter() {
        for w4 in fourth.iter() {
          let phrase = format!("{} {} {} {}", w1, w2, w3, w4);
          let hash = format!("{:x}", md5::compute(&phrase));
          if hash == hard_hash {
            println!("the hard phrase is: {}", phrase);
            return;
          }
        }
      }
    }
  }
}
