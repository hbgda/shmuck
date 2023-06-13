pub fn pad_buf(buf: &[u8], pad: usize) -> Vec<u8> {
    let mut padded: Vec<u8> = vec![0u8; buf.len() + pad];
    padded[pad..].copy_from_slice(buf);
    padded
}