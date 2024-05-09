use std::{
    any,
    hash::{DefaultHasher, Hash, Hasher},
};

use dashmap::DashMap;
use lazy_static::lazy_static;
lazy_static! {
    pub static ref ID_MAP: DashMap<&'static str, u64> = DashMap::new();
}

pub trait IStableAny: Send + Sync {
    fn stable_id(&self) -> u64;
}
pub trait UStableAny {
    const TYPE_ID: u64;
}
impl<T> UStableAny for T {
    const TYPE_ID: u64 = {
        let name: &'static [u8] = any::type_name::<T>().as_bytes();
        let mut hasher = DefaultHasher::new();
        hasher.write(name);
        hasher.finish()
    };
}

pub trait StableAny {
    fn downcast_ref<T: UStableAny + 'static>(&self) -> Option<&T>;
    fn downcast_mut<T: UStableAny + 'static>(&mut self) -> Option<&mut T>;
    fn downcast_ref_unchecked<T: 'static>(&self) -> &T;
    fn downcast_mut_unchecked<T: 'static>(&mut self) -> &mut T;
    fn is<T: UStableAny>(&self) -> bool;
}

impl StableAny for dyn IStableAny {
    fn downcast_ref<T: UStableAny + 'static>(&self) -> Option<&T> {
        if self.is::<T>() {
            Some(self.downcast_ref_unchecked())
        } else {
            None
        }
    }
    fn downcast_mut<T: UStableAny + 'static>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            Some(self.downcast_mut_unchecked())
        } else {
            None
        }
    }
    fn downcast_ref_unchecked<T: 'static>(&self) -> &T {
        unsafe { &*(self as *const _ as *const T) }
    }
    fn downcast_mut_unchecked<T: 'static>(&mut self) -> &mut T {
        unsafe { &mut *(self as *mut _ as *mut T) }
    }
    fn is<T: UStableAny>(&self) -> bool {
        T::TYPE_ID == self.stable_id()
    }
}

impl<T: Send + Sync> IStableAny for T {
    fn stable_id(&self) -> u64 {
        let name: &'static str = std::any::type_name_of_val(self);
        if let Some(id) = ID_MAP.get(name) {
            return *id;
        };
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        hasher.write(name.as_bytes());
        let hash = hasher.finish();
        ID_MAP.insert(name, hash);
        hash
    }
}
