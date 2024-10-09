use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize,BorshSerialize, Debug)
pub enum BtcPriceTrackerInstruction{
    InitPrice
}
