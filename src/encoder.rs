/// Traits that represent an a set of algorithm that can encode a string and then decoded that
/// encoded back to the original string
pub trait EncoderDecoder{
fn encode(data: &str) ->Self;
fn decode(&self) ->String;
} 
