use std::path::Path;

use crate::error::{IoOperation, WalError, WalResult};

pub(crate) fn fsync_dir(path: &Path) -> WalResult<()> {
    let dir =
        std::fs::File::open(path).map_err(WalError::at(path.to_path_buf(), IoOperation::Sync))?;
    dir.sync_all()
        .map_err(WalError::at(path.to_path_buf(), IoOperation::Sync))?;
    Ok(())
}
