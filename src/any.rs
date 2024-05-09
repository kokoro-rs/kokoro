use const_fnv1a_hash::fnv1a_hash_64;
use std::any;
pub trait StableAny: Send + Sync {
    fn stable_id(&self) -> u64;
}
pub trait StableID {
    const TYPE_ID: u64;
}
impl<T> StableID for T {
    const TYPE_ID: u64 = {
        let name: &'static [u8] = any::type_name::<T>().as_bytes();
        fnv1a_hash_64(name, None)
    };
}
impl dyn StableAny {
    pub fn downcast_ref<T: StableID + 'static>(&self) -> Option<&T> {
        if self.is::<T>() {
            Some(self.downcast_ref_unchecked())
        } else {
            None
        }
    }
    pub fn downcast_mut<T: StableID + 'static>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            Some(self.downcast_mut_unchecked())
        } else {
            None
        }
    }
    pub fn downcast_ref_unchecked<T: 'static>(&self) -> &T {
        unsafe { &*(self as *const _ as *const T) }
    }
    pub fn downcast_mut_unchecked<T: 'static>(&mut self) -> &mut T {
        unsafe { &mut *(self as *mut _ as *mut T) }
    }
    pub fn is<T: StableID>(&self) -> bool {
        T::TYPE_ID == self.stable_id()
    }
}
impl<T: Send + Sync> StableAny for T {
    fn stable_id(&self) -> u64 {
        T::TYPE_ID
    }
}
pub const fn stable_id<T>() -> u64 {
    T::TYPE_ID
}
