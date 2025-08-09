

//use map_err here

#[derive(Debug)]
pub enum AudioError {
    //also temp
    Default,
    IoError(std::io::Error),
    DecoderError(rodio::decoder::DecoderError),
    OutputStreamBuilderError(rodio::StreamError),
    StreamError(rodio::StreamError),
    FileNotLoaded,
    NoDevice,
}
