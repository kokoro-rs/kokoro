use std::{ops::Deref, sync::Arc};

use crate::event::{Event, EventID};

pub trait Query<T: EventID>: Deref<Target = T> {
    fn create(n: Arc<T>) -> Self;
    fn sub(n: &dyn Event) -> bool;
}

pub struct EventQuery<E: Event> {
    e: Arc<E>,
}

impl<E: Event> Deref for EventQuery<E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        self.e.as_ref()
    }
}

impl<E: Event + EventID> Query<E> for EventQuery<E> {
    fn create(n: Arc<E>) -> Self {
        EventQuery { e: n }
    }
    fn sub(n: &dyn Event) -> bool {
        n.is::<E>()
    }
}
