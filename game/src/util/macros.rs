//https://stackoverflow.com/questions/41143225/how-to-define-a-hashmap-with-contents-in-1-line-in-rust
#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}