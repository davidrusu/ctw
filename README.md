<a href="https://crates.io/crates/ctw"><img src="https://img.shields.io/crates/v/ctw.svg"></a>

# Context Tree Weighting (CTW)

CTW is a lightweight, practical and well performing sequence prediction algorithm discovered by Frans Willems, Yuri Shtarkov and Tjalling Tjalkens (1995).

It has good query and update performance (linear in the context length).

# Useage

The following example demonstrates CTW learning the altenating binary sequence 101010...

```rust
use ctw::CtwTree;
let mut rng = rand::thread_rng();
let mut tree = CtwTree::new(8); // context length is 8

let pattern = [true, false].into_iter().cycle().take(100); // true, false, true, ..
tree.update_batch(&Vec::from_iter(pattern));

let mut predictions = Vec::new();

for _ in 0..10 {
    let prediction = tree.sample(&mut rng);
    tree.update(prediction);

    // convert bool to 1 and 0 for legibility
    let bit = if prediction { 1 } else { 0 };
    predictions.push(bit);
}

println!("predictions: {predictions:?}"); // --> "prediction: [1, 0, 1, 0, 1, 0, 1, 0, 1, 0]"
assert_eq!(predictions, vec![1,0,1,0,1,0,1,0,1,0]);
```

# Resources

1. http://www.hutter1.net/publ/sctw.pdf
2. https://web.stanford.edu/class/ee378a/lecture-notes/lecture_9.pdf
3. http://www.data-compression.info/Algorithms/CTW/
4. https://www.cs.cmu.edu/~aarti/Class/10704_Spring15/CTW.pdf (original paper)
