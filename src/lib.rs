use hatchling::Hatchling;
use smithay::reexports::wayland_server::DisplayHandle;

pub mod client_state;
pub mod hatchling;
pub mod move_grab;
pub mod resize_grab;

pub struct LoopData {
    pub state: Hatchling,
    pub display_handle: DisplayHandle,
}
