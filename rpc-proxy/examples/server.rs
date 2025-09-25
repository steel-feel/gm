use gm_rpc_proxy::rpc_types::ResponsePayload;
use serde_json::Value;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let sd = CancellationToken::new();

    gm_rpc_proxy::serve(
        3000,
        &"",
        "http://127.0.0.1:8545".parse().unwrap(),
        sd,
        |req| {
            if req.method == "eth_blockNumber" {
                // Synchronous immidiate response
                Ok(gm_rpc_proxy::OverrideResult::Sync(
                    ResponsePayload::Success(Value::String("0x1".to_string())),
                ))
            } else {
                // This will cause the request to be forwarded to underlying rpc
                Ok(gm_rpc_proxy::OverrideResult::NoOverride)
            }
        },
    )
    .await
    .unwrap();
}
