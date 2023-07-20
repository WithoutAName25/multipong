use crate::packets::Packet::{LoginRequest, LoginResponse};
use crate::serialization::{serialize_flags, Serializable, SerializationError, check_buffer_size, deserialize_flags};

#[derive(Debug)]
#[repr(u8)]
pub enum Packet {
    LoginRequest { username: String } = 1,
    LoginResponse { success: bool } = 2,
}

impl Packet {
    fn type_id(&self) -> u8 {
        unsafe { *(self as *const Self as *const u8) }
    }
}

impl Serializable for Packet {
    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, SerializationError> {
        check_buffer_size(buffer, 1)?;
        let mut offset = 0;
        buffer[offset] = self.type_id();
        offset += 1;
        match self {
            LoginRequest { username } => {
                offset += username.serialize(&mut buffer[offset..])?;
            }
            LoginResponse { success } => {
                offset += serialize_flags(&mut buffer[offset..], &[success])?
            }
        }
        Ok(offset)
    }

    fn deserialize(buffer: &[u8]) -> Result<(usize, Packet), SerializationError> {
        check_buffer_size(buffer, 1)?;
        let mut offset = 0;
        let type_id = buffer[offset];
        offset += 1;
        match type_id {
            1 => {
                let (size, username) = String::deserialize(&buffer[1..])?;
                offset += size;
                Ok(LoginRequest { username })
            }
            2 => {
                let (size, [success]) = deserialize_flags::<1>(&buffer[1..])?;
                offset += size;
                Ok(LoginResponse { success })
            }
            _ => Err(SerializationError {
                message: "Invalid packet type".to_string(),
            }),
        }.map(|packet| (offset, packet))
    }
}
