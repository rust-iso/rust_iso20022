use futures::{SinkExt, StreamExt, future};
use rmcp::{
    RoleServer, ServiceExt,
    service::{RxJsonRpcMessage, TxJsonRpcMessage},
    transport::async_rw::JsonRpcMessageCodec,
};
use tokio_util::codec::{FramedRead, FramedWrite};

const MAX_REQUEST_BYTES: usize = rust_iso20022::ParseLimits::DEFAULT.max_input_bytes + 1024 * 1024;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = FramedRead::new(
        tokio::io::stdin(),
        JsonRpcMessageCodec::<RxJsonRpcMessage<RoleServer>>::new_with_max_length(MAX_REQUEST_BYTES),
    )
    .filter_map(|result| future::ready(result.ok()));
    let writer = FramedWrite::new(
        tokio::io::stdout(),
        JsonRpcMessageCodec::<TxJsonRpcMessage<RoleServer>>::default(),
    )
    .sink_map_err(std::io::Error::other);
    let service = rust_iso20022_mcp::Iso20022Mcp::new()
        .serve((writer, reader))
        .await?;
    service.waiting().await?;
    Ok(())
}
