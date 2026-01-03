#[macro_export]
macro_rules! html {
    ($tag:ident($($attr:ident = $val:expr),* $(,)?) { $($content:tt)* }) => {{
        format!(
            "<{}{}>{}</{}>",
            stringify!($tag),
            $crate::html!(@attrs $($attr = $val),*),
            $crate::html!(@children $($content)*),
            stringify!($tag)
        )
    }};

    ($tag:ident { $($content:tt)* }) => {{
        format!(
            "<{}>{}</{}>",
            stringify!($tag),
            $crate::html!(@children $($content)*),
            stringify!($tag)
        )
    }};

    (@attrs) => { String::new() };
    (@attrs $attr:ident = $val:expr) => {
        format!(r#" {}="{}""#, stringify!($attr), $val)
    };
    (@attrs $attr:ident = $val:expr, $($rest:tt)*) => {
        format!(
            r#" {}="{}"{}"#,
            stringify!($attr),
            $val,
            html!(@attrs $($rest)*)
        )
    };

    (@children) => { String::new() };

    (@children $text:literal) => { $text.to_string() };

    (@children { $expr:expr }) => { format!("{}", $expr) };

    (@children $tag:ident($($attr:ident = $val:expr),* $(,)?) { $($content:tt)* }) => {
        $crate::html!($tag($($attr = $val),*) { $($content)* })
    };

    (@children $tag:ident { $($content:tt)* }) => {
        $crate::html!($tag { $($content)* })
    };

    (@children $text:literal $($rest:tt)+) => {
        format!("{}{}", $text, $crate::html!(@children $($rest)+))
    };
    (@children { $expr:expr } $($rest:tt)+) => {
        format!("{}{}", $expr, $crate::html!(@children $($rest)+))
    };
    (@children $tag:ident($($attr:ident = $val:expr),* $(,)?) { $($content:tt)* } $($rest:tt)+) => {
        format!(
            "{}{}",
            html!($tag($($attr = $val),*) { $($content)* }),
            html!(@children $($rest)+)
        )
    };
    (@children $tag:ident { $($content:tt)* } $($rest:tt)+) => {
        format!(
            "{}{}",
            $crate::html!($tag { $($content)* }),
            $crate::html!(@children $($rest)+)
        )
    };
}
