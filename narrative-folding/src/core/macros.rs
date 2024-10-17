#[macro_export]
macro_rules! define_state {
    ($name:ident, $($field:ident: $type:ty),*) => {
        pub struct $name {
            $(pub $field: $type),*
        }

        impl State<$name> {
            pub fn new($($field: $type),*) -> Self {
                State { data: $name { $($field),* } }
            }
        }
    };
}
