// fn main() {
//     let vs = vec![1, 2, 3];

//     for v in vs {
//         // consumes vs, owned v
//     }

//     let vs = vec![1, 2, 3];
//     for v in vs.iter() {
//         // borrows vs, & to v
//     }

//     for v in &vs {
//         // equivalent to vs.iter()
//     }
// }

// Iterator VS IntoIterator
// IntoIterator 从定义上来看, 最直接的作用就是: 将 Self 的所有权 转移到 Iterator 上
// IntoIterator 直接将内容器的 Iter 转移出来

pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    front_inner: Option<<O::Item as IntoIterator>::IntoIter>,
    back_inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    pub fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            front_inner: None,
            back_inner: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;

    // 例如: [['a'], ['b'], ['c'], ['d']]
    // inner 会指向数组中的数组，然后再从数组里面取元素，当该数组元素被取完后，会指向下一个数组继续取元素
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut front_iter) = self.front_inner {
                if let Some(i) = front_iter.next() {
                    return Some(i);
                }
                self.front_inner = None;
            }

            if let Some(next_inner) = self.outer.next() {
                self.front_inner = Some(next_inner.into_iter())
            } else {
                // return None if end
                return self.back_inner.as_mut()?.next();
            }
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    /// let numbers = vec![1, 2, 3, 4, 5, 6];
    ///
    /// let mut iter = numbers.iter();
    ///
    /// assert_eq!(Some(&1), iter.next());
    /// assert_eq!(Some(&6), iter.next_back());
    /// assert_eq!(Some(&5), iter.next_back());
    /// assert_eq!(Some(&2), iter.next());
    /// assert_eq!(Some(&3), iter.next());
    /// assert_eq!(Some(&4), iter.next());
    /// assert_eq!(None, iter.next());
    /// assert_eq!(None, iter.next_back());
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut back_iter) = self.back_inner {
                if let Some(i) = back_iter.next_back() {
                    return Some(i);
                }
                self.back_inner = None;
            }

            if let Some(next_back_inner) = self.outer.next_back() {
                self.back_inner = Some(next_back_inner.into_iter())
            } else {
                // return None if end
                return self.front_inner.as_mut()?.next_back();
            }
        }
    }
}

pub trait IteratorExt: Iterator + Sized {
    fn our_flatten(self) -> Flatten<Self>
    where
        Self::Item: IntoIterator;
}

impl<T> IteratorExt for T
where
    T: Iterator,
{
    fn our_flatten(self) -> Flatten<Self>
    where
        Self::Item: IntoIterator,
    {
        flatten(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0)
    }

    #[test]
    fn empty_wide() {
        assert_eq!(flatten(vec![Vec::<()>::new(), vec![], vec![]]).count(), 0)
    }

    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1)
    }

    #[test]
    fn two() {
        assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 2)
    }

    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).count(), 2)
    }

    #[test]
    fn reverse() {
        assert_eq!(
            flatten(std::iter::once(vec!["a", "b"]))
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        )
    }

    #[test]
    fn reverse_wide() {
        assert_eq!(
            flatten(vec![vec!["a"], vec!["b"]])
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        )
    }

    #[test]
    fn both_ends() {
        let mut iter = flatten(vec![vec!["a", "b"], vec!["c", "d"]]);
        assert_eq!(iter.next(), Some("a"));
        assert_eq!(iter.next_back(), Some("d"));
        assert_eq!(iter.next(), Some("b"));
        assert_eq!(iter.next_back(), Some("c"));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn inf() {
        let mut iter = flatten((0..).map(|i| 0..i));
        // 0 => 0..0 => empty
        // 1 => 0..1 => [0]
        // 2 => 0..2 => [0, 1]
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
    }

    #[test]
    fn deep() {
        assert_eq!(
            flatten(flatten(flatten(vec![vec![vec![vec![1, 2]]]]))).count(),
            2
        )
    }

    #[test]
    fn ext() {
        assert_eq!(vec![vec![1, 2]].into_iter().our_flatten().count(), 2)
    }
}
