use super::audiobuffer;
use super::audiostream;
// use super::naive_estimator::Naive;
use hodges::Source;
use hodges::State;
use simple_bpm::SimpleEstimator;
use std::process::Command;
use std::process::Stdio;

#[flame]
pub fn h_fr(filename: String, estimator: &mut SimpleEstimator) -> f32 {
    flame::start("open_file");
    let state: State<f32> =
        State::from_file(filename.clone()).expect("Failed to open file with libhodges");
    flame::end("open_file");

    estimator.analyse(state)
}

#[flame]
pub fn h_br_ia(filename: String, estimator: &mut SimpleEstimator) -> f32 {
    flame::start("open_file");
    let state: State<&[f32]> =
        State::from_file(filename.clone()).expect("Failed to open file with libhodges");
    flame::end("open_file");

    flame::start("alloc");
    let mut vec: Vec<f32> = Vec::with_capacity(1024 * 1024);
    flame::end("alloc");

    flame::start("read_into_buffer");
    while let Ok(buffer) = state.get() {
        vec.extend_from_slice(buffer);
    }
    flame::end("read_into_buffer");

    estimator.analyse(vec.into_iter())
}

#[flame]
pub fn h_br_ni(filename: String, estimator: &mut SimpleEstimator) -> f32 {
    flame::start("open_file");
    let state: State<&[f32]> =
        State::from_file(filename.clone()).expect("Failed to open file with libhodges");
    flame::end("open_file");

    estimator.analyse(state.flatten().cloned())
}

#[flame]
pub fn f_fr_ia(filename: String, estimator: &mut SimpleEstimator) -> f32 {
    flame::start("call_ffmpeg");
    let mut command = Command::new("ffmpeg");

    let args: Vec<String> = vec![
        "-loglevel",
        "quiet",
        // the first ffmpeg argument is the input filename
        "-i",
        filename.as_str(),
        // next, the format of the output,
        "-f",
        "f32le",
        // then the codec
        "-acodec",
        "pcm_f32le",
        // then the number of channels in the output
        "-ac",
        "1",
        //  our output sample rate
        "-ar",
        "44100",
        // finally, tell ffmpeg to write to stdout
        "pipe:1",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    command.args(args);

    let mut child = command
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn ffmpeg child process");

    flame::end("call_ffmpeg");

    let buffer = match &mut child.stdout {
        Some(s) => Some(audiobuffer::AudioBuffer::from_stream(s)),
        None => panic!("Failed to get stdio from child stream!"),
    }
    .expect("Failed to construct buffer!");

    child.wait().expect("Failed to wait on ffmpeg child call!");

    estimator.analyse(buffer.into_iter())
}

#[flame]
pub fn f_fr(filename: String, estimator: &mut SimpleEstimator) -> f32 {
    flame::start("call_ffmpeg");
    let mut command = Command::new("ffmpeg");

    let args: Vec<String> = vec![
        "-loglevel",
        "quiet",
        // the first ffmpeg argument is the input filename
        "-i",
        filename.as_str(),
        // next, the format of the output,
        "-f",
        "f32le",
        // then the codec
        "-acodec",
        "pcm_f32le",
        // then the number of channels in the output
        "-ac",
        "1",
        //  our output sample rate
        "-ar",
        "44100",
        // finally, tell ffmpeg to write to stdout
        "pipe:1",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    command.args(args);

    let mut child = command
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn ffmpeg child process");

    flame::end("call_ffmpeg");

    let result = {
        let stream = match &mut child.stdout {
            Some(s) => Some(audiostream::AudioStream::from_stream(s)),
            None => panic!("Failed to get stdio from child stream!"),
        }
        .expect("Failed to construct stream!");

        let bpm = estimator.analyse(stream);
        bpm
    };

    child.wait().expect("Failed to wait on ffmpeg child call!");

    result
}
