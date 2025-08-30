pub mod domain {
    static UNIQUE_DOMAIN: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    const MAX_DOMAIN_BOUNDS: u32 = 233;

    pub fn unique_id() -> u32 {
        let counter = UNIQUE_DOMAIN.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let result = counter % MAX_DOMAIN_BOUNDS;
        result
    }
}

pub mod topic {
    use uuid::Uuid;

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Eq, PartialEq, Default)]
    pub struct Data {
        pub x: u32,
        pub y: i32,
        pub message: String,
    }

    pub fn unique_name() -> String {
        let uuid = Uuid::new_v4();
        uuid.to_string()
    }
}
