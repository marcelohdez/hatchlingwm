use std::{env, process, time::Duration};

use hatchlingwm::{hatchling::Hatchling, LoopData};
use smithay::{
    backend::{
        renderer::{
            damage::OutputDamageTracker, element::surface::WaylandSurfaceRenderElement,
            gles::GlesRenderer,
        },
        winit::{self, WinitEvent},
    },
    desktop,
    output::{Mode, Output, PhysicalProperties, Subpixel},
    reexports::{calloop::EventLoop, wayland_server::Display},
    utils::{Rectangle, Transform},
};

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut ev_loop: EventLoop<LoopData> = EventLoop::try_new()?;

    let display: Display<Hatchling> = Display::new()?;
    let display_handle = display.handle();
    let state = Hatchling::new(&mut ev_loop, display)?;

    let mut data = LoopData {
        state,
        display_handle,
    };

    init_winit(&mut ev_loop, &mut data)?;

    let cmd = env::args().nth(2).unwrap_or("foot".to_owned());
    process::Command::new(cmd).spawn().ok();

    ev_loop.run(None, &mut data, move |_| {
        // we runnin bois
    })?;

    Ok(())
}

fn init_winit(ev_loop: &mut EventLoop<LoopData>, data: &mut LoopData) -> anyhow::Result<()> {
    let (handle, state) = (&mut data.display_handle, &mut data.state);

    let (mut backend, winit) = winit::init().unwrap();
    let mode = Mode {
        size: backend.window_size(),
        refresh: 60_000,
    };

    let output = Output::new(
        "winit".to_owned(),
        PhysicalProperties {
            size: (0, 0).into(),
            subpixel: Subpixel::Unknown,
            make: "Smithay".into(),
            model: "Winit".into(),
        },
    );
    let _global = output.create_global::<Hatchling>(handle);
    output.change_current_state(
        Some(mode),
        Some(Transform::Flipped180),
        None,
        Some((0, 0).into()),
    );
    output.set_preferred(mode);

    state.space.map_output(&output, (0, 0));

    let mut damage_tracker = OutputDamageTracker::from_output(&output);
    std::env::set_var("WAYLAND_DISPLAY", &state.socket_name);

    ev_loop
        .handle()
        .insert_source(winit, move |event, _, data| {
            let display = &mut data.display_handle;
            let state = &mut data.state;

            match event {
                WinitEvent::Resized { size, .. } => {
                    output.change_current_state(
                        Some(Mode {
                            size,
                            refresh: 60_000,
                        }),
                        None,
                        None,
                        None,
                    );
                }
                WinitEvent::Input(event) => data.state.handle_input(event).unwrap(),
                WinitEvent::Redraw => {
                    let size = backend.window_size();
                    let damage = Rectangle::from_loc_and_size((0, 0), size);

                    backend.bind().unwrap();
                    desktop::space::render_output::<
                        _,
                        WaylandSurfaceRenderElement<GlesRenderer>,
                        _,
                        _,
                    >(
                        &output,
                        backend.renderer(),
                        1.0,
                        0,
                        [&state.space],
                        &[],
                        &mut damage_tracker,
                        [0.1, 0.1, 0.1, 1.0],
                    )
                    .unwrap();
                    backend.submit(Some(&[damage])).unwrap();

                    state.space.elements().for_each(|window| {
                        window.send_frame(
                            &output,
                            state.start_time.elapsed(),
                            Some(Duration::ZERO),
                            |_, _| Some(output.clone()),
                        )
                    });

                    state.space.refresh();
                    state.popups.cleanup();
                    let _ = display.flush_clients();

                    backend.window().request_redraw();
                }
                WinitEvent::CloseRequested => {
                    state.loop_signal.stop();
                }
                _ => (),
            }
        })
        .unwrap();

    Ok(())
}
