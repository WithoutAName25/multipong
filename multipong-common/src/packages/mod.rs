#[derive(Debug)]
pub struct BufferToSmallError;
#[derive(Debug)]
pub struct InvalidPacketError;

#[derive(Debug)]
pub struct LoginRequest {
    pub username: String,
}

pub trait Packet {
    fn packet_id() -> u8;
    fn write_data(&self, buffer: &mut [u8]) -> Result<usize, BufferToSmallError>;

    fn write_packet(&self, buffer: &mut [u8]) -> Result<usize, BufferToSmallError> {
        if buffer.len() <= 1 {
            return Err(BufferToSmallError);
        }
        buffer[0] = Self::packet_id();
        self.write_data(&mut buffer[1..]).map(|size| size + 1)
    }

    fn read_data(buffer: &[u8]) -> Result<Self, InvalidPacketError>
    where
        Self: Sized;
}

impl Packet for LoginRequest {
    fn packet_id() -> u8 {
        1
    }

    fn write_data(&self, buffer: &mut [u8]) -> Result<usize, BufferToSmallError> {
        let username = self.username.as_bytes();
        if buffer.len() < username.len() {
            return Err(BufferToSmallError);
        }
        buffer[..username.len()].copy_from_slice(username);
        Ok(username.len())
    }

    fn read_data(buffer: &[u8]) -> Result<LoginRequest, InvalidPacketError> {
        let username = String::from_utf8(buffer.to_vec()).map_err(|_| InvalidPacketError)?;
        Ok(LoginRequest { username })
    }
}
