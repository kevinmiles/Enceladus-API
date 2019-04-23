use std::{ops::Deref, time::SystemTime};

#[repr(transparent)]
#[derive(Debug)]
pub struct RequestTimer(Option<SystemTime>);

impl RequestTimer {
    #[inline(always)]
    pub fn begin() -> Self {
        Self(Some(SystemTime::now()))
    }

    #[inline(always)]
    pub const fn end() -> Self {
        Self(None)
    }
}

impl Deref for RequestTimer {
    type Target = Option<SystemTime>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
