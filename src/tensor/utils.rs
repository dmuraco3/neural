
// code from https://gist.github.com/Notgnoshi/b803e4c1eef7f1ba8ed453c8117349e8

pub trait RecursivelyFlattenIterator: Iterator + Sized {
    fn recursively_flatten<Depth, Item>(self) -> RecursivelyFlatten<Depth, Self, Item>
    where
        Self: RecursivelyFlattenIteratorImpl<Depth, Item>,
    {
        RecursivelyFlatten {
            inner: RecursivelyFlattenIteratorImpl::recursively_flatten_impl(self),
        }
    }
}

/// A helper trait that actually does the heavy lifting.
pub trait RecursivelyFlattenIteratorImpl<Depth, Item> {
    type RecursivelyFlatten: Iterator<Item = Item>;
    fn recursively_flatten_impl(self) -> Self::RecursivelyFlatten;
}

/// A wrapper type to help out type inference. Similar to template tags in C++.
pub struct RecursivelyFlatten<Depth, Iter, Item>
where
    Iter: RecursivelyFlattenIteratorImpl<Depth, Item>,
{
    inner: Iter::RecursivelyFlatten,
}

impl<Iter: Iterator> RecursivelyFlattenIteratorImpl<(), Iter::Item> for Iter {
    type RecursivelyFlatten = Self;
    fn recursively_flatten_impl(self) -> Self::RecursivelyFlatten {
        self
    }
}

impl<Depth, Iter: Iterator, Item> RecursivelyFlattenIteratorImpl<(Depth,), Item> for Iter
where
    std::iter::Flatten<Iter>: RecursivelyFlattenIteratorImpl<Depth, Item>,
    Iter: Iterator,
    <Iter as Iterator>::Item: IntoIterator,
{
    type RecursivelyFlatten = <std::iter::Flatten<Iter> as RecursivelyFlattenIteratorImpl<
        Depth,
        Item,
    >>::RecursivelyFlatten;

    fn recursively_flatten_impl(self) -> Self::RecursivelyFlatten {
        RecursivelyFlattenIteratorImpl::recursively_flatten_impl(self.flatten())
    }
}

// TODO: Somehow necessary for IntoIter lookups?
impl<Iter: Iterator> RecursivelyFlattenIterator for Iter {}

/// Implement Iterator for the wrapper type.
impl<Depth, Iter, Item> Iterator for RecursivelyFlatten<Depth, Iter, Item>
where
    Iter: RecursivelyFlattenIteratorImpl<Depth, Item>,
{
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}