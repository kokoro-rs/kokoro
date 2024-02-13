use super::scope::{LocalCache, Scope};
use crate::disposable::DisposableHandle;
use crate::event::Event;
use crate::schedule::{Schedule, WithNoneList, AROBS};
use crate::subscriber::Subscriber;
use either::*;
use flume::{Receiver, Sender};
use std::sync::{Arc, Weak};

type SSE = dyn Event + Send + Sync;
/// The heart of Kokoro
pub struct Context<T: LocalCache + ?Sized> {
    scope: Either<Weak<Scope<T>>, Arc<Scope<T>>>,
    receiver: Receiver<Arc<SSE>>,
    sender: Sender<Arc<SSE>>,
}
impl<T: LocalCache + ?Sized + 'static> Context<T> {
    /// Create a new Context
    #[inline(always)]
    pub fn new(scope: Arc<Scope<T>>, mpsc: (Sender<Arc<SSE>>, Receiver<Arc<SSE>>)) -> Self {
        Self {
            receiver: mpsc.1,
            sender: mpsc.0,
            scope: Right(scope),
        }
    }
    /// Get the scope of the Context
    #[inline(always)]
    pub fn scope(&self) -> Weak<Scope<T>> {
        match &self.scope {
            Left(s) => Weak::clone(&s),
            Right(s) => Arc::downgrade(&s),
        }
    }
    /// Place the current context in a new scope
    #[inline(always)]
    pub fn with<N: LocalCache + ?Sized>(&self, scope: Weak<Scope<N>>) -> Context<N> {
        Context {
            scope: Left(scope),
            receiver: self.receiver.clone(),
            sender: self.sender.clone(),
        }
    }
    /// Gets the schedule of the node in the current scope.
    ///
    /// Note: It is not a get-schedule that includes the parent node
    #[inline(always)]
    pub fn schedule(&self) -> Arc<Schedule<T>> {
        self.scope().upgrade().unwrap().schedule()
    }
    /// Publish an event to the main channel
    #[inline(always)]
    pub fn publish<E>(&self, event: E) -> &Self
    where
        E: Event + Send + Sync,
    {
        self.sender.send(Arc::new(event)).unwrap();
        self
    }
    /// Register a subscriber for the main channel
    #[inline]
    pub fn subscribe<Sub, Et>(&self, sub: Sub) -> DisposableHandle<WithNoneList<AROBS<T>, T>>
    where
        Sub: Subscriber<Et, T> + 'static + Send + Sync,
        Et: 'static + Sync + Send,
    {
        DisposableHandle::new(self.schedule().insert(sub))
    }
    /// Get a consumer of the primary channel
    ///
    /// Note: An event is consumed only once, even if multiple consumers cannot process the same event
    #[inline(always)]
    pub fn receiver(&self) -> Receiver<Arc<SSE>> {
        self.receiver.clone()
    }
    /// Get the cache from Context.scope
    #[inline(always)]
    pub fn cache(&self)->Option<Arc<T>>{
        Some(Arc::clone(&self.scope().upgrade()?.cache))
    }
}
/* impl<T: LocalCache + 'static> Deref for Context<T> {
    type Target = Weak<T>;

    fn deref(&self) -> &Self::Target {
    }
}*/
