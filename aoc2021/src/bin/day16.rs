use aoc;

fn main() {
    let input = aoc::read_input(16);
    let input = input.trim_end();

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

// PART 1
fn part_1(hex: &str) -> u64 {
    let binary = hex_string_to_binary(hex).unwrap();
    let packet = BITSPacket::new(&binary);
    packet.sum_versions()
}

// PART 2
fn part_2(hex: &str) -> u64 {
    let binary = hex_string_to_binary(hex).unwrap();
    let packet = BITSPacket::new(&binary);
    packet.value()
}

// COMMON
fn hex_string_to_binary(hex: &str) -> Option<String> {
    hex.chars().map(hex_char_to_binary).collect()
}

fn hex_char_to_binary(hex: char) -> Option<&'static str> {
    match hex {
        '0' => Some("0000"),
        '1' => Some("0001"),
        '2' => Some("0010"),
        '3' => Some("0011"),
        '4' => Some("0100"),
        '5' => Some("0101"),
        '6' => Some("0110"),
        '7' => Some("0111"),
        '8' => Some("1000"),
        '9' => Some("1001"),
        'A' => Some("1010"),
        'B' => Some("1011"),
        'C' => Some("1100"),
        'D' => Some("1101"),
        'E' => Some("1110"),
        'F' => Some("1111"),
        _ => None,
    }
}

enum BITSPacket {
    LiteralPacket(LiteralPacket),
    OperatorPacket(OperatorPacket),
}

impl BITSPacket {
    fn new(binary: &str) -> BITSPacket {
        let version =
            u8::from_str_radix(&binary[0..3], 2).expect("Failed to parse packet version!");
        let type_id =
            u8::from_str_radix(&binary[3..6], 2).expect("Failed to parse packet type ID!");

        match type_id {
            4 => BITSPacket::LiteralPacket(LiteralPacket::new(binary, version)),
            _ => BITSPacket::OperatorPacket(OperatorPacket::new(binary, version, type_id)),
        }
    }

    fn size(&self) -> usize {
        match self {
            BITSPacket::LiteralPacket(packet) => packet.size,
            BITSPacket::OperatorPacket(packet) => packet.size,
        }
    }

    fn value(&self) -> u64 {
        match self {
            BITSPacket::LiteralPacket(packet) => packet.value,
            BITSPacket::OperatorPacket(packet) => packet.value(),
        }
    }

    fn sum_versions(&self) -> u64 {
        match self {
            BITSPacket::LiteralPacket(packet) => packet.version as u64,
            BITSPacket::OperatorPacket(packet) => {
                packet.version as u64
                    + packet
                        .subpackets
                        .iter()
                        .map(|subpacket| subpacket.sum_versions())
                        .sum::<u64>()
            }
        }
    }
}

struct LiteralPacket {
    version: u8,
    size: usize,
    value: u64,
}

impl LiteralPacket {
    fn new(binary: &str, version: u8) -> LiteralPacket {
        let mut value = 0;

        let mut i = 6;
        loop {
            value *= 16;
            value += u64::from_str_radix(&binary[i + 1..i + 5], 2)
                .expect("Failed to parse packet literal value!");

            i += 5;

            if &binary[i - 5..i - 4] == "0" {
                break;
            }
        }

        LiteralPacket {
            version,
            size: i,
            value,
        }
    }
}

struct OperatorPacket {
    version: u8,
    type_id: u8,
    size: usize,
    subpackets: Vec<BITSPacket>,
}

impl OperatorPacket {
    fn new(binary: &str, version: u8, type_id: u8) -> OperatorPacket {
        let mut subpackets: Vec<BITSPacket> = vec![];

        let size;
        if &binary[6..7] == "0" {
            // the next 15 bits are a number that represents the total length in bits of the sub-packets
            let length =
                usize::from_str_radix(&binary[7..22], 2).expect("Failed to parse packet length!");
            size = 22 + length;

            let mut i = 22;
            while i < size {
                let subpacket = BITSPacket::new(&binary[i..]);
                i += subpacket.size();
                subpackets.push(subpacket);
            }
        } else {
            // the next 11 bits are a number that represents the number of sub-packets immediately contained by this packet
            let count = usize::from_str_radix(&binary[7..18], 2)
                .expect("Failed to parse number of packets!");

            let mut i = 18;
            for _ in 0..count {
                let subpacket = BITSPacket::new(&binary[i..]);
                i += subpacket.size();
                subpackets.push(subpacket);
            }

            size = i;
        }

        OperatorPacket {
            version,
            type_id,
            size,
            subpackets,
        }
    }

    fn value(&self) -> u64 {
        match self.type_id {
            0 => self
                .subpackets
                .iter()
                .map(|subpacket| subpacket.value())
                .sum(),
            1 => self
                .subpackets
                .iter()
                .map(|subpacket| subpacket.value())
                .product(),
            2 => self
                .subpackets
                .iter()
                .map(|subpacket| subpacket.value())
                .min()
                .unwrap(),
            3 => self
                .subpackets
                .iter()
                .map(|subpacket| subpacket.value())
                .max()
                .unwrap(),
            5 => (self.subpackets[0].value() > self.subpackets[1].value()).into(),
            6 => (self.subpackets[0].value() < self.subpackets[1].value()).into(),
            7 => (self.subpackets[0].value() == self.subpackets[1].value()).into(),
            type_id => panic!("Unknown packet type ID {}!", type_id),
        }
    }
}
