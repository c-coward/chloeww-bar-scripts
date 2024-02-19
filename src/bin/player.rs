use std::thread::sleep;

use mpris::{DBusError, Event, LoopStatus, Player, PlayerFinder};

fn main() {
    main_loop();
}

fn main_loop() {
    loop {
        if let Some(player) = get_player() {
            handle_player_events(&player);
        }
        // If we ever run out of events (i.e. the player dies), restart the main loop
        sleep(std::time::Duration::from_millis(200));
    }
}

fn handle_player_events(player: &Player) {
    let _ = player_widget(player);
    if let Ok(events) = player.events() {
        for event in events.flatten() {
            match event {
                Event::PlayerShutDown => {
                    println!();
                    break;
                }
                _ => {
                    let _ = player_widget(player);
                }
            }
        }
    }
}

fn player_widget(player: &Player) -> Result<(), DBusError> {
    let shuffle = format!(
        "(button :class \"player-shuffle shuffle-{}\" :onclick \"playerctl shuffle Toggle\" \"󰒟\")",
        player.get_shuffle()?
    );
    let prev_track = "(button :class \"player-prev\" :onclick \"playerctl previous\" \"\")";
    let next_track = "(button :class \"player-next\" :onclick \"playerctl next\" \"\")";
    let play_pause =
        "(button :class \"player-play-pause\" :onclick \"playerctl play-pause\" \"󰐎\")";
    let (loop_class, loop_icon, loop_action) = match player.get_loop_status()? {
        LoopStatus::None => ("loop-false", "󰑖", "playerctl loop Playlist"),
        LoopStatus::Playlist => ("loop-true", "󰑖", "playerctl loop Track"),
        LoopStatus::Track => ("loop-true", "󰑘", "playerctl loop None"),
    };
    let looping = format!(
        "(button :class \"player-loop {loop_class}\" :onclick \"{loop_action}\" \"{loop_icon}\")"
    );
    let metadata = player.get_metadata()?;
    let artist = metadata.artists().map_or_else(String::new, |a| a.concat());
    let title = metadata.title().unwrap_or("");
    let metadata_widget = format!("(label :class \"player-data\" :text \"{artist} - {title}\")");

    println!("(box :class \"player\" :space-evenly false :spacing player-spacing {metadata_widget} (box :class \"player-controls\" :space-evenly false :spacing player-spacing {shuffle} {prev_track} {play_pause} {next_track} {looping} ))");

    Ok(())
}

fn get_player() -> Option<Player> {
    PlayerFinder::new().ok()?.find_active().ok()
}
