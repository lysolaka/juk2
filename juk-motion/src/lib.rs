//! Motion profiles and trajectory generation for JUK2
#![no_std]

pub mod trap;

/// Trait for motion profiles allowing the use of a single interface.
pub trait Profile: Sized {
    /// Compute the delay until next step. Returns `None` when movement is finished.
    fn next_delay(&mut self) -> Option<f32>;

    /// Returns an iterator of delays values between steps.
    fn delays(&mut self) -> Delays<'_, Self> {
        Delays(self)
    }
}

/// Inter-step delays iterator
pub struct Delays<'a, P: Profile>(pub &'a mut P);

impl<'a, P> Iterator for Delays<'a, P>
where
    P: Profile,
{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next_delay()
    }
}

impl<'a, P> core::iter::FusedIterator for Delays<'a, P> where P: Profile {}
