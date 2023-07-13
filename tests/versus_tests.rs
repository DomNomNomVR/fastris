#[cfg(test)]
mod tests {
    use fastris::versus::*;
    extern crate fastris;

    extern crate flatbuffers;

    use flatbuffers::FlatBufferBuilder;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use rand_chacha::ChaChaRng;
    use std::collections::VecDeque;

    #[test]
    fn test_foo() {
        let versus = Versus::new(2, ChaCha8Rng::seed_from_u64(4));
    }
}
