use super::*;

pub fn round(size: usize, round: usize) -> usize {
    let round = round - 1;
    (size + round) & !round
}

pub trait IntoStream {
    fn into_stream(self) -> Vec<u8>;
}

impl IntoStream for Vec<u8> {
    fn into_stream(mut self) -> Self {
        self.resize(round(self.len(), 4), 0);
        self
    }
}
