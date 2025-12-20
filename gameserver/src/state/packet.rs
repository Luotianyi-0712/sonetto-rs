use sonettobuf::CmdId;

/// A fully-formed packet waiting to be flushed
#[derive(Clone)]
pub enum CommandPacket {
    Reply {
        cmd_id: CmdId,
        body: Vec<u8>,
        result_code: i16,
        up_tag: u8,
        down_tag: u8,
    },
    Push {
        cmd_id: CmdId,
        body: Vec<u8>,
        down_tag: u8,
    },
}
