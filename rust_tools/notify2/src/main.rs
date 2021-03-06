extern crate notify_rust as notify;
extern crate mpd;

use notify::{Notification as N, Timeout as T};
use mpd::{ Client, Song };

fn main() {
    let mut conn = Client::connect("127.0.0.1:6600").expect("Can't connect to mpd!");
    let song: Song = conn.currentsong().unwrap().unwrap();
    let album_art = "/tmp/cover_mpd.png";
    N::new()
        .summary(&song.title.unwrap())
        .body(&format!("{}\n{}", song.tags.get(&"Album".to_string()).unwrap(), song.tags.get(&"Artist".to_string()).unwrap()))
        .icon(album_art)
        .timeout(T::Milliseconds(5000))
        .show().unwrap();
}
