use crate::context::scope::{Mode, Resource};
use crate::context::Context;
use crate::disposable::Disposable;
use crate::event::Event;
use crate::subscriber::{ISubscriber, Subscriber, SubscriberCache};
use dashmap::DashMap;
use rayon::prelude::*;
use std::cell::Cell;
use std::sync::Arc;

/// Schedule, to hold the subscriber
pub struct Schedule<R: Resource + ?Sized, M: Mode + 'static> {
    /// The subscribers
    pub subscribers: Arc<DashMap<u128, Box<dyn ISubscriber<R, M> + Send + Sync>>>,
    it_id: Cell<u128>,
}
pub struct SubscriberHandle<R: Resource + ?Sized, M>(
    Arc<DashMap<u128, Box<dyn ISubscriber<R, M> + Send + Sync>>>,
    u128,
);

impl<R: Resource + Send + Sync + ?Sized + 'static, M: Mode> Schedule<R, M> {
    /// Create a schedule
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(DashMap::new()),
            it_id: Cell::new(0),
        }
    }
    /// Insert a subscriber into the schedule
    #[inline(always)]
    pub fn insert<Sub, const N: usize, Q, E>(&self, sub: Sub) -> SubscriberHandle<R, M>
    where
        Sub: Subscriber<N, Q, E, R, M> + 'static + Send + Sync,
        Q: 'static + Send + Sync,
        E: 'static + Send + Sync,
    {
        let boxed: Box<dyn ISubscriber<R, M> + Send + Sync> = Box::new(SubscriberCache::new(sub));
        let id = self.it_id.get().clone();
        self.subscribers.insert(id, boxed);
        self.it_id.set(id + 1);
        SubscriberHandle(Arc::clone(&self.subscribers), id)
    }
    /// Triggers a subscriber who has subscribed to an event in the schedule
    #[inline(always)]
    pub fn trigger(&self, e: Arc<dyn Event + Send + Sync>, ctx: &Context<R, M>) {
        self.subscribers
            .par_iter_mut()
            .for_each_with(&e, |e, mut sub| {
                if sub.sub(e.as_ref()) {
                    sub.run(Arc::clone(*e), ctx)
                }
            });
    }
}
impl<R: Resource + ?Sized, M> Disposable for SubscriberHandle<R, M> {
    #[inline(always)]
    unsafe fn dispose(&mut self) {
        self.0.remove(&self.1);
    }
}
