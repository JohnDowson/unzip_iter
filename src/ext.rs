use std::{cell::RefCell, collections::VecDeque, marker::PhantomData, rc::Rc};
use unzip_macros::*;

pub trait UnzipExt<A> {
    type Iters;
    fn unzip_iter(self) -> Self::Iters;
}
impl_ext!(2);
impl_ext!(3);
impl_ext!(4);

pub trait Get<T, A, const N: usize> {
    fn get(&mut self) -> &mut T;
}
impl_get!(2);
impl_get!(3);
impl_get!(4);

pub trait Wrap {
    type Wrapped;
}
impl_wrap!(2);
impl_wrap!(3);
impl_wrap!(4);

trait Splat<R, const E: usize>: Wrap + Sized {
    fn splat(self, to: &mut Self::Wrapped) -> R;
}
impl_splat!(2);
impl_splat!(3);
impl_splat!(4);

struct Unzipped<I>
where
    I: Iterator,
    I::Item: Wrap,
{
    inner: I,
    cache: <I::Item as Wrap>::Wrapped,
}
impl<I> Unzipped<I>
where
    I: Iterator,
    I::Item: Wrap,
    <I::Item as Wrap>::Wrapped: Default,
{
    fn new(inner: I) -> Rc<RefCell<Self>> {
        let unzipped = Self {
            inner,
            cache: Default::default(),
        };
        let unzipped = RefCell::new(unzipped);
        Rc::new(unzipped)
    }
}
trait UnzippedImpl<T, A, const N: usize> {
    fn get(&mut self) -> Option<T>;
}
impl_unzipped!(2);
impl_unzipped!(3);
impl_unzipped!(4);

pub struct UnzipIter<I, T, const N: usize>
where
    I: Iterator,
    I::Item: Wrap,
{
    inner: Rc<RefCell<Unzipped<I>>>,
    _item: PhantomData<T>,
}
impl<I, T, const N: usize> UnzipIter<I, T, N>
where
    I: Iterator,
    I::Item: Wrap,
{
    fn new(inner: &Rc<RefCell<Unzipped<I>>>) -> Self {
        Self {
            inner: Rc::clone(inner),
            _item: PhantomData,
        }
    }
}
impl<I, T, const N: usize> Iterator for UnzipIter<I, T, N>
where
    I: Iterator,
    I::Item: Wrap,
    Unzipped<I>: UnzippedImpl<T, I::Item, N>,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.borrow_mut().get()
    }
}
