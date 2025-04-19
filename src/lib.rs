#[derive(Debug, Default, Clone)]
pub struct Stack<T> {
    head: Option<Box<Node<T>>>,
}

#[derive(Debug, Clone)]
struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, item: T) {
        self.head = Some(Box::new(Node {
            item,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.item
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.item)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.item)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter(self.head.as_deref())
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut(self.head.as_deref_mut())
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        while let Some(mut node) = self.head.take() {
            self.head = node.next.take();
        }
    }
}

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

pub struct IntoIter<T>(Stack<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T>(Option<&'a Node<T>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.map(|node| {
            self.0 = node.next.as_deref();
            &node.item
        })
    }
}

pub struct IterMut<'a, T>(Option<&'a mut Node<T>>);

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node| {
            self.0 = node.next.as_deref_mut();
            &mut node.item
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;
    #[test]
    fn basics() {
        let mut s = Stack::new();
        s.push(2);
        s.push(3);
        s.push(4);

        assert_eq!(4, s.pop().unwrap());
        assert_eq!(3, s.pop().unwrap());

        s.push(5);

        assert_eq!(5, s.pop().unwrap());
        assert_eq!(2, s.pop().unwrap());
        assert_eq!(None, s.pop());
    }

    #[test]
    fn large_stack() {
        let mut s = Stack::new();
        for i in 1..=100_000 {
            s.push(i);
        }

        for i in (1..=100_000).rev() {
            assert_eq!(i, s.pop().unwrap());
        }
    }

    #[test]
    fn into_iter() {
        let mut s = Stack::new();
        for i in 1..=5 {
            s.push(i);
        }

        let mut iter = s.into_iter();
        for i in (1..=5).rev() {
            assert_eq!(i, iter.next().unwrap());
        }
        assert_eq!(None, iter.next());
    }

    #[test]
    fn iter() {
        let mut s = Stack::new();

        for i in 1..=5 {
            s.push(i);
        }

        let mut iter = s.iter();
        for i in (1..=5).rev() {
            assert_eq!(&i, iter.next().unwrap());
        }
        assert_eq!(None, iter.next());
    }

    #[test]
    fn iter_mut() {
        let mut s = Stack::new();

        for i in 1..=5 {
            s.push(i);
        }

        let mut iter = s.iter_mut();
        for mut i in (1..=5).rev() {
            assert_eq!(&mut i, iter.next().unwrap());
        }
        assert_eq!(None, iter.next());
    }
}
