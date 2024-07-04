use smithay::{
    desktop::Window,
    input::pointer::{GrabStartData, MotionEvent, PointerGrab, PointerInnerHandle},
    reexports::wayland_server::protocol::wl_surface::WlSurface,
    utils::{Logical, Point},
};

use crate::hatchling::Hatchling;

pub struct MoveSurfaceGrab {
    pub start_data: GrabStartData<Hatchling>,
    pub window: Window,
    pub init_loc: Point<i32, Logical>,
}

impl PointerGrab<Hatchling> for MoveSurfaceGrab {
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
