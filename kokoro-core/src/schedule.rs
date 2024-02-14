use crate::context::scope::{Mode, Resource};
use crate::context::Context;
use crate::disposable::Disposable;
use crate::event::{Event, EventId};
use crate::subscriber::{ISubscriber, Subscriber, SubscriberCache};
use dashmap::DashMap;
use parking_lot::RwLock;
use rayon::prelude::*;
use std::sync::Arc;

/// `Arc<RwLock<Option<Box<dyn ISubscriber<T> + Send + Sync>>>>`
pub type AROBS<R, M> = Arc<RwLock<Option<Box<dyn ISubscriber<R, M> + Send + Sync>>>>;

/// Schedule, to hold the subscriber
pub struct Schedule<R: Resource + ?Sized, M: Mode + 'static> {
    /// The subscribers
    pub subscribers: DashMap<EventId, Vec<AROBS<R, M>>>,
    /// AROBS that are assigned an empty value for reuse
    none: DashMap<EventId, Arc<RwLock<Vec<AROBS<R, M>>>>>,
}

impl<R: Resource + Send + Sync + ?Sized + 'static, M: Mode> Schedule<R, M> {
    /// Create a schedule
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            subscribers: DashMap::new(),
            none: DashMap::new(),
        }
    }
    /// Insert a subscriber into the schedule
    #[inline(always)]
    pub fn insert<Sub, Et>(&self, sub: Sub) -> WithNoneList<AROBS<R, M>, R, M>
        where
            Sub: Subscriber<Et, R, M> + 'static + Send + Sync,
            Et: 'static + Send + Sync,
    {
        let id = *sub.id();
        let boxed: Box<dyn ISubscriber<R, M> + Send + Sync> = Box::new(SubscriberCache::new(sub));
        if let Some(none_vec) = self.none.get(&id) {
            if !none_vec.read().is_empty() {
                let none = none_vec.write().pop().unwrap();
                none.write().replace(boxed);
                return self.with_none_list(none, &id);
            };
        }
        let inner = Arc::new(RwLock::new(Some(boxed)));
        if let Some(mut sub_vec) = self.subscribers.get_mut(&id) {
            sub_vec.push(Arc::clone(&inner));
        } else {
            let sub_vec = vec![Arc::clone(&inner)];
            self.subscribers.insert(id, sub_vec);
        }
        self.with_none_list(inner, &id)
    }
    /// Triggers a subscriber who has subscribed to an event in the schedule
    #[inline(always)]
    pub fn trigger(&self, e: Arc<dyn Event + Send + Sync>, ctx: &Context<R, M>) {
        if let Some(subs) = self.subscribers.get_mut(e.event_id()) {
            subs.par_iter().for_each_with(e.as_ref(), |e, sub| {
                if let Some(sub) = sub.write().as_mut() {
                    sub.run(*e, ctx)
                };
            });
        }
    }
    #[inline(always)]
    fn with_none_list<N>(&self, some: N, id: &EventId) -> WithNoneList<N, R, M> {
        let vec = if let Some(none_vec) = self.none.get(id) {
            Arc::clone(&none_vec)
        } else {
            let none_vec = Arc::new(RwLock::new(Vec::new()));
            self.none.insert(*id, Arc::clone(&none_vec));
            none_vec
        };
        WithNoneList::new(vec, some)
    }
}

/// Wrapper for recycling AROBS-None
pub struct WithNoneList<N, R: Resource + ?Sized, M> {
    none_list: Arc<RwLock<Vec<AROBS<R, M>>>>,
    some: N,
}

impl<N, R: Resource + ?Sized, M> WithNoneList<N, R, M> {
    #[inline(always)]
    fn new(none_list: Arc<RwLock<Vec<AROBS<R, M>>>>, some: N) -> Self {
        Self { none_list, some }
    }
}

impl<R: Resource + ?Sized, M> Disposable for AROBS<R, M> {
    #[inline(always)]
    fn dispose(self) {
        let _ = self.write().take();
    }
}

impl<R: Resource + ?Sized, M> Disposable for WithNoneList<AROBS<R, M>, R, M> {
    #[inline(always)]
    fn dispose(self) {
        let some_clone = Arc::clone(&self.some);
        self.some.dispose();
        self.none_list.write().push(some_clone);
    }
}
