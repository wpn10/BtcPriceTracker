use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct PriceData{
    pub price: u64, //Price in USD
    pub last_update: u64, //timestamp of the last update
}
