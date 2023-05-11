use crate::util::interpolate_color;
use crate::util::DARK_GREEN;
use crate::util::GREEN;
use crate::util::ORANGE;
use crate::util::RED;
use crate::util::YELLOW;
use derive_more::Display;
use derive_new::new;
use getset::{Getters, MutGetters, Setters};
use owo_colors::OwoColorize;
use parking_lot::RwLock;
use shrinkwraprs::Shrinkwrap;
use std::sync::Arc;
use std::time::Duration;
use typed_builder::TypedBuilder;

// Durations for different progress indicators
pub(crate) static SHORT_DURATION_START_TIME: Duration = Duration::from_secs(1);
pub(crate) static MEDIUM_DURATION_START_TIME: Duration = Duration::from_secs(4);
pub(crate) static LONG_DURATION_START_TIME: Duration = Duration::from_secs(8);
pub(crate) static VERY_LONG_DURATION_START_TIME: Duration = Duration::from_secs(12);

// Track in-progress and completed tasks
lazy_static::lazy_static! {
    static ref TASK_COUNTER: TaskTracker = TaskTracker::new(0, 0);
}

#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    PartialEq,
    Eq,
    Hash,
    TypedBuilder,
    Getters,
    MutGetters,
    Setters,
    new,
)]
#[display(fmt = "In progress {in_progress}, completed {completed}")]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
struct TaskTrackerData {
    in_progress: usize,
    completed: usize,
}

#[derive(Debug, Clone, Shrinkwrap)]
struct TaskTracker(Arc<RwLock<TaskTrackerData>>);

impl TaskTracker {
    pub fn new(in_progress: usize, completed: usize) -> Self {
        Self(Arc::new(RwLock::new(TaskTrackerData::new(
            in_progress,
            completed,
        ))))
    }
}

pub(crate) fn increment_in_progress_task() {
    let mut task_counter = TASK_COUNTER.0.write();
    task_counter.in_progress += 1;
}

pub(crate) fn increment_completed_task() {
    let mut task_counter = TASK_COUNTER.0.write();
    task_counter.completed += 1;
}

pub fn short_running_task_msg(duration: Duration) -> String {
    let elapsed_secs = duration.as_secs_f64();
    let gradient = interpolate_color(&DARK_GREEN, &GREEN, elapsed_secs / 3.0);
    format!(
        "{}{}{}",
        "[".black(),
        "Short".color(gradient).italic(),
        "]".black()
    )
    .into()
}

pub fn medium_running_task_msg(duration: Duration) -> String {
    let elapsed_secs = duration.as_secs_f64();
    let gradient = interpolate_color(&GREEN, &YELLOW, elapsed_secs / 3.0);
    format!(
        "{}{}{}",
        "[".black(),
        "Medium".color(gradient).italic(),
        "]".black()
    )
    .into()
}

pub fn long_running_task_msg(duration: Duration) -> String {
    let elapsed_secs = duration.as_secs_f64();
    let gradient = interpolate_color(&YELLOW, &ORANGE, elapsed_secs / 6.0);
    format!(
        "{}{}{}",
        "[".black(),
        "Long".color(gradient).italic(),
        "]".black()
    )
    .into()
}

pub fn very_long_running_task_msg(duration: Duration) -> String {
    let elapsed_secs = duration.as_secs_f64();
    let gradient = interpolate_color(&ORANGE, &RED, elapsed_secs / 6.0);
    format!(
        "{}{}{}",
        "[".black(),
        "Very Long".color(gradient).italic(),
        "]".black()
    )
    .into()
}
