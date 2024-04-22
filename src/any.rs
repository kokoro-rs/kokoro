use std::hash::Hasher;
use std::thread_local;

use dashmap::DashMap;
thread_local! {
    pub static ID_MAP:DashMap<&'static str,u64> = DashMap::new();
}

pub trait IStableAny: Send + Sync {
    fn id(&self) -> u64;
}
pub trait UStableAny {
    const TYPE_ID: u64;
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
        T::TYPE_ID == self.id()
    }
}

impl<T: Send + Sync> IStableAny for T {
    fn id(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        let name: &'static str = std::any::type_name_of_val(self);
        hasher.write(name.as_bytes());
        let hash = hasher.finish();
        ID_MAP.with(|map| {
            if let Some(id) = map.get(name) {
                return *id;
            }
            map.insert(name, hash);
            hash
        })
    }
}

#[macro_export]
macro_rules! downcast_ref {
    ($value:expr, $type:ty) => {{
        use std::hash::Hasher;
        let type_name = std::any::type_name::<$type>();
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        hasher.write(type_name.as_bytes());
        let hash = hasher.finish();
        let id = kokoro_neo::any::ID_MAP.with(|map| {
            if let Some(id) = map.get(type_name) {
                return *id;
            }
            map.insert(type_name, hash);
            hash
        });
        if $value.id() == id {
            Some($value.downcast_ref_unchecked::<$type>())
        } else {
            Option::<&$type>::None
        }
    }};
}
