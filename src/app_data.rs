use web3::types::H256;

pub struct AppData {
    pub private_key: H256,
}

impl AppData {
    pub fn new(private_key: H256) -> Self {
        Self { private_key }
    }
}
