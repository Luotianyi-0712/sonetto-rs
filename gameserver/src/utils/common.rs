use crate::error::AppError;
use crate::packet::{ClientPacket, ServerPacket};
use sonettobuf::{CmdId, prost::Message};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[allow(dead_code)]
pub const YEAR_MS: u64 = 31_536_000_000;
#[allow(dead_code)]
pub const DAY_MS: u64 = 86_400_000;

#[allow(dead_code)]
pub async fn send_server_message<T: Message + Default>(
    socket: &mut TcpStream,
    cmd_id: CmdId,
    data: T,
    result_code: i16,
    up_tag: u8,
    down_tag: u8,
) -> Result<(), AppError> {
    let packet = ServerPacket {
        cmd_id: cmd_id as u16,
        result_code: result_code as u16,
        up_tag,
        down_tag,
        data: data.encode_to_vec(),
    };
    socket.write_all(&packet.encode()).await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn send_message<T: Message + Default>(
    _socket: &mut TcpStream,
    _cmd_id: CmdId,
    data: T,
    _result_code: i16,
) -> Result<(), AppError> {
    let _data = data.encode_to_vec();
    //send_raw_buffer(socket, cmd_id, data, result_code).await?;
    Ok(())
}

pub async fn send_raw_server_message(
    socket: &mut TcpStream,
    cmd_id: CmdId,
    payload: Vec<u8>,
    result_code: i16,
    up_tag: u8,
    down_tag: u8,
) -> Result<(), AppError> {
    let packet = ServerPacket {
        cmd_id: cmd_id as u16,
        result_code: result_code as u16,
        up_tag,
        down_tag,
        data: payload,
    };

    socket.write_all(&packet.encode()).await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn send_client_message<T: Message + Default>(
    socket: &mut TcpStream,
    cmd_id: CmdId,
    data: T,
    sequence: i32,
    up_tag: u8,
) -> Result<(), AppError> {
    let packet = ClientPacket {
        sequence,
        cmd_id: cmd_id as i16,
        up_tag,
        data: data.encode_to_vec(),
    };
    socket.write_all(&packet.encode()).await?;
    Ok(())
}

#[allow(dead_code)]
pub fn encode_message<T: prost::Message>(msg: &T) -> Result<Vec<u8>, AppError> {
    let mut buf = Vec::new();
    msg.encode(&mut buf)
        .map_err(|e| AppError::Custom(e.to_string()))?;
    Ok(buf)
}

#[allow(dead_code)]
pub struct ByteWriting;

#[allow(dead_code)]
impl ByteWriting {
    const VAR_0_1: usize = 256;

    pub fn read_string(arg_12_0: &[u8], arg_12_1: usize) -> Option<(usize, String)> {
        if arg_12_1 + 1 >= arg_12_0.len() {
            return None;
        }

        let var_12_0 = arg_12_0[arg_12_1] as usize;
        let var_12_1 = arg_12_0[arg_12_1 + 1] as usize;
        let var_12_2 = var_12_0 * Self::VAR_0_1 + var_12_1;

        if arg_12_1 + 2 + var_12_2 > arg_12_0.len() {
            return None;
        }

        match String::from_utf8(arg_12_0[(arg_12_1 + 2)..(arg_12_1 + 2 + var_12_2)].to_vec()) {
            Ok(var_12_3) => Some((arg_12_1 + 2 + var_12_2, var_12_3)),
            Err(_) => None,
        }
    }

    pub fn write_string(arg_15_0: &mut [u8], arg_15_1: &str, arg_15_2: usize) {
        let var_15_0 = arg_15_1.as_bytes();
        arg_15_0[arg_15_2] = (var_15_0.len() / Self::VAR_0_1) as u8;
        arg_15_0[arg_15_2 + 1] = (var_15_0.len() % Self::VAR_0_1) as u8;
        arg_15_0[(arg_15_2 + 2)..(arg_15_2 + 2 + var_15_0.len())].copy_from_slice(var_15_0)
    }
}
