#[derive(Debug)]
pub struct Data {
    pub project: String,
    pub version: String,
    pub entries: Vec<Entry>,
}

#[derive(Debug)]
pub struct Entry {
    pub name: String,
    pub domain: String,
    pub role: String,
    pub uri: String,
    pub dispname: String,
    pub priority: i32,
}
