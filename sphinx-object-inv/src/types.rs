#[derive(Debug)]
pub struct SphinxObjectInv {
    pub project: String,
    pub version: String,
    pub entries: Vec<SphinxObjectInvEntry>,
}

#[derive(Debug)]
pub struct SphinxObjectInvEntry {
    pub name: String,
    pub domain: String,
    pub role: String,
    pub uri: String,
    pub dispname: String,
    pub priority: i32,
}
