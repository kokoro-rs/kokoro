use kokoro_core::context::scope::Mode;
use kokoro_core::{
    context::{scope::Resource, Context},
    disposable::{Disposable, DisposableHandle},
};
use std::sync::atomic::Ordering;
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

impl<T: Resource + 'static, M: Mode + 'static> ThreadContext for Context<T, M> {
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
pub struct ThreadHandle<T>(pub Option<JoinHandle<T>>, pub SignalHandle);

impl<T> ThreadHandle<T> {
    #[inline(always)]
    fn new(join_handle: JoinHandle<T>, signal_handle: SignalHandle) -> Self {
        Self(Some(join_handle), signal_handle)
    }
    #[inline(always)]
    fn done(&self) {
        self.1.done()
    }
    #[inline(always)]
    fn done_join(&mut self) {
        self.done();
        if let Some(handle) = self.0.take() {
            handle.join().unwrap();
        }
    }
}

/// A handle used to pass a single signal
pub struct SignalHandle {
    single: Arc<AtomicBool>,
    stopped: Arc<AtomicBool>,
}

impl SignalHandle {
    /// Creates a handle for passing a single signal
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            single: Arc::new(AtomicBool::new(true)),
            stopped: Arc::new(AtomicBool::new(false)),
        }
    }
    /// Set done
    pub fn done(&self) {
        self.single.store(false, Ordering::Relaxed);
    }
}

impl Clone for SignalHandle {
    fn clone(&self) -> Self {
        Self {
            single: Arc::clone(&self.single),
            stopped: Arc::clone(&self.stopped),
        }
    }
}

impl Iterator for SignalHandle {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stopped.load(Ordering::Relaxed) {
            None
        } else {
            thread::yield_now();
            let signal = !self.single.load(Ordering::Relaxed);
            if signal {
                self.stopped.store(true, Ordering::Relaxed);
            }
            Some(signal)
        }
    }
}

impl<T> Disposable for ThreadHandle<T> {
    #[inline(always)]
    unsafe fn dispose(&mut self) {
        self.done_join();
    }
}
