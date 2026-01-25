use crate::aoc_puzzle::AocPuzzle;
use std::fmt::Display;

pub fn silver(input: &str) -> Box<dyn Display> {
    let bits: String = input
        .chars()
        .map(|hex| to_binary(hex))
        .collect::<Vec<String>>()
        .join("");

    let packet = parse_packet_at(&bits, 0);
    Box::new(packet.packet_version_sum)
}

pub fn gold(input: &str) -> Box<dyn Display> {
    let bits: String = input
        .chars()
        .map(|hex| to_binary(hex))
        .collect::<Vec<String>>()
        .join("");

    let packet = parse_packet_at(&bits, 0);
    Box::new(packet.value)
}

pub fn puzzle() -> AocPuzzle {
    AocPuzzle::new(2021, 16, silver, gold)
}

fn to_binary(hex: char) -> String {
    let num = hex.to_digit(16).unwrap();
    format!("{:04b}", num)
}

struct Packet {
    value: i64,
    next_index: usize,
    packet_version_sum: i64,
}

fn parse_packet_at(s: &str, idx: usize) -> Packet {
    let mut idx = idx;

    let v = i64::from_str_radix(&s[idx..idx + 3], 2).unwrap();
    idx += 3;

    let t = i64::from_str_radix(&s[idx..idx + 3], 2).unwrap();
    idx += 3;

    let mut packet_version_sum = v;

    // Type 4: literal value
    if t == 4 {
        let mut binary_val = String::new();
        let mut last_group = false;
        while !last_group {
            last_group = &s[idx..idx + 1] == "0";
            binary_val.push_str(&s[idx + 1..idx + 5]);
            idx += 5;
        }
        return Packet {
            value: i64::from_str_radix(&binary_val, 2).unwrap(),
            next_index: idx,
            packet_version_sum,
        };
    }

    // Operator packet
    let mut sub_packets = Vec::new();
    let length_type_id = &s[idx..idx + 1];

    if length_type_id == "0" {
        // Total length in bits
        let mut l = usize::from_str_radix(&s[idx + 1..idx + 16], 2).unwrap();
        idx += 16;
        while l > 0 {
            let sp = parse_packet_at(s, idx);
            l = l - (sp.next_index - idx);
            idx = sp.next_index;
            packet_version_sum += sp.packet_version_sum;
            sub_packets.push(sp);
        }
    } else {
        // Number of sub-packets
        let mut l = usize::from_str_radix(&s[idx + 1..idx + 12], 2).unwrap();
        idx += 12;
        while l > 0 {
            let sp = parse_packet_at(s, idx);
            idx = sp.next_index;
            l -= 1;
            packet_version_sum += sp.packet_version_sum;
            sub_packets.push(sp);
        }
    }

    let packet_value = if sub_packets.len() == 1 {
        sub_packets[0].value
    } else if t == 0 {
        // Sum
        sub_packets.iter().map(|p| p.value).sum()
    } else if t == 1 {
        // Product
        sub_packets.iter().map(|p| p.value).product()
    } else if t == 2 {
        // Minimum
        sub_packets.iter().map(|p| p.value).min().unwrap()
    } else if t == 3 {
        // Maximum
        sub_packets.iter().map(|p| p.value).max().unwrap()
    } else if t == 5 {
        // Greater than
        if sub_packets[0].value > sub_packets[1].value {
            1
        } else {
            0
        }
    } else if t == 6 {
        // Less than
        if sub_packets[0].value < sub_packets[1].value {
            1
        } else {
            0
        }
    } else if t == 7 {
        // Equal to
        if sub_packets[0].value == sub_packets[1].value {
            1
        } else {
            0
        }
    } else {
        0
    };

    Packet {
        value: packet_value,
        next_index: idx,
        packet_version_sum,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn read_input(year: u16, day: u8) -> String {
        fs::read_to_string(format!("./input/{}_{:02}.txt", year, day))
            .unwrap()
            .trim_end()
            .to_string()
    }

    #[test]
    fn silver_test_input() {
        assert_eq!(silver("8A004A801A8002F478").to_string(), "16");
        assert_eq!(silver("620080001611562C8802118E34").to_string(), "12");
        assert_eq!(silver("C0015000016115A2E0802F182340").to_string(), "23");
        assert_eq!(silver("A0016C880162017C3686B18A3D4780").to_string(), "31");
    }

    #[test]
    fn silver_actual_input() {
        let input = read_input(2021, 16);
        assert_eq!(silver(&input).to_string(), "871");
    }

    #[test]
    fn gold_test_input() {
        assert_eq!(gold("C200B40A82").to_string(), "3");
        assert_eq!(gold("04005AC33890").to_string(), "54");
        assert_eq!(gold("880086C3E88112").to_string(), "7");
        assert_eq!(gold("CE00C43D881120").to_string(), "9");
        assert_eq!(gold("D8005AC2A8F0").to_string(), "1");
        assert_eq!(gold("F600BC2D8F").to_string(), "0");
        assert_eq!(gold("9C005AC2F8F0").to_string(), "0");
        assert_eq!(gold("9C0141080250320F1802104A08").to_string(), "1");
    }

    #[test]
    fn gold_actual_input() {
        let input = read_input(2021, 16);
        assert_eq!(gold(&input).to_string(), "68703010504");
    }
}
