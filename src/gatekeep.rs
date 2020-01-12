use super::*;

use futures::lock::Mutex;
use std::sync::Arc;

pub enum TransactError {
    HeadMoved,
}
#[derive(Clone)]
pub struct ChainRootHandle {
    inner: Arc<Mutex<ChainRootGatekeeper>>,
}
struct ChainRootGatekeeper {
    db_write: Arc<Mutex<LmdbUnique>>,
    db_read: LmdbRead,
}

impl ChainRootHandle {
    /// Create a handle to a source-chain root manager
    ///
    /// It is a bug if this function is called twice on the same Lmdb database
    pub fn new(db_write: Arc<Mutex<LmdbUnique>>, db_read: LmdbRead) -> Self {
        let gatekeeper = ChainRootGatekeeper { db_write, db_read };
        {
            let inner = Arc::new(Mutex::new(gatekeeper));
            Self { inner }
        }
    }

    pub async fn try_append_chain(
        &self,
        bundle: LmdbTransaction,
        valid_at: Address,
        rebasable: bool,
    ) -> Result<(), TransactError> {
        {
            self.inner
                .lock()
                .await
                .gatekeep(bundle, valid_at, rebasable)
                .await
        }
    }
}

impl ChainRootGatekeeper {
    pub async fn gatekeep(
        &mut self,
        mut next_write: LmdbTransaction,
        as_at: Address,
        rebasable: bool,
    ) -> Result<(), TransactError> {
        let chain_root_hash = get_source_chain_root_hash(&self.db_read);
        // check if transaction has been invalidated.
        if chain_root_hash != as_at {
            // check if we can recover.
            if rebasable {
                rebase_headers(&mut next_write, &chain_root_hash, &as_at);
            } else {
                // we can't recover. abort transaction.
                return Err(TransactError::HeadMoved);
            }
        }

        {
            let mut write_handle = self.db_write.lock().await;
            // provided that
            // 1. no other instances of gatekeep are running and
            // 2. no other code-paths modify the source-chain root,
            // which should both be true unless there is a bug,
            // the source chain root hasn't changed since the above check
            debug_assert_eq!(
                get_source_chain_root_hash(&write_handle.downgrade()),
                chain_root_hash
            );
            write_handle.apply(next_write);
        }
        Ok(())
    }
}
