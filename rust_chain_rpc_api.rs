//! 区块链 RPC 接口 - 外部服务链上交互
use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Serialize, Deserialize)]
pub struct RpcRequest {
    pub method: String,
    pub params: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RpcResponse {
    pub code: i32,
    pub data: String,
}

pub async fn start_rpc_server() {
    let route = warp::post()
        .and(warp::body::json())
        .map(|req: RpcRequest| {
            let data = format!("RPC Executed: {}", req.method);
            RpcResponse { code: 0, data }
        });
    warp::serve(route).run(([0,0,0,0], 8545)).await;
}
