use std::option::Option;
use std::marker::PhantomData;

pub trait Param {
    type Param;
}

pub trait ReParam<B>: Param {
    type Output: Param<Param=B>;
}

impl<'a, A: Param> Param for &'a A {
    type Param = A::Param;
}

impl<'a, A: Param> Param for &'a mut A {
    type Param = A::Param;
}

impl<A> Param for Option<A> {
    type Param = A;
}

impl<A, B> ReParam<B> for Option<A> {
    type Output = Option<B>;
}

impl<X, A> Param for (X, A) {
    type Param = A;
}

impl<X, A, B> ReParam<B> for (X, A) {
    type Output = (X, B);
}

impl<A> Param for PhantomData<A> {
    type Param = A;
}

impl<A, B> ReParam<B> for PhantomData<A> {
    type Output = PhantomData<B>;
}

impl<A> Param for Box<A> {
    type Param = A;
}

impl<A, B> ReParam<B> for Box<A> {
    type Output = Box<B>;
}

impl<A, E> Param for Result<A, E> {
    type Param = A;
}

impl<A, B, E> ReParam<B> for Result<A, E> {
    type Output = Result<B, E>;
}
