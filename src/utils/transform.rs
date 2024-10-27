pub fn transform_vec_to_string(vec: &Vec<u8>) -> String {
    let std_err: Vec<u8> = vec.iter().map(|&code| code).collect();
    String::from_utf8_lossy(&std_err).to_string()
}
