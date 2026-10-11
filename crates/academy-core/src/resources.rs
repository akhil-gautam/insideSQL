use crate::Result;
use std::path::Path;

pub(crate) const MIN_FREE_DISK_BYTES: u64 = 512 * 1024 * 1024;

pub(crate) fn available_disk_bytes(root: &Path) -> Result<u64> {
    #[cfg(unix)]
    {
        let stat = rustix::fs::statvfs(root)
            .map_err(|e| format!("Could not measure free disk space: {e}"))?;
        space_bytes(stat.f_bavail, stat.f_frsize)
    }
    #[cfg(not(unix))]
    {
        let _ = root;
        Err("Native disk checks are not implemented for this platform; the lab cannot allocate storage safely".into())
    }
}

#[cfg(unix)]
fn space_bytes(blocks: u64, fragment_size: u64) -> Result<u64> {
    if fragment_size == 0 {
        return Err("Free disk-space measurement has no block size".into());
    }
    blocks
        .checked_mul(fragment_size)
        .ok_or_else(|| "Free disk-space measurement overflowed".into())
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    #[test]
    fn resource_space_units_do_not_overflow_or_invent_capacity() {
        assert_eq!(space_bytes(1024, 4096).unwrap(), 4 * 1024 * 1024);
        assert_eq!(space_bytes(0, 4096).unwrap(), 0);
        assert!(space_bytes(1, 0).is_err());
        assert!(space_bytes(u64::MAX, 2).is_err());
    }
    #[test]
    fn resource_native_measurement_uses_owned_filesystem_and_fails_closed() {
        let home = tempfile::tempdir().unwrap();
        assert!(available_disk_bytes(home.path()).unwrap() > 0);
        assert!(available_disk_bytes(&home.path().join("missing")).is_err());
    }
}
