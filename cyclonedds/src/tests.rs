pub mod domain {
    static UNIQUE_DOMAIN: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

    pub fn unique_id() -> u32 {
        UNIQUE_DOMAIN.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
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

    impl crate::sample::Keyed for Data {
        type Key = std::convert::Infallible;

        fn from_key(_: &Self::Key) -> Self {
            Default::default()
        }

        fn into_key(self: Self) -> Self::Key {
            unreachable!()
        }
    }

    pub fn unique_name() -> String {
        let uuid = Uuid::new_v4();
        uuid.to_string()
    }
}
