#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IdNamespace(u64);

impl IdNamespace {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn as_u64(self) -> u64 {
        self.0
    }
}

macro_rules! scoped_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(u128);

        impl $name {
            pub const fn scoped(namespace: IdNamespace, local: u64) -> Self {
                Self(((namespace.as_u64() as u128) << 64) | local as u128)
            }

            pub const fn from_u128(value: u128) -> Self {
                Self(value)
            }

            pub const fn as_u128(self) -> u128 {
                self.0
            }

            pub const fn namespace(self) -> IdNamespace {
                IdNamespace::new((self.0 >> 64) as u64)
            }

            pub const fn local(self) -> u64 {
                self.0 as u64
            }
        }
    };
}

scoped_id!(EntityId);
scoped_id!(RelationshipId);
scoped_id!(ProjectionId);
scoped_id!(PrincipalId);
scoped_id!(IntentId);
scoped_id!(CommandId);
scoped_id!(EventId);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpatialState {
    pub position: Vec2,
    pub scale: f64,
    pub anchored: bool,
}

impl Default for SpatialState {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            scale: 1.0,
            anchored: false,
        }
    }
}
