mod arrow_left;
mod bulldozer;
mod calendar_dot;
mod calendar_dots;
mod clipboard_text;
mod floppy_disk;
mod plus_circle;
mod rocket_launch;
mod x_circle;

pub use arrow_left::ArrowLeft;
pub use bulldozer::Bulldozer;
pub use calendar_dot::CalendarDot;
pub use calendar_dots::CalendarDots;
pub use clipboard_text::ClipboardText;
pub use floppy_disk::FloppyDisk;
pub use plus_circle::PlusCircle;
pub use rocket_launch::RocketLaunch;
pub use x_circle::XCircle;

pub enum NavPointIcon {
    Bulldozer,
    CalenderDot,
    CalenderDots,
    ClipboardText,
}
