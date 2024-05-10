#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;

use deth_contract_types::*;
use piecrust_uplink as uplink;

static mut TOKEN: Token = Token {
    total_supply: [0, 0],
    balances: BTreeMap::new(),
    approvals: BTreeMap::new(),
};

struct Token {
    total_supply: Value,
    balances: BTreeMap<Address, Value>,
    approvals: BTreeMap<Address, BTreeMap<Address, Value>>,
}

impl Token {
    fn name(&self) -> String {
        String::from("Dusk Ether")
    }

    fn symbol(&self) -> String {
        String::from("dETH")
    }

    fn decimals(&self) -> u8 {
        18
    }

    fn supply(&self) -> Value {
        self.total_supply
    }

    fn balance(&self, address: Address) -> Value {
        self.balances.get(&address).copied().unwrap_or([0, 0])
    }

    fn transfer(&mut self, _transfer: Transfer) {
        todo!()
    }

    fn transfer_from(&mut self, _transfer_from: TransferFrom) {
        todo!()
    }

    fn approve(&mut self, _approve: Approve) {
        todo!()
    }

    fn allowance(&self, allowance: Allowance) -> Value {
        match self.approvals.get(&allowance.owner) {
            None => [0, 0],
            Some(approvals) => approvals.get(&allowance.spender).copied().unwrap_or([0, 0]),
        }
    }
}

#[no_mangle]
unsafe fn name(arg_len: u32) -> u32 {
    uplink::wrap_call(arg_len, |_: ()| TOKEN.name())
}

#[no_mangle]
unsafe fn symbol(arg_len: u32) -> u32 {
    uplink::wrap_call(arg_len, |_: ()| TOKEN.symbol())
}

#[no_mangle]
unsafe fn decimals(arg_len: u32) -> u32 {
    uplink::wrap_call(arg_len, |_: ()| TOKEN.decimals())
}

#[no_mangle]
unsafe fn balance(arg_len: u32) -> u32 {
    uplink::wrap_call(arg_len, |a| TOKEN.balance(a))
}

#[no_mangle]
unsafe fn supply(arg_len: u32) -> u32 {
    uplink::wrap_call(arg_len, |_: ()| TOKEN.supply())
}

#[no_mangle]
unsafe fn transfer(arg_len: u32) -> u32 {
    uplink::wrap_call(arg_len, |t| TOKEN.transfer(t))
}

#[no_mangle]
unsafe fn transfer_from(arg_len: u32) -> u32 {
    uplink::wrap_call(arg_len, |t| TOKEN.transfer_from(t))
}

#[no_mangle]
unsafe fn approve(arg_len: u32) -> u32 {
    uplink::wrap_call(arg_len, |a| TOKEN.approve(a))
}

#[no_mangle]
unsafe fn allowance(arg_len: u32) -> u32 {
    uplink::wrap_call(arg_len, |a| TOKEN.allowance(a))
}
