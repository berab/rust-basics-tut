pub trait SomeTrait {
   fn is_valid(&self) -> bool;
   // fn get_the_better_one(&self, some_other_dude: &Self) -> Self;
}

#[derive(Debug, Copy, Clone)]
pub struct RandomInfo {
   pub call_count: i64,
   pub some_bool: bool,
   pub some_int: i64
} // everything need to be public in order to be used outside the file

impl SomeTrait for RandomInfo {
   fn is_valid(&self) -> bool {
      self.some_bool
   }
}

// using impl, we can create associative function to a spesific struct type
impl RandomInfo{

   pub fn new(param_a: bool) -> Self { // Self is RandomInfo here since this is implemented especially for RandomInfo (Self)
      Self {
         call_count: 0,
         some_bool: !param_a,
         some_int: 8
      }
   } // this is like an initialization of a class xd

   pub fn is_smaller(&mut self, compare_to: i64) -> bool {
      self.call_count += 1;
      
      self.some_int < compare_to
   } // and this is like a function of a class

}