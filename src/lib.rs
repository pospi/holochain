pub mod gatekeep;
// pub mod zome_call;
pub mod entry;
pub mod transforms;

#[derive(Clone)]
pub struct Dna;

/// A 256-bit hash
// 256 bits = 32 bytes
#[derive(Eq, PartialEq, Debug)]
pub struct Address([u8; 32]);

pub struct LinkData;

#[derive(Clone)]
pub struct PublicKey;

pub struct Timestamp;

pub fn get_source_chain_root_hash(_lmdb: &LmdbRead) -> Address {
    unimplemented!()
}

pub fn rebase_headers(_transaction: &mut LmdbTransaction, _valid_at: &Address, _now: &Address) {
    unimplemented!()
}

pub struct LmdbUnique {}

impl LmdbUnique {
    pub fn apply(&mut self, _transaction: LmdbTransaction) {
        unimplemented!()
    }

    pub fn downgrade(&self) -> LmdbRead {
        unimplemented!()
    }
}

pub struct LmdbRead {}

pub struct LmdbTransaction {}
