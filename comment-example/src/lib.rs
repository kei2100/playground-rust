//! ここに包括的なコメントを書く

/// # MY_CONST
/// マークダウンが記述できる。
/// * foo
/// * bar
///
/// ## コードもかける
/// ```rust
/// pub const MY_CONST:u32 = 100;
/// ```
pub const MY_CONST:u32 = 100;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

