pub type Result<T> = core::result::Result<T, Box::<dyn std::error::Error>>;

pub struct Listener {
    parser: Packetizer
}

pub struct Connection {
    parser: Packetizer
}

pub struct Packetizer{}

impl Packetizer {
    pub fn write(&mut self, _: &mut [u8]) -> Result<usize> {
        todo!()
    }
} 

pub struct ConnectionRef{}

pub struct SendStream {
    conn_ref: ConnectionRef,
}

pub struct RecvStream {}

impl Listener {
    pub fn bind(addr: &str, config: ()) -> Result<Self> {
        todo!()
    }

    pub fn accept() -> Result<Connection> {
        todo!()
    }
}

impl Connection {
    pub fn connect(addr: &str, config:()) -> Result<Self> {
        todo!()
    }

    pub fn open_uni() -> Result<SendStream> {
        todo!()
    }

    pub fn open_bi() -> Result<SendStream> {
        todo!()
    }


    pub fn accept_uni() -> Result<RecvStream> {
        todo!()
    }
}

impl SendStream {
    pub fn write(&mut self, buf: &mut [u8]) -> Result<usize> {
        todo!()
    }
}

impl RecvStream {
    pub fn read(&self, buf: &[u8]) -> Result<usize> {
        todo!()
    }
}


