use std::{cmp::Reverse, collections::BinaryHeap, ops::Index};

use crate::bitstream::BitReader;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct OuterNode {
    freq: usize,
    inner: InnerNode,
}

#[derive(Debug)]
enum InnerNode {
    Leaf {
        index: usize,
    },
    Internal {
        left: Box<InnerNode>,
        right: Box<InnerNode>,
    },
}

impl Ord for InnerNode {
    fn cmp(&self, _: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for InnerNode {
    fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ordering::Equal)
    }
}

impl Eq for InnerNode {}

impl PartialEq for InnerNode {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

fn create_huffman_tree(freq_list: &[usize]) -> BinaryHeap<Reverse<OuterNode>> {
    let mut huffman_tree = BinaryHeap::new();
    for (index, freq) in freq_list.iter().enumerate() {
        huffman_tree.push(Reverse(OuterNode {
            freq: *freq,
            inner: InnerNode::Leaf { index },
        }))
    }

    while huffman_tree.len() > 1 {
        let (Reverse(min_0), Reverse(min_1)) =
            (huffman_tree.pop().unwrap(), huffman_tree.pop().unwrap());

        let new = OuterNode {
            freq: min_0.freq + min_1.freq,
            inner: InnerNode::Internal {
                left: Box::new(min_0.inner),
                right: Box::new(min_1.inner),
            },
        };

        huffman_tree.push(Reverse(new));
    }

    huffman_tree
}

fn create_length_table(
    huffman_tree: BinaryHeap<Reverse<OuterNode>>,
    operator_count: usize,
) -> Vec<usize> {
    let mut length_table = Vec::new();
    length_table.resize(operator_count, 0);

    fn create_length_list_walk(node: &InnerNode, depth: usize, out: &mut [usize]) {
        match node {
            InnerNode::Leaf { index } => {
                out[*index] = depth;
            }
            InnerNode::Internal { left, right } => {
                create_length_list_walk(left.as_ref(), depth + 1, out);
                create_length_list_walk(right.as_ref(), depth + 1, out);
            }
        }
    }

    let mut huffman_tree = huffman_tree;
    let root = huffman_tree.pop().unwrap().0.inner;
    create_length_list_walk(&root, 0, &mut length_table);

    length_table
}

fn create_code_table(length_table: &[usize]) -> Vec<u32> {
    struct IndexLengthEntry {
        index: usize,
        length: usize,
    }

    impl Ord for IndexLengthEntry {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            if self.length == other.length {
                if self.index == other.index {
                    std::cmp::Ordering::Equal
                } else if self.index < other.index {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            } else if self.length < other.length {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        }
    }

    impl PartialOrd for IndexLengthEntry {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Eq for IndexLengthEntry {}

    impl PartialEq for IndexLengthEntry {
        fn eq(&self, other: &Self) -> bool {
            matches!(self.cmp(other), std::cmp::Ordering::Equal)
        }
    }

    pub struct IndexCodeEntry {
        pub index: usize,
        pub code: u32,
    }

    impl IndexCodeEntry {
        #[inline]
        fn new(index: usize, code: u32) -> Self {
            Self { index, code }
        }
    }
    let mut length_heap = BinaryHeap::new();

    for (index, length) in length_table.iter().enumerate() {
        if *length != 0 {
            length_heap.push(Reverse(IndexLengthEntry {
                index,
                length: *length,
            }));
        }
    }

    let mut code_table = Vec::new();
    let first_index_length = length_heap.pop().unwrap().0;
    code_table.push(IndexCodeEntry::new(first_index_length.index, 0));

    let mut prev_value = 0;
    let mut prev_length = first_index_length.length;
    while let Some(Reverse(index_length_entry)) = length_heap.pop() {
        if index_length_entry.length == prev_length {
            code_table.push(IndexCodeEntry::new(
                index_length_entry.index,
                prev_value + 1,
            ));
            prev_value += 1;
        } else {
            let value = (prev_value + 1) << (index_length_entry.length - prev_length);
            code_table.push(IndexCodeEntry::new(index_length_entry.index, value));
            prev_value = value;
            prev_length = index_length_entry.length;
        }
    }

    let mut indexable_code_table = vec![0; length_table.len()];
    for index_code_entry in code_table {
        indexable_code_table[index_code_entry.index] = index_code_entry.code;
    }

    indexable_code_table
}

pub fn encode(
    index_list: &[usize],
    freq_table: &[usize], // (index, freq)
    operator_count: usize,
) -> (Vec<(u32, usize)>, Vec<usize>, u32, u32, usize) {
    let huffman_tree = create_huffman_tree(freq_table);
    let length_table = create_length_table(huffman_tree, operator_count);
    let code_table = create_code_table(&length_table);

    let mut encoded = Vec::new();
    let mut max_index = 0;
    let mut max_length = 0;
    let mut index_count = 0; // length=0 じゃない index の数

    for length in length_table.iter() {
        if *length != 0 {
            index_count += 1;
        }
    }

    for index in index_list {
        let length = length_table[*index];
        let code = code_table[*index];
        encoded.push((code, length));
        max_index = (*index).max(max_index);
        max_length = length.max(max_length);
    }

    (
        encoded,
        length_table,
        max_index.ilog2() + 1,
        max_length.ilog2() + 1,
        index_count,
    )
}

struct DecodeHelperTables {
    pub length_first_code_table: Vec<u32>,
    pub length_count_table: Vec<usize>,
    pub length_first_index_table: Vec<usize>,
    pub sorted_index_list: Vec<usize>,
}

fn create_decode_helper_tables(index_length_table: &[usize]) -> DecodeHelperTables {
    let mut index_length_pairs = index_length_table
        .iter()
        .enumerate()
        .map(|(index, length)| (index, *length))
        .collect::<Vec<(usize, usize)>>();

    // canonicalルールでindexとlengthのペアをソートする
    index_length_pairs.sort_by(|(index_0, length_0), (index_1, length_1)| {
        if length_0 == length_1 {
            if index_0 == index_1 {
                std::cmp::Ordering::Equal
            } else if index_0 < index_1 {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        } else if length_0 < length_1 {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    let sorted_index_list = index_length_pairs
        .iter()
        .map(|(index, _)| *index)
        .collect::<Vec<usize>>();

    let max_length = index_length_pairs.last().unwrap().1;

    // 各lengthの最初のindex-lengthペアのリストを取得する
    let mut length_first_index_table = vec![0; max_length];
    // let mut first_index_length_list = vec![];
    let mut prev_length = 0;
    for (index, length) in index_length_pairs.iter() {
        if prev_length != *length {
            length_first_index_table[*length] = *index;
        } else {
            continue;
        }

        prev_length = *length;
    }

    let code_table = create_code_table(index_length_table);

    // 各lengthと対応する最初のcodeのテーブルを作成する
    let mut length_first_code_table = vec![0; max_length];
    for (length, index) in length_first_index_table.iter().enumerate() {
        length_first_code_table[length] = code_table[*index];
    }

    // 各lengthに対応するindexがいくつ存在するかを表すテーブルを作成する
    let mut length_count_table = vec![0; max_length];
    for (length, _) in length_first_index_table.iter().enumerate() {
        length_count_table[length] += 1;
    }

    DecodeHelperTables {
        length_first_code_table,
        length_count_table,
        length_first_index_table,
        sorted_index_list,
    }
}

pub fn decode(reader: &mut BitReader, index_length_table: &[usize]) {
    let helper_tables = create_decode_helper_tables(index_length_table);
}
