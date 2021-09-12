#[derive(Copy, Clone)]
#[repr(C)]
pub struct SaltyId {
    salt: u16,
    index: u16,
}

impl SaltyId {
    pub fn new(salt: u16, index: u16) -> SaltyId {
        SaltyId {salt, index}
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
struct Salty<T: Copy + Default> {
    pub salt: u16,
    pub data: T,
}

/// A buffer intended for long-lived items which have stable weak references.
/// Items are always allocated at the first empty slot to help maintain locality.
/// New items cannot be added to the buffer once full.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SaltyBuffer<T: Copy + Default, const N: usize> {
    items: [Salty<T>; N],
    head: u16,
    count: u16,
}

impl<T: Copy + Default, const N: usize> SaltyBuffer<T, { N }> {
    pub fn new() -> SaltyBuffer<T, { N }> {
        SaltyBuffer {
            items: [Salty {salt: 0, data: T::default() }; N],
            head: 0,
            count: 0,
        }
    }

    pub fn capacity(&self) -> u16 {
        N as u16
    }

    pub fn count(&self) -> u16 {
        self.count
    }

    pub fn free(&self) -> u16 {
        (N as u16) - self.count
    }

    pub fn add(&mut self, item: T) -> Option<SaltyId> {
        if self.items[self.head as usize].salt != 0 {
            return None;
        }
        let slot = &mut self.items[self.head as usize];
        slot.data = item;
        self.count += 1;
        slot.salt = slot.salt.checked_add(1).unwrap_or(1);
        let id = SaltyId::new(self.head, slot.salt);
        while (self.head as usize) < N && self.items[self.head as usize].salt != 0 {
            self.head += 1;
        }
        Some(id)
    }

    pub fn get(&self, id: SaltyId) -> Option<&T> {
        match &self.items[id.index as usize] {
            Salty {salt, data: item} if *salt == id.salt && *salt != 0 => Some(item),
            _ => None
        }
    }

    pub fn get_mut(&mut self, id: SaltyId) -> Option<&mut T> {
        match &mut self.items[id.index as usize] {
            Salty {salt, data: item} if *salt == id.salt && *salt != 0 => Some(item),
            _ => None
        }
    }

    pub fn remove(&mut self, id: SaltyId) -> Option<T> {
        let mut slot = self.items[id.index as usize];
        if slot.salt == id.salt && slot.salt != 0 {
            let data = slot.data;
            slot.data = T::default();
            slot.salt = 0;
            self.head = std::cmp::min(id.index, self.head);
            self.count -= 1;
            return Some(data);
        }
        None
    }

    pub fn iter(&self) -> impl Iterator<Item=(SaltyId, &T)> {
        self.items.iter().enumerate().filter_map(|(i, item)| {
            if item.salt != 0 {
                return Some((SaltyId::new(item.salt, i as u16), &item.data));
            }
            None
        }).take(self.count as usize)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=(SaltyId, &mut T)> {
        self.items.iter_mut().enumerate().filter_map(|(i, item)| {
            if item.salt != 0 {
                return Some((SaltyId::new(item.salt, i as u16), &mut item.data));
            }
            None
        }).take(self.count as usize)
    }
}

mod tests {
    use super::*;
    use crate::math::Vec3f;

    #[derive(Copy, Clone, Default)]
    #[repr(C)]
    struct Example {
        a: bool,
        b: bool
    }

    #[test]
    fn test_salty_size() {
        assert_eq!(4, std::mem::size_of::<SaltyId>());

        assert_eq!(2, std::mem::size_of::<Example>());
        assert_eq!(4, std::mem::size_of::<Salty<Example>>());
        assert_eq!(4100, std::mem::size_of::<SaltyBuffer<Example, 1024>>());
    }
}
