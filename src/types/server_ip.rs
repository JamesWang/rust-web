use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ServerIp {
    pub id: i32,
    pub hostname: String,
    pub ip4_address: String,
    pub ip6_address: Option<String>,
    pub mac_address: Option<String>,
}

impl ServerIp {
    pub fn new(
        id: i32,
        hostname: String,
        ip4_address: String,
        ip6_address: Option<String>,
        mac_address: Option<String>,
    ) -> Self {
        ServerIp {
            id,
            hostname,
            ip4_address,
            ip6_address,
            mac_address,
        }
    }
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewServerIp {
    pub hostname: String,
    
    #[serde(rename = "ip4")]
    pub ip4_address: String,
    
    #[serde(rename = "ip6")]
    pub ip6_address: Option<String>,
    
    #[serde(rename = "mac")]
    pub mac_address: Option<String>,
}
