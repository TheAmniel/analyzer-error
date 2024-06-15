#[macro_use]
extern crate napi_derive;

#[napi]
pub struct MyJsClass {
  pub input: String,
}

#[napi]
impl MyJsClass {
  // Js constructor
  #[napi(constructor)]
  pub fn new(input: Option<String>) -> Self {
    MyJsClass {
      input: input.unwrap_or("none".to_string()),
    }
  }

  // class getter
  #[napi(getter)]
  pub fn val(&self) -> &String {
    &self.input
  }

  // class setter
  #[napi(setter)]
  pub fn set_val(&mut self, input: String) {
    self.input = input;
  }

  // class method
  #[napi]
  pub fn get_val(&self) -> String {
    self.input.clone()
  }
}

// this work
#[napi(constructor)]
pub struct MyJsClass2 {
  // new MyJsClass2("input param")
  pub input: String,
}

#[napi]
impl MyJsClass2 {
  #[napi(getter)]
  pub fn val(&self) -> &String {
    &self.input
  }

  #[napi(setter)]
  pub fn set_val(&mut self, input: String) {
    self.input = input;
  }

  #[napi]
  pub fn get_val(&self) -> String {
    self.input.clone()
  }
}
