use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum AppEnv {
    #[default]
    Development,
    Production,
}

#[derive(Debug, Clone, Default, Parser)]
pub struct Env {
    #[clap(env)]
    pub port: String,

    #[clap(env)]
    pub database_url: String,

    #[clap(env, default_value = "development", value_enum)]
    pub env: AppEnv,
}
