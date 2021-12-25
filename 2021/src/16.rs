use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct BitReader<'a> {
    bytes: &'a [u8],
    /// Position from the start of the slice, counted as bits instead of bytes
    position: u64,
    end_position: u64,
}

impl<'a> BitReader<'a> {
    /// Construct a new BitReader from a byte slice. The returned reader lives at most as long as
    /// the slice given to is valid.
    pub fn new(bytes: &'a [u8]) -> BitReader<'a> {
        BitReader {
            bytes,
            position: 0,
            end_position: bytes.len() as u64 * 8,
        }
    }

    pub fn relative_reader(&self, max: u64) -> BitReader<'a> {
        BitReader {
            bytes: self.bytes,
            position: self.position,
            end_position: self.position + max,
        }
    }

    pub fn skip(&mut self, bit_count: u64) {
        let end_position = self.position + bit_count;
        if end_position > self.end_position {
            panic!("NotEnoughData");
        }
        self.position = end_position;
    }

    /// Read at most 16 bits into a u16.
    pub fn read_u16(&mut self, bit_count: u8) -> Result<u16, ()> {
        let value = self.read_value(bit_count, 16)?;
        Ok((value & 0xffff) as u16)
    }

    fn read_value(&mut self, bit_count: u8, maximum_count: u8) -> Result<u64, ()> {
        if bit_count == 0 {
            return Ok(0);
        }
        if bit_count > maximum_count {
            panic!("too many bits");
        }
        let start_position = self.position;
        let end_position = self.position + bit_count as u64;
        if end_position > self.end_position {
            panic!("NotEnoughData");
        }

        let mut value: u64 = 0;

        for i in start_position..end_position {
            let byte_index = (i / 8) as usize;
            let byte = self.bytes[byte_index];
            let shift = 7 - (i % 8);
            let bit = (byte >> shift) as u64 & 1;
            value = (value << 1) | bit;
        }

        self.position = end_position;
        Ok(value)
    }

    /// Returns the number of bits not yet read from the underlying slice.
    pub fn remaining(&self) -> u64 {
        self.end_position - self.position
    }
}

fn read_packet(reader: &mut BitReader) -> Option<u16> {
    if reader.remaining() < 8 {
        // TODO What is the min packet size?
        // To catch the trailing zeros
        // TODO Check they are actually zeros
        return None;
    }

    let mut version = reader.read_u16(3).unwrap();
    let r#type = reader.read_u16(3).unwrap();

    match r#type {
        4 => {
            // literal
            let mut u = 0u64;
            loop {
                let group = reader.read_u16(5).unwrap();
                u = (u << 4) | (group & 0b1111) as u64;
                if (group & 0b10000) != 0b10000 {
                    // This was the last in the group
                    break;
                }
            }
        }
        _ => {
            let length_type = reader.read_u16(1).unwrap();
            match length_type {
                0 => {
                    // Read N bits
                    let len = reader.read_u16(15).unwrap();
                    assert!(u64::from(len) <= reader.remaining());

                    let mut sub_reader = reader.relative_reader(len.into());
                    reader.skip(len.into());

                    while sub_reader.remaining() > 0 {
                        version += read_packet(&mut sub_reader).unwrap();
                    }
                }
                1 => {
                    // Read N packets
                    let len = reader.read_u16(11).unwrap();
                    for _i in 0..len {
                        version += read_packet(reader).unwrap();
                    }
                }

                _ => panic!("unknown length_type {}", length_type),
            }
        }
    }

    Some(version)
}

fn part1(filename: &str) -> io::Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    let decoded = hex::decode(&lines[0]).expect("Decoding failed");
    let mut reader = BitReader::new(&decoded);

    let mut answer = 0i32;
    while let Some(version) = read_packet(&mut reader) {
        answer += version as i32;
    }

    Ok(answer)
}

fn read_packet2(reader: &mut BitReader) -> Option<u64> {
    if reader.remaining() < 8 {
        // TODO What is the min packet size?
        // To catch the trailing zeros
        // TODO Check they are actually zeros
        return None;
    }

    let _version = reader.read_u16(3).unwrap();
    let r#type = reader.read_u16(3).unwrap();

    let value = match r#type {
        4 => {
            // literal
            let mut u = 0u64;
            loop {
                let group = reader.read_u16(5).unwrap();
                u = (u << 4) | (group & 0b1111) as u64;
                if (group & 0b10000) != 0b10000 {
                    // This was the last in the group
                    break;
                }
            }
            u
        }
        _ => {
            let mut values = Vec::<u64>::new();
            let length_type = reader.read_u16(1).unwrap();
            match length_type {
                0 => {
                    // Read N bits
                    let len = reader.read_u16(15).unwrap();
                    assert!(u64::from(len) <= reader.remaining());

                    let mut sub_reader = reader.relative_reader(len.into());
                    reader.skip(len.into());

                    while sub_reader.remaining() > 0 {
                        values.push(read_packet2(&mut sub_reader).unwrap());
                    }
                }
                1 => {
                    // Read N packets
                    let len = reader.read_u16(11).unwrap();
                    for _i in 0..len {
                        values.push(read_packet2(reader).unwrap());
                    }
                }

                _ => panic!("unknown length_type {}", length_type),
            }

            match r#type {
                0 => values.iter().sum::<u64>(),
                1 => values.iter().product(),
                2 => *values.iter().min().unwrap(),
                3 => *values.iter().max().unwrap(),
                5 => {
                    assert!(values.len() == 2);
                    if values[0] > values[1] {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    assert!(values.len() == 2);
                    if values[0] < values[1] {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    assert!(values.len() == 2);
                    if values[0] == values[1] {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("unknown type"),
            }
        }
    };
    Some(value)
}

fn part2_str(line: &str) -> io::Result<u64> {
    let decoded = hex::decode(line).expect("Decoding failed");
    let mut reader = BitReader::new(&decoded);

    let answer = read_packet2(&mut reader).unwrap();
    Ok(answer)
}

fn part2(filename: &str) -> io::Result<u64> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    part2_str(&lines[0])
}

fn main() -> io::Result<()> {
    println!("Part 1: {}", part1("data/16.txt")?);
    println!("Part 2: {}", part2("data/16.txt")?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("data/16_test1.txt").unwrap(), 16);
        assert_eq!(part1("data/16_test2.txt").unwrap(), 12);
        assert_eq!(part1("data/16_test3.txt").unwrap(), 23);
        assert_eq!(part1("data/16_test4.txt").unwrap(), 31);

        assert_eq!(part1("data/16.txt").unwrap(), 934);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_str("C200B40A82").unwrap(), 3);
        assert_eq!(part2_str("04005AC33890").unwrap(), 54);
        assert_eq!(part2_str("880086C3E88112").unwrap(), 7);
        assert_eq!(part2_str("CE00C43D881120").unwrap(), 9);
        assert_eq!(part2_str("D8005AC2A8F0").unwrap(), 1);
        assert_eq!(part2_str("F600BC2D8F").unwrap(), 0);
        assert_eq!(part2_str("9C005AC2F8F0").unwrap(), 0);
        assert_eq!(part2_str("9C0141080250320F1802104A08").unwrap(), 1);

        assert_eq!(part2("data/16.txt").unwrap(), 912901337844);
    }
}
