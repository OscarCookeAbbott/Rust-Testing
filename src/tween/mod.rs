use std::fmt::Display;
use std::ops::*;
use std::time::SystemTime;

pub struct Tween<'a, T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<f64, Output = T> + Copy + Display,
{
    pub value: &'a mut T,
    pub start: T,
    pub end: T,
    pub duration: f64,
    pub on_progress: &'a dyn Fn(&T),
    progress: f64,
    start_time: SystemTime,
}

impl<'a, T> Tween<'a, T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<f64, Output = T> + Copy + Display,
{
    pub fn new(value: &'a mut T, start: T, end: T, duration: f64, on_progress: &'a impl Fn(&T)) -> Self {
        Tween {
            value,
            start,
            end,
            duration,
            on_progress: on_progress,
            progress: 0.0,
            start_time: SystemTime::now(),
        }
    }

    pub fn start(&mut self) {
        self.start_time = SystemTime::now();

        while self.progress < 1.0 {
            *self.value = self.start + (self.end - self.start) * self.progress;

            (self.on_progress)(self.value);

            self.progress = SystemTime::now()
                .duration_since(self.start_time)
                .expect("error")
                .as_secs_f64()
                / self.duration;
        }
    }
}
