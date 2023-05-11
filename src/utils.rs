// Copyright (c) 2023 tracing-subscriber-init developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;

use crate::TracingConfig;

#[cfg(debug_assertions)]
pub(crate) fn get_effective_level(quiet: u8, verbosity: u8) -> Level {
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
pub(crate) fn get_effective_level(_quiet: u8, verbosity: u8) -> Level {
    match verbosity {
        0 => Level::ERROR,
        1 => Level::WARN,
        2 => Level::INFO,
        3 => Level::DEBUG,
        4 | _ => Level::TRACE,
    }
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug)]
pub struct TestAll;

impl TracingConfig for TestAll {
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
    use super::get_effective_level;
    use crate::TracingConfig;
    use tracing::Level;

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

    #[derive(Clone, Debug)]
    pub(crate) struct TestConfig;

    impl TracingConfig for TestConfig {
        fn quiet(&self) -> u8 {
            0
        }

        fn verbose(&self) -> u8 {
            1
        }
    }

    #[derive(Clone, Debug)]
    pub(crate) struct TestJson;

    impl TracingConfig for TestJson {
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
