use std::hash::Hash;
/// Events that can be published
pub trait Event: 'static {
    /// Gets the identifier of the event
    fn event_id(&self) -> &'static EventId;
}

impl dyn Event {
    /// Is it event T
    #[inline(always)]
    pub fn is<T>(&self) -> bool
    where
        T: EventID,
    {
        self.event_id() == T::_EVENT_ID
    }
    /// Downcast to reference N Not checking downcast
    #[inline(always)]
    pub fn downcast_ref_unchecked<N>(&self) -> &N {
        unsafe { &*(self as *const Self as *const N) }
    }
    /// Downcast to reference N
    #[inline(always)]
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
    /// Downcast to a mutable reference N Not checking
    #[inline(always)]
    pub fn downcast_mut_unchecked<N>(&mut self) -> &mut N {
        unsafe { &mut *(self as *mut Self as *mut N) }
    }
    /// Downcast to a mutable reference N
    #[inline(always)]
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
    /// Is it event T
    #[inline(always)]
    pub fn is<T>(&self) -> bool
    where
        T: EventID,
    {
        T::_EVENT_ID == self.event_id()
    }
    /// Downcast to reference N Not checking downcast
    #[inline(always)]
    pub fn downcast_ref_unchecked<N>(&self) -> &N {
        unsafe { &*(self as *const Self as *const N) }
    }
    /// Downcast to reference N
    #[inline(always)]
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
    /// Downcast to a mutable reference N Not checking
    #[inline(always)]
    pub fn downcast_mut_unchecked<N>(&mut self) -> &mut N {
        unsafe { &mut *(self as *mut Self as *mut N) }
    }
    /// Downcast to a mutable reference N
    #[inline(always)]
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
/// The identifier of the event
#[derive(Clone, Copy)]
pub struct EventId(pub u64);
impl PartialEq for EventId {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for EventId {}
impl Hash for EventId {
    #[inline(always)]
    fn hash<H: std::hash::Hasher>(&self, s: &mut H) {
        s.write_u64(self.0)
    }
}
/// Static storage event identifier
pub trait EventID {
    /// Static storage event identifier
    const _EVENT_ID: &'static EventId;
}
/// Phantom events, which are used to trigger subscribers that have not subscribed to any event
pub struct PhantomEvent;
impl Event for PhantomEvent {
    #[inline(always)]
    fn event_id(&self) -> &'static EventId {
        &EventId(0)
    }
}
impl EventID for PhantomEvent {
    const _EVENT_ID: &'static EventId = &EventId(0);
}
