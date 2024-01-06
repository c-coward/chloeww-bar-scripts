use hyprland::data;
use hyprland::event_listener::EventListener;
use hyprland::prelude::*;
use hyprland::Result;

use clap::Parser;

fn main() -> Result<()> {
    let args = Args::parse();
    let mut listener = EventListener::new();
    let hndlr = move || handle(workspaces_widget(args.workspaces.clamp(1, 9), args.starting_workspace));
    listener.add_active_monitor_change_handler(move |_| hndlr());
    listener.add_window_moved_handler(move |_| hndlr());
    listener.add_window_close_handler(move |_| hndlr());
    listener.add_window_open_handler(move |_| hndlr());
    listener.add_workspace_change_handler(move |_| hndlr());
    listener.add_workspace_added_handler(move |_| hndlr());
    listener.add_workspace_destroy_handler(move |_| hndlr());
    
    hndlr();
    listener.start_listener()
}

#[derive(Parser)]
struct Args {
    /// Number of workspaces to include, clamped to [1,9]
    #[arg(short, long, default_value_t=9)]
    workspaces: i32,

    /// Workspace to start counting from. Useful for multi-monitor setups
    #[arg(short, long, default_value_t=1)]
    starting_workspace: i32
}

fn handle(result: Result<()>) -> () {
    match result {
        Ok(_) => (),
        Err(e) => panic!("{:#?}", e)
    }
}

fn workspaces_widget(workspace_count: i32, starting_workspace: i32) -> Result<()> {
    let open_workspaces: Vec<_> = data::Workspaces::get()?.map(|w| w.id).collect();
    let active_workspace = data::Workspace::get_active()?.id;

    let eventboxes: Vec<String> = (starting_workspace..(workspace_count + starting_workspace)).map(|i| {
        let img = if i == active_workspace {"active"} else if open_workspaces.contains(&i) {"open"} else {"empty"};
        let cmd = format!("hyprsome workspace {}", i);
        let image_w = format!("(image :image-height {{height}} :path \"./icons/{}.svg\")", img);
        let eventbox_w = format!("(eventbox :class \"ws-button\" :onclick \"{}\" {})", cmd, image_w);
        
        eventbox_w
    }).collect();

    println!("(box :class \"workspaces\" :orientation \"h\" :space-evenly false :spacing 10 :halign \"start\" {})", eventboxes.join(" "));
    Ok(())
}