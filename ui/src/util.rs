macro_rules! navigate1 {
    ($page:expr, $navigate:expr) => {{
        $navigate($page, NavigateOptions::default());
        unreachable!();
    }};
}

pub(crate) use navigate1;
