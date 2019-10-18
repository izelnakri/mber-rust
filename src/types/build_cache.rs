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
    pub fn insert<'a>(mut self, key: &str, value: &'static str) -> Self {
        match key {
            "vendor_appends" => { self.vendor_appends = &value },
            "vendor_prepends" => { self.vendor_prepends = &value },
            "application_appends" => { self.application_appends = &value },
            "application_prepends" => { self.application_prepends = &value },
            "test_appends" => { self.test_appends = &value },
            "test_prepends" => { self.test_prepends = &value },
            _ => {}
        };

        return self;
    }
}
