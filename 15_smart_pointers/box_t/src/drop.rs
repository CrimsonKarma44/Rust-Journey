pub struct CustomerSmartPointer {
  pub data: String,
}
// for early cleanup use the std::mem::drop function.
impl Drop for CustomerSmartPointer{
    fn drop(&mut self) {
        println!("Dropping CustomerSmartPointer with data `{}`!", self.data);
    }
}
