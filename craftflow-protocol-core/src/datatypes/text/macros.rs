/// Macro for generating a `Text` object.
///
/// Usage:
/// ```rust
/// # use craftflow_protocol_core::text;
/// let example = text!("Hello, world!");
/// let some_formatting = text!("This text will be bold and italic", bold, italic = true, underlined = false);
/// let colors = text!("This text will be red", color = "red");
/// ```
#[macro_export]
macro_rules! text {
    ($text:expr $(, $key:ident $(= $value:expr)? )* ) => {
        $crate::datatypes::text::Text::Object(::std::boxed::Box::new(
        	$crate::datatypes::text::TextObject {
	            content: $crate::datatypes::text::TextContent::Text {
					text: $text.to_string()
				},
	            extra: ::std::vec::Vec::new(),
	            $($key: text!(@format $key $(= $value)?),)*
	            ..<$crate::datatypes::text::TextObject as ::std::default::Default>::default()
         	}
        ))
    };

    // Helper macro for formatting options
    (@format $key:ident) => {
        Some(true)
    };
    (@format $key:ident = $value:expr) => {
        Some($value.to_owned())
    };
}
