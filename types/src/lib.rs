#![no_std]

use bytecheck::CheckBytes;
use rkyv::{Archive, Deserialize, Serialize};

pub type Address = [u8; 20];
pub type Value = [u128; 2];

#[derive(Archive, Deserialize, Serialize)]
#[archive_attr(derive(CheckBytes))]
pub struct Transfer {
    pub to: Address,
    pub value: Value,
}

#[derive(Archive, Deserialize, Serialize)]
#[archive_attr(derive(CheckBytes))]
pub struct TransferFrom {
    pub from: Address,
    pub to: Address,
    pub value: Value,
}

#[derive(Archive, Deserialize, Serialize)]
#[archive_attr(derive(CheckBytes))]
pub struct Approve {
    pub spender: Address,
    pub value: Value,
}

#[derive(Archive, Deserialize, Serialize)]
#[archive_attr(derive(CheckBytes))]
pub struct Allowance {
    pub owner: Address,
    pub spender: Address,
}
