#[derive(Clone, Debug)]
pub enum V1RpcRoutes {
    V1QueryHeight,
}

impl V1RpcRoutes {
    pub fn url(&self, rpc_url: &str) -> String {
        match self {
            V1RpcRoutes::V1QueryHeight => format!("{}/{}", rpc_url, "v1/query/height"),
        }
    }
}
