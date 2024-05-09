use const_fnv1a_hash::fnv1a_hash_64;
use std::any;
pub trait KAny: Send + Sync {
    fn stable_id(&self) -> u64;
}
pub trait KID {
    const KID: u64;
}
impl<T> KID for T {
    const KID: u64 = {
        let name: &'static [u8] = any::type_name::<T>().as_bytes();
        fnv1a_hash_64(name, None)
    };
}
impl dyn KAny {
    pub fn downcast_ref<T: KID + 'static>(&self) -> Option<&T> {
        if self.is::<T>() {
            Some(self.downcast_ref_unchecked())
        } else {
            None
        }
    }
    pub fn downcast_mut<T: KID + 'static>(&mut self) -> Option<&mut T> {
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
    pub fn is<T: KID>(&self) -> bool {
        T::KID == self.stable_id()
    }
}
impl<T: Send + Sync> KAny for T {
    fn stable_id(&self) -> u64 {
        T::KID
    }
}
pub const fn stable_id<T>() -> u64 {
    T::KID
}
