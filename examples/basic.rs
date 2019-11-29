extern crate ctrlc;
extern crate tuxdroid;
use crate::tuxdroid::tuxdroid::TuxDroid;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() -> () {
    let fux = tuxdroid::fux::Fux::new().expect("Failed to create Fux");

    fux.get_status().unwrap();
    let _id = fux.get_id().unwrap();

    // Handle Ctrl+C
    let running = Arc::new(AtomicUsize::new(0));
    let r = running.clone();
    ctrlc::set_handler(move || {
        let prev = r.fetch_add(1, Ordering::SeqCst);
        if prev == 0 {
            println!("Exiting...");
        } else {
            std::process::exit(0);
        }
    })
    .expect("Error setting Ctrl-C handler");

    println!("Running...");

    // while running.load(Ordering::SeqCst) {
    //     thread::sleep(Duration::from_millis(10));
    // }
    // if running.load(Ordering::SeqCst) > 0 {
    //     break;
    // }
    //fux.flippers_stop();

    fux.wake_up_tux();
    thread::sleep(Duration::from_secs(2));

    println!("Woke up Tux... that lazy bum!");
    fux.eyes_open().unwrap();

    println!("eye lights on...");
    fux.eyes_on().unwrap();
    thread::sleep(Duration::from_secs(3));

    let effect = tuxdroid::fux::LedEffect {
        effect_type: tuxdroid::fux::LedEffectType::GRADIENT_DELTA,
        speed: 1.5,
        step: 10,
    };

    println!("Pulsing left eye...");
    fux.led_pulse(tuxdroid::fux::Leds::Left, 100, 220, 6, 1.0, &effect);
    thread::sleep(Duration::from_secs(10));

    println!("Pulsing right eye...");
    fux.led_pulse(tuxdroid::fux::Leds::Right, 100, 220, 2, 1.0, &effect);
    thread::sleep(Duration::from_secs(10));

    fux.open_mouth();
    thread::sleep(Duration::from_secs(2));
    fux.eyes_blink();
    thread::sleep(Duration::from_secs(20));
    fux.close_mouth();

    thread::sleep(Duration::from_secs(1));

    fux.eyes_close().unwrap();
    thread::sleep(Duration::from_secs(1));
    fux.eyes_open().unwrap();

    thread::sleep(Duration::from_secs(5));

    fux.flippers_raise();
    thread::sleep(Duration::from_secs(2));
    fux.flippers_lower();

    thread::sleep(Duration::from_secs(5));

    fux.flippers_wave();
    thread::sleep(Duration::from_secs(60));
    fux.flippers_stop();

    let _t = std::thread::spawn(move || {
        fux.start();
    });

    // fux.stop();
    // t.join()
    //match fux {
    //    Some(tuxdroid) => tuxdroid.start(),
    //    None => p
    //}
}
