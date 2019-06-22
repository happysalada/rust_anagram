pub mod hashmap {
  use std::collections::HashMap;
  use std::hash::{Hash, Hasher};
  #[derive(Debug)]
  pub struct CharFreq {
    pub hashmap: HashMap<char, i32>,
  }

  impl CharFreq {
    pub fn new(string: &str) -> CharFreq {
      CharFreq {
        hashmap: char_freq(string),
      }
    }

    pub fn to_string(&self) -> String {
      let mut chars = self.hashmap.keys().collect::<Vec<&char>>();
      chars.sort_by(|a, b| a.cmp(b));
      chars
        .iter()
        .map(|c| format!("{} {}", c, self.hashmap[c]))
        .collect::<String>()
    }

    pub fn contains(&self, other: &CharFreq) -> bool {
      for (k, v1) in &self.hashmap {
        match &other.hashmap.get(k) {
          Some(v2) => {
            if v1 < v2 {
              return false;
            }
          }
          None => {}
        }
      }
      true
    }
  }
  impl PartialEq for CharFreq {
    fn eq(&self, other: &CharFreq) -> bool {
      self.hashmap == other.hashmap
    }
  }
  impl Eq for CharFreq {}

  impl Hash for CharFreq {
    fn hash<H: Hasher>(&self, state: &mut H) {
      self.to_string().hash(state)
    }
  }

  fn char_freq(word: &str) -> HashMap<char, i32> {
    word
      .to_lowercase()
      .chars()
      // assumption here
      .filter(|&c| c.is_ascii_alphabetic())
      .fold(HashMap::new(), |mut hash_map, next_char| {
        let count = hash_map.entry(next_char).or_insert(0);
        *count += 1;
        hash_map
      })
  }
}

pub mod array {
  use std::hash::{Hash, Hasher};
  use std::ops::Add;

  #[derive(Debug, Clone)]
  pub struct CharFreq(pub [u8; 26]);

  impl CharFreq {
    pub fn new(string: &str) -> CharFreq {
      CharFreq(char_freq(string))
    }

    pub fn contains(&self, other: &CharFreq) -> bool {
      self.0.iter().zip(other.0.iter()).all(|(a, b)| a >= b)
    }

    pub fn add(&self, other: &CharFreq) -> CharFreq {
      let mut vec = [0; 26];
      for i in 0..self.0.len() {
        vec[i] = self.0[i] + other.0[i];
      }
      CharFreq(vec)
    }
  }

  impl PartialEq for CharFreq {
    fn eq(&self, other: &CharFreq) -> bool {
      self.0.iter().zip(other.0.iter()).all(|(a, b)| a == b)
    }
  }
  impl Eq for CharFreq {}

  impl Hash for CharFreq {
    fn hash<H: Hasher>(&self, state: &mut H) {
      self.0.hash(state);
    }
  }

  impl Add for CharFreq {
    type Output = Self;

    fn add(self, other: Self) -> Self {
      let mut vec = [0; 26];
      for i in 0..vec.len() {
        vec[i] = self.0[i] + other.0[i];
      }
      Self(vec)
    }
  }

  fn char_freq(word: &str) -> [u8; 26] {
    word
      .to_lowercase()
      .chars()
      // assumption here
      .filter(|&c| c.is_ascii_alphabetic())
      .fold([0; 26], |mut vec, next_char| {
        let index = next_char as usize - 97;
        vec[index] += 1;
        vec
      })
  }
}
