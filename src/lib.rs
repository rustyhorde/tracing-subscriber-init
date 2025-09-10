// Copyright (c) 2023 tracing-subscriber-init developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Convenience trait and functions to ease [`tracing-subscriber`][tracing-subscriber] initialization.
//!
//! Program configuration can come from multiple sources. This crate supplies the [`TracingConfig`] trait to allow the grouping
//! of [`tracing-subscriber`][tracing-subscriber] initialization related items.
//!
//! For example, I often have some configuration from the command line (quiet and verbose flags),
//! some configuration from a configuration file, and some configuration (secrets) loaded from external sources.  I implement this
//! trait on a struct to collect the [`tracing-subscriber`][tracing-subscriber] related configuration, then use functions such as
//! [`full_filtered`](crate::full_filtered) to configure layers as appropriate.
//!
//! There are also convenience functions such as [`set_default`](crate::set_default) that will
//! setup a [`Registry`](tracing_subscriber::registry::Registry), add the given vector of [`Layer`](tracing_subscriber::Layer),
//! and initialize per the upstream functions of the
//! [same name](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/util/trait.SubscriberInitExt.html#method.set_default).
//!
//! [tracing-subscriber]: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/
//! # Example
//! ```rust
//! # use anyhow::Result;
//! # use std::fs::File;
//! # use tracing::{info, Level, span};
//! # use tracing_subscriber::{Layer, fmt::format::FmtSpan};
//! # use tracing_subscriber_init::{TracingConfig, full, full_filtered, set_default};
//! #
//! # pub fn main() -> Result<()> {
//! #[derive(Clone, Debug, Default)]
//! struct TomlConfig {
//!     // ...other configuration
//!     tracing: Tracing,
//!     tracing_file: TracingFile,
//!     // ...other configuration
//! }
//!
//! #[derive(Clone, Debug, Default)]
//! struct Tracing {
//!     target: bool,
//!     thread_ids: bool,
//!     thread_names: bool,
//!     line_numbers: bool,
//! }
//!
//! impl TracingConfig for Tracing {
//!     // Normally pulled from command line arguments, i.e. prog -qq
//!     fn quiet(&self) -> u8 {
//!         0
//!     }
//!
//!     // Normally pulled from command line arguments, i.e. prog -vv
//!     fn verbose(&self) -> u8 {
//!         2
//!     }
//!
//!     fn with_line_number(&self) -> bool {
//!         self.line_numbers
//!     }
//!
//!     fn with_target(&self) -> bool {
//!         self.target
//!     }
//!
//!     fn with_thread_ids(&self) -> bool {
//!         self.thread_ids
//!     }
//!
//!     fn with_thread_names(&self) -> bool {
//!         self.thread_names
//!     }
//! }
//!
//! #[derive(Clone, Debug, Default)]
//! struct TracingFile;
//!
//! impl TracingConfig for TracingFile {
//!     fn quiet(&self) -> u8 {
//!         0
//!     }
//!
//!     fn verbose(&self) -> u8 {
//!         3
//!     }
//!
//!     fn with_ansi(&self) -> bool {
//!         false
//!     }
//! }
//!
//! // Load configuration and pull out the tracing specific.
//! let toml_config = TomlConfig::default();
//! let tracing_config = toml_config.tracing;
//! let tracing_file_config = toml_config.tracing_file;
//!
//! // Setup a full format, filtered layer.  The filtering is set based on the quiet
//! // and verbose values from the configuration
//! let layer = full_filtered(&tracing_config);
//!
//! // Setup a second full format layer to write to a file.  Use the non-filtered
//! // version when you wish to modify items such as the writer, or the time format.
//! // You can also chose to ignore the generated level filter and apply your own.
//! let file = File::create("trace.log")?;
//! let (file_layer, level_filter) = full(&tracing_file_config);
//! let file_layer = file_layer.with_writer(file).with_filter(level_filter);
//!
//! // Create a Registry, add the layers, and set this subscriber as the default
//! // for this scope
//! let _unused = set_default(vec![layer.boxed(), file_layer.boxed()]);
//!
//! // Create a new span and enter it.
//! let span = span!(Level::INFO, "a new span");
//! let _enter = span.enter();
//!
//! // Trace away...
//! info!("info level");
//! #    Ok(())
//! # }
//! ```

// rustc lints
#![cfg_attr(
    all(feature = "unstable", nightly),
    feature(
        multiple_supertrait_upcastable,
        must_not_suspend,
        non_exhaustive_omitted_patterns_lint,
        rustdoc_missing_doc_code_examples,
        strict_provenance_lints,
        supertrait_item_shadowing,
        unqualified_local_imports,
    )
)]
#![cfg_attr(nightly, allow(single_use_lifetimes))]
#![cfg_attr(
    nightly,
    deny(
        absolute_paths_not_starting_with_crate,
        ambiguous_glob_imports,
        ambiguous_glob_reexports,
        ambiguous_negative_literals,
        ambiguous_wide_pointer_comparisons,
        anonymous_parameters,
        array_into_iter,
        asm_sub_register,
        async_fn_in_trait,
        bad_asm_style,
        bare_trait_objects,
        boxed_slice_into_iter,
        break_with_label_and_loop,
        clashing_extern_declarations,
        closure_returning_async_block,
        coherence_leak_check,
        confusable_idents,
        const_evaluatable_unchecked,
        const_item_mutation,
        dangling_pointers_from_temporaries,
        dead_code,
        dependency_on_unit_never_type_fallback,
        deprecated,
        deprecated_in_future,
        deprecated_safe_2024,
        deprecated_where_clause_location,
        deref_into_dyn_supertrait,
        deref_nullptr,
        double_negations,
        drop_bounds,
        dropping_copy_types,
        dropping_references,
        duplicate_macro_attributes,
        dyn_drop,
        edition_2024_expr_fragment_specifier,
        elided_lifetimes_in_paths,
        ellipsis_inclusive_range_patterns,
        explicit_outlives_requirements,
        exported_private_dependencies,
        ffi_unwind_calls,
        forbidden_lint_groups,
        forgetting_copy_types,
        forgetting_references,
        for_loops_over_fallibles,
        function_item_references,
        hidden_glob_reexports,
        if_let_rescope,
        impl_trait_overcaptures,
        impl_trait_redundant_captures,
        improper_ctypes,
        improper_ctypes_definitions,
        inline_no_sanitize,
        internal_features,
        invalid_from_utf8,
        invalid_macro_export_arguments,
        invalid_nan_comparisons,
        invalid_value,
        irrefutable_let_patterns,
        keyword_idents_2018,
        keyword_idents_2024,
        large_assignments,
        late_bound_lifetime_arguments,
        legacy_derive_helpers,
        let_underscore_drop,
        macro_use_extern_crate,
        map_unit_fn,
        meta_variable_misuse,
        mismatched_lifetime_syntaxes,
        missing_abi,
        missing_copy_implementations,
        missing_debug_implementations,
        missing_docs,
        missing_unsafe_on_extern,
        mixed_script_confusables,
        named_arguments_used_positionally,
        never_type_fallback_flowing_into_unsafe,
        no_mangle_generic_items,
        non_ascii_idents,
        non_camel_case_types,
        non_contiguous_range_endpoints,
        non_fmt_panics,
        non_local_definitions,
        non_shorthand_field_patterns,
        non_snake_case,
        non_upper_case_globals,
        noop_method_call,
        opaque_hidden_inferred_bound,
        out_of_scope_macro_calls,
        overlapping_range_endpoints,
        path_statements,
        private_bounds,
        private_interfaces,
        ptr_to_integer_transmute_in_consts,
        redundant_imports,
        redundant_lifetimes,
        redundant_semicolons,
        refining_impl_trait_internal,
        refining_impl_trait_reachable,
        renamed_and_removed_lints,
        repr_transparent_external_private_fields,
        rust_2021_incompatible_closure_captures,
        rust_2021_incompatible_or_patterns,
        rust_2021_prefixes_incompatible_syntax,
        rust_2021_prelude_collisions,
        rust_2024_guarded_string_incompatible_syntax,
        rust_2024_incompatible_pat,
        rust_2024_prelude_collisions,
        self_constructor_from_outer_item,
        semicolon_in_expressions_from_macros,
        single_use_lifetimes,
        special_module_name,
        stable_features,
        static_mut_refs,
        suspicious_double_ref_op,
        tail_expr_drop_order,
        trivial_bounds,
        trivial_casts,
        trivial_numeric_casts,
        type_alias_bounds,
        tyvar_behind_raw_pointer,
        uncommon_codepoints,
        unconditional_recursion,
        uncovered_param_in_projection,
        unexpected_cfgs,
        unfulfilled_lint_expectations,
        ungated_async_fn_track_caller,
        uninhabited_static,
        unit_bindings,
        unknown_lints,
        unknown_or_malformed_diagnostic_attributes,
        unnameable_test_items,
        unnameable_types,
        unpredictable_function_pointer_comparisons,
        unreachable_code,
        unreachable_patterns,
        unreachable_pub,
        unsafe_attr_outside_unsafe,
        unsafe_code,
        unsafe_op_in_unsafe_fn,
        unstable_name_collisions,
        unstable_syntax_pre_expansion,
        unused_allocation,
        unused_assignments,
        unused_associated_type_bounds,
        unused_attributes,
        unused_braces,
        unused_comparisons,
        unused_crate_dependencies,
        unused_doc_comments,
        unused_extern_crates,
        unused_features,
        unused_import_braces,
        unused_imports,
        unused_labels,
        unused_lifetimes,
        unused_macro_rules,
        unused_macros,
        unused_must_use,
        unused_mut,
        unused_parens,
        unused_qualifications,
        unused_results,
        unused_unsafe,
        unused_variables,
        useless_ptr_null_checks,
        uses_power_alignment,
        variant_size_differences,
        while_true,
    )
)]
// If nightly and unstable, allow `incomplete_features` and `unstable_features`
#![cfg_attr(
    all(feature = "unstable", nightly),
    allow(incomplete_features, unstable_features)
)]
// If nightly and not unstable, deny `incomplete_features` and `unstable_features`
#![cfg_attr(
    all(not(feature = "unstable"), nightly),
    deny(incomplete_features, unstable_features)
)]
// The unstable lints
#![cfg_attr(
    all(feature = "unstable", nightly),
    deny(
        fuzzy_provenance_casts,
        lossy_provenance_casts,
        multiple_supertrait_upcastable,
        must_not_suspend,
        non_exhaustive_omitted_patterns,
        supertrait_item_shadowing_definition,
        supertrait_item_shadowing_usage,
        unqualified_local_imports,
    )
)]
// clippy lints
#![cfg_attr(nightly, deny(clippy::all, clippy::pedantic))]
#![allow(clippy::ref_option)]
// rustdoc lints
#![cfg_attr(
    nightly,
    deny(
        rustdoc::bare_urls,
        rustdoc::broken_intra_doc_links,
        rustdoc::invalid_codeblock_attributes,
        rustdoc::invalid_html_tags,
        rustdoc::missing_crate_level_docs,
        rustdoc::private_doc_tests,
        rustdoc::private_intra_doc_links,
    )
)]
#![cfg_attr(
    all(nightly, feature = "unstable"),
    deny(rustdoc::missing_doc_code_examples)
)]
#![cfg_attr(all(doc, nightly), feature(doc_auto_cfg))]
#![cfg_attr(all(docsrs, nightly), feature(doc_cfg))]
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

mod config;
mod format;
mod initialize;
mod utils;

pub use self::config::Config as TracingConfig;
pub use self::format::compact::compact;
pub use self::format::compact::filtered as compact_filtered;
pub use self::format::full::filtered as full_filtered;
pub use self::format::full::full;
#[cfg(feature = "json")]
pub use self::format::json::filtered as json_filtered;
#[cfg(feature = "json")]
pub use self::format::json::json;
pub use self::format::pretty::filtered as pretty_filtered;
pub use self::format::pretty::pretty;
pub use self::initialize::init;
pub use self::initialize::set_default;
pub use self::initialize::try_init;
pub use self::utils::TestAll;

#[cfg(feature = "time")]
#[doc(no_inline)]
pub use time::format_description::well_known::Iso8601;
#[cfg(feature = "time")]
#[doc(no_inline)]
pub use time::format_description::well_known::Rfc2822;
#[cfg(feature = "time")]
#[doc(no_inline)]
pub use time::format_description::well_known::Rfc3339;
#[cfg(feature = "tstime")]
#[doc(no_inline)]
pub use tracing_subscriber::fmt::time::OffsetTime;
#[cfg(feature = "tstime")]
#[doc(no_inline)]
pub use tracing_subscriber::fmt::time::SystemTime;
#[cfg(feature = "tstime")]
#[doc(no_inline)]
pub use tracing_subscriber::fmt::time::Uptime;
#[cfg(feature = "tstime")]
#[doc(no_inline)]
pub use tracing_subscriber::fmt::time::UtcTime;
#[cfg(feature = "tstime")]
#[doc(no_inline)]
pub use tracing_subscriber::Layer;
