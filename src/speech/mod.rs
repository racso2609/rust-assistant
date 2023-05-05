use ds_transcriber;
use anyhow::{anyhow};


pub fn get_audio(){

}

pub fn audio_to_text(file_path:&str)->Result<String, anyhow::Error>{
let mut model = ds_transcriber::model::instance_model(
    "model_file.pbmm",
    Some(file_path),
)?;
let config = ds_transcriber::StreamSettings::default();
let i_said = ds_transcriber::transcribe(config, &mut model)?;
println!("I said: {}", i_said);
Ok(i_said)
}
