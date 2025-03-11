#![doc = include_str!("../README.md")]

use shapely_core::Partial;
use std::convert::TryInto;

mod errors;
use errors::Error as DecodeError;

mod constants;
use constants::*;

#[cfg(test)]
mod tests;

pub fn from_msgpack<'input>(
    partial: &mut Partial,
    msgpack: &'input [u8],
) -> Result<(), DecodeError> {
    let mut decoder = Decoder::new(msgpack);

    fn deserialize_value(
        decoder: &mut Decoder,
        partial: &mut Partial,
    ) -> Result<(), DecodeError> {
        let shape_desc = partial.shape();
        let shape = shape_desc.get();

        match &shape.innards {
            shapely_core::Innards::Scalar(scalar) => {
                let slot = partial.scalar_slot().expect("Scalar slot");
                match scalar {
                    shapely_core::Scalar::String => {
                        let value = decoder.decode_string()?;
                        slot.fill(value);
                    }
                    shapely_core::Scalar::U64 => {
                        let value = decoder.decode_u64()?;
                        slot.fill(value);
                    }
                    _ => {
                        println!("Unsupported scalar type: {:?}", scalar);
                        todo!()
                    }
                }
            }
            shapely_core::Innards::Struct { .. } => {
                let map_len = decoder.decode_map_len()?;

                for _ in 0..map_len {
                    let key = decoder.decode_string()?;
                    let slot = partial
                        .slot_by_name(&key)
                        .map_err(|_| DecodeError::UnknownField(key))?;

                    let mut partial_field = Partial::alloc(slot.shape());
                    deserialize_value(decoder, &mut partial_field)?;
                    slot.fill_from_partial(partial_field);
                }
            }
            _ => {
                println!("Unsupported shape: {:?}", shape.innards);
                todo!()
            }
        }

        Ok(())
    }

    deserialize_value(&mut decoder, partial)
}

struct Decoder<'input> {
    input: &'input [u8],
    offset: usize,
}

impl<'input> Decoder<'input> {
    fn new(input: &'input [u8]) -> Self {
        Decoder { input, offset: 0 }
    }

    fn decode_u8(&mut self) -> Result<u8, DecodeError> {
        if self.offset >= self.input.len() {
            return Err(DecodeError::InsufficientData);
        }
        let value = self.input[self.offset];
        self.offset += 1;
        Ok(value)
    }

    fn decode_u16(&mut self) -> Result<u16, DecodeError> {
        if self.offset + 2 > self.input.len() {
            return Err(DecodeError::InsufficientData);
        }
        let value = u16::from_be_bytes(self.input[self.offset..self.offset + 2].try_into().unwrap());
        self.offset += 2;
        Ok(value)
    }

    fn decode_u32(&mut self) -> Result<u32, DecodeError> {
        if self.offset + 4 > self.input.len() {
            return Err(DecodeError::InsufficientData);
        }
        let value = u32::from_be_bytes(self.input[self.offset..self.offset + 4].try_into().unwrap());
        self.offset += 4;
        Ok(value)
    }

    fn decode_u64(&mut self) -> Result<u64, DecodeError> {
        match self.decode_u8()? {
            MSGPACK_UINT8 => Ok(self.decode_u8()? as u64),
            MSGPACK_UINT16 => Ok(self.decode_u16()? as u64),
            MSGPACK_UINT32 => Ok(self.decode_u32()? as u64),
            MSGPACK_UINT64 => {
                if self.offset + 8 > self.input.len() {
                    return Err(DecodeError::InsufficientData);
                }
                let value = u64::from_be_bytes(self.input[self.offset..self.offset + 8].try_into().unwrap());
                self.offset += 8;
                Ok(value)
            }
            prefix @ MSGPACK_POSFIXINT_MIN..=MSGPACK_POSFIXINT_MAX => Ok(prefix as u64),
            _ => Err(DecodeError::UnexpectedType),
        }
    }

    fn decode_string(&mut self) -> Result<String, DecodeError> {
        let prefix = self.decode_u8()?;

        let len = match prefix {
            prefix @ MSGPACK_FIXSTR_MIN..=MSGPACK_FIXSTR_MAX => (prefix & 0x1f) as usize,
            MSGPACK_STR8 => self.decode_u8()? as usize,
            MSGPACK_STR16 => self.decode_u16()? as usize,
            MSGPACK_STR32 => self.decode_u32()? as usize,
            _ => return Err(DecodeError::UnexpectedType),
        };

        if self.offset + len > self.input.len() {
            return Err(DecodeError::InsufficientData);
        }

        let value = String::from_utf8(self.input[self.offset..self.offset + len].to_vec())
            .map_err(|_| DecodeError::InvalidData)?;
        self.offset += len;
        Ok(value)
    }

    fn decode_map_len(&mut self) -> Result<usize, DecodeError> {
        let prefix = self.decode_u8()?;

        match prefix {
            prefix @ MSGPACK_FIXMAP_MIN..=MSGPACK_FIXMAP_MAX => Ok((prefix & 0x0f) as usize),
            MSGPACK_MAP16 => Ok(self.decode_u16()? as usize),
            MSGPACK_MAP32 => Ok(self.decode_u32()? as usize),
            _ => return Err(DecodeError::UnexpectedType),
        }
    }
}
