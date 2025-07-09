// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SaturatingU16 {
    inner: u16,
}

impl From<u16> for SaturatingU16 {
    fn from(inner: u16) -> Self {
        Self { inner }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(inner: u8) -> Self {
        Self { inner: inner as u16 }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(inner: &u16) -> Self {
        Self { inner: *inner }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(inner: &u8) -> Self {
        Self { inner: *inner as u16 }
    }
}

impl Add<SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: SaturatingU16) -> Self::Output {
        Self {
            inner: self.inner.saturating_add(rhs.inner),
        }
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        Self {
            inner: self.inner.saturating_add(rhs.inner),
        }
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: u16) -> Self::Output {
        Self {
            inner: self.inner.saturating_add(rhs),
        }
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &u16) -> Self::Output {
        Self {
            inner: self.inner.saturating_add(*rhs),
        }
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, rhs: &u16) -> bool {
        self.inner == *rhs
    }
}