use std::sync::{atomic::AtomicBool, Arc};

use gm_rpc_proxy::rpc_types::ResponsePayload;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let sd = Arc::new(AtomicBool::new(false));

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
