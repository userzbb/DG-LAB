//! BLE 设备扫描器

/// 扫描结果
#[derive(Debug, Clone)]
pub struct ScanResult {
    /// 设备 ID
    pub id: String,
    /// 设备名称
    pub name: String,
    /// 设备地址
    pub address: String,
    /// 信号强度
    pub rssi: Option<i16>,
}

/// BLE 扫描器
pub struct BleScanner {
    /// 扫描结果
    results: Vec<ScanResult>,
}

impl BleScanner {
    /// 创建新的扫描器
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    /// 获取扫描结果
    pub fn results(&self) -> &[ScanResult] {
        &self.results
    }

    /// 清空扫描结果
    pub fn clear(&mut self) {
        self.results.clear();
    }

    /// 添加扫描结果
    pub fn add_result(&mut self, result: ScanResult) {
        // 检查是否已存在相同 ID 的设备
        if let Some(existing) = self.results.iter_mut().find(|r| r.id == result.id) {
            *existing = result;
        } else {
            self.results.push(result);
        }
    }

    /// 按名称查找设备
    pub fn find_by_name(&self, name: &str) -> Option<&ScanResult> {
        self.results
            .iter()
            .find(|r| r.name.to_lowercase().contains(&name.to_lowercase()))
    }

    /// 按 ID 查找设备
    pub fn find_by_id(&self, id: &str) -> Option<&ScanResult> {
        self.results.iter().find(|r| r.id == id)
    }
}

impl Default for BleScanner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_result(id: &str, name: &str, address: &str, rssi: Option<i16>) -> ScanResult {
        ScanResult {
            id: id.to_string(),
            name: name.to_string(),
            address: address.to_string(),
            rssi,
        }
    }

    #[test]
    fn test_new_scanner_empty() {
        let scanner = BleScanner::new();
        assert!(scanner.results().is_empty());
    }

    #[test]
    fn test_default_scanner_empty() {
        let scanner = BleScanner::default();
        assert!(scanner.results().is_empty());
    }

    #[test]
    fn test_add_result() {
        let mut scanner = BleScanner::new();
        scanner.add_result(make_result(
            "id1",
            "DG-LAB Coyote",
            "AA:BB:CC:DD:EE:FF",
            Some(-50),
        ));
        assert_eq!(scanner.results().len(), 1);
        assert_eq!(scanner.results()[0].name, "DG-LAB Coyote");
    }

    #[test]
    fn test_add_multiple_results() {
        let mut scanner = BleScanner::new();
        scanner.add_result(make_result(
            "id1",
            "Device A",
            "AA:BB:CC:DD:EE:01",
            Some(-40),
        ));
        scanner.add_result(make_result(
            "id2",
            "Device B",
            "AA:BB:CC:DD:EE:02",
            Some(-60),
        ));
        assert_eq!(scanner.results().len(), 2);
    }

    #[test]
    fn test_add_result_deduplicates_by_id() {
        let mut scanner = BleScanner::new();
        scanner.add_result(make_result("id1", "Old Name", "addr1", Some(-80)));
        scanner.add_result(make_result("id1", "New Name", "addr2", Some(-30)));
        assert_eq!(scanner.results().len(), 1);
        assert_eq!(scanner.results()[0].name, "New Name");
        assert_eq!(scanner.results()[0].address, "addr2");
        assert_eq!(scanner.results()[0].rssi, Some(-30));
    }

    #[test]
    fn test_clear() {
        let mut scanner = BleScanner::new();
        scanner.add_result(make_result("id1", "Device", "addr", None));
        scanner.add_result(make_result("id2", "Device2", "addr2", None));
        assert_eq!(scanner.results().len(), 2);

        scanner.clear();
        assert!(scanner.results().is_empty());
    }

    #[test]
    fn test_find_by_name_exact() {
        let mut scanner = BleScanner::new();
        scanner.add_result(make_result("id1", "DG-LAB Coyote", "addr", Some(-50)));
        let found = scanner.find_by_name("DG-LAB Coyote");
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, "id1");
    }

    #[test]
    fn test_find_by_name_case_insensitive() {
        let mut scanner = BleScanner::new();
        scanner.add_result(make_result("id1", "DG-LAB Coyote", "addr", None));
        assert!(scanner.find_by_name("dg-lab").is_some());
        assert!(scanner.find_by_name("COYOTE").is_some());
        assert!(scanner.find_by_name("dg-lab coyote").is_some());
    }

    #[test]
    fn test_find_by_name_partial_match() {
        let mut scanner = BleScanner::new();
        scanner.add_result(make_result("id1", "DG-LAB Coyote V3", "addr", None));
        assert!(scanner.find_by_name("Coyote").is_some());
    }

    #[test]
    fn test_find_by_name_not_found() {
        let mut scanner = BleScanner::new();
        scanner.add_result(make_result("id1", "DG-LAB Coyote", "addr", None));
        assert!(scanner.find_by_name("nonexistent").is_none());
    }

    #[test]
    fn test_find_by_id() {
        let mut scanner = BleScanner::new();
        scanner.add_result(make_result("device-001", "DG-LAB", "addr", None));
        scanner.add_result(make_result("device-002", "Other", "addr2", None));

        let found = scanner.find_by_id("device-001");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "DG-LAB");
    }

    #[test]
    fn test_find_by_id_not_found() {
        let scanner = BleScanner::new();
        assert!(scanner.find_by_id("nonexistent").is_none());
    }

    #[test]
    fn test_rssi_stored_correctly() {
        let mut scanner = BleScanner::new();
        scanner.add_result(make_result("id1", "Device", "addr", Some(-75)));
        scanner.add_result(make_result("id2", "Device2", "addr2", None));

        assert_eq!(scanner.results()[0].rssi, Some(-75));
        assert_eq!(scanner.results()[1].rssi, None);
    }
}
