use crate::program::args::{self, Args, Commands};
use crate::services::commands;

pub fn start() {
    let args = args::get_args();
    let app = App::new(args);

    app.run();
}

struct App {
    args: Args,
}
impl App {
    fn new(args: Args) -> App {
        App { args }
    }
    fn run(&self) {
        let command = &self.args.command;

        match command {
            Commands::Pause => commands::pause(),
            Commands::Play => commands::play(),
            Commands::Next => commands::next(),
            Commands::Prev => commands::prev(),
            Commands::Last => commands::last(),
            Commands::Open => commands::open_itunes(),
            Commands::Status => commands::status(),
            Commands::Pl { playlist } => {
                commands::play_playlist(&playlist);
            }
            Commands::Vol { amount } => {
                commands::set_volume(&amount);
            }
            Commands::Up { amount } => {
                commands::increase_volume(&amount);
            }
            Commands::Down { amount } => {
                commands::decrease_volume(&amount);
            }
        }
    }
}
