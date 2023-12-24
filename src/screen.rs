pub struct ScreenResolution {
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) scale: f32
}

pub enum ResolutionMode {
    High,
    Medium,
    Low
}
pub static mut CURRENT_MODE:ResolutionMode = ResolutionMode::Low;

pub unsafe fn get_current_screen_resolution() -> ScreenResolution {
    let mode = CURRENT_MODE.get_resolution();
    ScreenResolution{
        width: mode.width,
        height: mode.height,
        scale: mode.scale
    }
}

pub unsafe fn set_current_screen_resolution(mode: ResolutionMode) {
    CURRENT_MODE = mode;
}
impl ResolutionMode {
    pub fn get_resolution(&self) -> ScreenResolution {
        match &self {
            ResolutionMode::High => ScreenResolution{
                width: 1920.,
                height: 1080.,
                scale: 1.5
            },
            ResolutionMode::Medium => ScreenResolution {
                width: 1600.,
                height: 900.,
                scale: 1.25,
            },
            ResolutionMode::Low => ScreenResolution{
                width:1280.,
                height:720.,
                scale: 1.
            }
        }
    }
}

