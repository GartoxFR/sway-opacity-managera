use clap::Parser;
use swayipc::{Connection, Event, EventType, Fallible, WindowChange, WindowEvent};

#[derive(clap::Parser)]
struct Args {
    /// Opacity of the focused window
    focused_opacity: f32,
    /// Opacity of unfocused windows
    unfocused_opacity: f32,
}

fn main() -> Fallible<()> {
    let args = Args::parse();

    let subs = [EventType::Window];

    let sub_connection = Connection::new()?;
    let events = sub_connection.subscribe(subs)?;

    let mut command_connection = Connection::new()?;

    let mut last_focus: Option<swayipc::Node> = None;

    for event in events {
        let event = event?;
        if let Event::Window(window_event) = event {
            if let WindowChange::Focus = window_event.change {
                if let Some(last_focus) = last_focus {
                    command_connection.run_command(format!("[con_id = {}] opacity {}", last_focus.id, args.unfocused_opacity))?;
                }

                last_focus = Some(window_event.container);
                command_connection.run_command(format!("[con_id = __focused__] opacity {}", args.focused_opacity))?;
            }
        }
    }

    Ok(())
}
