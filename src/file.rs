#[derive(PartialEq, PartialOrd, Ord, Eq)]
pub struct File<'a> {
    pub filename: String,
    pub size: u64,
    pub file_type: &'a str,
    pub permissions: &'a str,
    pub creation_time: u64,
}

impl File<'_> {
    pub fn parse_size(&self) -> String {
        if self.size < 1024 {
            format!("{} Bytes", self.size)
        } else if self.size < 1024 * 1024 {
            let size_in_kb = self.size as f64 / 1024.0;
            format!("{:.2} KB", size_in_kb)
        } else if self.size < 1024 * 1024 * 1024 {
            let size_in_mb = self.size as f64 / (1024.0 * 1024.0);
            format!("{:.2} MB", size_in_mb)
        } else {
            let size_in_gb = self.size as f64 / (1024.0 * 1024.0 * 1024.0);
            format!("{:.2} GB", size_in_gb)
        }
    }
}
