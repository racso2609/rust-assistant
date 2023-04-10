use crate::actions::{execute, Action, MUSIC};
use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

const ASSISTANT_NAME: &str = "yarvis";
static AVAILABLE_COMMANDS: [&str; 1] = [MUSIC];

pub fn crab_say(message: String) {
    let stdout = stdout();
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

pub fn invoke_crab(speech: &str) {
    _crab_logic(speech)
}

pub fn search_word(speech: &str, target: &str) -> bool {
    for word in speech.split_whitespace() {
        if word == target {
            return true;
        }
    }
    return false;
}

pub fn get_command_to_execute<'a>(speech: &'a str, target: &'a str) -> String {
    let command = speech.replace(ASSISTANT_NAME, "").replace(target, "");
    return String::from(command);
}

fn _assistant_is_called(speech: &str) -> bool {
    return search_word(speech, ASSISTANT_NAME);
}

fn crab_responses(action: &Action) -> String {
    let mut response: String = match action.action.as_str() {
        MUSIC => String::from("playing:"),
        _ => "unrecognize action ".to_string(),
    };

    response.push_str(action.command.as_str());

    return response;
}

fn _crab_logic(speech: &str) {
    if _assistant_is_called(speech) {
        if search_word(speech, "reproduce") {
            for action in AVAILABLE_COMMANDS.iter() {
                if search_word(speech, action) {
                    let command = get_command_to_execute(speech, action);
                    let action = Action {
                        action: action.to_string(),
                        command: command.to_string(),
                    };

                    crab_say(crab_responses(&action));
                    execute(action);
                    break;
                }
            }
        }
    }
}
