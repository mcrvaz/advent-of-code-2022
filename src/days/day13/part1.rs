use std::{
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
        p1.value.print();
        println!();
        p2.value.print();
        println!();

        packets.push(p1);
        packets.push(p2);
        _ = iter.next();
    }
    1
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
    let mut result = PacketValue::None;
    result.push(parse_value(&mut line.chars()));
    Packet { value: result }
}

fn parse_value(iter: &mut Chars) -> PacketValue {
    let mut result = PacketValue::List(Vec::new());
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

struct Packet {
    value: PacketValue,
}

enum PacketValue {
    None,
    List(Vec<PacketValue>),
    Value(u32),
}

impl PacketValue {
    fn print(&self) {
        match self {
            PacketValue::List(l1) => {
                print!("[");
                for l in l1 {
                    l.print();
                }
                print!("]");
            }
            PacketValue::Value(v) => print!("{v},"),
            _ => {}
        };
    }

    fn push(&mut self, other: Self) {
        match self {
            PacketValue::List(l1) => l1.push(other),
            _ => panic!("Trying to push to value!"),
        };
    }
    fn pop(&mut self) -> Option<PacketValue> {
        match self {
            PacketValue::List(l) => l.pop(),
            _ => panic!("Trying to pop from value!"),
        }
    }
    fn try_add_value(&mut self, number: Option<String>) {
        if let Some(n) = number {
            let mut current = self.pop().unwrap();
            let num = n.parse::<u32>().unwrap();
            match current {
                PacketValue::List(ref mut l) => l.push(PacketValue::Value(num)),
                _ => {}
            };
            self.push(current);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        const PATH: &str = "src/days/day13/test-input.txt";
        const EXPECTED: i32 = 13;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn result() {
        const PATH: &str = "src/days/day13/input.txt";
        const EXPECTED: i32 = -1;
        let result = internal_solve(PATH);
        assert_eq!(result, EXPECTED);
    }
}
