/// What can be disposed of
pub trait Disposable {
    /// Dispose of it
    fn dispose(self);
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
impl<D: Disposable> Disposable for DisposableHandle<D> {
    fn dispose(self) {
        self.0.dispose()
    }
}
pub fn dispose<D: Disposable>(disposable: D) {
    disposable.dispose()
}
#[cfg(feature = "nightly")]
impl<D: Disposable> FnOnce<()> for DisposableHandle<D> {
    type Output = ();

    #[inline(always)]
    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        self.0.dispose();
    }
}
