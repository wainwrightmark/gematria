use crate::core::prelude::*;
use crate::state::prelude::*;
use itertools::Itertools;
use num::ToPrimitive;
use serde::*;
use std::default;
use std::rc::Rc;
use yewdux::prelude::*;

#[derive(PartialEq, Eq, Store, Clone)]
pub struct MapState {
    pub map: NumberMap,
}

impl Default for MapState{
    fn default() -> Self {
        Self { map: NumberMap::bad_words_map() }
    }
}

#[derive(PartialEq, Eq, Store, Clone, Default)]
pub struct WordState{
    pub input : String,
    pub result: Option<Word>
}


pub struct ChangeInputMsg{
    pub input: String
}

impl Reducer<WordState> for ChangeInputMsg{
    fn apply(&self, state: Rc<WordState>) -> Rc<WordState> {
        let map = Dispatch::<MapState>::new().get();

        let score = Word::from(self.input.clone()).score;

        let result = map.map.solve(score);

        WordState{
            input: self.input.clone(),
            result
        }.into()
    }
}