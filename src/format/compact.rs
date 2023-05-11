// Copyright (c) 2023 tracing-subscriber-init developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use tracing::{metadata::LevelFilter, Subscriber};
use tracing_subscriber::{
    filter::Filtered,
    fmt::{
        self,
        format::{Compact, DefaultFields, Format},
    },
    Layer,
};

use crate::{utils::get_effective_level, TracingConfig};

/// Create a [`Compact`](tracing_subscriber::fmt::format::Compact) format layer configured from the given [`TracingConfig`].
///
/// # Example
/// ```rust
/// # use anyhow::Result;
/// # use tracing::info;
/// # use tracing_subscriber::Layer;
/// # use tracing_subscriber_init::{compact, set_default, TestAll, TracingConfig};
/// #
/// # pub fn main() -> Result<()> {
/// let config = TestAll;
/// let (layer, level_filter) = compact(&config);
/// let layer = layer.with_filter(level_filter);
/// let _unused = set_default(vec![layer.boxed()]);
/// info!("info level");
/// #   Ok(())
/// # }
/// ```
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
/// ```rust
/// # use anyhow::Result;
/// # use tracing::info;
/// # use tracing_subscriber::Layer;
/// # use tracing_subscriber_init::{compact_filtered, set_default, TestAll, TracingConfig};
/// #
/// # pub fn main() -> Result<()> {
/// let config = TestAll;
/// let layer = compact_filtered(&config);
/// let _unused = set_default(vec![layer.boxed()]);
/// info!("info level");
/// #   Ok(())
/// # }
/// ```
pub fn filtered<C, S>(
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

#[cfg(test)]
mod test {
    use tracing::{debug, error, info, span, trace, warn, Level};
    use tracing_subscriber::Layer;

    use super::filtered as compact_filtered;

    use crate::{utils::test::TestConfig, set_default, TestAll};

    #[test]
    fn compact_filtered_works() {
        let config = TestConfig;
        let layer = compact_filtered(&config);
        let _unused = set_default(vec![layer.boxed()]);
        let span = span!(Level::INFO, "compact_filtered_works");
        let _enter = span.enter();
        error!("error level");
        warn!("warn level");
        info!("info level");
        debug!("debug level");
        trace!("trace level");
    }

    #[test]
    fn compact_filtered_all_works() {
        let config = TestAll;
        let layer = compact_filtered(&config);
        let _unused = set_default(vec![layer.boxed()]);
        let span = span!(Level::TRACE, "compact_filtered_all_works");
        let _enter = span.enter();
        error!("error level");
        warn!("warn level");
        info!("info level");
        debug!("debug level");
        trace!("trace level");
    }

    #[cfg(feature = "tstime")]
    #[test]
    fn compact_utc_works() {
        use super::compact;
        use time::format_description::well_known::Iso8601;
        use tracing_subscriber::fmt::time::UtcTime;

        let config = TestConfig;
        let (layer, level_filter) = compact(&config);
        let filtered_layer = layer
            .with_timer(UtcTime::new(Iso8601::DEFAULT))
            .with_filter(level_filter);
        let _unused = set_default(vec![filtered_layer.boxed()]);
        let span = span!(Level::INFO, "compact_utc_works");
        let _enter = span.enter();
        error!("error level");
        warn!("warn level");
        info!("info level");
        debug!("debug level");
        trace!("trace level");
    }
}
