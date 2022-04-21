#[derive(Clone, Debug)]
pub enum V1RpcRoutes {
    ClientChallenge,
    ClientDispatch,
    ClientRawTx,
    ClientRelay,
    QueryAccount,
    QueryAccountTxs,
    QueryAllParams,
    QueryApp,
    QueryAppParams,
    QueryApps,
    QueryBalance,
    QueryBlock,
    QueryHeight,
    QueryNode,
    QueryNodeClaim,
    QueryNodeClaims,
    QueryNodeParams,
    QueryNodeReceipt,
    QueryNodeReceipts,
    QueryNodes,
    QueryPocketParams,
    QuerySupply,
    QuerySupportedChains,
    QueryTx,
    QueryUpgrade,
}

impl V1RpcRoutes {
    pub fn url(&self, rpc_url: &str) -> String {
        match self {
            V1RpcRoutes::ClientChallenge => format!("{}/{}", rpc_url, "v1/client/challenge"),
            V1RpcRoutes::ClientDispatch => format!("{}/{}", rpc_url, "v1/client/dispatch"),
            V1RpcRoutes::ClientRawTx => format!("{}/{}", rpc_url, "/v1/client/rawtx"),
            V1RpcRoutes::ClientRelay => format!("{}/{}", rpc_url, "v1/client/relay"),
            V1RpcRoutes::QueryAccount => format!("{}/{}", rpc_url, "v1/query/account"),
            V1RpcRoutes::QueryAccountTxs => format!("{}/{}", rpc_url, "v1/query/accounttxs"),
            V1RpcRoutes::QueryAllParams => format!("{}/{}", rpc_url, "/v1/query/allparams"),
            V1RpcRoutes::QueryApp => format!("{}/{}", rpc_url, "v1/query/app"),
            V1RpcRoutes::QueryAppParams => format!("{}/{}", rpc_url, "v1/query/appparams"),
            V1RpcRoutes::QueryApps => format!("{}/{}", rpc_url, "v1/query/apps"),
            V1RpcRoutes::QueryBalance => format!("{}/{}", rpc_url, "v1/query/balance"),
            V1RpcRoutes::QueryBlock => format!("{}/{}", rpc_url, "v1/query/block"),
            V1RpcRoutes::QueryHeight => format!("{}/{}", rpc_url, "v1/query/height"),
            V1RpcRoutes::QueryNode => format!("{}/{}", rpc_url, "v1/query/ndoe"),
            V1RpcRoutes::QueryNodeClaim => format!("{}/{}", rpc_url, "v1/query/nodeclaim"),
            V1RpcRoutes::QueryNodeClaims => format!("{}/{}", rpc_url, "v1/query/nodeclaims"),
            V1RpcRoutes::QueryNodeParams => format!("{}/{}", rpc_url, "v1/query/nodeparams"),
            V1RpcRoutes::QueryNodeReceipt => format!("{}/{}", rpc_url, "v1/query/nodereceipt"),
            V1RpcRoutes::QueryNodeReceipts => format!("{}/{}", rpc_url, "v1/query/nodereceipts"),
            V1RpcRoutes::QueryNodes => format!("{}/{}", rpc_url, "v1/query/nodes"),
            V1RpcRoutes::QueryPocketParams => format!("{}/{}", rpc_url, "v1/query/pocketparams"),
            V1RpcRoutes::QuerySupply => format!("{}/{}", rpc_url, "v1/query/supply"),
            V1RpcRoutes::QuerySupportedChains => {
                format!("{}/{}", rpc_url, "v1/query/supportedchains")
            }
            V1RpcRoutes::QueryTx => format!("{}/{}", rpc_url, "v1/query/tx"),
            V1RpcRoutes::QueryUpgrade => format!("{}/{}", rpc_url, "v1/query/upgrade"),
        }
    }
}
