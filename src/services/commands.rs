use crate::utils::shell;

// Play commands
pub fn pause() {
    let process = "osascript -e 'tell application \"Music\"' \
        -e 'pause' \
        -e 'end tell'";

    shell::system_shell(process);
}
pub fn play() {
    let process = "osascript -e 'tell application \"Music\"' \
        -e 'play' \
        -e 'end tell'";

    shell::system_shell(process);
}
pub fn next() {
    let process = "osascript -e 'tell application \"Music\"' \
        -e 'next track' \
        -e 'end tell'";

    shell::system_shell(process);
}
pub fn prev() {
    let process = "osascript -e 'tell application \"Music\"' \
        -e 'back track' \
        -e 'end tell'";

    shell::system_shell(process);
}
pub fn last() {
    let process = "osascript -e 'tell application \"Music\"' \
        -e 'previous track' \
        -e 'end tell'";

    shell::system_shell(process);
}

// Volume commands
pub fn set_volume(amount: &str) {
    let process = format!(
        "osascript -e 'tell application \"Music\"' \
        -e 'set the sound volume to {}' \
        -e 'end tell'",
        amount
    );

    shell::system_shell(&process);
}
pub fn increase_volume(amount: &str) {
    let process = format!(
        "osascript -e 'tell application \"Music\"' \
        -e 'set sound volume to (sound volume + {})' \
        -e 'end tell'",
        amount
    );

    shell::system_shell(&process);
}
pub fn decrease_volume(amount: &str) {
    let process = format!(
        "osascript -e 'tell application \"Music\"' \
        -e 'set sound volume to (sound volume - {})' \
        -e 'end tell'",
        amount
    );

    shell::system_shell(&process);
}

// Playlist commands
pub fn play_playlist(playlist: &str) {
    let process = format!(
        "osascript -e 'tell application \"Music\" to play playlist \"{}\"'",
        playlist
    );

    shell::system_shell(&process);
}

// Information commands
pub fn status() {
    let process = "osascript -e 'tell application \"Music\"' \
        -e 'set currentTrack to (\"track:  \" & name of current track as string)' \
        -e 'set currentArtist to (\"artist: \" & artist of current track as string)' \
        -e 'set currentVolume to (\"volume: \" & sound volume as string)' \
        -e 'set output to (currentTrack & \"\n\" & currentArtist & \"\n\" & currentVolume)' \
        -e 'copy output to stdout' \
        -e 'end tell'";

    shell::system_shell(process);
}

// General commands
pub fn open_itunes() {
    let process = "open -a music";

    shell::system_shell(process);
}
