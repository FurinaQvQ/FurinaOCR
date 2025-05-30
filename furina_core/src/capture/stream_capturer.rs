use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{atomic, Arc};
use std::thread;
use std::thread::JoinHandle;

use anyhow::Result;
use image::{GenericImage, RgbImage};

use crate::capture::{Capturer, GenericCapturer};
use crate::positioning::Rect;

pub struct StreamingCapturer {
    region: Rect<i32>,
    capturer: Box<dyn Capturer<RgbImage> + Send>,

    is_cancelled: Arc<AtomicBool>,
}

impl StreamingCapturer {
    pub fn new(region: Rect<i32>) -> Self {
        Self {
            region,
            capturer: Box::new(GenericCapturer::new().unwrap()),
            is_cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start_transform<F, S>(
        self,
        tx: Sender<S>,
        transform: F,
    ) -> (JoinHandle<Result<()>>, impl Fn())
    where
        F: Fn(RgbImage) -> S + Send + Sync + 'static,
        S: Send + Sync + 'static,
    {
        let is_cancelled = self.is_cancelled.clone();

        let handle = thread::spawn(move || -> Result<()> {
            let mut _it = 0;
            loop {
                if self.is_cancelled.load(atomic::Ordering::Relaxed) {
                    break;
                }

                let image = self.capturer.capture_rect(self.region);
                if let Ok(im) = image {
                    tx.send(transform(im))?
                }

                _it += 1;
            }

            Ok(())
        });

        let cancel = move || {
            is_cancelled.store(true, atomic::Ordering::Relaxed);
        };

        (handle, cancel)
    }
}
