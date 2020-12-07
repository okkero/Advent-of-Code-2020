use std::io::BufRead;

pub struct Group<'a, I>
where
    I: Iterator,
{
    inner: &'a mut I,
}

impl<I, T> Iterator for Group<'_, I>
where
    I: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

impl<'a, T> Drop for Group<'a, T>
where
    T: Iterator + 'a,
{
    fn drop(&mut self) {
        while let Some(s) = self.next() {}
    }
}

pub struct SplitOnEmptyLine<I> {
    inner: Option<I>,
}

impl<'a, I> Iterator for SplitOnEmptyLine<I>
where
    I: Iterator<Item = &'a str> + 'a,
{
    type Item = Group<'a, I>;

    fn next<'b: 'a>(&'b mut self) -> Option<Self::Item> {
        let inner = self.inner.as_mut()?;
        Some(Group { inner })
    }
}

pub fn split_on_empty_line<'a, I>(lines: I) -> SplitOnEmptyLine<I>
where
    I: Iterator<Item = &'a str>,
{
    unimplemented!()
}
