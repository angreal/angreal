/// Pythonize a rust object with pythonize
macro_rules! pythonize_this {
    ($o:ident) => {{
        Python::attach(|py| -> Py<PyAny> { pythonize(py, &$o).unwrap().unbind() })
    }};
}

macro_rules! attr_copy {
    // Handle string attributes
    (str, $o:ident, $v:ident, $a:ident) => {
        if let Some(value) = $a.$v {
            $o = $o.$v(Box::leak(Box::new(value.to_string())).as_str());
        }
    };
    // Handle bool attributes
    (bool, $o:ident, $v:ident, $a:ident) => {
        if let Some(value) = $a.$v {
            $o = $o.$v(*Box::leak(Box::new(value)));
        }
    };
    // Handle char attributes
    (char, $o:ident, $v:ident, $a:ident) => {
        if let Some(value) = $a.$v {
            $o = $o.$v(*Box::leak(Box::new(value)));
        }
    };
    // Handle u64 attributes
    (u64, $o:ident, $v:ident, $a:ident) => {
        if let Some(value) = $a.$v {
            // Assuming the original intent was to ensure the value is u64 before leaking
            let leaked_value: &'static u64 = Box::leak(Box::new(value as u64));
            $o = $o.$v((*leaked_value).try_into().unwrap());
        }
    };
}

#[allow(unused_macros)]
macro_rules! result_or_return_err {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(err) => return Err(err).map_err(Into::into),
        }
    };
}

#[allow(unused_macros)]
macro_rules! value_or_return_err {
    ( $e:expr ) => {
        match $e {
            Some(x) => x,
            None => return Err(anyhow!("No value returned when one was expected.")),
        }
    };
}
