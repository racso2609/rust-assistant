use text_bot::invoke_crab;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let message = "yarvis reproduce un verano sin ti";
    invoke_crab(message);
}
