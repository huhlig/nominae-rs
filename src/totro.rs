//
// Copyright 2020 Hans W. Uhlig. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! Reimplementation of the Totro Name Generation Algorithm created by David A Wheeler.

use rand::Rng;

/// 0 dot-in-word
const NIW: u8 = 0b00000000;
/// 4 beginning-of-word
const BOW: u8 = 0b00000100;
/// 2 middle-of-word
const MOW: u8 = 0b00000010;
/// 1 end-of-word
const EOW: u8 = 0b00000001;
/// 6 beginning-middle-word
const BMW: u8 = BOW | MOW;
/// 5 beginning-end-word
const BEW: u8 = BOW | EOW;
/// 3 middle-end-word
const MEW: u8 = MOW | EOW;
/// 7 all-in-word
const AIW: u8 = BOW | MOW | EOW;

pub struct Totro;

impl Totro {
    pub fn generate<T: Rng>(min: u8, max: u8, rng: &mut T) -> String {
        let length = if min < max { rng.gen_range(min, max) } else { min } as usize;
        let mut output = String::with_capacity(length * 2);
        let mut vowel = rng.gen();
        for idx in 0..length {
            loop {
                let token = if vowel {
                    VOWELS.get(rng.gen::<usize>() % VOWELS.len()).unwrap()
                } else {
                    CONSONANTS.get(rng.gen::<usize>() % CONSONANTS.len()).unwrap()
                };
                if idx == 0 && ((token.1 & BOW) != BOW) {
                    continue;
                } else if idx == (length - 1) && ((token.1 & EOW) != EOW) {
                    continue;
                } else if (token.1 & MOW) != MOW {
                    continue;
                }
                vowel = !vowel;
                output.push_str(&token.0);
                break;
            }
        }
        output.get_mut(0..1).unwrap().make_ascii_uppercase();
        output
    }
}

const CONSONANTS: [(&str, u8); 91] = [
    // Letter Singles
    ("b", AIW), ("c", AIW), ("d", AIW), ("f", AIW),
    ("g", AIW), ("h", AIW), ("j", AIW), ("k", AIW),
    ("l", AIW), ("m", AIW), ("n", AIW), ("p", AIW),
    ("qu", BMW), ("r", AIW), ("s", AIW), ("t", AIW),
    ("v", AIW), ("w", AIW), ("x", AIW), ("y", AIW),
    ("z", AIW),
    ("sc", AIW),
    // Blends
    ("ch", AIW), ("gh", AIW), ("ph", AIW), ("sh", AIW),
    ("th", AIW), ("wh", BMW), ("ck", BEW), ("nk", BEW),
    ("rk", BEW), ("sk", AIW), ("wk", NIW),
    ("cl", BMW), ("fl", BMW), ("gl", BMW), ("kl", BMW),
    ("ll", BMW), ("pl", BMW), ("sl", BMW),
    ("br", BMW), ("cr", BMW), ("dr", BMW), ("fr", BMW),
    ("gr", BMW), ("kr", BMW), ("pr", BMW), ("sr", BMW),
    ("tr", BMW),
    ("ss", BEW),
    ("st", AIW),
    ("str", BMW),
    // More copies to increase frequency
    ("b", AIW), ("c", AIW), ("d", AIW), ("f", AIW),
    ("g", AIW), ("h", AIW), ("j", AIW), ("k", AIW),
    ("l", AIW), ("m", AIW), ("n", AIW), ("p", AIW),
    ("r", AIW), ("s", AIW), ("t", AIW), ("v", AIW),
    ("w", AIW), ("b", AIW), ("c", AIW), ("d", AIW),
    ("f", AIW), ("g", AIW), ("h", AIW), ("j", AIW),
    ("k", AIW), ("l", AIW), ("m", AIW), ("n", AIW),
    ("p", AIW), ("r", AIW), ("s", AIW), ("t", AIW),
    ("v", AIW), ("w", AIW), ("br", BMW), ("dr", BMW),
    ("fr", BMW), ("gr", BMW), ("kr", BMW),
];

const VOWELS: [(&str, u8); 83] = [
    ("a", AIW), ("e", AIW), ("i", AIW), ("o", AIW), ("u", AIW),
    ("a", AIW), ("e", AIW), ("i", AIW), ("o", AIW), ("u", AIW),
    ("a", AIW), ("e", AIW), ("i", AIW), ("o", AIW), ("u", AIW),
    ("a", AIW), ("e", AIW), ("i", AIW), ("o", AIW), ("u", AIW),
    ("a", AIW), ("e", AIW), ("i", AIW), ("o", AIW), ("u", AIW),
    ("a", AIW), ("e", AIW), ("i", AIW), ("o", AIW), ("u", AIW),
    ("a", AIW), ("e", AIW), ("i", AIW), ("o", AIW), ("u", AIW),
    ("a", AIW), ("e", AIW), ("i", AIW), ("o", AIW), ("u", AIW),
    ("a", AIW), ("e", AIW), ("i", AIW), ("o", AIW), ("u", AIW),
    ("a", AIW), ("e", AIW), ("i", AIW), ("o", AIW), ("u", AIW),
    ("a", AIW), ("e", AIW), ("i", AIW), ("o", AIW), ("u", AIW),
    ("a", AIW), ("e", AIW), ("i", AIW), ("o", AIW), ("u", AIW),
    // Vowel Blends
    ("aa", AIW), ("ae", AIW), ("ai", AIW), ("ao", AIW), ("au", AIW),
    ("ea", AIW), ("ee", AIW), ("ei", AIW), ("eo", AIW), ("eu", AIW),
    ("ia", AIW), ("ie", AIW), ("ii", AIW), ("io", AIW), ("iu", AIW),
    ("oa", AIW), ("oe", AIW), ("oi", AIW), ("oo", AIW), ("ou", AIW),
    ("eau", AIW), ("'", MEW), ("y", AIW),
];

#[cfg(test)]
mod tests {
    use super::Totro;

    #[test]
    fn it_works() {
        let mut rng = rand::thread_rng();
        for i in 4..10 {
            println!("3..{} - {}", i, Totro::generate(3, i, &mut rng));
        }
    }
}