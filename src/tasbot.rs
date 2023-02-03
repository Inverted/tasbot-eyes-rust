use std::cell::{Ref, RefCell};
use std::fmt::Arguments;
use std::path::{Path, PathBuf};
use std::sync::{Arc, LockResult, Mutex, MutexGuard};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use log::{error, info, warn};
use once_cell::sync::Lazy;
use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;

use crate::{arguments};
use crate::arguments::{ARGUMENTS, fallback_arguments};
use crate::color::{get_base_or_blink_color, get_random_color_from_palette, GREEN};
use crate::file_operations::{BASE_PATH, BLINK_PATH, files_in_directory, OTHER_PATH, STARTUP_PATH};
use crate::renderer::{play_animation_from_path, Renderer};

pub fn run_eyes<T: Renderer>(mut renderer: T, queue: Arc<Mutex<Vec<PathBuf>>>, running: Arc<AtomicBool>) {
    let default_args = fallback_arguments();
    let args = ARGUMENTS.get().unwrap_or(&default_args);

    //Startup sequence
    if !args.skip_startup_animation {
        startup(&mut renderer);
    }

    //Normal flow
    while running.load(Ordering::SeqCst) { //todo: somehow interrupt this, to exit faster
        show_base(&mut renderer, args.color_overwrite && args.color_overwrite_all);
        do_blink_cycle(&mut renderer, args.color_overwrite && args.color_overwrite_all);

        let que = queue.lock();
        match que {
            Ok(q) => {
                show_next_animation(&mut renderer, q, args.color_overwrite);
            }
            Err(e) => error!("Can't lock queue: {}", e.to_string())
        }
    }
}

fn startup<T: Renderer>(renderer: &mut T) {
    info!("Play startup animation");
    let startup_anim_path = Path::new(STARTUP_PATH);
    play_animation_from_path(renderer, startup_anim_path.to_path_buf(), None);
    info!("Done playing startup animation");
}

fn show_base<T: Renderer>(renderer: &mut T, ran_color: bool) {
    info!("Play base animation");
    let default_args = fallback_arguments();
    let args = ARGUMENTS.get().unwrap_or(&default_args);

    //skip base, when no blinks at all
    if args.max_blinks != 0 && args.min_delay != 0 {
        let base_path = Path::new(BASE_PATH);

        //Render with that color, whatever it is now
        play_animation_from_path(renderer, base_path.to_path_buf(), get_base_or_blink_color(ran_color));
    }
    info!("Done playing base animation");
}

fn do_blink_cycle<T: Renderer>(renderer: &mut T, ran_color: bool) {
    info!("Enter blink cycle");
    let default_args = fallback_arguments();
    let args = ARGUMENTS.get().unwrap_or(&default_args);

    let blink_amount = get_blink_amount(args.max_blinks);
    info!("Blinking {} time/times", blink_amount);

    let blink_anims_path = Path::new(BLINK_PATH);
    let files = files_in_directory(&blink_anims_path);

    blink_sleep(get_blink_delay(args.min_delay, args.max_delay, args.playback_speed) as u64);
    for _ in 0..blink_amount {
        match &files {
            Ok(files) => {
                let mut rng = thread_rng();
                let random_blink = files.choose(&mut rng);
                match random_blink {
                    None => { warn!("Can't choose a random animation"); }
                    Some(path) => {
                        play_animation_from_path(renderer, path.to_path_buf(), get_base_or_blink_color(ran_color));
                    }
                }
            }
            Err(err) => {
                warn!("Can't read files in directory ({}): {}", blink_anims_path.to_str().unwrap_or("Invalid path"), err.to_string());
            }
        }
        blink_sleep(get_blink_delay(args.min_delay, args.max_delay, args.playback_speed) as u64);
    }
    info!("Exit blink cycle");
}

fn show_next_animation<T: Renderer>(renderer: &mut T, mut queue: MutexGuard<Vec<PathBuf>>, ran_color: bool) {
    info!("Play other animation");

    let path = queue.pop();
    match path {
        None => {
            //Queue is empty, create a new one
            let other_path = Path::new(OTHER_PATH);
            let files = files_in_directory(other_path);
            match files {
                Ok(mut files) => {
                    //Shuffle all files
                    let mut rng = thread_rng();
                    files.shuffle(&mut rng);

                    //Make space for queue
                    queue.clear();
                    queue.extend(files);

                    //If new queue is not empty, start over again
                    if queue.len() > 0 {
                        info!("Created new queue");

                        //Recursive call itself, to actually show a animation
                        show_next_animation(renderer, queue, ran_color);
                    } else {
                        let message = "Directory seems empty, please check!";
                        error!("{}", message);
                        panic!("{}", message)
                    }
                }
                Err(err) => {
                    warn!("Can't read directory ({}): {}", other_path.to_str().unwrap_or("Invalid path"), err.to_string());
                }
            }
        }
        Some(path) => {
            //Queue is not empty, play animation
            let color = if ran_color { Some(get_random_color_from_palette()) } else { None };
            play_animation_from_path(renderer, path, color);
        }
    }

    info!("Done playing other animation");
}

fn blink_sleep(delay: u64) {
    info!("Sleeping for blink for {} ms", delay);
    thread::sleep(Duration::from_millis(delay));
}

fn get_blink_delay(min_delay: u16, max_delay: u16, playback_speed: f32) -> u64 {
    if min_delay == max_delay {
        return ((min_delay as f32) * (1.0 / playback_speed)) as u64;
    }

    let mut rng = rand::thread_rng();
    let delay: u16 = rng.gen_range(min_delay..=max_delay);
    ((delay as f32) * (1.0 / playback_speed)) as u64 //return
}

fn get_blink_amount(max_blinks: u8) -> u8 {
    if max_blinks <= 0 {
        return 0;
    }

    let mut rng = thread_rng();
    rng.gen_range(1..=max_blinks) //return
}

#[cfg(test)]
mod tests {
    use super::*;

    //min_delay == max_delay
    #[test]
    fn test_get_blink_delay_min_delay_equals_max_delay() {
        let mut rng = rand::thread_rng();

        let delay = get_blink_delay(2000, 2000, 1.0);
        assert_eq!(delay, 2000);

        let delay = get_blink_delay(2000, 2000, 2.0);
        assert_eq!(delay, 1000);
    }

    //min_delay < max_delay
    #[test]
    fn test_get_blink_delay_min_delay_less_than_max_delay() {
        let mut rng = rand::thread_rng();

        let delay = get_blink_delay(1000, 2000, 1.0);
        assert!(delay >= 1000 && delay <= 2000);

        let delay = get_blink_delay(1000, 2000, 2.0);
        assert!(delay >= 500 && delay <= 1000);
    }
}