// https://leetcode.com/problems/kth-largest-sum-in-a-binary-tree/
// 2024/10/25

use std::collections::HashMap;

#[derive(Default, Debug)]
struct TrieNode {
    is_end_of_dir: bool,
    children: HashMap<String, TrieNode>,
}

#[derive(Default, Debug)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, directory: &str) {
        let mut current_node = &mut self.root;

        for d in directory.split('/').skip(1) {
            current_node = current_node.children.entry(d.to_string()).or_default();

            if current_node.is_end_of_dir {
                return;
            }
        }
        current_node.is_end_of_dir = true;
    }

    pub fn contains(&self, directory: &str) -> bool {
        let mut current_node = &self.root;

        for d in directory.split('/').skip(1) {
            match current_node.children.get(d) {
                Some(node) => current_node = node,
                None => return false,
            }

            if current_node.is_end_of_dir {
                return true;
            }
        }

        current_node.is_end_of_dir
    }
}

impl Solution {
    pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
        let mut trie = Trie::new();
        let mut result: Vec<String> = Vec::with_capacity(folder.len());

        folder.sort_unstable();

        for d in folder {
            if !trie.contains(&d) {
                trie.insert(&d);
                result.push(d);
            }
        }

        result
    }
}

pub struct Solution;

fn main() {
    println!(
        "{:?}",
        Solution::remove_subfolders(
            ["/a", "/a/b", "/c/d", "/c/d/e", "/c/f"]
                .map(|e| e.to_string())
                .to_vec()
        )
    ); // ["/a","/c/d","/c/f"]

    println!(
        "{:?}",
        Solution::remove_subfolders(["/a", "/a/b/c", "/a/b/d"].map(|e| e.to_string()).to_vec())
    ); // ["/a"]

    println!(
        "{:?}",
        Solution::remove_subfolders(
            ["/a/b/c", "/a/b/ca", "/a/b/d"]
                .map(|e| e.to_string())
                .to_vec()
        )
    ); // ["/a/b/c","/a/b/ca","/a/b/d"]
}
