use querystring_tiny::QueryString;
use std::{ collections::BTreeMap, ops::Deref };


/// Creates a new map
pub fn map<I, K, V>(pairs: I) -> BTreeMap<Vec<u8>, Vec<u8>>
    where I: IntoIterator<Item = (K, V)>, K: Into<Vec<u8>>, V: Into<Vec<u8>>
{
    pairs.into_iter()
        .map(|(k, v)| (k.into(), v.into()))
        .collect()
}



#[derive(Debug)]
struct Test {
    raw: &'static [u8],
    expected: BTreeMap<Vec<u8>, Vec<u8>>
}
impl Test {
    pub fn test(self) {
        // Parse the query string
        let query = QueryString::decode(self.raw).expect("Failed to decode query string");
        assert_eq!(&self.expected, query.deref(), "Invalid decoded query string");

        // Perform a re-encode+decode cycle to test encoding
        let reencoded = query.encode();
        let query2 = QueryString::decode(&reencoded).expect("Failed to decode query string (re-encoded)");
        assert_eq!(query.deref(), query2.deref(), "Invalid query string (re-encoded)");
    }
}
#[test]
fn test() {
    Test {
        raw: b"code=M696be062-f150-bb19-9944-0c3a0ca60b48&state=99f4bd624dbe53d0ae330eabda904ac4",
        expected: map([
            ("code", "M696be062-f150-bb19-9944-0c3a0ca60b48"),
            ("state", "99f4bd624dbe53d0ae330eabda904ac4")
        ])
    }.test();
    Test {
        raw: b"q=tree+-swing&l=commderiv&d=taken-20000101-20051231&ct=0&lol&mt=all&adv=1&&",
        expected: map([
            ("q", "tree+-swing"),
            ("l", "commderiv"),
            ("d", "taken-20000101-20051231"),
            ("ct", "0"),
            ("mt", "all"),
            ("adv", "1"),
            ("lol", "")
        ])
    }.test();
    Test {
        raw: b"",
        expected: BTreeMap::new()
    }.test();
}
