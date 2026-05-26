use iroh::{Endpoint, EndpointAddr, endpoint::presets};
use n0_error::{Result, StdResultExt};

pub const ALPN: &[u8] = b"p2p-parquet/0";

pub async fn listen() -> Result<()> {
    println!("listen");
    Ok(())
}
pub async fn connect(addr: EndpointAddr) -> Result<()> {
    println!("connect");
    Ok(())
}