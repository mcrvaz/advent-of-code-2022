use std::{
    cmp::Ordering,
    fmt::Display,
    fs::read_to_string,
    str::{Chars, Lines},
};

pub fn solve() {
    let result = internal_solve("src/days/day13/input.txt");
    println!("Result: {}", result);
}

fn internal_solve(path: &str) -> i32 {
    let content = read_to_string(path).expect("Fail to read file.");
    let mut iter = content.lines();
    let mut packets = Vec::new();
    while let Some((p1, p2)) = parse_packet_pair(&mut iter) {
        packets.push(p1);
        packets.push(p2);
        _ = iter.next();
    }
    let (div1, div2) = (parse_packet("[[2]]"), parse_packet("[[6]]"));
    packets.push(div1.clone());
    packets.push(div2.clone());
    packets.sort_unstable();
    let div1_index = packets.iter().position(|x| *x == div1).unwrap() + 1;
    let div2_index = packets.iter().position(|x| *x == div2).unwrap() + 1;
    (div1_index * div2_index) as i32
}

fn parse_packet_pair(iter: &mut Lines) -> Option<(Packet, Packet)> {
    let line1 = iter.next();
    let line2 = iter.next();
    if let (Some(l1), Some(l2)) = (line1, line2) {
        return Some((parse_packet(l1), parse_packet(l2)));
    }
    None
}

fn parse_packet(line: &str) -> Packet {
    parse_value(&mut line.chars()).pop().unwrap()
}

fn parse_value(iter: &mut Chars) -> Packet {
    let mut result = Packet::List(Vec::new());
    let mut current_number: Option<String> = None;
    while let Some(c) = iter.next() {
        if c == ',' {
            result.try_add_value(current_number);
            current_number = None;
        } else if c == '[' {
            let recursive_value = parse_value(iter);
            result.push(recursive_value);
        } else if c == ']' {
            result.try_add_value(current_number);
            break;
        } else {
            current_number = if let Some(mut copy) = current_number.clone() {
                copy.push(c);
                Some(copy)
            } else {
                Some(c.to_string())
            }
        }
    }
    result
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Packet {
    List(Vec<Packet>),
    Value(u32),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::List(l1), Packet::List(l2)) => l1.cmp(l2),
            (Packet::List(l1), Packet::Value(v2)) => l1.cmp(&vec![Packet::Value(*v2)]),
            (Packet::Value(v1), Packet::List(l2)) => vec![Packet::Value(*v1)].cmp(&l2),
            (Packet::Value(v1), Packet::Value(v2)) => v1.cmp(v2),
        }
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::List(l1) => {
                let mut result = String::new();
                for l in l1 {
                    let str = l.to_string();
                    result.push_str(&str);
                    result.push(',')
                }
                if let Some(r) = result.pop() {
                    if r != ',' {
                        result.push(r);
                    }
                }
                write!(f, "[{result}]")
            }
            Packet::Value(v) => write!(f, "{v}"),
        }
    }
}

impl Packet {
    fn push(&mut self, other: Self) {
        match self {
            Packet::List(l1) => l1.push(other),
            Packet::Value(_) => panic!("Trying to push to value!"),
        };
    }

    fn pop(&mut self) -> Option<Packet> {
        match self {
            Packet::List(l) => l.pop(),
            Packet::Value(_) => panic!("Trying to pop from value!"),
        }
    }

    fn try_add_value(&mut self, number: Option<String>) {
        if let Some(n) = number {
            let num = n.parse::<u32>().unwrap();
            match self {
                Packet::List(l) => l.push(Packet::Value(num)),
                _ => {}
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        const PATH: &str = "src/days/day13/test-input.txt";
        const EXPECTED: i32 = 140;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn result() {
        const PATH: &str = "src/days/day13/input.txt";
        const EXPECTED: i32 = 19261;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }
}
