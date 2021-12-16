#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use advent::prelude::*;
use byteorder::ReadBytesExt as _;
use std::io;

#[derive(Debug)]
struct Hex(Vec<u8>);

impl HasParser for Hex {
    #[into_parser]
    fn parser() -> _ {
        let hex_digit = || {
            choice((
                digit(),
                token('A'),
                token('B'),
                token('C'),
                token('D'),
                token('E'),
                token('F'),
            ))
        };
        let byte = (hex_digit(), hex_digit())
            .map(|(c1, c2)| u8::from_str_radix(&format!("{}{}", c1, c2), 16).unwrap());
        many1(byte).map(Self)
    }
}

struct BitReader<R> {
    buffer: u8,
    buffer_size: usize,

    reader: R,
    bits_read: usize,
}

impl<R> BitReader<R> {
    fn new(reader: R) -> Self {
        Self {
            buffer: 0,
            buffer_size: 0,
            reader,
            bits_read: 0,
        }
    }
}

impl<R: io::Read> BitReader<R> {
    fn read_bits(&mut self, bits_wanted: usize) -> io::Result<u8> {
        assert!(bits_wanted <= 8);
        if bits_wanted == 0 {
            return Ok(0);
        }

        if self.buffer_size == 0 {
            self.buffer = self.reader.read_u8()?;
            self.buffer_size = 8;
        }

        let mut res = 0;
        let mut bits_written = 0;
        while self.buffer_size > 0 && bits_written < bits_wanted {
            let left_shift = self.buffer_size - 1;
            let mask = 1 << left_shift;
            if self.buffer & mask != 0 {
                let left_shift = bits_wanted - bits_written - 1;
                res |= 1 << left_shift;
            }
            self.buffer_size -= 1;
            bits_written += 1;
            self.bits_read += 1;
        }
        Ok(res | self.read_bits(bits_wanted - bits_written)?)
    }

    fn assert_eof(&mut self) {
        let padding = if self.bits_read % 8 != 0 {
            8 - (self.bits_read % 8)
        } else {
            0
        };

        assert_eq!(self.buffer_size, padding);

        assert!(self.reader.read_u8().is_err());
    }
}

#[test]
fn bit_reader() {
    let mut br = BitReader::new(&[0b11001100, 0b11001100][..]);
    assert_eq!(br.read_bits(2).unwrap(), 0b11);
    assert_eq!(br.read_bits(2).unwrap(), 0b00);
    assert_eq!(br.read_bits(6).unwrap(), 0b110011);
    assert_eq!(br.read_bits(6).unwrap(), 0b001100);
}

#[derive(Debug)]
struct Literal(u64);

impl Literal {
    fn from_bits(r: &mut BitReader<impl io::Read>) -> io::Result<Self> {
        let mut value = 0u64;
        loop {
            let data = r.read_bits(5)?;
            let value_mask = 0b01111;
            value = (value << 4) | (data & value_mask) as u64;
            if data & !value_mask == 0 {
                break Ok(Self(value));
            }
        }
    }

    fn evaluate(&self) -> u64 {
        self.0
    }
}

#[derive(Debug)]
enum Operator {
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl Operator {
    fn from_id(id: u8) -> Self {
        match id {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Min,
            3 => Self::Max,
            5 => Self::GreaterThan,
            6 => Self::LessThan,
            7 => Self::EqualTo,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Operation {
    op: Operator,
    packets: Vec<Packet>,
}

impl Operation {
    fn from_bits(op: Operator, r: &mut BitReader<impl io::Read>) -> io::Result<Self> {
        let length_type = r.read_bits(1)?;
        let length_length = if length_type == 0 { 15 } else { 11 };

        let length = (r.read_bits(8)? as usize) << (length_length - 8)
            | r.read_bits(length_length - 8)? as usize;

        let mut packets = vec![];

        if length_type == 0 {
            let bits_read_before = r.bits_read;
            while (r.bits_read - bits_read_before) < length {
                packets.push(Packet::from_bits(r)?);
            }
        } else {
            for _ in 0..length {
                packets.push(Packet::from_bits(r)?);
            }
        }

        Ok(Self { op, packets })
    }

    fn visit_packets(&self, v: &mut dyn FnMut(&Packet)) {
        for p in &self.packets {
            p.visit_packets(v);
        }
    }

    fn evaluate(&self) -> u64 {
        let values: Vec<_> = self.packets.iter().map(|p| p.evaluate()).collect();
        match self.op {
            Operator::Sum => values.into_iter().sum(),
            Operator::Product => values.into_iter().product(),
            Operator::Min => values.into_iter().min().unwrap(),
            Operator::Max => values.into_iter().max().unwrap(),
            Operator::GreaterThan => (values[0] > values[1]) as u64,
            Operator::LessThan => (values[0] < values[1]) as u64,
            Operator::EqualTo => (values[0] == values[1]) as u64,
        }
    }
}

#[derive(Debug)]
enum PacketData {
    Literal(Literal),
    Operation(Operation),
}

impl PacketData {
    fn from_bits(r: &mut BitReader<impl io::Read>) -> io::Result<Self> {
        let ty = r.read_bits(3)?;
        match ty {
            4 => Ok(Self::Literal(Literal::from_bits(r)?)),
            id => Ok(Self::Operation(Operation::from_bits(
                Operator::from_id(id),
                r,
            )?)),
        }
    }

    fn visit_packets(&self, v: &mut dyn FnMut(&Packet)) {
        if let Self::Operation(op) = self {
            op.visit_packets(v);
        }
    }

    fn evaluate(&self) -> u64 {
        match self {
            Self::Literal(l) => l.evaluate(),
            Self::Operation(o) => o.evaluate(),
        }
    }
}

#[derive(Debug)]
struct Packet {
    version: u8,
    data: PacketData,
}

impl Packet {
    fn from_bits(r: &mut BitReader<impl io::Read>) -> io::Result<Self> {
        Ok(Self {
            version: r.read_bits(3)?,
            data: PacketData::from_bits(r)?,
        })
    }

    fn visit_packets(&self, v: &mut dyn FnMut(&Packet)) {
        v(self);
        self.data.visit_packets(v);
    }

    fn evaluate(&self) -> u64 {
        self.data.evaluate()
    }
}

#[part_one]
fn part_one(bytes: Hex) -> u64 {
    let mut r = BitReader::new(&bytes.0[..]);
    let p = Packet::from_bits(&mut r).unwrap();
    r.assert_eof();

    let mut version_sum = 0;
    p.visit_packets(&mut |p: &Packet| version_sum += p.version as u64);
    version_sum
}

#[part_two]
fn part_two(bytes: Hex) -> u64 {
    let mut r = BitReader::new(&bytes.0[..]);
    let p = Packet::from_bits(&mut r).unwrap();
    r.assert_eof();

    p.evaluate()
}

harness!();
