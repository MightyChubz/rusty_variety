use clap::{App, Arg};
use std::fs::{remove_file, File};
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;

const LOCK_FILE_PATH: &str = "/tmp/rsvariety.lockfile";

struct Lockfile;

impl Default for Lockfile {
    fn default() -> Self {
        File::create(LOCK_FILE_PATH).expect("Couldn't create file!");
        Self
    }
}

impl Drop for Lockfile {
    fn drop(&mut self) {
        remove_lock();
    }
}

/// Sets wallpaper via `feh` through random selection.
///
/// # Arguments
///
/// * `directory`: The folder with all the wallpapers to use
///
/// returns: ()
///
/// # Examples
///
/// ```
/// set_wallpaper("~/Pictures/Wallpapers/");
/// ```
fn set_wallpaper(directory: &str) {
    Command::new("feh")
        .arg("--bg-fill")
        .arg("--randomize")
        .arg(directory)
        .spawn()
        .unwrap();
}

fn main() {
    let app = App::new("Rusty Variety")
        .about("A variety wallpaper program that quickly and quietly sets the wallpaper for you!")
        .version("1.0.0a")
        .author("Siv")
        .arg(
            Arg::with_name("time")
                .long("time")
                .short("t")
                .help("How much time before the wallpaper changes (in secs)")
                .required(true)
                .takes_value(true)
                .value_name("TIME"),
        )
        .arg(
            Arg::with_name("input")
                .long("input")
                .short("i")
                .help("the input directory where the wallpaper images are stored")
                .required(true)
                .takes_value(true)
                .value_name("INPUT"),
        )
        .arg(
            Arg::with_name("kill-all")
                .long("kill-all")
                .short("k")
                .help("Kills all instances of rusty_variety and removes the lockfile if it exists"),
        )
        .get_matches();

    // Checks if the kill-all flag is present, if so, kill all instances of rusty_variety and delete lockfile.
    let lock_path = Path::new(LOCK_FILE_PATH);
    if app.is_present("kill-all") {
        Command::new("killall")
            .arg("rusty_variety")
            .spawn()
            .unwrap();
        if lock_path.exists() {
            remove_lock();
        }
    }

    if lock_path.exists() {
        println!("An instance of this program is already running!");
        return;
    }

    // This is created through default which makes a lockfile automatically before deleting it after
    // it is dropped (at end of scope)
    let _lockfile = Lockfile::default();

    // Both of these values are required by the program as arguments, so are ensured to have a value
    // at runtime.
    let time = app
        .value_of("time")
        .and_then(|e| e.trim().parse::<u64>().ok())
        .unwrap();
    let input_dir = app.value_of("input").unwrap();

    if Path::new(input_dir).exists() {
        // Sets wallpaper then sleeps for however many seconds given by user.
        loop {
            set_wallpaper(input_dir);
            thread::sleep(Duration::from_secs(time));
        }
    }
}

fn remove_lock() {
    remove_file(LOCK_FILE_PATH).unwrap();
}
