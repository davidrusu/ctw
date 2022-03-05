#![feature(option_get_or_insert_default)]

mod ctw_node;
mod ctw_tree;

use ctw_tree::CtwTree;

use crate::ctw_node::CtwNode;

fn main() {
    let mut rng = rand::thread_rng();
    let mut tree = CtwTree::new(16);
    tree.update_batch(&[
        true, false, true, false, true, false, true, false, true, false, true, false, true, false,
    ]);

    for _ in 0..10 {
        let prediction = tree.sample(&mut rng);
        print!("{}", if prediction { 1 } else { 0 });
        tree.update(prediction);
    }
    println!("");
}
