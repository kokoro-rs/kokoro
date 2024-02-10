use kokoro::{
    context::{scope::LocalCache, Context},
    disposable::{Disposable, DisposableHandle},
};
use std::{
    sync::{atomic::AtomicBool, Arc},
    thread::JoinHandle,
};

pub trait ThreadContext {
    fn spawn<F: FnOnce(&Self, SignalHandle) + Send + 'static>(
        &self,
        f: F,
    ) -> DisposableHandle<ThreadHandle<()>>;
}
impl<T: LocalCache + 'static> ThreadContext for Context<T> {
    fn spawn<F: FnOnce(&Self, SignalHandle) + Send + 'static>(
        &self,
        f: F,
    ) -> DisposableHandle<ThreadHandle<()>> {
        let ctx = self.with(self.scope());
        let signal_handle = SignalHandle::new();
        let s = signal_handle.get();
        let join_handle = std::thread::spawn(move || f(&ctx, s));
        DisposableHandle::new(ThreadHandle::new(join_handle, signal_handle))
    }
}

pub struct ThreadHandle<T>(pub JoinHandle<T>, pub SignalHandle);
impl<T> ThreadHandle<T> {
    #[inline]
    pub fn new(join_handle: JoinHandle<T>, signal_handle: SignalHandle) -> Self {
        Self(join_handle, signal_handle)
    }
    #[inline]
    pub fn yes(&self) {
        self.1.yes()
    }
    #[inline]
    pub fn end(self) {
        self.yes();
        self.0.join().unwrap();
    }
}
pub struct SignalHandle(Arc<AtomicBool>);
impl SignalHandle {
    #[inline]
    pub fn new() -> Self {
        Self(Arc::new(AtomicBool::new(false)))
    }
    #[inline]
    pub fn is(&self) -> bool {
        unsafe { *self.0.as_ptr() }
    }
    #[inline]
    pub fn get(&self) -> Self {
        Self(self.0.clone())
    }
    #[inline]
    pub fn yes(&self) {
        unsafe { *(self.0.as_ptr()) = true }
    }
    #[inline]
    pub fn no(&self) {
        unsafe { *(self.0.as_ptr()) = false }
    }
}
impl Disposable for ThreadHandle<()> {
    #[inline]
    fn dispose(self) {
        self.end()
    }
}