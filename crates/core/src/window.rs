//! Opaque window identifier for sans-IO domain operations.

/// Opaque window handle. Kept as a plain integer so fixtures and pure planners
/// need no native Win32 window handles.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct WindowId(pub isize);

impl WindowId {
    #[inline]
    pub const fn new(id: isize) -> Self {
        Self(id)
    }

    #[inline]
    pub const fn as_isize(self) -> isize {
        self.0
    }
}

impl From<isize> for WindowId {
    #[inline]
    fn from(val: isize) -> Self {
        Self(val)
    }
}
