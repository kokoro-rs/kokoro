use super::scope::{LocalCache, Scope};
use crate::disposable::DisposableHandle;
use crate::event::Event;
use crate::schedule::{Schedule, WithNoneList, AROBS};
use crate::subscriber::Subscriber;
use flume::{Receiver, Sender};
use std::ops::Deref;
use std::sync::Arc;

type SSE = dyn Event + Send + Sync;

/// The heart of Kokoro
pub struct Context<T: LocalCache + ?Sized> {
    scope: Arc<Scope<T>>,
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
            scope,
        }
    }
    /// Get the scope of the Context
    #[inline(always)]
    pub fn scope(&self) -> Arc<Scope<T>> {
        Arc::clone(&self.scope)
    }
    /// Place the current context in a new scope
    #[inline(always)]
    pub fn with<N: LocalCache + ?Sized>(&self, scope: Arc<Scope<N>>) -> Context<N> {
        Context {
            scope: scope,
            receiver: self.receiver.clone(),
            sender: self.sender.clone(),
        }
    }
    /// Gets the schedule of the node in the current scope.
    ///
    /// Note: It is not a get-schedule that includes the parent node
    #[inline(always)]
    pub fn schedule(&self) -> Arc<Schedule<T>> {
        self.scope().schedule()
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
}
impl<T: LocalCache + 'static> Deref for Context<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.scope.as_ref().cache.as_ref()
    }
}
