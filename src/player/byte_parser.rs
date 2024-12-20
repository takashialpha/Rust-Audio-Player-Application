pub fn to_type_little_endian<T: SupportedType>(bytes: &[u8]) -> Vec<T> {
    let type_size = std::mem::size_of::<T>();
    let buffer_size = bytes.len() / type_size;

    let mut buffer = Vec::with_capacity(buffer_size);

    for i in (0..bytes.len()).step_by(type_size) {
        let value = T::from_le_bytes(&bytes[i..i + type_size]);
        buffer.push(value);
    }

    buffer
}

pub trait SupportedType {
    fn from_le_bytes(bytes: &[u8]) -> Self;
}

macro_rules! impl_supported_type {
    ($($T:ty)*) => {
        $(
            impl SupportedType for $T {
                fn from_le_bytes(bytes: &[u8]) -> Self {
                    Self::from_le_bytes(bytes.try_into().unwrap())
                }
            }
        )*
    }
}

impl_supported_type!(u8 u16 u32 u64 i8 i16 i32 i64 f32 f64);
