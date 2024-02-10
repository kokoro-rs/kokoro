pub trait Disposable {
    fn dispose(self);
}
pub struct DisposableHandle<D: Disposable>(D);
impl<D: Disposable> DisposableHandle<D> {
    #[inline]
    pub fn new(disposable: D) -> Self {
        Self(disposable)
    }
    #[inline]
    pub fn dispose(self) {
        self.0.dispose()
    }
}
#[cfg(feature = "nightly")]
impl<D: Disposable> FnOnce<()> for DisposableHandle<D> {
    type Output = ();

    #[inline]
    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        self.0.dispose();
    }
}
