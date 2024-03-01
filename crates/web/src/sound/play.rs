use web_sys::HtmlAudioElement;

#[cfg(not(target_arch = "wasm32"))]
use tokio::task::spawn_local;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

pub fn play_intro() -> anyhow::Result<()> {
    play("intro.mp3")
}

pub fn play_music() -> anyhow::Result<()> {
    play("track_23.mp3")
}

fn play(name: &'static str) -> anyhow::Result<()> {
    spawn_local(async move {
        loop {
            let result = HtmlAudioElement::new_with_src(name);
            let promise = match result {
                Ok(a) => match a.play() {
                    Ok(a) => a,
                    Err(err) => {
                        log::trace!("failed to play - {}", err.as_string().unwrap_or_default());
                        crate::sleep::sleep(1000).await;
                        continue;
                    }
                },
                Err(err) => {
                    log::trace!("failed to play - {}", err.as_string().unwrap_or_default());
                    crate::sleep::sleep(1000).await;
                    continue;
                }
            };

            if let Err(err) = wasm_bindgen_futures::JsFuture::from(promise).await {
                log::trace!("failed to play - {}", err.as_string().unwrap_or_default());
                crate::sleep::sleep(1000).await;
                continue;
            }
            break;
        }
    });

    Ok(())
}
