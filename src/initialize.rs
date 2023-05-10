// Copyright (c) 2023 tracing-subscriber-init developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use anyhow::Result;
use tracing::{metadata::LevelFilter, subscriber::DefaultGuard, Level, Subscriber};
use tracing_subscriber::{
    filter::Filtered,
    fmt::{
        self,
        format::{Compact, DefaultFields, Format, Full, Pretty},
    },
    prelude::__tracing_subscriber_SubscriberExt,
    registry,
    util::SubscriberInitExt,
    Layer, Registry,
};

#[cfg(feature = "json")]
use {tracing_subscriber::fmt::format::Json, tracing_subscriber::fmt::format::JsonFields};

use crate::TracingConfig;

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

/// Create a [`Full`](tracing_subscriber::fmt::format::Full) format layer configured from the given [`TracingConfig`].
///
/// # Example
pub fn full<C, S>(config: &C) -> (fmt::Layer<S, DefaultFields, Format<Full>>, LevelFilter)
where
    C: TracingConfig,
    S: Subscriber,
    for<'a> S: tracing_subscriber::registry::LookupSpan<'a>,
{
    let layer = fmt::layer()
        .with_ansi(config.with_ansi())
        .with_file(config.with_file())
        .with_level(config.with_level())
        .with_target(config.with_target())
        .with_thread_ids(config.with_thread_ids())
        .with_thread_names(config.with_thread_names())
        .with_line_number(config.with_line_number());
    let layer = if let Some(fmt_span) = config.with_span_events() {
        layer.with_span_events(fmt_span)
    } else {
        layer
    };
    let level = get_effective_level(config.quiet(), config.verbose());
    let level_filter = LevelFilter::from(level);
    (layer, level_filter)
}

/// Create a [`Full`](tracing_subscriber::fmt::format::Full) format filtered layer configured from the given [`TracingConfig`].
///
/// # Example
pub fn full_filtered<C, S>(
    config: &C,
) -> Filtered<fmt::Layer<S, DefaultFields, Format<Full>>, LevelFilter, S>
where
    C: TracingConfig,
    S: Subscriber,
    for<'a> S: tracing_subscriber::registry::LookupSpan<'a>,
{
    let (layer, level_filter) = full(config);
    layer.with_filter(level_filter)
}

/// Create a [`Compact`](tracing_subscriber::fmt::format::Compact) format layer configured from the given [`TracingConfig`].
///
/// # Example
pub fn compact<C, S>(config: &C) -> (fmt::Layer<S, DefaultFields, Format<Compact>>, LevelFilter)
where
    C: TracingConfig,
    S: Subscriber,
    for<'a> S: tracing_subscriber::registry::LookupSpan<'a>,
{
    let layer = fmt::layer()
        .compact()
        .with_ansi(config.with_ansi())
        .with_file(config.with_file())
        .with_level(config.with_level())
        .with_target(config.with_target())
        .with_thread_ids(config.with_thread_ids())
        .with_thread_names(config.with_thread_names())
        .with_line_number(config.with_line_number());
    let layer = if let Some(fmt_span) = config.with_span_events() {
        layer.with_span_events(fmt_span)
    } else {
        layer
    };
    let level = get_effective_level(config.quiet(), config.verbose());
    let level_filter = LevelFilter::from(level);
    (layer, level_filter)
}

/// Create a [`Compact`](tracing_subscriber::fmt::format::Compact) format filtered layer configured from the given [`TracingConfig`].
///
/// # Example
pub fn compact_filtered<C, S>(
    config: &C,
) -> Filtered<fmt::Layer<S, DefaultFields, Format<Compact>>, LevelFilter, S>
where
    C: TracingConfig,
    S: Subscriber,
    for<'a> S: tracing_subscriber::registry::LookupSpan<'a>,
{
    let (layer, level_filter) = compact(config);
    layer.with_filter(level_filter)
}

/// Create a [`Pretty`](tracing_subscriber::fmt::format::Pretty) format layer configured from the given [`TracingConfig`].
///
/// # Example
pub fn pretty<C, S>(config: &C) -> (fmt::Layer<S, Pretty, Format<Pretty>>, LevelFilter)
where
    C: TracingConfig,
    S: Subscriber,
    for<'a> S: tracing_subscriber::registry::LookupSpan<'a>,
{
    let layer = fmt::layer()
        .pretty()
        .with_ansi(config.with_ansi())
        .with_file(config.with_file())
        .with_level(config.with_level())
        .with_target(config.with_target())
        .with_thread_ids(config.with_thread_ids())
        .with_thread_names(config.with_thread_names())
        .with_line_number(config.with_line_number());
    let layer = if let Some(fmt_span) = config.with_span_events() {
        layer.with_span_events(fmt_span)
    } else {
        layer
    };
    let level = get_effective_level(config.quiet(), config.verbose());
    let level_filter = LevelFilter::from(level);
    (layer, level_filter)
}

/// Create a [`Pretty`](tracing_subscriber::fmt::format::Pretty) format filtered layer configured from the given [`TracingConfig`].
///
/// # Example
pub fn pretty_filtered<C, S>(
    config: &C,
) -> Filtered<fmt::Layer<S, Pretty, Format<Pretty>>, LevelFilter, S>
where
    C: TracingConfig,
    S: Subscriber,
    for<'a> S: tracing_subscriber::registry::LookupSpan<'a>,
{
    let layer = fmt::layer()
        .pretty()
        .with_ansi(config.with_ansi())
        .with_file(config.with_file())
        .with_level(config.with_level())
        .with_target(config.with_target())
        .with_thread_ids(config.with_thread_ids())
        .with_thread_names(config.with_thread_names())
        .with_line_number(config.with_line_number());
    let layer = if let Some(fmt_span) = config.with_span_events() {
        layer.with_span_events(fmt_span)
    } else {
        layer
    };
    let level = get_effective_level(config.quiet(), config.verbose());
    let level_filter = LevelFilter::from(level);
    layer.with_filter(level_filter)
}

#[cfg(feature = "json")]
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
/// Create a [`Json`](tracing_subscriber::fmt::format::Json) format layer configured from the given [`TracingConfig`].
///
/// # Example
pub fn json<C, S>(config: &C) -> (fmt::Layer<S, JsonFields, Format<Json>>, LevelFilter)
where
    C: TracingConfig,
    S: Subscriber,
    for<'a> S: tracing_subscriber::registry::LookupSpan<'a>,
{
    let layer = fmt::layer()
        .json()
        .with_ansi(config.with_ansi())
        .with_file(config.with_file())
        .with_level(config.with_level())
        .with_target(config.with_target())
        .with_thread_ids(config.with_thread_ids())
        .with_thread_names(config.with_thread_names())
        .with_line_number(config.with_line_number())
        .with_current_span(config.with_current_span())
        .with_span_list(config.with_span_list());

    let layer = if let Some(fmt_span) = config.with_span_events() {
        layer.with_span_events(fmt_span)
    } else {
        layer
    };
    let level = get_effective_level(config.quiet(), config.verbose());
    let level_filter = LevelFilter::from(level);
    (layer, level_filter)
}

#[cfg(feature = "json")]
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
/// Create a [`Json`](tracing_subscriber::fmt::format::Json) format layer configured from the given [`TracingConfig`].
///
/// # Example
pub fn json_filtered<C, S>(
    config: &C,
) -> Filtered<fmt::Layer<S, JsonFields, Format<Json>>, LevelFilter, S>
where
    C: TracingConfig,
    S: Subscriber,
    for<'a> S: tracing_subscriber::registry::LookupSpan<'a>,
{
    let (layer, level_filter) = json(config);
    layer.with_filter(level_filter)
}

#[cfg(debug_assertions)]
#[allow(dead_code)]
fn get_effective_level(quiet: u8, verbosity: u8) -> Level {
    if verbosity > 0 {
        match verbosity {
            1 => Level::DEBUG,
            _ => Level::TRACE,
        }
    } else if quiet > 0 {
        match quiet {
            1 => Level::WARN,
            _ => Level::ERROR,
        }
    } else {
        Level::INFO
    }
}

#[cfg(not(debug_assertions))]
#[allow(dead_code)]
fn get_effective_level(_quiet: u8, verbosity: u8) -> Level {
    match verbosity {
        0 => Level::ERROR,
        1 => Level::WARN,
        2 => Level::INFO,
        3 => Level::DEBUG,
        4 | _ => Level::TRACE,
    }
}

#[cfg(test)]
mod test {
    use super::{
        compact_filtered, full_filtered, get_effective_level, pretty_filtered, set_default,
    };
    use crate::config::test::{TestAllConfig, TestConfig};
    use tracing::{debug, error, info, span, trace, warn, Level};
    use tracing_subscriber::Layer;
    #[cfg(feature = "json")]
    use {super::json_filtered, crate::config::test::TestJson};

    #[cfg(debug_assertions)]
    #[test]
    fn get_effective_level_works() {
        assert_eq!(Level::INFO, get_effective_level(0, 0));
        assert_eq!(Level::DEBUG, get_effective_level(0, 1));
        assert_eq!(Level::TRACE, get_effective_level(0, 2));
        assert_eq!(Level::TRACE, get_effective_level(0, 3));
        assert_eq!(Level::WARN, get_effective_level(1, 0));
        assert_eq!(Level::DEBUG, get_effective_level(1, 1));
        assert_eq!(Level::TRACE, get_effective_level(1, 2));
        assert_eq!(Level::TRACE, get_effective_level(1, 3));
        assert_eq!(Level::ERROR, get_effective_level(2, 0));
        assert_eq!(Level::DEBUG, get_effective_level(2, 1));
        assert_eq!(Level::TRACE, get_effective_level(2, 2));
        assert_eq!(Level::TRACE, get_effective_level(2, 3));
    }

    #[cfg(not(debug_assertions))]
    #[test]
    fn get_effective_level_works() {
        assert_eq!(Level::ERROR, get_effective_level(0, 0));
        assert_eq!(Level::WARN, get_effective_level(0, 1));
        assert_eq!(Level::INFO, get_effective_level(0, 2));
        assert_eq!(Level::DEBUG, get_effective_level(0, 3));
        assert_eq!(Level::TRACE, get_effective_level(0, 4));
        assert_eq!(Level::TRACE, get_effective_level(0, 5));
    }

    #[test]
    fn full_filtered_works() {
        let config = TestConfig;
        let layer = full_filtered(&config);
        let _unused = set_default(vec![layer.boxed()]);
        let span = span!(Level::INFO, "full_filtered_works");
        let _enter = span.enter();
        info!("info level");
    }

    #[test]
    fn full_filtered_all_works() {
        let config = TestAllConfig;
        let layer = full_filtered(&config);
        let _unused = set_default(vec![layer.boxed()]);
        let span = span!(Level::TRACE, "full_filtered_all_works");
        let _enter = span.enter();
        error!("error level");
        warn!("warn level");
        info!("info level");
        debug!("debug level");
        trace!("trace level");
    }

    #[test]
    fn compact_filtered_works() {
        let config = TestConfig;
        let layer = compact_filtered(&config);
        let _unused = set_default(vec![layer.boxed()]);
        let span = span!(Level::INFO, "compact_filtered_works");
        let _enter = span.enter();
        info!("info level");
    }

    #[test]
    fn pretty_filtered_works() {
        let config = TestConfig;
        let layer = pretty_filtered(&config);
        let _unused = set_default(vec![layer.boxed()]);
        let span = span!(Level::INFO, "pretty_filtered_works");
        let _enter = span.enter();
        info!("info level");
        debug!("debug level");
        trace!("trace level");
    }

    #[cfg(feature = "tstime")]
    #[test]
    fn full_utc_works() {
        use crate::full;
        use time::format_description::well_known::Iso8601;
        use tracing_subscriber::fmt::time::UtcTime;

        let config = TestConfig;
        let (layer, level_filter) = full(&config);
        let filtered_layer = layer
            .with_timer(UtcTime::new(Iso8601::DEFAULT))
            .with_filter(level_filter);
        let _unused = set_default(vec![filtered_layer.boxed()]);
        let span = span!(Level::INFO, "full_utc_works");
        let _enter = span.enter();
        info!("info level");
    }

    #[test]
    #[cfg(feature = "json")]
    fn json_filtered_works() {
        let config = TestJson;
        let layer = json_filtered(&config);
        let _unused = set_default(vec![layer.boxed()]);
        let span = span!(Level::INFO, "json_filtered_works");
        let _enter = span.enter();
        info!("info level");
    }
}
