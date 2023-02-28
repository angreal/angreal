//!

/// set a string value on an objects attribute
macro_rules! attr_copy_str {
    ($o:ident, $v:ident, $a:ident) => {
        if $a.$v.is_some() {
            let w = Box::leak(Box::new($a.$v.unwrap()));
            $o = $o.$v(w.as_str());
        }
    };
}

/// set a bool value on an objects attribute
macro_rules! attr_copy_bool {
    ($o:ident, $v:ident, $a:ident) => {
        if $a.$v.is_some() {
            let w = Box::leak(Box::new($a.$v.unwrap()));
            $o = $o.$v(*w);
        }
    };
}

/// set a char value on an objects attribute
macro_rules! attr_copy_char {
    ($o:ident, $v:ident, $a:ident) => {
        if $a.$v.is_some() {
            let w = Box::leak(Box::new($a.$v.unwrap()));
            $o = $o.$v(*w);
        }
    };
}

/// set a uint64 value on an objects attribute
macro_rules! attr_copy_u64 {
    ($o:ident, $v:ident, $a:ident) => {
        if $a.$v.is_some() {
            let w = Box::leak(Box::new($a.$v.unwrap()));
            $o = $o.$v((*w as u64).try_into().unwrap());
        }
    };
}
