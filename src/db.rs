//use vt100;
use std::cell::RefCell;
use std::fmt::Write as FmtWrite;
use std::fmt::Display;

pub struct Frame {
    pub show: bool,
    pub module_path: String,
    pub line: usize,
    pub bracket_indent: Box<Display>,
    pub bracket_close: Box<Display>,
}


thread_local!(pub static LOGO_PRINT:
              RefCell<bool> =
              RefCell::new(true));

thread_local!(pub static SEAM_COUNT:
              RefCell<usize> =
              RefCell::new(0));

thread_local!(pub static INDENT:
              RefCell<Vec<Frame>> =
              RefCell::new(vec![]));

pub fn seam_count_bump() -> usize {
    SEAM_COUNT.with(
        |x| { let y = x.borrow().clone();
              *x.borrow_mut() += 1;
              y })
}

pub fn logo() {
    LOGO_PRINT.with(
        |x| {
            if x.borrow().clone() {
                use vt100;
                println!("\n{}", vt100::FungiLogo5Lines20180811C{});
            };            
            *x.borrow_mut() = false;
        })
}

macro_rules! fgi_db {
    ( $fmt_string:expr ) => {{
        use db;
        //db::logo();
        if db::test_show() {
            use regex::Regex;
            let re1 = Regex::new(r"\n").unwrap();
            let re2 = Regex::new(r"\^").unwrap();
            let s = format!( $fmt_string );
            let adjust = if re2.is_match(s.as_str()) { -1 } else { 0 };
            let s = format!("{}{}{}",
                            db::indent_string(adjust),
                            if adjust == -1 { "├᚜" } else { "" },
                            re1.replace_all(s.as_str(), db::newline_indent_string(0).as_str()));
            let s = format!("{}", re2.replace_all(s.as_str(), ""));
            println!("{}", s);
        }
    }};
    ( $fmt_string:expr, $( $arg:expr ),* ) => {{
        use db;
        //db::logo();
        if db::test_show() {
            use regex::Regex;
            let re1 = Regex::new(r"\n").unwrap();
            let re2 = Regex::new(r"\^").unwrap();
            let s = format!( $fmt_string, $( $arg ),* );
            let adjust = if re2.is_match(s.as_str()) { -1 } else { 0 };
            let s = format!("{}{}{}",
                            db::indent_string(adjust),
                            if adjust == -1 { "├᚜\x1B[1m" } else { "" },
                            re1.replace_all(s.as_str(), db::newline_indent_string(0).as_str()));
            let s = format!("{}", re2.replace_all(s.as_str(), ""));
            println!("{}\x1B[0;0m", s);
        }}
    }
}

macro_rules! db_region_open {
    () => {{ db_region_open!(false ; true ; vt100::NormBracket ) }}
    ;
    ($show:expr, $($br:tt)+) => {{
        db_region_open!( false ; $show ; $($br)+ )
    }};
    // Custom bracket style
    ($is_seam:expr ; $show:expr ; $($br:tt)+) => {{
        use db;
        use vt100;
        if $show {
            fgi_db!("{}{}{}{}:{}",
                    vt100::Normal{},
                    $($br)+::Open,
                    vt100::Lo{},
                    module_path!(), line!());
        }
        db::INDENT.with(
            |x| (*x.borrow_mut()).push(
                db::Frame{
                    show:$show,
                    bracket_indent:Box::new($($br)+::Indent),
                    bracket_close:Box::new($($br)+::Close),
                    module_path:module_path!().to_owned(),
                    line:line!() as usize,
                }));
    }}
}

macro_rules! db_region_close {
    () => {{
        use db;
        use vt100;
        let frame = db::INDENT.with(|x| (*x.borrow_mut()).pop().unwrap() );
        //fgi_db!("└᚜═╸╸╸╸᚜\x1B[2m{}:{}\x1B[0;0m", module_path!(), line!());
        if frame.show {
            fgi_db!("{}{}{}{}:{}",
                    vt100::Normal{},
                    frame.bracket_close,
                    vt100::Lo{},
                    module_path!(), line!());
        }
    }}
}

pub fn write_indent<Wr:FmtWrite>(wr: &mut Wr, _adjust:i64) {
    use vt100;
    INDENT.with(|x| for fr in (*x.borrow()).iter() {
        write!(wr, "{}{}", vt100::Normal{}, fr.bracket_indent).unwrap();
    });
}

pub fn test_show() -> bool {
    let mut show = true;
    INDENT.with(|x| for fr in (*x.borrow()).iter() {
        if fr.show == false { show = false; break }
        else { continue }
    });
    return show
}

pub fn newline_indent_string(adjust:i64) -> String {
    let mut s = String::new();
    write!(s, "\n").unwrap();
    write_indent(&mut s, adjust);
    s
}

pub fn indent_string(adjust:i64) -> String {
    let mut s = String::new();
    write_indent(&mut s, adjust);
    s
}
