use querystring_tiny::PercentCoded;
use std::ops::Deref;

#[derive(Debug)]
struct Test {
    raw: &'static [u8],
    expected: PercentCoded,
}
impl Test {
    pub fn test(self) {
        // Test decode
        let decoded = PercentCoded::decode(self.raw).expect("Failed to percent-decode bytes");
        assert_eq!(decoded.deref(), self.expected.deref(), "Invalid percent-encoded bytes");

        // Test decode
        let encoded = self.expected.encode();
        assert_eq!(&encoded, self.raw, "Invalid percent-decoded bytes");
    }
}
#[test]
fn test() {
    Test {
        raw: b"%2FVolumes%2FData%2F%F0%9F%8D%86%0A",
        expected: PercentCoded::new(b"/Volumes/Data/\xF0\x9F\x8D\x86\x0A".as_slice()),
    }
    .test();
}
