#[macro_export]
macro_rules! impl_validation_traits {
    ($ty:ty) => {
        impl $crate::prelude::ValidateFile for $ty {}
        impl $crate::prelude::ValidateDirectory for $ty {}
        impl $crate::prelude::Validate for $ty {}
    };
}
