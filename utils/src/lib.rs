pub mod encoding;

pub fn xor_buffers(input1: &[u8], input2: &[u8]) -> Result<Vec<u8>, String> {
    if input1.len() != input2.len() {
        return Err(format!(
            "size of input 1 ({}) does not match input 2 ({})",
            input1.len(),
            input2.len()
        ));
    }

    Ok(input1
        .into_iter()
        .zip(input2.into_iter())
        .map(|(a, b)| a ^ b)
        .collect())
}
