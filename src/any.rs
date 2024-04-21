pub trait StableAny: Send + Sync {
    fn id(&self) -> u128;
}
pub trait UStableAny {
    const TYPE_ID: u128;
    fn downcast_ref<T: StableAny + 'static>(&self) -> Option<&T>;
    fn downcast_mut<T: StableAny + 'static>(&mut self) -> Option<&mut T>;
    fn downcast_ref_unchecked<T: StableAny + 'static>(&self) -> &T;
    fn downcast_mut_unchecked<T: StableAny + 'static>(&mut self) -> &mut T;
    fn is<T: StableAny>(src: T) -> bool;
}

impl StableAny for () {
    fn id(&self) -> u128 {
        0
    }
}
impl StableAny for &str {
    fn id(&self) -> u128 {
        1
    }
}
