use termprogress::prelude::*;

pub struct ProgressSpinner {
    spinner: Option<Spin>,
}

impl ProgressSpinner {
    pub fn with_title(title: &str, verbose_level: u8) -> Self {
        if verbose_level >= 1 {
            let mut spinner = Spin::default();
            spinner.set_title(title);
            Self {
                spinner: Some(spinner),
            }
        } else {
            Self { spinner: None }
        }
    }

    pub fn bump(&mut self) {
        if let Some(spinner) = self.spinner.as_mut() {
            spinner.bump();
        }
    }

    pub fn complete_with(self, message: &str) {
        if let Some(spinner) = self.spinner {
            spinner.complete_with(message);
        }
    }
}
