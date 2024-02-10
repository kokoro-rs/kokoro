use crate::{
    context::{scope::LocalCache, Context},
    event::*,
};
use std::marker::PhantomData;
/// Abstract subscriber
pub trait Subscriber<Et, T: LocalCache + ?Sized>
where
    Et: 'static + Send + Sync,
{
    /// Run the subscriber
    #[inline(always)]
    fn run(&mut self, e: &dyn Event, ctx: &Context<T>) {
        unsafe { self.run_uncheck(e, ctx) }
    }
    /// Run the subscriber without checking if the event matches
    unsafe fn run_uncheck(&mut self, e: &dyn Event, ctx: &Context<T>);
    /// Whether to subscribe to an event
    fn sub(&self, e: &dyn Event) -> bool;
    /// Identifier of the subscribed event
    fn id(&self) -> &EventId;
}

impl<F, T: LocalCache> Subscriber<(), T> for F
where
    F: FnMut() + 'static,
{
    #[inline(always)]
    unsafe fn run_uncheck(&mut self, _: &dyn Event, _: &Context<T>) {
        self()
    }
    #[inline(always)]
    fn sub(&self, _: &dyn Event) -> bool {
        true
    }

    #[inline(always)]
    fn id(&self) -> &EventId {
        PhantomEvent::_EVENT_ID
    }
}
impl<F, Et, T: LocalCache> Subscriber<Et, T> for F
where
    F: FnMut(&Et) + 'static,
    Et: EventID + 'static + Send + Sync,
{
    #[inline(always)]
    unsafe fn run_uncheck(&mut self, e: &dyn Event, _: &Context<T>) {
        self(e.downcast_ref_unchecked())
    }
    #[inline(always)]
    fn sub(&self, e: &dyn Event) -> bool {
        e.is::<Et>()
    }

    #[inline(always)]
    fn id(&self) -> &EventId {
        Et::_EVENT_ID
    }
}
impl<F, T: LocalCache + 'static> Subscriber<Context<T>, T> for F
where
    F: FnMut(&Context<T>) + 'static,
{
    #[inline(always)]
    unsafe fn run_uncheck(&mut self, _: &dyn Event, ctx: &Context<T>) {
        self(ctx)
    }

    #[inline(always)]
    fn sub(&self, _: &dyn Event) -> bool {
        true
    }

    #[inline(always)]
    fn id(&self) -> &EventId {
        PhantomEvent::_EVENT_ID
    }
}
impl<F, Et, T: LocalCache + 'static> Subscriber<(Context<T>, Et), T> for F
where
    F: FnMut(&Context<T>, &Et) + 'static,
    Et: EventID + 'static + Send + Sync,
{
    #[inline(always)]
    unsafe fn run_uncheck(&mut self, e: &dyn Event, ctx: &Context<T>) {
        self(ctx, e.downcast_ref_unchecked())
    }

    #[inline(always)]
    fn sub(&self, e: &dyn Event) -> bool {
        e.is::<Et>()
    }

    #[inline(always)]
    fn id(&self) -> &EventId {
        Et::_EVENT_ID
    }
}
/// Wrapper for storing subscribers
pub struct SubscriberCache<Sub, Et, T: LocalCache + ?Sized>
where
    Sub: Subscriber<Et, T>,
    Et: 'static + Send + Sync,
{
    inner: Sub,
    _et: PhantomData<Et>,
    _s: PhantomData<T>,
}
impl<Sub, Et, T: LocalCache + ?Sized> SubscriberCache<Sub, Et, T>
where
    Sub: Subscriber<Et, T>,
    Et: 'static + Send + Sync,
{
    /// Wrap a subscriber
    #[inline(always)]
    pub fn new(sub: Sub) -> Self {
        Self {
            inner: sub,
            _et: PhantomData,
            _s: PhantomData,
        }
    }
}
/// Abstract subscriber erasure types
pub trait ISubscriber<T: LocalCache + ?Sized> {
    /// Run the subscriber
    fn run(&mut self, e: &dyn Event, ctx: &Context<T>);
    /// Run the subscriber without checking if the event matches
    unsafe fn run_uncheck(&mut self, e: &dyn Event, ctx: &Context<T>);
    /// Whether to subscribe to an event
    fn sub(&self, e: &dyn Event) -> bool;
    /// Identifier of the subscribed event
    fn id(&self) -> &EventId;
}
impl<Sub, Et, T: LocalCache + ?Sized> ISubscriber<T> for SubscriberCache<Sub, Et, T>
where
    Sub: Subscriber<Et, T>,
    Et: 'static + Send + Sync,
{
    #[inline(always)]
    fn run(&mut self, e: &dyn Event, ctx: &Context<T>) {
        self.inner.run(e, ctx)
    }

    #[inline(always)]
    unsafe fn run_uncheck(&mut self, e: &dyn Event, ctx: &Context<T>) {
        self.inner.run_uncheck(e, ctx)
    }

    #[inline(always)]
    fn sub(&self, e: &dyn Event) -> bool {
        self.inner.sub(e)
    }

    #[inline(always)]
    fn id(&self) -> &EventId {
        self.inner.id()
    }
}
