#[derive(Debug, Clone)]
pub struct PrepressProfile { pub cmyk: bool, pub bleed_mm: f32 }
impl PrepressProfile {
    pub fn default_print() -> Self { Self { cmyk: true, bleed_mm: 3.0 } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_prepress() {
        let p = PrepressProfile::default_print();
        assert!(p.cmyk);
    }
}
