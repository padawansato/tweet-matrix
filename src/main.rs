use crate::view::GreenCodeView;
use cursive::theme::Color;
use cursive::theme::Theme;
use cursive::Cursive;
use egg_mode::direct::Timeline;

mod twitter;
mod view;

fn main() {
    twitter();
    let size = siv.screen_size();
    let mut theme = Theme::default();
    theme.palette.set_color("view", Color::TerminalDefault);
    siv.set_theme(theme);

    siv.add_fullscreen_layer(GreenCodeView::new(1, size));
    siv.set_autorefresh(true);
    siv.run();
}
