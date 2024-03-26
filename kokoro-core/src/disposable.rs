/// What can be disposed of
pub trait Disposable {
    /// Dispose of it
    unsafe fn dispose(&mut self);
}
/// Used to wrap what can be disposed of
pub struct DisposableHandle<D: Disposable>(D);
impl<D: Disposable> DisposableHandle<D> {
    /// Create a wrapper that can be disposed of
    #[inline(always)]
    pub fn new(disposable: D) -> Self {
        Self(disposable)
    }
}
pub struct DisposableCache(Box<dyn Disposable + Send + Sync>);
impl Disposable for DisposableCache {
    unsafe fn dispose(&mut self) {
        unsafe { self.0.dispose() }
    }
}
impl DisposableCache {
    pub fn warp<D: Disposable + Send + Sync + 'static>(disposable: D) -> Self {
        Self(Box::new(disposable))
    }
}

impl<D: Disposable> Disposable for DisposableHandle<D> {
    unsafe fn dispose(&mut self) {
        self.0.dispose()
    }
}
/// Dispose of something
pub fn dispose<D: Disposable>(disposable: D) {
    #[allow(invalid_reference_casting)]
    unsafe {
        (&mut *(&disposable as *const D as *mut D)).dispose()
    }
    drop(disposable)
}
#[cfg(feature = "nightly")]
impl<D: Disposable> FnOnce<()> for DisposableHandle<D> {
    type Output = ();

    #[inline(always)]
    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        self.0.dispose();
    }
}
