use std::{cmp::Reverse, collections::BinaryHeap};

use crate::bitstream::BitReader;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct OuterNode {
    freq: usize,
    inner: InnerNode,
}

#[derive(Debug)]
enum InnerNode {
    Leaf {
        operator_index: usize,
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

fn create_huffman_tree(operator_index_freq_table: &[usize]) -> Option<InnerNode> {
    let mut heap: BinaryHeap<Reverse<OuterNode>> = BinaryHeap::new();

    for (operator_index, &freq) in operator_index_freq_table.iter().enumerate() {
        if freq > 0 {
            heap.push(Reverse(OuterNode {
                freq,
                inner: InnerNode::Leaf { operator_index },
            }));
        }
    }

    while heap.len() > 1 {
        let Reverse(a) = heap.pop().unwrap();
        let Reverse(b) = heap.pop().unwrap();
        heap.push(Reverse(OuterNode {
            freq: a.freq + b.freq,
            inner: InnerNode::Internal {
                left: Box::new(a.inner),
                right: Box::new(b.inner),
            },
        }));
    }

    heap.pop().map(|Reverse(n)| n.inner)
}

fn create_operator_index_length_table(root: InnerNode, operator_count: usize) -> Vec<usize> {
    let mut operator_index_length_table = vec![0; operator_count];

    fn create_length_list_walk(node: &InnerNode, depth: usize, out: &mut [usize]) {
        match node {
            InnerNode::Leaf { operator_index } => out[*operator_index] = depth,
            InnerNode::Internal { left, right } => {
                create_length_list_walk(left, depth + 1, out);
                create_length_list_walk(right, depth + 1, out);
            }
        }
    }

    create_length_list_walk(&root, 0, &mut operator_index_length_table);
    operator_index_length_table
}

struct CanonicalTables {
    operator_index_code_table: Vec<u32>,
    // canonical 順序 (length 昇順、同 length なら index 昇順) に並んだ演算子インデックスの配列
    sorted_index_list: Vec<usize>,
    // ビット長 L を添字に取って、ビット長が L の最小符号値を引くテーブル
    length_first_operator_index_code_table: Vec<u32>,
    // ビット長 L を添字に取って、ビット長が L の符号値の個数を引くテーブル
    length_count_table: Vec<usize>,
    // ビット長 L を添字に取って、sorted_index_list 内のビット長 L の最小符号値に対応する演算子インデックスの一を引くテーブル
    length_first_position_in_sorted_index_list_table: Vec<usize>,
}

fn create_canonical_tables(operator_index_length_table: &[usize]) -> CanonicalTables {
    let operator_count = operator_index_length_table.len();

    let mut index_length_pairs: Vec<(usize, usize)> = operator_index_length_table
        .iter()
        .enumerate()
        .filter(|&(_, &length)| length > 0)
        .map(|(operator_index, &length)| (operator_index, length))
        .collect();
    index_length_pairs.sort_by_key(|&(index, length)| (length, index));

    let sorted_index_list: Vec<usize> = index_length_pairs
        .iter()
        .map(|&(operator_index, _)| operator_index)
        .collect();

    let max_length = index_length_pairs
        .last()
        .map(|&(_, length)| length)
        .unwrap_or(0);

    let mut operator_index_code_table = vec![0u32; operator_count];
    let mut length_first_operator_index_code_table = vec![0u32; max_length + 1];
    let mut length_first_position_in_sorted_index_list_table = vec![0usize; max_length + 1];

    let mut prev_value = 0u32;
    let mut prev_length = 0usize;
    for (position, &(operator_index, length)) in index_length_pairs.iter().enumerate() {
        if length != prev_length {
            if prev_length != 0 {
                prev_value = (prev_value + 1) << (length - prev_length);
            }
            length_first_operator_index_code_table[length] = prev_value;
            length_first_position_in_sorted_index_list_table[length] = position;
            prev_length = length;
        } else {
            prev_value += 1;
        }
        operator_index_code_table[operator_index] = prev_value;
    }

    let mut length_count_table = vec![0usize; max_length + 1];
    for &length in operator_index_length_table.iter() {
        if length > 0 {
            length_count_table[length] += 1;
        }
    }

    CanonicalTables {
        operator_index_code_table,
        sorted_index_list,
        length_first_operator_index_code_table,
        length_count_table,
        length_first_position_in_sorted_index_list_table,
    }
}

pub struct EncodeResult {
    pub encoded: Vec<(u32, usize)>,
    pub operator_index_length_table: Vec<usize>,
}

pub fn encode(operator_index_list: &[usize], operator_count: usize) -> EncodeResult {
    // repeatを表す擬似的なindex
    let repeat_index = operator_count;

    // 頻度表を構築する
    let mut operator_index_freq_table = vec![0usize; operator_count + 1];
    for &operator_index in operator_index_list {
        operator_index_freq_table[operator_index] += 1;
    }
    let nonzero_freq_list = operator_index_freq_table
        .iter()
        .copied()
        .filter(|freq| *freq > 0)
        .collect::<Vec<usize>>();
    let average_freq = nonzero_freq_list.iter().sum::<usize>() / nonzero_freq_list.len();
    operator_index_freq_table[repeat_index] = average_freq;

    let root = create_huffman_tree(&operator_index_freq_table).unwrap();
    let operator_index_length_table = create_operator_index_length_table(root, operator_count + 1);
    let canonical = create_canonical_tables(&operator_index_length_table);

    let flush = |encoded: &mut Vec<(u32, usize)>,
                 repeat_count_length: usize,
                 prev_op_index: usize,
                 code_count| {
        let prev_length = operator_index_length_table[prev_op_index];
        let repeat_length = operator_index_length_table[repeat_index];
        let repeat_cost = repeat_length + prev_length + repeat_count_length;
        let prev_code = canonical.operator_index_code_table[prev_op_index];

        if repeat_cost < prev_length * code_count {
            // REPEAT,CODE,N
            let repeat_code = canonical.operator_index_code_table[repeat_index];

            encoded.push((repeat_code, repeat_length));
            encoded.push((prev_code, prev_length));
            encoded.push((code_count as u32, repeat_count_length));
        } else {
            for _ in 0..code_count {
                encoded.push((prev_code, prev_length));
            }
        }
    };

    let mut encoded = vec![];
    let repeat_count_length = 4 as usize;
    let mut code_count = 1;
    let mut prev_operator_index = None;

    for operator_index in operator_index_list {
        if Some(*operator_index) == prev_operator_index
            && code_count < ((1 as usize) << repeat_count_length) - 1
        {
            code_count += 1;
            continue;
        }

        if let Some(prev_op_index) = prev_operator_index {
            flush(&mut encoded, repeat_count_length, prev_op_index, code_count);
        }

        prev_operator_index = Some(*operator_index);
        code_count = 1;
    }

    if let Some(prev_op_index) = prev_operator_index {
        flush(&mut encoded, repeat_count_length, prev_op_index, code_count);
    }

    EncodeResult {
        encoded,
        operator_index_length_table,
    }
}

pub fn decode(
    reader: &mut BitReader,
    operator_index_length_table: &[usize],
    operator_index_count: usize,
) -> Vec<usize> {
    let canonical = create_canonical_tables(operator_index_length_table);
    let max_length = canonical.length_count_table.len() - 1;
    let repeat_index = operator_index_length_table.len() - 1;
    let repeat_count_length = 4 as usize;

    let decode_one = |reader: &mut BitReader| -> Option<usize> {
        let mut value = 0u32;
        let mut length = 0usize;
        loop {
            let bit = reader.read_msb(1)?;
            value = (value << 1) + bit;
            length += 1;

            if length > max_length {
                return None;
            }

            if canonical.length_count_table[length] > 0
                && value >= canonical.length_first_operator_index_code_table[length]
            {
                let offset =
                    (value - canonical.length_first_operator_index_code_table[length]) as usize;
                if offset < canonical.length_count_table[length] {
                    let base = canonical.length_first_position_in_sorted_index_list_table[length];
                    return Some(canonical.sorted_index_list[base + offset]);
                }
            }
        }
    };

    let mut decoded = Vec::with_capacity(operator_index_count);

    while decoded.len() < operator_index_count {
        let Some(operator_index) = decode_one(reader) else {
            break;
        };

        if operator_index == repeat_index {
            // REPEAT,CODE,N
            let Some(repeated_operator_index) = decode_one(reader) else {
                break;
            };
            let code_count = reader.read_msb(repeat_count_length).unwrap() as usize;
            for _ in 0..code_count {
                decoded.push(repeated_operator_index);
            }
        } else {
            decoded.push(operator_index);
        }
    }

    decoded
}
