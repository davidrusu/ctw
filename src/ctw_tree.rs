use std::collections::VecDeque;

use rand::Rng;

use crate::ctw_node::CtwNode;

#[derive(Debug, Clone)]
pub struct CtwTree {
    pub root: CtwNode,
    pub context: VecDeque<bool>,
    pub context_max: usize,
}

impl CtwTree {
    pub fn new(context_max: usize) -> Self {
        Self {
            root: CtwNode::default(),
            context: VecDeque::new(),
            context_max,
        }
    }

    pub fn update(&mut self, symbol: bool) {
        // TODO: change update to take an iterator
        let context = self.context.iter().copied().collect::<Vec<_>>();
        self.root.update(symbol, &context);

        self.context.push_front(symbol);
        if self.context.len() > self.context_max {
            self.context.pop_back();
        }
    }

    pub fn update_batch(&mut self, symbols: &[bool]) {
        // TODO: change update_batch to take an iterator
        for symbol in symbols.iter() {
            self.update(*symbol);
        }
    }

    pub fn sample(&mut self, rng: impl Rng) -> bool {
        // TODO: change update to take an iterator
        let context = self.context.iter().copied().collect::<Vec<_>>();
        self.root.sample(&context, rng)
    }

    pub fn clear_context(&mut self) {
        self.context.clear();
    }
}
