//! Debug Logging

use job::Job;
use thread_pool::WorkerThread;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Event {
    StartWorking { index: usize },
    InjectJobs { count: usize },
    WaitForWork { worker: usize, was_active: bool },
    StoleWork { worker: usize, job: *mut Job },
    Join { worker: Option<WorkerThread> },
    PoppedJob { worker: WorkerThread },
    LostJob { worker: WorkerThread },
}

macro_rules! log {
    ($event:expr) => {
        // println!("{:?}", $event);
    }
}

pub static STOLEN_JOB = ATOMIC_USIZE_INIT;

macro_rules! stat_stolen {
    () => {
    }
}

pub static POPPED_JOB = ATOMIC_USIZE_INIT;

macro_rules! stat_popped {
    () => {
    }
}
