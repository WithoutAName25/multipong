#[derive(Debug)]
pub struct SerializationError {
    pub message: String,
}

pub fn check_buffer_size(buffer: &[u8], size: usize) -> Result<(), SerializationError> {
    if buffer.len() < size {
        return Err(SerializationError {
            message: "Buffer too small".to_string(),
        });
    }
    Ok(())
}

pub fn serialize_flags<const L: usize>(
    buffer: &mut [u8],
    flags: &[&bool; L],
) -> Result<usize, SerializationError> {
    let len = L / 8 + 1;
    check_buffer_size(buffer, len)?;
    for i in 0..L {
        if *flags[i] {
            buffer[i / 8] |= 1 << (7 - i % 8);
        }
    }
    Ok(len)
}

pub fn deserialize_flags<const L: usize>(
    buffer: &[u8],
) -> Result<(usize, [bool; L]), SerializationError> {
    let len = L / 8 + 1;
    check_buffer_size(buffer, len)?;
    let mut flags = [false; L];
    for i in 0..L {
        flags[i] = buffer[i / 8] & (1 << (7 - i % 8)) != 0;
    }
    Ok((len, flags))
}

pub trait Serializable {
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, SerializationError>;
    fn deserialize(buffer: &[u8]) -> Result<(usize, Self), SerializationError>
    where
        Self: Sized;
}

impl Serializable for String {
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, SerializationError> {
        let bytes = self.as_bytes();
        let len = bytes.len();
        if len > u8::MAX as usize {
            return Err(SerializationError {
                message: "String too long".to_string(),
            });
        }
        check_buffer_size(buffer, len + 1)?;
        buffer[0] = len as u8;
        buffer[1..len + 1].copy_from_slice(bytes);
        Ok(len + 1)
    }

    fn deserialize(buffer: &[u8]) -> Result<(usize, String), SerializationError>
    where
        Self: Sized,
    {
        check_buffer_size(buffer, 1)?;
        let len = buffer[0] as usize;
        check_buffer_size(buffer, len + 1)?;
        let bytes = &buffer[1..len + 1];
        String::from_utf8(bytes.to_vec())
            .map(|string| (len + 1, string))
            .map_err(|_| SerializationError {
                message: "Invalid string".to_string(),
            })
    }
}
