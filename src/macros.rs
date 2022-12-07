macro_rules! arg_set_raw {
    ($o:ident, $v:ident, $a:ident) => {
        
        let w = &*Box::leak(Box::new($a.$v));
        if w.is_some() {
            $o = $o.$v(w.unwrap());
        }
    };
}

macro_rules! arg_set_str {
    ($o:ident, $v:ident, $a:ident) => {
        
        let w = &*Box::leak(Box::new($a.$v));
        if w.is_some() {
            $o = $o.$v(w.unwrap().as_str());
        }
    };
}

macro_rules! arg_set_bool {
    ($o:ident, $v:ident, $a:ident) => {
        
        let w = &*Box::leak(Box::new($a.$v));
        if w.is_some() {
            $o = $o.$v(w.as_bool());
        }
    };
}

macro_rules! arg_set_char {
    ($o:ident, $v:ident, $a:ident) => {
        
        let w = &*Box::leak(Box::new($a.$v));
        if w.is_some() {
            $o = $o.$v(w.as_char());
        }
    };
}

macro_rules! arg_set_u64 {
    ($o:ident, $v:ident, $a:ident) => {
        
        let w = &*Box::leak(Box::new($a.$v));
        if w.is_some() {
            $o = $o.$v(w.as_i64() as u64);
        }
    };
}

macro_rules! arg_set_usize {
    ($o:ident, $v:ident, $a:ident) => {
        let w = &*Box::leak(Box::new($a.$v));
        if w.is_some() {
            $o = $o.$v(w.as_i64() as usize);
        }
    };
}