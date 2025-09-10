// Copyright (c) 2023 tracing-subscriber-init developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use anyhow::Result;
use tracing::subscriber::DefaultGuard;
use tracing_subscriber::{
    Layer, Registry, prelude::__tracing_subscriber_SubscriberExt, registry, util::SubscriberInitExt,
};

/// Creates a [`Registry`](tracing_subscriber::registry::Registry), adds the given [`Layer`s](tracing_subscriber::Layer)
/// to it, and sets itself as the default subscriber in the current scope, returning a guard that will unset it
/// when dropped.
///
/// See [`set_default`](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/util/trait.SubscriberInitExt.html#method.set_default)
///
/// # Errors
/// * An error can be thrown on registry initialization
///
#[must_use]
pub fn set_default(layers: Vec<Box<dyn Layer<Registry> + Send + Sync + 'static>>) -> DefaultGuard {
    registry().with(layers).set_default()
}

/// Creates a [`Registry`](tracing_subscriber::registry::Registry), adds the given [`Layer`s](tracing_subscriber::Layer)
/// to it, and attempts to set itself as the global default subscriber in the current scope, panicking if this fails.
///
/// See [`init`](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/util/trait.SubscriberInitExt.html#method.init)
///
/// # Errors
/// * An error can be thrown on registry initialization
///
pub fn init(layers: Vec<Box<dyn Layer<Registry> + Send + Sync + 'static>>) {
    registry().with(layers).init();
}

/// Creates a [`Registry`](tracing_subscriber::registry::Registry), adds the given [`Layer`s](tracing_subscriber::Layer)
/// to it, and attempts to set self as the global default subscriber in the current scope, returning an error if one is already set.
///
/// See [`try_init`](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/util/trait.SubscriberInitExt.html#method.try_init)
///
/// # Errors
/// * An error can be thrown on registry initialization
///
pub fn try_init(layers: Vec<Box<dyn Layer<Registry> + Send + Sync + 'static>>) -> Result<()> {
    Ok(registry().with(layers).try_init()?)
}

#[cfg(test)]
mod test {
    use tracing_subscriber::Layer;

    use crate::{TestAll, full_filtered};

    use super::set_default;

    #[test]
    fn set_default_works() {
        let config = TestAll;
        let layer = full_filtered(&config);
        let _unused = set_default(vec![layer.boxed()]);
    }
}
