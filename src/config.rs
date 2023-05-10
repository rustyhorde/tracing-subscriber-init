// Copyright (c) 2023 tracing-subscriber-init developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use tracing_subscriber::fmt::format::FmtSpan;

/// Implement this trait to supply tracing configuration that can be used to build a [`Layer`](tracing_subscriber::Layer)
/// with functions such as [`full_filtered`](crate::full_filtered).
pub trait Config {
    /// Get the quiet count (these are normally pulled from the command line arguments)
    fn quiet(&self) -> u8;
    /// Get the verbose count (these are normally pulled from the command line arguments)
    fn verbose(&self) -> u8;
    /// Sets whether or not the formatter emits ANSI terminal escape codes for colors and other text formatting.
    /// This defaults to true
    fn with_ansi(&self) -> bool {
        true
    }
    /// Sets whether or not the formatter will include the current span in formatted events.
    /// This defaults to false
    #[cfg(feature = "json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "json")))]
    fn with_current_span(&self) -> bool {
        false
    }
    /// Sets whether or not an event’s source code file path is displayed.
    /// This defaults to false
    fn with_file(&self) -> bool {
        false
    }
    /// Sets whether or not an event’s source code line number is displayed.
    /// This defaults to false
    fn with_line_number(&self) -> bool {
        false
    }
    /// Sets whether or not an event’s level is displayed.
    /// This defaults to true
    fn with_level(&self) -> bool {
        true
    }
    /// Configures how synthesized events are emitted at points in the span lifecycle.
    /// This defaults to [`None`](std::option::Option::None).
    /// See [with_span_event](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/struct.Layer.html#method.with_span_events)
    fn with_span_events(&self) -> Option<FmtSpan> {
        None
    }
    /// Sets whether or not the formatter will include a list (from root to leaf) of all currently entered spans in formatted events.
    /// This defaults to false
    #[cfg(feature = "json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "json")))]
    fn with_span_list(&self) -> bool {
        false
    }
    /// Sets whether or not an event’s target is displayed.
    /// This defaults to false
    fn with_target(&self) -> bool {
        false
    }
    /// Sets whether or not the thread ID of the current thread is displayed when formatting events.
    /// This defaults to false
    fn with_thread_ids(&self) -> bool {
        false
    }
    /// Sets whether or not the name of the current thread is displayed when formatting events.
    /// This defaults to false
    fn with_thread_names(&self) -> bool {
        false
    }
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug)]
pub struct TestAll;

impl Config for TestAll {
    fn quiet(&self) -> u8 {
        0
    }

    fn verbose(&self) -> u8 {
        3
    }

    #[cfg(feature = "json")]
    fn with_current_span(&self) -> bool {
        true
    }

    fn with_file(&self) -> bool {
        true
    }

    fn with_line_number(&self) -> bool {
        true
    }

    fn with_span_events(&self) -> Option<FmtSpan> {
        Some(FmtSpan::FULL)
    }

    #[cfg(feature = "json")]
    fn with_span_list(&self) -> bool {
        true
    }

    fn with_target(&self) -> bool {
        true
    }

    fn with_thread_ids(&self) -> bool {
        true
    }

    fn with_thread_names(&self) -> bool {
        true
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::Config;

    #[derive(Clone, Debug)]
    pub(crate) struct TestConfig;

    impl Config for TestConfig {
        fn quiet(&self) -> u8 {
            0
        }

        fn verbose(&self) -> u8 {
            1
        }
    }

    #[derive(Clone, Debug)]
    pub(crate) struct TestJson;

    impl Config for TestJson {
        fn quiet(&self) -> u8 {
            0
        }

        fn verbose(&self) -> u8 {
            1
        }

        #[cfg(feature = "json")]
        fn with_current_span(&self) -> bool {
            true
        }

        #[cfg(feature = "json")]
        fn with_span_list(&self) -> bool {
            true
        }
    }
}
