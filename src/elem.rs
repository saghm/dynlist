use std::fmt;

use super::{DynList, DynListIntoIter};

#[derive(Debug)]
pub enum DynElem<'a, T: 'a> {
    Scalar(T),
    List(DynList<'a, T>),
}

impl<'a, T> DynElem<'a, T> {
    pub fn inner_ref(&self) -> DynElem<&T> {
        match *self {
            DynElem::Scalar(ref t) => DynElem::Scalar(t),
            DynElem::List(ref list) => DynElem::List(list.inner_ref()),
        }
    }
}

impl<'a, T> fmt::Display for DynElem<'a, T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DynElem::Scalar(ref t) => write!(f, "{}", t),
            DynElem::List(ref list) => write!(f, "{}", list),
        }
    }
}

impl<'a, T: 'a> IntoIterator for DynElem<'a, T> {
    type Item = T;
    type IntoIter = ElemIntoIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            DynElem::Scalar(t) => ElemIntoIter::Scalar(Some(t)),
            DynElem::List(list) => ElemIntoIter::List(list.into_iter()),
        }
    }
}

pub enum ElemIntoIter<'a, T: 'a> {
    Scalar(Option<T>),
    List(DynListIntoIter<'a, T>),
}

impl<'a, T: 'a> Iterator for ElemIntoIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match *self {
            ElemIntoIter::Scalar(ref mut t) => t.take(),
            ElemIntoIter::List(ref mut list) => list.next(),
        }
    }
}

#[macro_export]
macro_rules! dyn_elem {
    ([$($elem:tt),*]) => {{
        use dynlist::DynElem;

        DynElem::List(dyn_list![$($elem),*])
    }};

    ($singleton:expr) => {{
        use dynlist::DynElem;

        DynElem::Scalar($singleton)
    }};
}
