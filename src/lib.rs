pub mod gatekeep;
// pub mod zome_call;
pub mod entry;
pub mod transforms;

pub trait Sign {}

impl Sign for entry::HeaderWithEntry {}
impl Sign for entry::Header {}
impl Sign for entry::NormalHeader {}
impl Sign for entry::UpdateHeader {}
impl Sign for entry::ChainHeaderWithEntry {}

pub struct Signed<T> {
    pub sig: Signature,
    pub data: T,
}

impl<T> std::ops::Deref for Signed<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> std::ops::DerefMut for Signed<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> Signed<T> {
    pub fn into_inner(self) -> T {
        self.data
    }
}

fn hash<T>(data: T) -> Address {
    todo!()
}

pub struct Signature;

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
