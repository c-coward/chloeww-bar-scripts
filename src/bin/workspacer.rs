use hyprland::data;
use hyprland::event_listener::EventListener;
use hyprland::prelude::*;
use hyprland::Result;

fn main() -> Result<()> {
    let mut listener = EventListener::new();
    let hndlr = move || panic_on_err(workspaces_widget());
    listener.add_active_monitor_change_handler(move |_| hndlr());
    listener.add_window_moved_handler(move |_| hndlr());
    listener.add_window_close_handler(move |_| hndlr());
    listener.add_window_open_handler(move |_| hndlr());
    listener.add_workspace_change_handler(move |_| hndlr());
    listener.add_workspace_added_handler(move |_| hndlr());
    listener.add_workspace_destroy_handler(move |_| hndlr());
    listener.add_active_window_change_handler(move |_| hndlr());

    hndlr();
    listener.start_listener()
}

fn panic_on_err<T>(result: Result<T>) -> T {
    match result {
        Ok(x) => x,
        Err(e) => panic!("{e:#?}"),
    }
}

fn workspaces_widget() -> Result<()> {
    let open_workspaces: Vec<_> = data::Workspaces::get()?.map(|w| w.id).collect();
    let active_workspace = data::Workspace::get_active()?.id;

    let first_half: Vec<String> = (1..=5)
        .map(|i| make_ws_button(i, &open_workspaces, active_workspace))
        .collect();
    let second_half: Vec<String> = (6..=10)
        .map(|i| make_ws_button(i, &open_workspaces, active_workspace))
        .collect();
    let middle = "(box :class \"ws-sep\")".to_string();
    let buttons = [&first_half[..], &[middle], &second_half[..]].concat();

    println!("(box :class \"workspaces\" :orientation \"h\" :space-evenly false :spacing ws-spacing :halign \"start\" {})", buttons.join(" "));
    Ok(())
}

fn make_ws_button(workspace_id: i32, open_workspaces: &[i32], active_workspace: i32) -> String {
    let class = if active_workspace == workspace_id {
        "active"
    } else if open_workspaces.contains(&workspace_id) {
        "full"
    } else {
        "inactive"
    };
    let cmd = format!("hyprctl dispatch workspace {workspace_id}");
    // let ws_string = if workspace_id == 10 {
    //     "X".to_string()
    // } else {
    //     workspace_id.to_string()
    // };
    format!("(button :class \"ws-button ws-{class}\" :onclick \"{cmd}\" \"{workspace_id}\")")
}
