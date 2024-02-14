use crate::{
    context::{scope::Resource, Context},
    event::*,
};
use std::marker::PhantomData;
use crate::context::scope::Mode;

/// Abstract subscriber
pub trait Subscriber<Et, R: Resource + ?Sized, M: Mode>
    where
        Et: 'static + Send + Sync,
{
    /// Run the subscriber
    #[inline(always)]
    fn run(&mut self, e: &dyn Event, ctx: &Context<R, M>) {
        unsafe { self.run_uncheck(e, ctx) }
    }
    /// Run the subscriber without checking if the event matches
    unsafe fn run_uncheck(&mut self, e: &dyn Event, ctx: &Context<R, M>);
    /// Whether to subscribe to an event
    fn sub(&self, e: &dyn Event) -> bool;
    /// Identifier of the subscribed event
    fn id(&self) -> &EventId;
}

impl<F, R: Resource, M: Mode> Subscriber<(), R, M> for F
    where
        F: FnMut() + 'static,
{
    #[inline(always)]
    unsafe fn run_uncheck(&mut self, _: &dyn Event, _: &Context<R, M>) {
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

impl<F, Et, R: Resource, M: Mode> Subscriber<Et, R, M> for F
    where
        F: FnMut(&Et) + 'static,
        Et: EventID + 'static + Send + Sync,
{
    #[inline(always)]
    unsafe fn run_uncheck(&mut self, e: &dyn Event, _: &Context<R, M>) {
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

impl<F, R: Resource + 'static, M: Mode + 'static> Subscriber<Context<R, M>, R, M> for F
    where
        F: FnMut(&Context<R, M>) + 'static,
{
    #[inline(always)]
    unsafe fn run_uncheck(&mut self, _: &dyn Event, ctx: &Context<R, M>) {
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

impl<F, Et, R: Resource + 'static, M: Mode + 'static> Subscriber<(Context<R, M>, Et), R, M> for F
    where
        F: FnMut(&Context<R, M>, &Et) + 'static,
        Et: EventID + 'static + Send + Sync,
{
    #[inline(always)]
    unsafe fn run_uncheck(&mut self, e: &dyn Event, ctx: &Context<R, M>) {
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
pub struct SubscriberCache<Sub, Et, R: Resource + ?Sized, M: Mode>
    where
        Sub: Subscriber<Et, R, M>,
        Et: 'static + Send + Sync,
{
    inner: Sub,
    _et: PhantomData<Et>,
    _s: PhantomData<R>,
    _g: PhantomData<M>,
}

impl<Sub, Et, R: Resource + ?Sized, M: Mode> SubscriberCache<Sub, Et, R, M>
    where
        Sub: Subscriber<Et, R, M>,
        Et: 'static + Send + Sync,
{
    /// Wrap a subscriber
    #[inline(always)]
    pub fn new(sub: Sub) -> Self {
        Self {
            inner: sub,
            _et: PhantomData,
            _s: PhantomData,
            _g: PhantomData,
        }
    }
}

/// Abstract subscriber erasure types
pub trait ISubscriber<R: Resource + ?Sized, M: Mode> {
    /// Run the subscriber
    fn run(&mut self, e: &dyn Event, ctx: &Context<R, M>);
    /// Run the subscriber without checking if the event matches
    unsafe fn run_uncheck(&mut self, e: &dyn Event, ctx: &Context<R, M>);
    /// Whether to subscribe to an event
    fn sub(&self, e: &dyn Event) -> bool;
    /// Identifier of the subscribed event
    fn id(&self) -> &EventId;
}

impl<Sub, Et, R: Resource + ?Sized, M: Mode> ISubscriber<R, M> for SubscriberCache<Sub, Et, R, M>
    where
        Sub: Subscriber<Et, R, M>,
        Et: 'static + Send + Sync,
{
    #[inline(always)]
    fn run(&mut self, e: &dyn Event, ctx: &Context<R, M>) {
        self.inner.run(e, ctx)
    }

    #[inline(always)]
    unsafe fn run_uncheck(&mut self, e: &dyn Event, ctx: &Context<R, M>) {
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
