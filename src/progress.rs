use termprogress::prelude::*;

pub struct Progress {
    progress_bar: Option<Bar>,
    num_steps_per_update: usize,
    steps: usize,
}

impl Progress {
    pub fn with_title(title: &str, verbose_level: u8, steps: usize) -> Self {
        if verbose_level >= 1 {
            let num_steps_per_update = steps / 100;
            let mut progress_bar = Bar::default();
            progress_bar.set_title(title);
            Self {
                progress_bar: Some(progress_bar),
                num_steps_per_update,
                steps,
            }
        } else {
            Self {
                progress_bar: None,
                num_steps_per_update: 0,
                steps: 0,
            }
        }
    }

    pub fn set_progress(&mut self, num_step: &usize) {
        if let Some(progress_bar) = self.progress_bar.as_mut() {
            if *num_step % self.num_steps_per_update as usize == 0 {
                progress_bar.set_progress(*num_step as f64 / self.steps as f64);
            }
        }
    }

    pub fn complete(self) {
        if let Some(progress_bar) = self.progress_bar {
            progress_bar.complete();
        }
    }
}
