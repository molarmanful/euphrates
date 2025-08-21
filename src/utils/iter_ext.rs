pub trait IterExt: Iterator {
    fn skip_while_ok<T, E, P>(self, predicate: P) -> SkipWhileOk<Self, P>
    where
        Self: Iterator<Item = Result<T, E>> + Sized,
        P: FnMut(&T) -> Result<bool, E>,
    {
        SkipWhileOk {
            iter: self,
            predicate,
            flag: false,
        }
    }
}

impl<I: Iterator> IterExt for I {}

#[derive(Clone)]
pub struct SkipWhileOk<I, P> {
    iter: I,
    flag: bool,
    predicate: P,
}

impl<T, E, I, P> Iterator for SkipWhileOk<I, P>
where
    I: Iterator<Item = Result<T, E>>,
    P: FnMut(&T) -> Result<bool, E>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.iter.next()?;
        if self.flag {
            return Some(r);
        }
        for r in &mut self.iter {
            if let o @ Some(_) = r
                .and_then(|t| {
                    (self.predicate)(&t).map(|b| {
                        self.flag = !b;
                        self.flag.then_some(t)
                    })
                })
                .transpose()
            {
                return o;
            }
        }
        None
    }
}
