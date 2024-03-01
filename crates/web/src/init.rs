use crate::{
    render::Mode,
    sound::play::{play_intro, play_music},
    state::local_app::FocusOn,
    LocalApp,
};

impl LocalApp {
    pub fn init(&mut self, frame: &mut eframe::Frame) -> anyhow::Result<()> {
        play_intro()?;
        play_music()?;

        self.mode = Mode::Login;
        self.focus_on.replace(FocusOn::Username);
        self.dialog_visible = false;
        self.username = Default::default();
        self.password = Default::default();
        self.confirm_password = Default::default();

        self.save_state(frame);

        Ok(())
    }
}
