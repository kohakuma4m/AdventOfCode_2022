use itertools::Itertools;
use serde::Deserialize;
use std::{cmp::Ordering, fmt::Display};

pub fn solution1(data: String) -> i32 {
    let packets = read_packets(data);

    #[cfg(test)]
    {
        // Debug logs...
        for p in packets.iter() {
            println!("Packet --> {}", p);
        }
    }

    // Counting packets in correct order
    let mut chunk_idx = 0;
    let mut right_order_chunk_indices = vec![];
    for chunk in &packets.into_iter().chunks(2) {
        let (left, right) = chunk.collect_tuple().unwrap();

        if let Some(true) = is_right_order_pair(&left.data, &right.data) {
            right_order_chunk_indices.push(chunk_idx + 1);
        }

        chunk_idx += 1;
    }

    let result = right_order_chunk_indices.iter().sum();

    println!("=========================");
    println!("Indices: {:?}", right_order_chunk_indices);
    println!("=========================");
    println!("Solution1: {result}");
    println!("=========================");

    result
}

pub fn solution2(data: String) -> usize {
    let mut packets = read_packets(data);

    // Adding packet dividers
    packets.push(Packet { data: vec![PacketData::List(Box::new(vec![PacketData::Value(2)]))] });
    packets.push(Packet { data: vec![PacketData::List(Box::new(vec![PacketData::Value(6)]))] });

    // Sorting packets
    let sorted_packets: Vec<Packet> = packets.into_iter().sorted().collect();

    let divider_indices = sorted_packets
        .iter()
        .enumerate()
        .map(|(idx, packet)| (idx, packet))
        .filter(|(_, packet)| {
            if packet.data.len() != 1 {
                return false;
            }

            return match &packet.data[0] {
                PacketData::List(list) => {
                    if list.len() != 1 {
                        return false;
                    }

                    match &list[0] {
                        PacketData::Value(2) | PacketData::Value(6) => true,
                        _ => false
                    }
                },
                _ => false
            };
        })
        .map(|(idx, _)| idx + 1)
        .collect::<Vec<usize>>();

    let result = divider_indices.iter().product();

    #[cfg(test)]
    {
        // Debug logs...
        for p in sorted_packets.iter() {
            println!("Packet --> {}", p);
        }
    }

    println!("=========================");
    for (idx, indice) in divider_indices.iter().enumerate() {
        println!("Divider #{} at position {indice} --> {}", idx + 1, &sorted_packets[indice - 1]);
    }
    println!("=========================");
    println!("Solution2: {result}");
    println!("=========================");

    result
}

/////////////////////////////////////////////////

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
enum PacketData {
    Value(i32),
    List(Box<Vec<PacketData>>)
}

impl Display for PacketData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            PacketData::Value(value) => format!("{}", value),
            PacketData::List(list) => format!("[{}]", list.iter().map(|data| format!("{}", data)).join(","))
        };

        write!(f, "{}", content)
    }
}

#[derive(Debug)]
struct Packet {
    data: Vec<PacketData>
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.data.iter().join(","))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match is_right_order_pair(&self.data, &other.data) {
            Some(true) => Ordering::Less,     // Left is first, in right order
            Some(false) => Ordering::Greater, // Left should be right
            None => Ordering::Equal
        }
    }

    fn max(self, other: Self) -> Self {
        match self.cmp(&other) {
            Ordering::Less | Ordering::Equal => other,
            Ordering::Greater => self
        }
    }

    fn min(self, other: Self) -> Self {
        match self.cmp(&other) {
            Ordering::Less | Ordering::Equal => self,
            Ordering::Greater => other
        }
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        assert!(min <= max);
        if self < min {
            min
        }
        else if self > max {
            max
        }
        else {
            self
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Packet {}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        if self.data.len() != other.data.len() {
            return false;
        }

        for i in 0..self.data.len() {
            if self.data[i] != other.data[i] {
                return false;
            }
        }

        true
    }
}

fn read_packets(data: String) -> Vec<Packet> {
    data.lines()
        .filter(|l| *l != "") // Removing empty lines
        .map(|line| {
            let data: Vec<PacketData> = serde_json::from_str(&line).unwrap();
            Packet { data }
        })
        .collect()
}

fn is_right_order_pair(left: &Vec<PacketData>, right: &Vec<PacketData>) -> Option<bool> {
    for (idx, data_left) in left.iter().enumerate() {
        let data_right = match right.get(idx) {
            Some(d) => d,
            None => return Some(false) // Right list ran out of items first
        };

        match (data_left, data_right) {
            // Both are integer
            (PacketData::Value(value_left), PacketData::Value(value_right)) => {
                if value_left != value_right {
                    return Some(value_left < value_right); // Left values must always come first
                }
            },
            // Both are lists
            (PacketData::List(list_left), PacketData::List(list_right)) => {
                match is_right_order_pair(list_left, list_right) {
                    Some(b) => return Some(b), // Comparison result
                    None => continue
                }
            },
            // Left is an integer and right is a list
            (PacketData::Value(value_left), PacketData::List(list_right)) => {
                let list_left = &vec![PacketData::Value(value_left.clone())];
                match is_right_order_pair(list_left, list_right) {
                    Some(b) => return Some(b), // Comparison result
                    None => continue
                }
            },
            // Left is a list and right is an integer
            (PacketData::List(list_left), PacketData::Value(value_right)) => {
                let list_right = &vec![PacketData::Value(value_right.clone())];
                match is_right_order_pair(list_left, list_right) {
                    Some(b) => return Some(b), // Comparison result
                    None => continue
                }
            }
        }
    }

    if left.len() < right.len() {
        return Some(true); // Left list ran out of items first
    }

    None
}

/////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    fn read_test_file() -> String {
        let current_file = std::file!();
        let test_file = current_file.replace("mod.rs", "test.txt");
        return crate::read_file(&test_file).unwrap();
    }

    #[test]
    fn test_solution1() {
        let data = read_test_file();
        assert_eq!(13 + 9, solution1(data));
    }

    #[test]
    fn test_solution2() {
        let data = read_test_file();
        assert_eq!((10 + 2) * (14 + 4), solution2(data));
    }
}
