use crate::{gatekeep::ChainRootHandle, Dna};
use tokio::task;

pub struct ZomeCall<'a> {
    token: Capability,
    payload: &'a [u8],
}

pub async fn handle_zome_call(
    dna: Arc<Dna>,
    chain_root: ChainRootHandle,
    call: ZomeCall<'_>,
    as_at: Address,
) -> Result<Vec<u8>, ZomeCallError> {
    let call_result = {
        let dna = Arc::clone(&dna);
        task::spawn_blocking(move || call.apply(dna, &as_at)).await?
    };


}
