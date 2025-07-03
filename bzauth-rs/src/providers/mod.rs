pub mod discord;
pub mod error;
pub mod github;
pub mod google;
pub mod mock;

pub use discord::DiscordProvider;
// use github::GithubProvider;
pub use google::GoogleProvider;
