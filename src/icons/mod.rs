mod bulldozer;
mod calendar_dot;
mod calendar_dots;
mod clipboard_text;
mod rocket_launch;

pub use bulldozer::Bulldozer;
pub use calendar_dot::CalendarDot;
pub use calendar_dots::CalendarDots;
pub use clipboard_text::ClipboardText;
pub use rocket_launch::RocketLaunch;

pub enum NavPointIcon {
    Bulldozer,
    CalenderDot,
    CalenderDots,
    ClipboardText,
}
