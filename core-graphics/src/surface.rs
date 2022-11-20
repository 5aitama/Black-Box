use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

pub struct Surface {
    pub(crate) display: raw_window_handle::RawDisplayHandle,
    pub(crate) window: raw_window_handle::RawWindowHandle,
}

impl Surface {
    pub fn new(surface: &(impl HasRawDisplayHandle + HasRawWindowHandle)) -> Self {
        Self {
            display: surface.raw_display_handle(),
            window: surface.raw_window_handle(),
        }
    }
}