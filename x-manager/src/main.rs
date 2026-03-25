mod auth;
mod api;
mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "x-manager", about = "X/Twitter account manager")]
struct Cli {
    /// Account name (default: "default")
    #[arg(long, default_value = "default")]
    account: String,

    /// Output format: json or text
    #[arg(long, default_value = "json")]
    format: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Store API credentials for an account
    Auth {
        #[arg(long)]
        api_key: String,
        #[arg(long)]
        api_secret: String,
        #[arg(long)]
        access_token: String,
        #[arg(long)]
        access_secret: String,
    },
    /// Post a tweet
    Post {
        #[arg(long)]
        text: String,
        #[arg(long)]
        media: Option<String>,
    },
    /// Post a thread (tweets separated by \n---\n)
    Thread {
        #[arg(long)]
        text: String,
    },
    /// Reply to a tweet
    Reply {
        #[arg(long)]
        to: String,
        #[arg(long)]
        text: String,
    },
    /// Delete a tweet
    Delete {
        #[arg(long)]
        id: String,
    },
    /// Get your home timeline
    Timeline {
        #[arg(long, default_value = "20")]
        limit: u32,
    },
    /// Get a user's recent posts
    UserPosts {
        #[arg(long)]
        username: String,
        #[arg(long, default_value = "10")]
        limit: u32,
    },
    /// Search recent tweets
    Search {
        #[arg(long)]
        query: String,
        #[arg(long, default_value = "20")]
        limit: u32,
    },
    /// Get a specific tweet
    Get {
        #[arg(long)]
        id: String,
    },
    /// Get your mentions
    Mentions {
        #[arg(long, default_value = "20")]
        limit: u32,
    },
    /// Monitor for tweets matching a query
    Monitor {
        #[arg(long)]
        query: String,
        #[arg(long, default_value = "60")]
        interval: u64,
    },
    /// Like a tweet
    Like {
        #[arg(long)]
        id: String,
    },
    /// Retweet
    Retweet {
        #[arg(long)]
        id: String,
    },
    /// Quote tweet
    Quote {
        #[arg(long)]
        id: String,
        #[arg(long)]
        text: String,
    },
    /// Bookmark a tweet
    Bookmark {
        #[arg(long)]
        id: String,
    },
    /// Get your bookmarks
    Bookmarks {
        #[arg(long, default_value = "20")]
        limit: u32,
    },
    /// Get your followers
    Followers {
        #[arg(long, default_value = "50")]
        limit: u32,
        /// Show only new followers since last check
        #[arg(long)]
        new: bool,
    },
    /// Get who you follow
    Following {
        #[arg(long, default_value = "50")]
        limit: u32,
    },
    /// Follow a user
    Follow {
        #[arg(long)]
        username: String,
    },
    /// Unfollow a user
    Unfollow {
        #[arg(long)]
        username: String,
    },
    /// Get trending topics
    Trends {
        #[arg(long, default_value = "1")]
        location: u64,
    },
    /// Get engagement analytics
    Analytics {
        #[arg(long)]
        id: Option<String>,
        #[arg(long, default_value = "7")]
        days: u32,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "x_manager=info".into()),
        )
        .with_target(false)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Auth { api_key, api_secret, access_token, access_secret } => {
            auth::store_credentials(&cli.account, &api_key, &api_secret, &access_token, &access_secret)?;
            output_success("Credentials stored", &cli.format);
        }
        Commands::Post { text, media } => {
            let creds = auth::load_credentials(&cli.account)?;
            let result = commands::post_tweet(&creds, &text, media.as_deref()).await?;
            output_json(&result, &cli.format);
        }
        Commands::Thread { text } => {
            let creds = auth::load_credentials(&cli.account)?;
            let tweets: Vec<&str> = text.split("\n---\n").collect();
            let result = commands::post_thread(&creds, &tweets).await?;
            output_json(&result, &cli.format);
        }
        Commands::Reply { to, text } => {
            let creds = auth::load_credentials(&cli.account)?;
            let result = commands::reply_tweet(&creds, &to, &text).await?;
            output_json(&result, &cli.format);
        }
        Commands::Delete { id } => {
            let creds = auth::load_credentials(&cli.account)?;
            let result = commands::delete_tweet(&creds, &id).await?;
            output_json(&result, &cli.format);
        }
        Commands::Timeline { limit } => {
            let creds = auth::load_credentials(&cli.account)?;
            let result = commands::get_timeline(&creds, limit).await?;
            output_json(&result, &cli.format);
        }
        Commands::UserPosts { username, limit } => {
            let creds = auth::load_credentials(&cli.account)?;
            let result = commands::get_user_posts(&creds, &username, limit).await?;
            output_json(&result, &cli.format);
        }
        Commands::Search { query, limit } => {
            let creds = auth::load_credentials(&cli.account)?;
            let result = commands::search_tweets(&creds, &query, limit).await?;
            output_json(&result, &cli.format);
        }
        Commands::Get { id } => {
            let creds = auth::load_credentials(&cli.account)?;
            let result = commands::get_tweet(&creds, &id).await?;
            output_json(&result, &cli.format);
        }
        Commands::Mentions { limit } => {
            let creds = auth::load_credentials(&cli.account)?;
            let result = commands::get_mentions(&creds, limit).await?;
            output_json(&result, &cli.format);
        }
        Commands::Monitor { query, interval } => {
            let creds = auth::load_credentials(&cli.account)?;
            commands::monitor_tweets(&creds, &query, interval).await?;
        }
        Commands::Like { id } => {
            let creds = auth::load_credentials(&cli.account)?;
            let result = commands::like_tweet(&creds, &id).await?;
            output_json(&result, &cli.format);
        }
        Commands::Retweet { id } => {
            let creds = auth::load_credentials(&cli.account)?;
            let result = commands::retweet(&creds, &id).await?;
            output_json(&result, &cli.format);
        }
        Commands::Quote { id, text } => {
            let creds = auth::load_credentials(&cli.account)?;
            let result = commands::quote_tweet(&creds, &id, &text).await?;
            output_json(&result, &cli.format);
        }
        Commands::Bookmark { id } => {
            let creds = auth::load_credentials(&cli.account)?;
            let result = commands::bookmark_tweet(&creds, &id).await?;
            output_json(&result, &cli.format);
        }
        Commands::Bookmarks { limit } => {
            let creds = auth::load_credentials(&cli.account)?;
            let result = commands::get_bookmarks(&creds, limit).await?;
            output_json(&result, &cli.format);
        }
        Commands::Followers { limit, new } => {
            let creds = auth::load_credentials(&cli.account)?;
            let result = commands::get_followers(&creds, limit, new).await?;
            output_json(&result, &cli.format);
        }
        Commands::Following { limit } => {
            let creds = auth::load_credentials(&cli.account)?;
            let result = commands::get_following(&creds, limit).await?;
            output_json(&result, &cli.format);
        }
        Commands::Follow { username } => {
            let creds = auth::load_credentials(&cli.account)?;
            let result = commands::follow_user(&creds, &username).await?;
            output_json(&result, &cli.format);
        }
        Commands::Unfollow { username } => {
            let creds = auth::load_credentials(&cli.account)?;
            let result = commands::unfollow_user(&creds, &username).await?;
            output_json(&result, &cli.format);
        }
        Commands::Trends { location } => {
            let creds = auth::load_credentials(&cli.account)?;
            let result = commands::get_trends(&creds, location).await?;
            output_json(&result, &cli.format);
        }
        Commands::Analytics { id, days } => {
            let creds = auth::load_credentials(&cli.account)?;
            let result = commands::get_analytics(&creds, id.as_deref(), days).await?;
            output_json(&result, &cli.format);
        }
    }

    Ok(())
}

fn output_json(value: &serde_json::Value, format: &str) {
    if format == "text" {
        // Pretty print for humans
        if let Some(data) = value.get("data") {
            println!("{}", serde_json::to_string_pretty(data).unwrap_or_default());
        } else {
            println!("{}", serde_json::to_string_pretty(value).unwrap_or_default());
        }
    } else {
        println!("{}", serde_json::to_string_pretty(value).unwrap_or_default());
    }
}

fn output_success(msg: &str, format: &str) {
    if format == "text" {
        println!("{msg}");
    } else {
        println!(r#"{{"success": true, "message": "{msg}"}}"#);
    }
}
