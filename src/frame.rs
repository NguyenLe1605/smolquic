pub(crate) enum Frame {
    Stream,
    ResetStream,
    StreamDataBlocked,
}

const FIN: u8 = 0;

impl Frame {
    pub fn encode(buf: &[u8]) -> Self {
        todo!()
    }

    pub fn decode(&self, buf: &mut [u8]) {
        todo!()
    }

    pub fn close_stream(&self, buf: &mut [u8]) {
        // set FIN bit
        self.decode(buf);
        buf[buf.len() - 1] = FIN;
        todo!()
    }

    pub fn is_fin(&self) -> bool {
        todo!()
    }

    pub fn is_reset_stream(&self) -> bool {
        match self {
            Frame::ResetStream => true,
            _ => false,
        }
    }
}
