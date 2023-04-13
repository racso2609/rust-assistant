pub mod music;
pub mod google;

use music::reproduce ;
pub const MUSIC: &str = "reproduce";

pub struct Action {
    pub action: String,
    pub command: String,
}

pub fn execute(action: Action) {
    match action.action.as_str() {
        MUSIC => reproduce(action.command.as_str()),
        _ => (),
    }
}
