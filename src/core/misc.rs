use std::collections::*;

use itertools::Itertools;


#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct NumberMap {
    pub items: BTreeMap<usize, Vec<Word>>,
}

impl NumberMap {
    pub fn bad_words_map() -> Self {
        NumberMap::new(include_str!("nicewords.txt").to_string())
    }

    pub fn new(text: String) -> Self {
        let groups = text
            .split_whitespace()
            .map(|w| Word::from(w.to_string()))
            .sorted_by_key(|x| x.score)
            .group_by(|x| x.score);

        let mut items = BTreeMap::new();

        for (score, words) in &groups {
            items.insert(score, words.into_iter().collect_vec());
        }

        Self { items }
    }

    pub fn solve(&self, num: usize) -> Option<Word> {
        let mut used = BTreeSet::<usize>::new();

        self.solve_rec(num, &mut used).map(|w| Word {
            score: num,
            text: w.into_iter().join(" "),
        })
    }

    fn solve_rec(&self, num: usize, mut used: &mut BTreeSet<usize>) -> Option<VecDeque<String>> {
        for (size, words) in self.items.range(1..=num).rev() {
            if used.insert(*size) {
                let rem = num - size;
                if rem == 0 {
                    return Some(VecDeque::from([words[0].text.clone()]));
                } else {
                    let r = self.solve_rec(rem, &mut used);
                    if let Some(mut right) = r {
                        right.push_front(words[0].text.clone());
                        return Some(right);
                    }
                }
            }
        }
        None
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone)]
pub struct Word {
    pub text: String,
    pub score: usize,
}

impl From<String> for Word {
    fn from(s: String) -> Self {
        let score = s
            .to_ascii_lowercase()
            .chars()
            .filter(|x| x.is_ascii_alphabetic())
            .map(|c| c as usize - ('a' as usize) + 1)
            .sum();

        Self { text: s, score }
    }
}
