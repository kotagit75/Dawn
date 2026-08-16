use vdf_rs::{PietrzakVDF, PietrzakVDFParams, VDF, VDFParams};

const VDF_BITS: u16 = 1024;

fn create_vdf() -> PietrzakVDF {
    PietrzakVDFParams(VDF_BITS).new()
}

pub fn verify_solution(difficulty: u64, challenge: &[u8], solution: &[u8]) -> bool {
    create_vdf().verify(challenge, difficulty, solution).is_ok()
}

pub fn solve(challenge: &[u8], difficulty: u64) -> Result<Vec<u8>, vdf_rs::InvalidIterations> {
    create_vdf().solve(challenge, difficulty)
}

pub fn solution_to_string(solution: &[u8]) -> String {
    solution.iter().map(|n| n.to_string()).collect::<String>()
}
