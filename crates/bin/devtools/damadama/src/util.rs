use crate::task::{
    long_running_task_msg, medium_running_task_msg, short_running_task_msg,
    very_long_running_task_msg, LONG_DURATION_START_TIME, MEDIUM_DURATION_START_TIME,
    SHORT_DURATION_START_TIME, VERY_LONG_DURATION_START_TIME,
};
use indicatif::ProgressState;
use lazy_static::lazy_static;
use owo_colors::{OwoColorize, Rgb};

macro_rules! italicize {
    ($msg:expr) => {
        $msg.italic().to_string().into()
    };
}

macro_rules! bold {
    ($msg:expr) => {
        $msg.bold().to_string().into()
    };
}

macro_rules! color {
    ($msg:expr, $color:expr) => {
        $msg.color($color).to_string().into()
    };
}

lazy_static! {
    // Colors
    pub(crate) static ref RED: Rgb = Rgb(255, 0, 0);
    pub(crate) static ref ORANGE: Rgb = Rgb(255, 165, 0);
    pub(crate) static ref YELLOW: Rgb = Rgb(255, 255, 0);
    pub(crate) static ref GREEN: Rgb = Rgb(0, 255, 0);
    pub(crate) static ref DARK_GREEN: Rgb = Rgb(0, 100, 0);
    pub(crate) static ref BLUE: Rgb = Rgb(0, 0, 255);
    pub(crate) static ref INDIGO: Rgb = Rgb(75, 0, 130);
    pub(crate) static ref VIOLET: Rgb = Rgb(238, 130, 238);
    pub(crate) static ref PURPLE: Rgb = Rgb(255, 0, 255);
    pub(crate) static ref CYAN: Rgb = Rgb(0, 255, 255);
    pub(crate) static ref WHITE: Rgb = Rgb(255, 255, 255);
    pub(crate) static ref BLACK: Rgb = Rgb(0, 0, 0);

    // Symbols
    pub(crate) static ref RIGHT_ARROW: String = italicize!("↳ ");
    pub(crate) static ref RIGHT_ARROW_SYMBOL: String = color!(&*RIGHT_ARROW, Rgb(0, 0, 0));
}

pub(crate) fn elapsed_subsec(state: &ProgressState, writer: &mut dyn std::fmt::Write) {
    let seconds = state.elapsed().as_secs();
    let sub_seconds = (state.elapsed().as_millis() % 1000) / 100;
    let duration = state.elapsed();
    let elapsed_secs = duration.as_secs_f64();

    let elapsed = state.elapsed();

    if elapsed > VERY_LONG_DURATION_START_TIME {
        let gradient = interpolate_color(
            &ORANGE,
            &RED,
            (elapsed_secs - VERY_LONG_DURATION_START_TIME.as_secs_f64()) / 3.0,
        );
        let _ = write!(
            writer,
            "{}",
            format!("{seconds}.{sub_seconds}s").color(gradient)
        );
    } else if elapsed > LONG_DURATION_START_TIME {
        let gradient = interpolate_color(
            &YELLOW,
            &ORANGE,
            (elapsed_secs - LONG_DURATION_START_TIME.as_secs_f64()) / 3.0,
        );
        let _ = write!(
            writer,
            "{}",
            format!("{seconds}.{sub_seconds}s").color(gradient)
        );
    } else if elapsed > MEDIUM_DURATION_START_TIME {
        let gradient = interpolate_color(
            &GREEN,
            &YELLOW,
            (elapsed_secs - MEDIUM_DURATION_START_TIME.as_secs_f64()) / 3.0,
        );
        let _ = write!(
            writer,
            "{}",
            format!("{seconds}.{sub_seconds}s").color(gradient).italic()
        );
    } else {
        let gradient = interpolate_color(&DARK_GREEN, &GREEN, elapsed_secs / 3.0);
        let _ = write!(
            writer,
            "{}",
            format!("{seconds}.{sub_seconds}s").color(gradient).italic()
        );
    }
}

pub(crate) fn interpolate_color(from: &Rgb, to: &Rgb, t: f64) -> Rgb {
    let r = interpolate(from.0, to.0, t);
    let g = interpolate(from.1, to.1, t);
    let b = interpolate(from.2, to.2, t);
    Rgb(r, g, b)
}

fn interpolate(a: u8, b: u8, t: f64) -> u8 {
    let result = a as f64 * (1.0 - t) + b as f64 * t;
    result.round() as u8
}

pub(crate) fn color_start(state: &ProgressState, writer: &mut dyn std::fmt::Write) {
    task_msg_display(state, writer);
}

fn task_msg_display(state: &ProgressState, writer: &mut dyn std::fmt::Write) {
    let elapsed = state.elapsed();

    if elapsed > VERY_LONG_DURATION_START_TIME {
        let _ = write!(
            writer,
            " {} ",
            very_long_running_task_msg(elapsed - VERY_LONG_DURATION_START_TIME)
        );
    } else if elapsed > LONG_DURATION_START_TIME {
        let _ = write!(
            writer,
            " {} ",
            long_running_task_msg(elapsed - LONG_DURATION_START_TIME)
        );
    } else if elapsed > MEDIUM_DURATION_START_TIME {
        let _ = write!(
            writer,
            " {} ",
            medium_running_task_msg(elapsed - MEDIUM_DURATION_START_TIME)
        );
    } else if elapsed > SHORT_DURATION_START_TIME {
        let _ = write!(
            writer,
            " {} ",
            short_running_task_msg(elapsed - SHORT_DURATION_START_TIME)
        );
    }
}
