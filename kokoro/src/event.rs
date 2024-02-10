use std::hash::Hash;

pub trait Event: 'static {
    fn event_id(&self) -> &'static EventId;
}

impl dyn Event {
    #[inline]
    pub fn is<T>(&self) -> bool
    where
        T: EventID,
    {
        self.event_id() == T::_EVENT_ID
    }
    #[inline]
    pub fn downcast_ref_unchecked<N>(&self) -> &N {
        unsafe { &*(self as *const Self as *const N) }
    }
    #[inline]
    pub fn downcast_ref<N>(&self) -> Option<&N>
    where
        N: EventID,
    {
        if self.is::<N>() {
            Some(self.downcast_ref_unchecked())
        } else {
            None
        }
    }
    #[inline]
    pub fn downcast_mut_unchecked<N>(&mut self) -> &mut N {
        unsafe { &mut *(self as *mut Self as *mut N) }
    }
    #[inline]
    pub fn downcast_mut<N>(&mut self) -> Option<&mut N>
    where
        N: EventID,
    {
        if self.is::<N>() {
            Some(self.downcast_mut_unchecked())
        } else {
            None
        }
    }
}

impl dyn Event + Send + Sync {
    #[inline]
    pub fn is<T>(&self) -> bool
    where
        T: EventID,
    {
        T::_EVENT_ID == self.event_id()
    }
    #[inline]
    pub fn downcast_ref_unchecked<N>(&self) -> &N {
        unsafe { &*(self as *const Self as *const N) }
    }
    #[inline]
    pub fn downcast_ref<N>(&self) -> Option<&N>
    where
        N: EventID,
    {
        if self.is::<N>() {
            Some(self.downcast_ref_unchecked())
        } else {
            None
        }
    }
    #[inline]
    pub fn downcast_mut_unchecked<N>(&mut self) -> &mut N {
        unsafe { &mut *(self as *mut Self as *mut N) }
    }
    #[inline]
    pub fn downcast_mut<N>(&mut self) -> Option<&mut N>
    where
        N: EventID,
    {
        if self.is::<N>() {
            Some(self.downcast_mut_unchecked())
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub struct EventId(pub u64);
impl PartialEq for EventId {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for EventId {}
impl Hash for EventId {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, s: &mut H) {
        s.write_u64(self.0)
    }
}
pub trait EventID {
    const _EVENT_ID: &'static EventId;
}

pub struct PhantomEvent;
impl Event for PhantomEvent {
    #[inline]
    fn event_id(&self) -> &'static EventId {
        &EventId(0)
    }
}
impl EventID for PhantomEvent {
    const _EVENT_ID: &'static EventId = &EventId(0);
}
