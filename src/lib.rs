#[macro_use]
mod elem;

use std::fmt;
use std::marker::PhantomData;

pub use self::elem::DynElem;

#[derive(Debug)]
pub struct DynList<'a, T: 'a> {
    inner: Vec<DynElem<'a, T>>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> DynList<'a, T> {
    pub fn new<I>(i: I) -> Self
    where
        I: IntoIterator<Item = DynElem<'a, T>>,
    {
        DynList {
            inner: i.into_iter().collect(),
            phantom: PhantomData,
        }
    }

    pub fn inner_ref(&self) -> DynList<&T> {
        DynList {
            inner: self.inner.iter().map(DynElem::inner_ref).collect(),
            phantom: PhantomData,
        }
    }

    pub fn iter(&self) -> DynListIntoIter<&T> {
        self.inner_ref().into_iter()
    }
}

impl<'a, T> fmt::Display for DynList<'a, T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;

        for (i, elem) in self.inner.iter().enumerate() {
            if i != 0 {
                write!(f, ",")?;
            }

            write!(f, "{}", elem)?;
        }

        write!(f, "]")?;
        Ok(())
    }
}

impl<'a, T: 'a> IntoIterator for DynList<'a, T> {
    type Item = T;
    type IntoIter = DynListIntoIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        DynListIntoIter {
            inner: Box::new(self.inner.into_iter().flat_map(IntoIterator::into_iter)),
        }
    }
}

#[macro_export]
macro_rules! dyn_list {
    ($($elem:tt),*) => {{
        use dynlist::DynList;

        DynList::new(vec![$(dyn_elem!($elem)),*])
    }};
}

pub struct DynListIntoIter<'a, T: 'a> {
    inner: Box<Iterator<Item = T> + 'a>,
}

impl<'a, T: 'a> Iterator for DynListIntoIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
