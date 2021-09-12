use std::ops::Index;

/// A fixed buffer intended for short-lived entries which do not need stable long term
/// referencing.
pub struct FixedRingBuffer<T: Copy, const N: usize> {
    pub items: [Option<T>; N],
    head: usize,
    count: usize,
}

impl<T: Copy, const N: usize> FixedRingBuffer<T, { N }> {
    pub fn new() -> FixedRingBuffer<T, { N }> {
        FixedRingBuffer {
            items: [Option::<T>::None; N],
            head: 0,
            count: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn free(&self) -> usize {
        N - self.count
    }

    #[inline]
    fn prev_index(i: usize) -> usize {
        if i == 0 {
            return N - 1;
        }
        i - 1
    }

    #[inline]
    fn next_index(i: usize) -> usize {
        if i == N - 1 {
            return 0;
        }
        i + 1
    }

    // indices are inclusive
    fn find_reverse(&self, start: usize, end: usize, occupied: bool) -> Option<usize> {
        let mut i = start;
        loop {
            if self.items[i].is_some() == occupied {
                return Some(i);
            }
            if i == end {
                break;
            }
            i = Self::prev_index(i);
        }
        None
    }

    fn compact(&mut self) {
        let mut empty_candidate = Self::prev_index(self.head);
        while let Some(found_empty) = self.find_reverse(empty_candidate, Self::next_index(self.head), false) {
            if found_empty == self.head {
                break;
            }
            let nonempty_candidate = Self::prev_index(found_empty);
            if let Some(found_nonempty) = self.find_reverse(nonempty_candidate, self.head, true) {
                self.items[found_empty] = self.items[found_nonempty];
                self.items[found_nonempty] = None;
                empty_candidate = Self::prev_index(found_empty);
            } else {
                break;
            }
        }
    }

    pub fn push(&mut self, item: T) -> usize {
        if self.count != N && self.items[self.head].is_some() {
            self.compact();
        }
        let i = self.head;
        if self.items[i].is_none() {
            self.count += 1;
        }
        self.items[i] = Some(item);
        self.head = Self::next_index(i);
        i
    }

    pub fn try_push(&mut self, item: T) -> Option<usize> {
        if self.count == N {
            return None;
        }
        if self.items[self.head].is_some() {
            self.compact();
        }
        if self.items[self.head].is_some() {
            return None;
        }
        let i = self.head;
        self.items[i] = Some(item);
        self.head = Self::next_index(i);
        self.count += 1;
        Some(i)
    }

    pub fn remove(&mut self, i: usize) -> bool {
        if self.items[i].is_some() {
            self.items[i] = None;
            self.count -= 1;
            return true;
        }
        false
    }
}

impl<T: Copy, const N: usize> Index<usize> for FixedRingBuffer<T, { N }> {
    type Output = Option<T>;

    fn index(&self, i: usize) -> &Self::Output {
        &self.items[i]
    }
}

// impl<T: Copy, const N: usize> IntoIterator for FixedRingBuffer<T, { N }> {
//     type Item = Option<T>;
//     type IntoIter = std::slice::Iter<'a, Self::Item>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self.items.into_iter()
//     }
// }

mod tests {
    use super::*;

    #[test]
    fn test_fixedringbuffer_single() {
        let mut buf = FixedRingBuffer::<u32, 1>::new();
        //['_]
        assert_eq!(1, buf.capacity());
        assert_eq!(0, buf.count());
        assert_eq!(1, buf.free());

        assert_eq!(0, buf.push(1));
        //['1]
        assert_eq!(1, buf.count());
        assert_eq!(0, buf.free());
        assert_eq!(Some(1), buf[0]);

        assert_eq!(0, buf.push(2));
        //['2]
        assert_eq!(1, buf.count());
        assert_eq!(0, buf.free());
        assert_eq!(Some(2), buf[0]);

        assert_eq!(None, buf.try_push(3));
        //['2]
    }

    #[test]
    fn test_fixedringbuffer_double() {
        let mut buf = FixedRingBuffer::<u32, 2>::new();
        //['_,_]
        assert_eq!(2, buf.capacity());
        assert_eq!(0, buf.count());
        assert_eq!(2, buf.free());

        assert_eq!(0, buf.push(1));
        //[1,'_]
        assert_eq!(1, buf.count());
        assert_eq!(1, buf.free());
        assert_eq!(Some(1), buf[0]);

        assert_eq!(1, buf.push(2));
        //['1,2]
        assert_eq!(2, buf.count());
        assert_eq!(0, buf.free());
        assert_eq!(Some(1), buf[0]);
        assert_eq!(Some(2), buf[1]);

        assert_eq!(None, buf.try_push(3));
        //['1,2]

        assert_eq!(true, buf.remove(1));
        assert_eq!(false, buf.remove(1));
        //['1,_]

        assert_eq!(1, buf.count());
        assert_eq!(1, buf.free());
        assert_eq!(None, buf[1]);

        assert_eq!(Some(0), buf.try_push(3));
        //[3,'1]

        assert_eq!(None, buf.try_push(4));
        assert_eq!(1, buf.push(4));
        //[3,'4]
    }

    #[test]
    fn test_fixedringbuffer_typical() {
        let mut buf = FixedRingBuffer::<u32, 8>::new();
        buf.push(1);
        buf.push(2);
        buf.push(3);
        buf.push(4);
        buf.push(5);
        buf.push(6);
        //[1,2,3,4,5,6,'_,_]
        assert_eq!(6, buf.count());
        assert_eq!(2, buf.free());
        assert_eq!(Some(1), buf[0]);
        assert_eq!(Some(6), buf[5]);
        assert_eq!(None, buf[6]);

        buf.remove(1);
        //[1,_,3,4,5,6,'_,_]
        assert_eq!(5, buf.count());

        assert_eq!(Some(6), buf.try_push(7));
        assert_eq!(Some(7), buf.try_push(8));
        //['1,_,3,4,5,6,7,8]
        assert_eq!(1, buf.free());

        assert_eq!(0, buf.push(9));
        //[9,'1,3,4,5,6,7,8]
        assert_eq!(0, buf.free());

        buf.remove(2);
        buf.remove(3);
        buf.remove(5);
        buf.remove(7);
        buf.remove(7); //idempotent
        //[9,'1,_,_,5,_,7,_]
        assert_eq!(4, buf.free());

        assert_eq!(1, buf.push(10));
        //[9,10,'_,_,_,1,5,7]
        assert_eq!(3, buf.free());

        assert_eq!(Some(2), buf.try_push(11));
        assert_eq!(Some(3), buf.try_push(12));
        assert_eq!(Some(4), buf.try_push(13));
        assert_eq!(None, buf.try_push(14));
        //[9,10,11,12,13,'1,5,7]
        assert_eq!(Some(1), buf[5]);
    }
}
