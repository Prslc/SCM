#[macro_export]
macro_rules! opt_getter {
    ($name:ident, $field:ident) => {
        pub fn $name(&self) -> Option<i32> {
            self.$field
        }
    };
    
    ($name:ident, $field:ident, str) => {
        pub fn $name(&self) -> Option<&str> {
            self.$field.as_deref()
        }
    };
}