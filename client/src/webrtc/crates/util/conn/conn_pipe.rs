use super::*;

use std::io::{Error, ErrorKind};
use std::str::FromStr;
use tokio::sync::{mpsc, Mutex};

struct Pipe {
    rd_rx: Mutex<mpsc::Receiver<Vec<u8>>>,
    wr_tx: Mutex<mpsc::Sender<Vec<u8>>>,
}

#[async_trait]
impl Conn for Pipe {
    async fn connect(&self, _addr: SocketAddr) -> Result<()> {
        Err(Error::new(ErrorKind::Other, "Not applicable").into())
    }

    async fn recv(&self, b: &mut [u8]) -> Result<usize> {
        let mut rd_rx = self.rd_rx.lock().await;
        let v = match rd_rx.recv().await {
            Some(v) => v,
            None => return Err(Error::new(ErrorKind::UnexpectedEof, "Unexpected EOF").into()),
        };
        let l = std::cmp::min(v.len(), b.len());
        b[..l].copy_from_slice(&v[..l]);
        Ok(l)
    }

    async fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr)> {
        let n = self.recv(buf).await?;
        Ok((n, SocketAddr::from_str("0.0.0.0:0")?))
    }

    async fn send(&self, b: &[u8]) -> Result<usize> {
        let wr_tx = self.wr_tx.lock().await;
        match wr_tx.send(b.to_vec()).await {
            Ok(_) => {}
            Err(err) => return Err(Error::new(ErrorKind::Other, err.to_string()).into()),
        };
        Ok(b.len())
    }

    async fn send_to(&self, _buf: &[u8], _target: SocketAddr) -> Result<usize> {
        Err(Error::new(ErrorKind::Other, "Not applicable").into())
    }

    async fn local_addr(&self) -> Result<SocketAddr> {
        Err(Error::new(ErrorKind::AddrNotAvailable, "Addr Not Available").into())
    }

    async fn remote_addr(&self) -> Option<SocketAddr> {
        None
    }

    async fn close(&self) -> Result<()> {
        Ok(())
    }
}
