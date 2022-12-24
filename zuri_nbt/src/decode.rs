use bytes::{Buf, Bytes};
use crate::err::{NbtError, Res};

pub trait Reader {
    fn bool(&mut self) -> Res<bool>;
    fn u8(&mut self) -> Res<u8>;
    fn i16(&mut self) -> Res<i16>;
    fn i32(&mut self) -> Res<i32>;
    fn i64(&mut self) -> Res<i64>;
    fn f32(&mut self) -> Res<f32>;
    fn f64(&mut self) -> Res<f64>;

    fn end(&mut self) -> Res<()> {
        let t = self.u8()?;
        if t != 0 {
            return Err(NbtError::ParseError(format!("expected TAG_end, got {}", t)));
        }
        Ok(())
    }

    fn string(&mut self) -> Res<String> {
        let len = self.i16()?;
        if len < 0 {
            return Err(NbtError::ParseError("string length must be greater than 0".to_string()));
        }

        let mut buf = Vec::with_capacity(len as usize);
        for _ in 0..len {
            buf.push(self.u8()?);
        }

        Ok(String::from_utf8(buf)?)
    }

    fn u8_vec(&mut self) -> Res<Vec<u8>> {
        let len = self.i32()?;
        if len < 0 {
            return Err(NbtError::ParseError("vec length must be greater than 0".to_string()));
        }

        let mut buf = Vec::with_capacity(len as usize);
        for _ in 0..len {
            buf.push(self.u8()?);
        }

        Ok(buf)
    }

    fn i32_vec(&mut self) -> Res<Vec<i32>> {
        let len = self.i32()?;
        if len < 0 {
            return Err(NbtError::ParseError("vec length must be greater than 0".to_string()));
        }

        let mut buf = Vec::with_capacity(len as usize);
        for _ in 0..len {
            buf.push(self.i32()?);
        }

        Ok(buf)
    }

    fn i64_vec(&mut self) -> Res<Vec<i64>> {
        let len = self.i32()?;
        if len < 0 {
            return Err(NbtError::ParseError("vec length must be greater than 0".to_string()));
        }

        let mut buf = Vec::with_capacity(len as usize);
        for _ in 0..len {
            buf.push(self.i64()?);
        }

        Ok(buf)
    }
}

#[derive(Default)]
pub struct LittleEndianReader {
    buf: Bytes,
}

impl From<Bytes> for LittleEndianReader {
    fn from(value: Bytes) -> Self {
        Self { buf: value }
    }
}

impl Reader for LittleEndianReader {
    fn bool(&mut self) -> Res<bool> {
        Ok(self.buf.get_u8() != 0)
    }

    fn u8(&mut self) -> Res<u8> {
        Ok(self.buf.get_u8())
    }

    fn i16(&mut self) -> Res<i16> {
        Ok(self.buf.get_i16_le())
    }

    fn i32(&mut self) -> Res<i32> {
        Ok(self.buf.get_i32_le())
    }

    fn i64(&mut self) -> Res<i64> {
        Ok(self.buf.get_i64_le())
    }

    fn f32(&mut self) -> Res<f32> {
        Ok(self.buf.get_f32_le())
    }

    fn f64(&mut self) -> Res<f64> {
        Ok(self.buf.get_f64_le())
    }
}

#[derive(Default)]
pub struct NetworkLittleEndianReader {
    buf: Bytes,
}

impl From<Bytes> for NetworkLittleEndianReader {
    fn from(value: Bytes) -> Self {
        Self { buf: value }
    }
}

impl Reader for NetworkLittleEndianReader {
    fn bool(&mut self) -> Res<bool> {
        Ok(self.buf.get_u8() != 0)
    }

    fn u8(&mut self) -> Res<u8> {
        Ok(self.buf.get_u8())
    }

    fn i16(&mut self) -> Res<i16> {
        Ok(self.buf.get_i16_le())
    }

    fn i32(&mut self) -> Res<i32> {
        let mut v: u32 = 0;
        for i in (0..35).step_by(7) {
            let b = self.u8()?;

            v |= ((b & 0x7f) as u32) << i;
            if b & 0x80 == 0 {
                let x = (v >> 1) as i32;
                return Ok(if v & 1 != 0 {
                    -x
                } else {
                    x
                });
            }
        }
        Err(NbtError::ParseError("varint overflows integer".to_string()))
    }

    fn i64(&mut self) -> Res<i64> {
        let mut v: u64 = 0;
        for i in (0..70).step_by(7) {
            let b = self.u8()?;

            v |= ((b & 0x7f) as u64) << i;
            if b & 0x80 == 0 {
                let x = (v >> 1) as i64;
                return Ok(if v & 1 != 0 {
                    -x
                } else {
                    x
                });
            }
        }
        Err(NbtError::ParseError("varint overflows integer".to_string()))
    }

    fn f32(&mut self) -> Res<f32> {
        Ok(self.buf.get_f32_le())
    }

    fn f64(&mut self) -> Res<f64> {
        Ok(self.buf.get_f64_le())
    }

    fn string(&mut self) -> Res<String> {
        let len = 'var_len: {
            let mut v: u32 = 0;
            for i in (0..35).step_by(7) {
                let b = self.u8()?;

                v |= ((b & 0x7f) as u32) << i;
                if b & 0x80 == 0 {
                    break 'var_len v;
                }
            }
            return Err(NbtError::ParseError("varint overflows integer".to_string()));
        };

        let mut buf = Vec::with_capacity(len as usize);
        for _ in 0..len {
            buf.push(self.u8()?);
        }

        Ok(String::from_utf8(buf)?)
    }
}
