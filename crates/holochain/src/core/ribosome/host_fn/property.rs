use crate::core::ribosome::error::RibosomeResult;
use crate::core::ribosome::CallContext;
use crate::core::ribosome::RibosomeT;
use std::sync::Arc;

pub fn property(
    _ribosome: Arc<impl RibosomeT>,
    _call_context: Arc<CallContext>,
    _input: (),
) -> RibosomeResult<()> {
    unimplemented!();
}
