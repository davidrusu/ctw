#![feature(option_get_or_insert_default)]

// this allows doc-tests to run on our README.md
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

pub mod ctw_node;
pub mod ctw_tree;

pub use ctw_node::CtwNode;
pub use ctw_tree::CtwTree;
