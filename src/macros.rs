/// Pythonize a rust object with pythonize
macro_rules! pythonize_this {
    ($o:ident) => {{
        Python::with_gil(|py| -> Py<PyAny> { pythonize(py, &$o).unwrap() })
    }};
}
/// set a string value on an objects attribute
macro_rules! attr_copy_str {
    ($o:ident, $v:ident, $a:ident) => {
        if let Some(value) = $a.$v {
            $o = $o.$v(Box::leak(Box::new(value)).as_str());
        }
    };
}

/// set a bool value on an objects attribute
macro_rules! attr_copy_bool {
    ($o:ident, $v:ident, $a:ident) => {
        if let Some(value) = $a.$v {
            $o = $o.$v(*Box::leak(Box::new(value)));
        }
    };
}

/// set a char value on an objects attribute
macro_rules! attr_copy_char {
    ($o:ident, $v:ident, $a:ident) => {
        if let Some(value) = $a.$v {
            $o = $o.$v(*Box::leak(Box::new(value)));
        }
    };
}

/// set a uint64 value on an objects attribute
macro_rules! attr_copy_u64 {
    ($o:ident, $v:ident, $a:ident) => {
        if let Some(value) = $a.$v {
            $o = $o.$v((*Box::leak(Box::new(value)) as u64).try_into().unwrap());
        }
    };
}
