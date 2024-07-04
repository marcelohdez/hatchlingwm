use smithay::{
    desktop::Window,
    input::pointer::{GrabStartData, MotionEvent, PointerGrab, PointerInnerHandle},
    reexports::{
        wayland_protocols::xdg::shell::server::xdg_toplevel,
        wayland_server::protocol::wl_surface::WlSurface,
    },
    utils::{Logical, Point, Rectangle, Size},
};

use crate::hatchling::Hatchling;

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ResizeEdge: u32 {
        const TOP       = 0b0001;
        const BOTTOM    = 0b0010;
        const LEFT      = 0b0100;
        const RIGHT     = 0b1000;

        const TOP_LEFT      = Self::TOP.bits() | Self::LEFT.bits();
        const TOP_RIGHT     = Self::TOP.bits() | Self::RIGHT.bits();
        const BOTTOM_LEFT   = Self::BOTTOM.bits() | Self::LEFT.bits();
        const BOTTOM_RIGHT  = Self::BOTTOM.bits() | Self::RIGHT.bits();
    }
}

pub struct ResizeSurfaceGrab {
    pub start_data: GrabStartData<Hatchling>,
    pub window: Window,

    edges: ResizeEdge,

    init_rect: Rectangle<i32, Logical>,
    last_size: Size<i32, Logical>,
}

impl ResizeSurfaceGrab {
    pub fn start(
        start_data: GrabStartData<Hatchling>,
        window: Window,
        edges: ResizeEdge,
        init_rect: Rectangle<i32, Logical>,
    ) -> Self {
        todo!()
    }
}

impl From<xdg_toplevel::ResizeEdge> for ResizeEdge {
    #[inline]
    fn from(x: xdg_toplevel::ResizeEdge) -> Self {
        Self::from_bits(x as u32).unwrap()
    }
}

impl PointerGrab<Hatchling> for ResizeSurfaceGrab {
    fn motion(
        &mut self,
        data: &mut Hatchling,
        handle: &mut PointerInnerHandle<'_, Hatchling>,
        _focus: Option<(WlSurface, Point<f64, Logical>)>,
        event: &MotionEvent,
    ) {
        todo!()
    }

    fn relative_motion(
        &mut self,
        data: &mut Hatchling,
        handle: &mut PointerInnerHandle<'_, Hatchling>,
        focus: Option<(
            <Hatchling as smithay::input::SeatHandler>::PointerFocus,
            Point<f64, Logical>,
        )>,
        event: &smithay::input::pointer::RelativeMotionEvent,
    ) {
        todo!()
    }

    fn button(
        &mut self,
        data: &mut Hatchling,
        handle: &mut PointerInnerHandle<'_, Hatchling>,
        event: &smithay::input::pointer::ButtonEvent,
    ) {
        todo!()
    }

    fn axis(
        &mut self,
        data: &mut Hatchling,
        handle: &mut PointerInnerHandle<'_, Hatchling>,
        details: smithay::input::pointer::AxisFrame,
    ) {
        todo!()
    }

    fn frame(&mut self, data: &mut Hatchling, handle: &mut PointerInnerHandle<'_, Hatchling>) {
        todo!()
    }

    fn gesture_swipe_begin(
        &mut self,
        data: &mut Hatchling,
        handle: &mut PointerInnerHandle<'_, Hatchling>,
        event: &smithay::input::pointer::GestureSwipeBeginEvent,
    ) {
        todo!()
    }

    fn gesture_swipe_update(
        &mut self,
        data: &mut Hatchling,
        handle: &mut PointerInnerHandle<'_, Hatchling>,
        event: &smithay::input::pointer::GestureSwipeUpdateEvent,
    ) {
        todo!()
    }

    fn gesture_swipe_end(
        &mut self,
        data: &mut Hatchling,
        handle: &mut PointerInnerHandle<'_, Hatchling>,
        event: &smithay::input::pointer::GestureSwipeEndEvent,
    ) {
        todo!()
    }

    fn gesture_pinch_begin(
        &mut self,
        data: &mut Hatchling,
        handle: &mut PointerInnerHandle<'_, Hatchling>,
        event: &smithay::input::pointer::GesturePinchBeginEvent,
    ) {
        todo!()
    }

    fn gesture_pinch_update(
        &mut self,
        data: &mut Hatchling,
        handle: &mut PointerInnerHandle<'_, Hatchling>,
        event: &smithay::input::pointer::GesturePinchUpdateEvent,
    ) {
        todo!()
    }

    fn gesture_pinch_end(
        &mut self,
        data: &mut Hatchling,
        handle: &mut PointerInnerHandle<'_, Hatchling>,
        event: &smithay::input::pointer::GesturePinchEndEvent,
    ) {
        todo!()
    }

    fn gesture_hold_begin(
        &mut self,
        data: &mut Hatchling,
        handle: &mut PointerInnerHandle<'_, Hatchling>,
        event: &smithay::input::pointer::GestureHoldBeginEvent,
    ) {
        todo!()
    }

    fn gesture_hold_end(
        &mut self,
        data: &mut Hatchling,
        handle: &mut PointerInnerHandle<'_, Hatchling>,
        event: &smithay::input::pointer::GestureHoldEndEvent,
    ) {
        todo!()
    }

    fn start_data(&self) -> &smithay::input::pointer::GrabStartData<Hatchling> {
        todo!()
    }

    fn unset(&mut self, data: &mut Hatchling) {
        todo!()
    }
}
