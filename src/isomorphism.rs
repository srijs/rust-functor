use parametric::{Param, ReParam};
use super::{Invariant, InvariantOnce};

pub trait Iso<A, B> {
    fn to(&self, A) -> B;
    fn from(&self, B) -> A;
}

pub trait IsoOnce<A, B> {
    fn to_once(self, A) -> B;
    fn from_once(self, B) -> A;
}

impl<A, B, F, G> Iso<A, B> for (F, G) where F: Fn(A) -> B, G: Fn(B) -> A {
    fn to(&self, a: A) -> B { self.0(a) }
    fn from(&self, b: B) -> A { self.1(b) }
}

impl<A, B, F, G> IsoOnce<A, B> for (F, G) where F: FnOnce(A) -> B, G: FnOnce(B) -> A {
    fn to_once(self, a: A) -> B { self.0(a) }
    fn from_once(self, b: B) -> A { self.1(b) }
}
