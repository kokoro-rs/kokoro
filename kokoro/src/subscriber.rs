use crate::{
    context::{scope::LocalCache, Context},
    event::*,
};
use std::marker::PhantomData;
pub trait Subscriber<Et, T: LocalCache + ?Sized>
where
    Et: 'static + Send + Sync,
{
    fn run(&mut self, e: &dyn Event, ctx: &Context<T>) {
        unsafe { self.run_uncheck(e, ctx) }
    }
    unsafe fn run_uncheck(&mut self, e: &dyn Event, ctx: &Context<T>);
    fn sub(&self, e: &dyn Event) -> bool;
    fn id(&self) -> &EventId;
}

impl<F, T: LocalCache> Subscriber<(), T> for F
where
    F: FnMut() + 'static,
{
    unsafe fn run_uncheck(&mut self, _: &dyn Event, _: &Context<T>) {
        self()
    }
    fn sub(&self, _: &dyn Event) -> bool {
        true
    }

    fn id(&self) -> &EventId {
        PhantomEvent::_EVENT_ID
    }
}
impl<F, Et, T: LocalCache> Subscriber<Et, T> for F
where
    F: FnMut(&Et) + 'static,
    Et: EventID + 'static + Send + Sync,
{
    unsafe fn run_uncheck(&mut self, e: &dyn Event, _: &Context<T>) {
        self(e.downcast_ref_unchecked())
    }
    fn sub(&self, e: &dyn Event) -> bool {
        e.is::<Et>()
    }

    fn id(&self) -> &EventId {
        Et::_EVENT_ID
    }
}
impl<F, T: LocalCache + 'static> Subscriber<Context<T>, T> for F
where
    F: FnMut(&Context<T>) + 'static,
{
    unsafe fn run_uncheck(&mut self, _: &dyn Event, ctx: &Context<T>) {
        self(ctx)
    }

    fn sub(&self, _: &dyn Event) -> bool {
        true
    }

    fn id(&self) -> &EventId {
        PhantomEvent::_EVENT_ID
    }
}
impl<F, Et, T: LocalCache + 'static> Subscriber<(Context<T>, Et), T> for F
where
    F: FnMut(&Context<T>, &Et) + 'static,
    Et: EventID + 'static + Send + Sync,
{
    unsafe fn run_uncheck(&mut self, e: &dyn Event, ctx: &Context<T>) {
        self(ctx, e.downcast_ref_unchecked())
    }

    fn sub(&self, e: &dyn Event) -> bool {
        e.is::<Et>()
    }

    fn id(&self) -> &EventId {
        Et::_EVENT_ID
    }
}

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
    pub fn new(sub: Sub) -> Self {
        Self {
            inner: sub,
            _et: PhantomData,
            _s: PhantomData,
        }
    }
}
pub trait ISubscriber<T: LocalCache + ?Sized> {
    fn run(&mut self, e: &dyn Event, ctx: &Context<T>);
    unsafe fn run_uncheck(&mut self, e: &dyn Event, ctx: &Context<T>);
    fn sub(&self, e: &dyn Event) -> bool;
    fn id(&self) -> &EventId;
}
impl<Sub, Et, T: LocalCache + ?Sized> ISubscriber<T> for SubscriberCache<Sub, Et, T>
where
    Sub: Subscriber<Et, T>,
    Et: 'static + Send + Sync,
{
    fn run(&mut self, e: &dyn Event, ctx: &Context<T>) {
        self.inner.run(e, ctx)
    }

    unsafe fn run_uncheck(&mut self, e: &dyn Event, ctx: &Context<T>) {
        self.inner.run_uncheck(e, ctx)
    }

    fn sub(&self, e: &dyn Event) -> bool {
        self.inner.sub(e)
    }

    fn id(&self) -> &EventId {
        self.inner.id()
    }
}
