use chrono::{DateTime, Local, TimeZone};
use clap::{App, Arg};

struct Clock;

impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }

    #[cfg(not(windows))]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) {
        use std::mem::zeroed;

        use libc::{settimeofday, suseconds_t, time_t, timeval, timezone};

        let t = t.with_timezone(&Local);
        let mut u: timeval = unsafe { zeroed() };

        u.tv_sec = t.timestamp() as time_t;
        u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;

        unsafe {
            let mock_tz: *const timezone = std::ptr::null();
            settimeofday(&u as *const timeval, mock_tz);
        }
    }
}
fn main() {
    let app = App::new("clock")
        .version("0.1")
        .about("Get and (aspirationally) sets the time.")
        .arg(
            Arg::with_name("action")
                .takes_value(true)
                .possible_values(&["get", "set"])
                .default_value("get"),
        )
        .arg(
            Arg::with_name("std")
                .short("s")
                .long("use-shandard")
                .takes_value(true)
                .possible_values(&["rfc2822", "rfc3339", "timestamp"])
                .default_value("rfc3339"),
        )
        .arg(Arg::with_name("datetime").help(
            "When <action> is 'set', apply <datetime>. \
            Otherwise, ignore.",
        ));

    let args = app.get_matches();

    let action = args.value_of("action").unwrap();

    let std = args.value_of("std").unwrap();

    if action == "set" {
        unimplemented!()
    }

    let now = Clock::get();

    match std {
        "timestamp" => println!("{}", now.timestamp()),
        "rfc2822" => println!("{}", now.to_rfc2822()),
        "rfc3339" => println!("{}", now.to_rfc3339()),
        _ => unreachable!(),
    }
}
