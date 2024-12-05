pub struct KeyConfig {
    pub key_a: char,
    pub key_b: char,
    pub key_c: char,
    pub key_d: char,
    pub key_q: char,
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self{
            key_a: 'a',
            key_b: 'b',
            key_c:'c',
            key_d:'d',
            key_q:'q'
        }
    }
}