//! An in-memory file system, not backed by any physical device.

/// A backing implementation for [`FileSystem`] storing all files in-memory.
///
/// # WARNING
///
/// This has no physical backing store, thus any files in memory are erased as soon as this object
/// is dropped.
pub struct InMemoryFileSystem {
    // TODO
}

impl InMemoryFileSystem {
    /// Construct a new `InMemoryFileSystem` instance
    ///
    /// This function is expected to only be invoked once per platform, as an initialiation step,
    /// and the created `FileSystem` handle is expected to be shared across all usage over the
    /// system.
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for InMemoryFileSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl super::private::Sealed for InMemoryFileSystem {}

impl super::FileSystem for InMemoryFileSystem {
    fn open(
        &self,
        path: impl crate::path::Arg,
        flags: super::OFlags,
        mode: super::Mode,
    ) -> Result<crate::fd::FileFd, super::errors::OpenError> {
        todo!()
    }

    fn close(&self, fd: crate::fd::FileFd) -> Result<(), super::errors::CloseError> {
        todo!()
    }

    fn read(
        &self,
        fd: &crate::fd::FileFd,
        buf: &mut [u8],
    ) -> Result<usize, super::errors::ReadError> {
        todo!()
    }

    fn write(
        &self,
        fd: &crate::fd::FileFd,
        buf: &[u8],
    ) -> Result<usize, super::errors::WriteError> {
        todo!()
    }

    fn chmod(
        &self,
        path: impl crate::path::Arg,
        mode: super::Mode,
    ) -> Result<(), super::errors::ChmodError> {
        todo!()
    }

    fn unlink(&self, path: impl crate::path::Arg) -> Result<(), super::errors::UnlinkError> {
        todo!()
    }

    fn mkdir(
        &self,
        path: impl crate::path::Arg,
        mode: super::Mode,
    ) -> Result<(), super::errors::MkdirError> {
        todo!()
    }

    fn rmdir(&self, path: impl crate::path::Arg) -> Result<(), super::errors::RmdirError> {
        todo!()
    }
}
