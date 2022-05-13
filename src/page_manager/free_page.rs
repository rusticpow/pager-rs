trait FreePage {
     fn get_free_pids(&self, chunks_length: usize) -> Vec<u32>;
}

struct FreePageImpl {}

impl FreePageImpl {
    fn get_free_pids(&self, chunks_length: usize) -> Vec<u32> {

    }
}

