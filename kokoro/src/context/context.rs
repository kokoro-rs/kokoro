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
    #[inline]
    pub fn new(scope: Arc<Scope<T>>, mpsc: (Sender<Arc<SSE>>, Receiver<Arc<SSE>>)) -> Self {
        Self {
            receiver: mpsc.1,
            sender: mpsc.0,
            scope,
        }
    }
    /// Get the scope of the Context
    #[inline]
    pub fn scope(&self) -> Arc<Scope<T>> {
        Arc::clone(&self.scope)
    }
    /// Place the current context in a new scope
    #[inline]
    pub fn with<N: LocalCache + ?Sized>(&self, scope: Arc<Scope<N>>) -> Context<N> {
        Context {
            scope,
            receiver: self.receiver.clone(),
            sender: self.sender.clone(),
        }
    }
    /// Gets the timeline of the node in the current scope.
    ///
    /// Note: It is not a get-schedule that includes the parent node
    #[inline]
    pub fn schedule(&self) -> Arc<Schedule<T>> {
        self.scope().schedule()
    }
    #[inline]
    pub fn publish<E>(&self, event: E) -> &Self
    where
        E: Event + Send + Sync,
    {
        self.sender.send(Arc::new(event)).unwrap();
        self
    }
    #[inline]
    pub fn subscribe<Sub, Et>(&self, sub: Sub) -> DisposableHandle<WithNoneList<AROBS<T>, T>>
    where
        Sub: Subscriber<Et, T> + 'static + Send + Sync,
        Et: 'static + Sync + Send,
    {
        DisposableHandle::new(self.schedule().insert(sub))
    }
    #[inline]
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
/*pub fn default_runner(ctx: Context<RootScope>) {
    for e in ctx.receiver() {
        let ctx = ctx.get_global();
        thread::spawn(move || {
            ctx.all_schedule()
                .par_bridge()
                .for_each(|s| s.read().trigger(Arc::clone(&e), &ctx))
        });
    }
}*/
