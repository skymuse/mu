use clap::{Parser, Subcommand};

pub fn get_args() -> Args {
    let args = Args::parse();

    args
}

#[derive(Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Pauses the current song
    Pause,
    /// Plays the current song
    Play,
    /// Plays the next song
    Next,
    /// Plays the previous song or repeats the current one
    Prev,
    /// Plays the last song
    Last,
    /// Opens itunes
    Open,
    /// Displays the current track, artist, and volume
    Status,
    /// Plays the given playlist
    Pl {
        /// The playlist to play
        playlist: String,
    },
    /// Sets the volume by a given amount
    Vol {
        /// The volume amount, from 0...100 inclusive
        amount: String,
    },
    /// Increases volume by a given amount
    Up {
        /// The volume amount
        amount: String,
    },
    /// Decreases volume by a given amount
    Down {
        /// The volume amount
        amount: String,
    },
}
