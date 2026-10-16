//! Platform-agnostic window identification.

/// Abstract opaque window identifier across desktop environments.
/// On Windows, represents the underlying HWND handle as an `isize`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct WindowId(pub isize);

impl WindowId {
    #[inline]
    pub const fn new(id: isize) -> Self {
        Self(id)
    }

    #[inline]
    pub const fn raw(self) -> isize {
        self.0
    }
}
