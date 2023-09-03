// use hello_rust::text_bot::invoke_crab;
use dotenv::dotenv;
use hello_rust::speech::get_audio;

fn main() {
    dotenv().ok();
    get_audio().ok();
    // let message = "yarvis reproduce un verano sin ti";
    // invoke_crab(message);
}
