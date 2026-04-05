use crate::utils::read_lines;

const FILENAME: &str = "src/day16/input.txt";

pub fn solve() -> Result<(), String> {
    let packet = packet_from_hex(&read_lines(FILENAME)[0]);

    let mut result = part1(&packet);
    println!("Part 1: {result}");

    result = part2(&packet);
    println!("Part 2: {result}");

    Ok(())
}

fn part1(packet: &Box<dyn Packet>) -> u64 {
    packet.get_version_sum()
}

fn part2(packet: &Box<dyn Packet>) -> u64 {
    packet.evaluate()
}

enum TypeId {
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum = 3,
    Literal = 4,
    Greater = 5,
    Less = 6,
    Equal = 7,
}

impl TypeId {
    fn from_binary(id: &str) -> Self {
        match isize::from_str_radix(id, 2).unwrap() {
            0 => TypeId::Sum,
            1 => TypeId::Product,
            2 => TypeId::Minimum,
            3 => TypeId::Maximum,
            4 => TypeId::Literal,
            5 => TypeId::Greater,
            6 => TypeId::Less,
            7 => TypeId::Equal,
            _ => panic!(),
        }
    }
}

enum LengthTypeId {
    Total = 0,
    Amount = 1,
}

impl LengthTypeId {
    fn from_binary(id: &str) -> Self {
        match isize::from_str_radix(id, 2).unwrap() {
            0 => LengthTypeId::Total,
            1 => LengthTypeId::Amount,
            _ => panic!(),
        }
    }
}

trait Packet {
    fn get_version_sum(&self) -> u64;
    fn evaluate(&self) -> u64;
}

struct Header {
    version: u8,
    type_id: TypeId,
}

struct LiteralPacket {
    header: Header,
    literal: u64,
}

impl LiteralPacket {
    fn create(header: Header, bits: &str) -> (Self, String) {
        let mut literal = 0;
        let mut offset = 0;
        let mut literal_bits = String::new();

        for chunk in bits.chars().collect::<Vec<char>>().chunks(5) {
            if chunk.starts_with(&['1']) {
                literal_bits.push_str(&chunk.iter().skip(1).collect::<String>());
                offset = offset + 5;
                continue;
            }

            literal_bits.push_str(&chunk.iter().skip(1).collect::<String>());
            offset = offset + 5;

            literal = u64::from_str_radix(&literal_bits, 2).unwrap();
            break;
        }

        (Self { header, literal }, bits[offset..].to_string())
    }
}

impl Packet for LiteralPacket {
    fn get_version_sum(&self) -> u64 {
        self.header.version as u64
    }

    fn evaluate(&self) -> u64 {
        self.literal
    }
}

struct OperatorPacket {
    header: Header,
    sub_packets: Vec<Box<dyn Packet>>,
}

impl OperatorPacket {
    fn create(header: Header, bits: &str) -> (Self, String) {
        let length_type_id = LengthTypeId::from_binary(&bits[0..1]);

        match length_type_id {
            LengthTypeId::Total => {
                let length = usize::from_str_radix(&bits[1..16], 2).unwrap();

                let (sub_packets, _) = packets_from_bits(&bits[16..(16 + length)], None);

                (
                    OperatorPacket {
                        header,
                        sub_packets,
                    },
                    bits[(16 + length)..].to_string(),
                )
            }
            LengthTypeId::Amount => {
                let amount = usize::from_str_radix(&bits[1..12], 2).unwrap();

                let (sub_packets, remaining) = packets_from_bits(&bits[12..], Some(amount));

                (
                    OperatorPacket {
                        header,
                        sub_packets,
                    },
                    remaining,
                )
            }
        }
    }
}

impl Packet for OperatorPacket {
    fn get_version_sum(&self) -> u64 {
        self.header.version as u64
            + self
                .sub_packets
                .iter()
                .fold(0, |acc, packet| acc + packet.get_version_sum())
    }

    fn evaluate(&self) -> u64 {
        let mut values = self.sub_packets.iter().map(|x| x.evaluate());

        match self.header.type_id {
            TypeId::Sum => values.sum(),
            TypeId::Product => values.product(),
            TypeId::Minimum => values.min().unwrap(),
            TypeId::Maximum => values.max().unwrap(),
            TypeId::Greater => (values.next().unwrap() > values.next().unwrap()) as u64,
            TypeId::Less => (values.next().unwrap() < values.next().unwrap()) as u64,
            TypeId::Equal => (values.next().unwrap() == values.next().unwrap()) as u64,
            _ => panic!(),
        }
    }
}

fn packet_from_hex(hex: &str) -> Box<dyn Packet> {
    let bits: String = hex
        .chars()
        .map(|x| match x {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => panic!(),
        })
        .collect();

    let version = u8::from_str_radix(&bits[0..3], 2).unwrap();
    let type_id = TypeId::from_binary(&bits[3..6]);

    let header = Header { version, type_id };

    match header.type_id {
        TypeId::Literal => Box::new(LiteralPacket::create(header, &bits[6..]).0),
        _ => Box::new(OperatorPacket::create(header, &bits[6..]).0),
    }
}

fn packets_from_bits(bits: &str, amount: Option<usize>) -> (Vec<Box<dyn Packet>>, String) {
    let mut remaining = bits.to_string();
    let mut result = vec![];
    let mut i = 0;

    while !remaining.is_empty() {
        i = i + 1;

        let version = u8::from_str_radix(&remaining[0..3], 2).unwrap();
        let type_id = TypeId::from_binary(&remaining[3..6]);

        let header = Header { version, type_id };

        let (packet, bits): (Box<dyn Packet>, String) = match header.type_id {
            TypeId::Literal => {
                let (p, b) = LiteralPacket::create(header, &remaining[6..]);
                (Box::new(p), b)
            }
            _ => {
                let (p, b) = OperatorPacket::create(header, &remaining[6..]);
                (Box::new(p), b)
            }
        };

        remaining = bits;
        result.push(packet);

        if amount.is_some_and(|x| x == i) {
            break;
        }
    }

    (result, remaining)
}
