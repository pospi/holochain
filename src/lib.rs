pub mod gatekeep;
// pub mod zome_call;
pub mod transforms;

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

#[derive(Eq, PartialEq, Debug)]
pub struct Address {}
