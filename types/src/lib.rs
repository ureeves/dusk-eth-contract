#![no_std]

pub use digest;
pub use rand;
pub use secp256k1;
pub use sha3;

use bytecheck::CheckBytes;
use rkyv::{Archive, Deserialize, Serialize};

use digest::consts::N32;
use digest::{Digest, Output, OutputSizeUser};
use rand::{CryptoRng, RngCore};
use secp256k1::{Message, Secp256k1, SecretKey, SignOnly};

/// An Ethereum address - without the prefix.
pub type Address = [u8; 20];
/// A 256bit number, representing the amount of currency.
pub type Value = [u128; 2];
/// An secp256k1 ECDSA signature.
pub type Signature = [u8; 64];

const ZERO_SIGNATURE: Signature = [0u8; 64];

/// Transfer funds to an address.
#[derive(Archive, Deserialize, Serialize)]
#[archive_attr(derive(CheckBytes))]
pub struct Transfer {
    /// Address to transfer funds to.
    pub to: Address,
    /// Value to transfer.
    pub value: Value,
    /// Nonce to prevent replay.
    pub nonce: u64,
    /// Signature of the call.
    pub sig: Signature,
}

impl Transfer {
    pub fn new<R, D>(rng: &mut R, sk: &SecretKey, to: Address, value: Value) -> Self
    where
        R: RngCore + CryptoRng + ?Sized,
        D: Digest,
        D::OutputSize: OutputSizeUser<OutputSize = N32>,
    {
        let secp = Secp256k1::<SignOnly>::gen_new();

        let mut this = Self {
            to,
            value,
            nonce: rng.next_u64(),
            sig: ZERO_SIGNATURE,
        };

        let hash = this.hash::<D>();
        let mut hash_array = [0u8; 32];
        hash_array.copy_from_slice(&hash);

        let msg = Message::from_digest(hash_array);
        this.sig = secp.sign_ecdsa(&msg, sk).serialize_compact();

        this
    }

    pub fn hash<D: Digest>(&self) -> Output<D> {
        let mut hasher = D::new();

        hasher.update(&self.to);
        hasher.update(&self.value[0].to_le_bytes());
        hasher.update(&self.value[1].to_le_bytes());
        hasher.update(&self.nonce.to_le_bytes());

        hasher.finalize()
    }
}

/// Transfer funds from a given address, to another address.
#[derive(Archive, Deserialize, Serialize)]
#[archive_attr(derive(CheckBytes))]
pub struct TransferFrom {
    /// Address to transfer from.
    pub from: Address,
    /// Address to transfer to.
    pub to: Address,
    /// Value to transfer.
    pub value: Value,
    /// Nonce to prevent replay.
    pub nonce: u64,
    /// Signature of the call.
    pub sig: Signature,
}

impl TransferFrom {
    pub fn new<R, D>(rng: &mut R, sk: &SecretKey, from: Address, to: Address, value: Value) -> Self
    where
        R: RngCore + CryptoRng + ?Sized,
        D: Digest,
        D::OutputSize: OutputSizeUser<OutputSize = N32>,
    {
        let secp = Secp256k1::<SignOnly>::gen_new();

        let mut this = Self {
            from,
            to,
            value,
            nonce: rng.next_u64(),
            sig: ZERO_SIGNATURE,
        };

        let hash = this.hash::<D>();
        let mut hash_array = [0u8; 32];
        hash_array.copy_from_slice(&hash);

        let msg = Message::from_digest(hash_array);
        this.sig = secp.sign_ecdsa(&msg, sk).serialize_compact();

        this
    }

    pub fn hash<D: Digest>(&self) -> Output<D> {
        let mut hasher = D::new();

        hasher.update(&self.from);
        hasher.update(&self.to);
        hasher.update(&self.value[0].to_le_bytes());
        hasher.update(&self.value[1].to_le_bytes());
        hasher.update(&self.nonce.to_le_bytes());

        hasher.finalize()
    }
}

/// Approve an address to transfer funds.
#[derive(Archive, Deserialize, Serialize)]
#[archive_attr(derive(CheckBytes))]
pub struct Approve {
    /// Address to approve for spending.
    pub spender: Address,
    /// Value to approve the transfer of.
    pub value: Value,
    /// Nonce to prevent replay.
    pub nonce: u64,
    /// Signature of the call.
    pub sig: Signature,
}

impl Approve {
    pub fn new<R, D>(rng: &mut R, sk: &SecretKey, spender: Address, value: Value) -> Self
    where
        R: RngCore + CryptoRng + ?Sized,
        D: Digest,
        D::OutputSize: OutputSizeUser<OutputSize = N32>,
    {
        let secp = Secp256k1::<SignOnly>::gen_new();

        let mut this = Self {
            spender,
            value,
            nonce: rng.next_u64(),
            sig: ZERO_SIGNATURE,
        };

        let hash = this.hash::<D>();
        let mut hash_array = [0u8; 32];
        hash_array.copy_from_slice(&hash);

        let msg = Message::from_digest(hash_array);
        this.sig = secp.sign_ecdsa(&msg, sk).serialize_compact();

        this
    }

    pub fn hash<D: Digest>(&self) -> Output<D> {
        let mut hasher = D::new();

        hasher.update(&self.spender);
        hasher.update(&self.value[0].to_le_bytes());
        hasher.update(&self.value[1].to_le_bytes());
        hasher.update(&self.nonce.to_le_bytes());

        hasher.finalize()
    }
}

/// Query the allowance an address' owner has allowed a given spender.
#[derive(Archive, Deserialize, Serialize)]
#[archive_attr(derive(CheckBytes))]
pub struct Allowance {
    /// The address of the owner.
    pub owner: Address,
    /// The address of the allowed spender.
    pub spender: Address,
}
