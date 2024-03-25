use std::marker::PhantomData;
use std::sync::Arc;

use crate::context::scope::Mode;
use crate::query::Query;
use crate::{
    context::{scope::Resource, Context},
    event::*,
};

/// Abstract subscriber
pub trait Subscriber<const N: usize, Q, E, R: Resource + ?Sized, M: Mode>
where
    Q: 'static + Send + Sync,
{
    /// Run the subscriber
    #[inline(always)]
    fn run(&mut self, e: Arc<dyn Event>, ctx: &Context<R, M>) {
        if self.sub(e.as_ref()) {
            unsafe { self.run_uncheck(e, ctx) }
        }
    }
    /// Run the subscriber without checking if the event matches
    unsafe fn run_uncheck(&mut self, e: Arc<dyn Event>, ctx: &Context<R, M>);
    /// Whether to subscribe to an event
    fn sub(&self, e: &dyn Event) -> bool;
}

impl<F, R: Resource, M: Mode> Subscriber<10, (), (), R, M> for F
where
    F: FnMut() + 'static,
{
    #[inline(always)]
    unsafe fn run_uncheck(&mut self, _: Arc<dyn Event>, _: &Context<R, M>) {
        self()
    }
    #[inline(always)]
    fn sub(&self, _: &dyn Event) -> bool {
        true
    }
}
impl<F, R: Resource + 'static, M: Mode + 'static> Subscriber<11, (), (), R, M> for F
where
    F: FnMut(&Context<R, M>) + 'static,
{
    #[inline(always)]
    unsafe fn run_uncheck(&mut self, _: Arc<dyn Event>, ctx: &Context<R, M>) {
        self(ctx)
    }

    #[inline(always)]
    fn sub(&self, _: &dyn Event) -> bool {
        true
    }
}
impl<F, Q, R: Resource, M: Mode, E> Subscriber<20, Q, E, R, M> for F
where
    F: FnMut(Q) + 'static,
    Q: Query<E> + 'static + Send + Sync,
    E: EventID + 'static + Send + Sync,
{
    #[inline(always)]
    unsafe fn run_uncheck(&mut self, e: Arc<dyn Event>, _: &Context<R, M>) {
        self(Q::create(Arc::clone(unsafe {
            &*(&e as *const Arc<dyn Event> as *const Arc<E>)
        })))
    }
    #[inline(always)]
    fn sub(&self, e: &dyn Event) -> bool {
        Q::sub(e)
    }
}
impl<F, Q, R: Resource + 'static, M: Mode + 'static, E> Subscriber<21, Q, E, R, M> for F
where
    F: FnMut(&Context<R, M>, Q) + 'static,
    Q: Query<E> + 'static + Send + Sync,
    E: EventID + 'static + Send + Sync,
{
    #[inline(always)]
    unsafe fn run_uncheck(&mut self, e: Arc<dyn Event>, ctx: &Context<R, M>) {
        self(
            ctx,
            Q::create(Arc::clone(unsafe {
                &*(&e as *const Arc<dyn Event> as *const Arc<E>)
            })),
        )
    }

    #[inline(always)]
    fn sub(&self, e: &dyn Event) -> bool {
        Q::sub(e)
    }
}

/// Wrapper for storing subscribers
pub struct SubscriberCache<Sub, const N: usize, Q, E, R: Resource + ?Sized, M: Mode>
where
    Sub: Subscriber<N, Q, E, R, M>,
    Q: 'static + Send + Sync,
    E: 'static + Send + Sync,
{
    inner: Sub,
    _e: PhantomData<E>,
    _q: PhantomData<Q>,
    _s: PhantomData<R>,
    _g: PhantomData<M>,
}

impl<Sub, const N: usize, Q, E, R: Resource + ?Sized, M: Mode> SubscriberCache<Sub, N, Q, E, R, M>
where
    Sub: Subscriber<N, Q, E, R, M>,
    Q: 'static + Send + Sync,
    E: 'static + Send + Sync,
{
    /// Wrap a subscriber
    #[inline(always)]
    pub fn new(sub: Sub) -> Self {
        Self {
            inner: sub,
            _e: PhantomData,
            _q: PhantomData,
            _s: PhantomData,
            _g: PhantomData,
        }
    }
}

/// Abstract subscriber erasure types
pub trait ISubscriber<R: Resource + ?Sized, M: Mode> {
    /// Run the subscriber
    fn run(&mut self, e: Arc<dyn Event + Send + Sync>, ctx: &Context<R, M>);
    /// Run the subscriber without checking if the event matches
    unsafe fn run_uncheck(&mut self, e: Arc<dyn Event>, ctx: &Context<R, M>);
    /// Whether to subscribe to an event
    fn sub(&self, e: &dyn Event) -> bool;
}

impl<'a, Sub, const N: usize, Q, E, R: Resource + ?Sized, M: Mode> ISubscriber<R, M>
    for SubscriberCache<Sub, N, Q, E, R, M>
where
    Sub: Subscriber<N, Q, E, R, M>,
    Q: 'static + Send + Sync,
    E: 'static + Send + Sync,
{
    #[inline(always)]
    fn run(&mut self, e: Arc<dyn Event + Send + Sync>, ctx: &Context<R, M>) {
        self.inner.run(e, ctx)
    }

    #[inline(always)]
    unsafe fn run_uncheck(&mut self, e: Arc<dyn Event>, ctx: &Context<R, M>) {
        self.inner.run_uncheck(e, ctx)
    }

    #[inline(always)]
    fn sub(&self, e: &dyn Event) -> bool {
        self.inner.sub(e)
    }
}
