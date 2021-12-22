use std::str::FromStr;

struct Packet {
    version: u8,
    content: PacketContent,
}

enum PacketContent {
    Literal(usize),
    Operator(u8, Vec<Packet>),
}

fn decode_packet<I: Iterator<Item = u8>>(iter: &mut I) -> Result<Packet, String> {
    let version: u8 = iter.take(3).fold(0, |acc, x| (acc << 1) | x);
    let packet_type: u8 = iter.take(3).fold(0, |acc, x| (acc << 1) | x);
    if packet_type == 4 {
        let mut value: usize = 0;
        // Literal value decode
        let mut continuation_marker_bit = iter
            .next()
            .ok_or("First continuation marker bit not found for literal packet.")?;
        loop {
            value = (value << 4) | iter.take(4).fold(0, |acc, x| (acc << 1) | x) as usize;
            if continuation_marker_bit == 0 {
                break;
            } else {
                continuation_marker_bit = iter
                    .next()
                    .ok_or("Following continuation marker bit not found for literal packet.")?;
            }
        }

        Ok(Packet {
            version,
            content: PacketContent::Literal(value),
        })
    } else {
        let length_type_id: u8 = iter
            .next()
            .ok_or("Length type id not found for op packet.")?;

        if length_type_id == 1 {
            let subpacket_count: u16 = iter.take(11).fold(0, |acc, x| (acc << 1) | x as u16);
            let subpackets: Vec<Packet> = (0..subpacket_count)
                .map(|_| decode_packet(iter))
                .collect::<Result<Vec<Packet>, String>>()?;

            Ok(Packet {
                version,
                content: PacketContent::Operator(packet_type, subpackets),
            })
        } else {
            let subbit_count: u16 = iter.take(15).fold(0, |acc, x| (acc << 1) | x as u16);
            let subpacket_bits: Vec<u8> = iter.take(subbit_count as usize).collect();
            let mut subpacket_bit_iter = subpacket_bits.into_iter().peekable();
            let mut subpackets = Vec::new();
            while subpacket_bit_iter.peek().is_some() {
                subpackets.push(decode_packet(&mut subpacket_bit_iter)?);
            }

            Ok(Packet {
                version,
                content: PacketContent::Operator(packet_type, subpackets),
            })
        }
    }
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.chars().all(|c| c.is_digit(16)) {
            return Err(format!("Non-hexadecimal digits found in packet: \"{}\"", s));
        }
        let bits: Vec<u8> = s
            .chars()
            .map(|c| ((c.to_digit(16).unwrap()) as u8))
            .flat_map(|b| [(b >> 3) & 1, (b >> 2) & 1, (b >> 1) & 1, b & 1])
            .collect();

        decode_packet(&mut (bits.into_iter()))
    }
}

fn part1(p: &Packet) -> usize {
    match &p.content {
        PacketContent::Literal(_) => p.version as usize,
        PacketContent::Operator(_, subpackets) => {
            p.version as usize + subpackets.iter().map(part1).sum::<usize>()
        }
    }
}

fn main() {
    // Get filename from command line
    let filename: String = std::env::args().nth(1).expect("No filename provided.");
    // Read file
    let file: Packet = std::fs::read_to_string(filename)
        .expect("Could not read file.")
        .parse()
        .expect("Could not parse file.");

    println!("Part 1: {}", part1(&file));
}
