#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/openh264.rs"));


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_version() {
        let version: OpenH264Version = unsafe { WelsGetCodecVersion() };
        
        assert_eq!(version.uMajor, 1);
        assert_eq!(version.uMinor, 8);
        assert_eq!(version.uRevision, 0);
        assert_eq!(version.uReserved, 1806);
    }
}