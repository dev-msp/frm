use futures::{StreamExt, TryStreamExt};
use lru::LruCache;
use tokio::{sync::Mutex, task::spawn_blocking};

use std::{convert::TryInto, env, io, path::PathBuf, sync::Arc};

use crate::ffmpeg::{frame::Frame, frame_range::FrameRange, ErrorKind};

#[derive(Debug, Clone)]
pub struct AppState {
    source_file: String,
    cache: Arc<Mutex<LruCache<usize, Frame>>>,
    image_processor: Arc<Mutex<()>>,
}

fn cached_path_for(root: PathBuf, i: usize) -> PathBuf {
    root.join(format!("{i}.png"))
}
impl AppState {
    pub fn new(file: String, capacity: usize) -> Self {
        Self {
            source_file: file,
            cache: Arc::new(Mutex::new(LruCache::new(capacity.try_into().unwrap()))),
            image_processor: Arc::new(Mutex::new(())),
        }
    }

    pub fn file(&self) -> &str {
        &self.source_file
    }

    #[tracing::instrument(skip_all)]
    pub async fn frame_from_file(&self, i: usize) -> Result<Frame, ErrorKind> {
        let cache_root = PathBuf::from(env::var("CACHE_DIR").unwrap());
        if !cache_root.exists() || !cache_root.is_dir() {
            return Err(ErrorKind::Unhandled(format!(
                "Path to cache root \"{}\" either does not exist or is not a directory",
                cache_root.to_string_lossy()
            )));
        }
        let pb = cached_path_for(cache_root, i);

        let mut frm = Frame::new(&self.source_file, i)?;

        match tokio::fs::read(pb).await {
            Ok(bs) => frm.set_data(bs),
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e.into()),
        }

        Ok(frm)
    }

    #[tracing::instrument(skip_all)]
    pub async fn ingest_frame_range(
        &self,
        start: usize,
        n: usize,
        step: usize,
    ) -> Result<(), ErrorKind> {
        let cache_root = PathBuf::from(env::var("CACHE_DIR").unwrap());
        let output = {
            let _g = self.image_processor.lock().await;
            let mut range = FrameRange::new(
                &self.source_file,
                &cache_root.to_string_lossy(),
                start,
                n,
                step,
            )?;
            spawn_blocking(|| async move { range.read() })
                .await
                .expect("failed to join blocking task")
        }
        .await?;

        futures::stream::iter(output)
            .map(|code| self.frame_from_file(code))
            .buffer_unordered(4)
            .try_for_each(|frm| async {
                let mut cache = self.cache.lock().await;
                cache.push(frm.timecode(), frm);
                Ok(())
            })
            .await?;

        Ok(())
    }

    #[tracing::instrument(skip_all)]
    pub async fn request_frame(&self, i: usize) -> Result<Vec<u8>, ErrorKind> {
        let mut frame = self.frame_from_file(i).await?;
        if frame.has_data() {
            let mut cache = self.cache.lock().await;
            cache.push(i, frame.clone());
        } else {
            let mut cache = self.cache.lock().await;
            if let Some(frame) = cache.get_mut(&i) {
                if frame.has_data() {
                    return frame.write();
                }
            }
        }

        {
            let _g = self.image_processor.lock().await;
            let mut fc = frame.clone();
            tokio::task::spawn_blocking(move || fc.read())
                .await
                .map_err(|_| ErrorKind::Unhandled("oops".into()))??;
        }

        let mut cache = self.cache.lock().await;

        cache.push(i, frame.clone());
        frame.write()
    }
}
