// Copyright 2022 CeresDB Project Authors. Licensed under Apache-2.0.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Configurations of [`Runner`].
///
/// [`Runner`]: crate::Runner
#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct Config {
    pub case_dir: String,
    /// Default value: `sql`
    #[builder(default = "Config::default_test_case_extension()")]
    #[serde(default = "Config::default_test_case_extension")]
    pub test_case_extension: String,
    /// Default value: `result`
    #[builder(default = "Config::default_result_extension()")]
    #[serde(default = "Config::default_result_extension")]
    pub result_extension: String,
    /// Default value: `-- SQLNESS`
    #[builder(default = "Config::default_interceptor_prefix()")]
    #[serde(default = "Config::default_interceptor_prefix")]
    pub interceptor_prefix: String,
    /// Default value: `config.toml`
    #[builder(default = "Config::default_env_config_file()")]
    #[serde(default = "Config::default_env_config_file")]
    pub env_config_file: String,
    /// Fail this run as soon as one case fails if true
    #[builder(default = "true")]
    #[serde(default = "Config::default_fail_fast")]
    pub fail_fast: bool,
    /// If specified, only run cases containing this string in their names.
    #[builder(default = "Config::default_test_filter()")]
    #[serde(default = "Config::default_test_filter")]
    pub test_filter: String,
    /// Whether follow symbolic links when searching test case files.
    /// Defaults to "false" (not follow symbolic links).
    #[builder(default = "false")]
    #[serde(default = "Config::default_follow_links")]
    pub follow_links: bool,
}

impl Config {
    fn default_test_case_extension() -> String {
        "sql".to_string()
    }

    fn default_result_extension() -> String {
        "result".to_string()
    }

    fn default_interceptor_prefix() -> String {
        "-- SQLNESS".to_string()
    }

    fn default_env_config_file() -> String {
        "config.toml".to_string()
    }

    fn default_fail_fast() -> bool {
        true
    }

    fn default_test_filter() -> String {
        "".to_string()
    }

    fn default_follow_links() -> bool {
        false
    }
}
