pub enum MyOption<T>  {
    Some(T),
    None,
}
pub enum MyResult<T, E>  {
    Ok(T),
    Err(E),
}