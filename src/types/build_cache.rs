#[derive(Debug)]
pub struct BuildCache {
    pub vendor_appends: &'static str,
    pub vendor_prepends: &'static str,
    pub application_appends: &'static str,
    pub application_prepends: &'static str,
    pub test_appends: &'static str,
    pub test_prepends: &'static str,
}

impl BuildCache {
    pub fn new() -> Self {
        BuildCache {
            vendor_appends: "",
            vendor_prepends: "",
            application_appends: "",
            application_prepends: "",
            test_appends: "",
            test_prepends: ""
        }
    }
}
