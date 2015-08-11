pub mod isomorphism;
pub mod parametric;

use isomorphism::{Iso, IsoOnce};
use parametric::{Param, ReParam};

pub trait Covariant<'a, B>: ReParam<B> {
    fn fmap<F: 'a + Fn(Self::Param) -> B>(self, F) -> Self::Output;
}

pub trait CovariantOnce<'a, B>: ReParam<B> {
    fn fmap_once<F: 'a + FnOnce(Self::Param) -> B>(self, F) -> Self::Output;
}

pub trait Contravariant<'a, B>: ReParam<B> {
    fn contramap<F: 'a + Fn(B) -> Self::Param>(self, F) -> Self::Output;
}

pub trait ContravariantOnce<'a, B>: ReParam<B> {
    fn contramap_once<F: 'a + FnOnce(B) -> Self::Param>(self, F) -> Self::Output;
}

pub trait Invariant<'a, B>: ReParam<B> {
    fn invmap<F: 'a + Iso<Self::Param, B>>(self, F) -> Self::Output;
}

pub trait InvariantOnce<'a, B>: ReParam<B> {
    fn invmap_once<F: 'a + IsoOnce<Self::Param, B>>(self, F) -> Self::Output;
}

pub trait Bivariant<'a, B>: ReParam<B> {
    fn xmap(self) -> Self::Output;
}

impl<'a, A, B> Covariant<'a, B> for std::option::Option<A> {
    fn fmap<F: 'a + Fn(A) -> B>(self, f: F) -> Self::Output {
        self.map(|a| f(a))
    }
}

impl<'a, A, B> CovariantOnce<'a, B> for std::option::Option<A> {
    fn fmap_once<F: 'a + FnOnce(A) -> B>(self, f: F) -> Self::Output {
        self.map(|a| f(a))
    }
}

impl<'a, X, A, B> Covariant<'a, B> for (X, A) {
    fn fmap<F: Fn(A) -> B>(self, f: F) -> Self::Output {
        (self.0, f(self.1))
    }
}

impl<'a, X, A, B> CovariantOnce<'a, B> for (X, A) {
    fn fmap_once<F: FnOnce(A) -> B>(self, f: F) -> Self::Output {
        (self.0, f(self.1))
    }
}

impl<'a, A, B> Bivariant<'a, B> for std::marker::PhantomData<A> {
    fn xmap(self) -> Self::Output { std::marker::PhantomData }
}

#[test]
fn it_works() {
}
