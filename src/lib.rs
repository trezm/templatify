// For tests
extern crate bytes;

#[macro_export]
macro_rules! templatify {
  ( $head_template:expr $(;$key:expr; $template:expr)* ) => {
    {
      let mut total_length = 0;
      total_length += $head_template.len();

      $(
        total_length = total_length + $key.len() + $template.len();
      )*

      let mut output_string = String::with_capacity(total_length);
      output_string.push_str($head_template);

      $(
        output_string.push_str($key);
        output_string.push_str($template);
      )*

      output_string
    }
  }
}

#[macro_export]
macro_rules! templatify_buffer {
  ( $buffer:ident, $head_template:expr $(;$key:expr; $template:expr)* ) => {
    {
      let mut total_length = 0;
      total_length += $head_template.len();

      $(
        total_length = total_length + $key.len() + $template.len();
      )*

      $buffer.reserve(total_length);
      $buffer.put($head_template);

      $(
        $buffer.put($key);
        $buffer.put($template);
      )*
    }
  }
}

#[cfg(test)]
mod tests {
  use bytes::{BytesMut, BufMut};

  #[test]
  fn templatify_should_work() {
    let world = "world";
    let results: String = templatify! { "hello, "; world ;"!" };
    assert!(results == "hello, world!");
  }

  #[test]
  fn templatify_buffer_should_work() {
    let mut buf = BytesMut::new();

    let world = "world";
    templatify_buffer! { buf, "hello, "; world ;"!" };
    assert!(buf == "hello, world!");
  }
}

