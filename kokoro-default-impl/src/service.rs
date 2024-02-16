/// Initializing service
#[macro_export]
macro_rules! init_service {
    ($ctx:expr, $name:expr, $trait:ident) => {
        $ctx.cache().default($name, || {
            let service_cache = ::std::sync::Arc::downgrade(&$ctx.scope().resource) as ::std::sync::Weak<dyn $trait>;
            ::std::sync::Arc::new(service_cache)
        })
    };
}
/// Getting service
#[macro_export]
macro_rules! get_service {
    ($ctx:expr, $name:expr, $trait:ident) => {
        $ctx.cache().get::<::std::sync::Weak<dyn $trait>>($name)?.upgrade()
    };
}