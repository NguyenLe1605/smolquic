use crate::Result;
use crate::frame::Frame;
use crate::varint::VarInt;

struct Assembler {}

pub struct RecvStream {
    assembler: Assembler,
    stream_id: VarInt,
    conn_ref: ConnectionRef,
}

pub struct ConnectionRef{}

impl ConnectionRef {
    pub fn send(&self) -> Result<usize> {
        todo!()
    }

    pub fn send_frame(&self, frame: Frame) -> Result<usize> {
        todo!()
    }

    pub fn get_mut_buf(&mut self) -> &mut [u8] {
        todo!()
    }
}

//     o
//     | Create Stream (Sending)
//     | Peer Creates Bidirectional Stream
//     v
// +-------+
// | Ready | Send RESET_STREAM
// |       |-----------------------.
// +-------+                       |
//     |                           |
//     | Send STREAM /             |
//     |      STREAM_DATA_BLOCKED  |
//     v                           |
// +-------+                       |
// | Send  | Send RESET_STREAM     |
// |       |---------------------->|
// +-------+                       |
//     |                           |
//     | Send STREAM + FIN         |
//     v                           v
// +-------+                   +-------+
// | Data  | Send RESET_STREAM | Reset |
// | Sent  |------------------>| Sent  |
// +-------+                   +-------+
//     |                           |
//     | Recv All ACKs             | Recv ACK
//     v                           v
// +-------+                   +-------+
// | Data  |                   | Reset |
// | Recvd |                   | Recvd |
// +-------+                   +-------+


enum SendState {
    Ready,
    Send,
    DataSent,
    ResetSent,
    DataRecvd,
    ResetRecvd,
}

pub struct SendStream {
    state: SendState,
    stream_id: VarInt,
    conn_ref: ConnectionRef,
}

impl SendStream {
    pub fn send(&mut self, buf: &[u8]) -> Result<usize> {
        let frame = Frame::encode(buf);
        match self.state {
            SendState::Ready => {
                match frame {
                    Frame::ResetStream => {
                        self.state = SendState::ResetSent;
                    }
                    Frame::Stream | Frame::StreamDataBlocked => {
                        self.state = SendState::Send;
                    }
                    _ => unimplemented!()
                }
            }
            SendState::Send => {
                match frame {
                    Frame::ResetStream => {
                        self.state = SendState::ResetSent;
                    }
                    Frame::Stream if frame.is_fin() => {
                        self.state = SendState::DataSent;
                    }
                    _ => unimplemented!()
                }
            }
            SendState::DataSent => {
                if frame.is_reset_stream() {
                    self.state = SendState::ResetSent;
                }
                self.state = SendState::DataRecvd
            }
            SendState::ResetSent => {
                self.state = SendState::ResetRecvd;
            }
            SendState::DataRecvd => {
                unimplemented!()
            }
            SendState::ResetRecvd => {
                unimplemented!()
            }
        }  
        self.conn_ref.send_frame(frame)
    }

    pub fn close(&mut self) -> Result<()> {
        let frame = Frame::Stream;
        let buf = self.conn_ref.get_mut_buf();
        frame.close_stream(buf);
        self.conn_ref.send()?;
        Ok(())
    }

    pub fn reset(&mut self) -> Result<()> {
        let frame = Frame::ResetStream;
        let buf = self.conn_ref.get_mut_buf();
        frame.decode(buf);
        self.conn_ref.send()?;
        Ok(())
    }
}

impl RecvStream {
    pub fn read(&self, buf: &[u8]) -> Result<usize> {
        todo!()
    }
}

