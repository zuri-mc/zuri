use crate::proto::io::{Readable, Reader, Writable, Writer};

#[derive(Debug, Clone, Copy, Default)]
pub struct VarU32(pub u32);

impl From<usize> for VarU32 {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}

impl From<VarU32> for usize {
    fn from(value: VarU32) -> Self {
        value.0 as usize
    }
}

impl From<u32> for VarU32 {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<VarU32> for u32 {
    fn from(value: VarU32) -> Self {
        value.0
    }
}

impl Writable for VarU32 {
    #[inline]
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.0);
    }
}

impl Readable<VarU32> for VarU32 {
    #[inline]
    fn read(reader: &mut Reader) -> VarU32 {
        Self(reader.var_u32())
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct VarU64(pub u64);

impl From<usize> for VarU64 {
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}

impl From<VarU64> for usize {
    fn from(value: VarU64) -> Self {
        value.0 as usize
    }
}

impl From<u64> for VarU64 {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<VarU64> for u64 {
    fn from(value: VarU64) -> Self {
        value.0
    }
}

impl Writable for VarU64 {
    #[inline]
    fn write(&self, writer: &mut Writer) {
        writer.var_u64(self.0);
    }
}

impl Readable<VarU64> for VarU64 {
    #[inline]
    fn read(reader: &mut Reader) -> VarU64 {
        Self(reader.var_u64())
    }
}


#[derive(Debug, Clone, Copy, Default)]
pub struct VarI32(pub i32);

impl From<usize> for VarI32 {
    fn from(value: usize) -> Self {
        Self(value as i32)
    }
}

impl From<VarI32> for usize {
    fn from(value: VarI32) -> Self {
        value.0 as usize
    }
}

impl From<i32> for VarI32 {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<VarI32> for i32 {
    fn from(value: VarI32) -> Self {
        value.0
    }
}

impl Writable for VarI32 {
    #[inline]
    fn write(&self, writer: &mut Writer) {
        writer.var_i32(self.0);
    }
}

impl Readable<VarI32> for VarI32 {
    #[inline]
    fn read(reader: &mut Reader) -> VarI32 {
        Self(reader.var_i32())
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct VarI64(pub i64);

impl From<usize> for VarI64 {
    fn from(value: usize) -> Self {
        Self(value as i64)
    }
}

impl From<VarI64> for usize {
    fn from(value: VarI64) -> Self {
        value.0 as usize
    }
}

impl From<i64> for VarI64 {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<VarI64> for i64 {
    fn from(value: VarI64) -> Self {
        value.0
    }
}

impl Writable for VarI64 {
    #[inline]
    fn write(&self, writer: &mut Writer) {
        writer.var_i64(self.0);
    }
}

impl Readable<VarI64> for VarI64 {
    #[inline]
    fn read(reader: &mut Reader) -> VarI64 {
        Self(reader.var_i64())
    }
}
