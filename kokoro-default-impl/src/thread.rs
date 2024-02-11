use kokoro_core::{
    context::{scope::LocalCache, Context},
    disposable::{Disposable, DisposableHandle},
};
use std::{
    sync::{atomic::AtomicBool, Arc},
    thread::{self, Builder, JoinHandle},
};
/// Spawn threads for the Context
pub trait ThreadContext {
    /// Spawn a thread
    fn spawn<F: FnOnce(&Self, SignalHandle) + Send + 'static>(
        &self,
        f: F,
    ) -> DisposableHandle<ThreadHandle<()>>;
    /// Spawn a thread with Builder
    fn spawn_with_builder<F: FnOnce(&Self, SignalHandle) + Send + 'static>(
        &self,
        builder: Builder,
        f: F,
    ) -> Result<DisposableHandle<ThreadHandle<()>>, std::io::Error>;
}
impl<T: LocalCache + 'static> ThreadContext for Context<T> {
    #[inline(always)]
    fn spawn<F: FnOnce(&Self, SignalHandle) + Send + 'static>(
        &self,
        f: F,
    ) -> DisposableHandle<ThreadHandle<()>> {
        let ctx = self.with(self.scope());
        let signal_handle = SignalHandle::new();
        let s = signal_handle.clone();
        let join_handle = thread::spawn(move || f(&ctx, s));
        DisposableHandle::new(ThreadHandle::new(join_handle, signal_handle))
    }

    fn spawn_with_builder<F: FnOnce(&Self, SignalHandle) + Send + 'static>(
        &self,
        builder: Builder,
        f: F,
    ) -> Result<DisposableHandle<ThreadHandle<()>>, std::io::Error> {
        let ctx = self.with(self.scope());
        let signal_handle = SignalHandle::new();
        let s = signal_handle.clone();
        let join_handle = builder.spawn(move || f(&ctx, s))?;
        Ok(DisposableHandle::new(ThreadHandle::new(
            join_handle,
            signal_handle,
        )))
    }
}
/// A handle used to dispose of a thread
pub struct ThreadHandle<T>(pub JoinHandle<T>, pub SignalHandle);
impl<T> ThreadHandle<T> {
    #[inline(always)]
    fn new(join_handle: JoinHandle<T>, signal_handle: SignalHandle) -> Self {
        Self(join_handle, signal_handle)
    }
    #[inline(always)]
    fn yes(&self) {
        self.1.yes()
    }
    #[inline(always)]
    fn end(self) {
        self.yes();
        self.0.join().unwrap();
    }
}
/// A handle used to pass a single signal
pub struct SignalHandle(Arc<AtomicBool>);
impl SignalHandle {
    /// Creates a handle for passing a single signal
    #[inline(always)]
    pub fn new() -> Self {
        Self(Arc::new(AtomicBool::new(false)))
    }
    /// If the signal is true
    #[inline(always)]
    pub fn is(&self) -> bool {
        unsafe { *self.0.as_ptr() }
    }
    /// Set the signal to true
    #[inline(always)]
    pub fn yes(&self) {
        unsafe { *(self.0.as_ptr()) = true }
    }
    /// Set the signal to false
    #[inline(always)]
    pub fn no(&self) {
        unsafe { *(self.0.as_ptr()) = false }
    }
}
impl Clone for SignalHandle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Disposable for ThreadHandle<()> {
    #[inline(always)]
    fn dispose(self) {
        self.end()
    }
}
