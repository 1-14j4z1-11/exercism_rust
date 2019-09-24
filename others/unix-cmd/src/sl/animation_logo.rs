#[rustfmt::skip] const LOGO1: &'static str = "     ++      +------ ____                 ____________________ ____________________ ";
#[rustfmt::skip] const LOGO2: &'static str = "     ||      |+-+ |  |   \\@@@@@@@@@@@     |  ___ ___ ___ ___ | |  ___ ___ ___ ___ | ";
#[rustfmt::skip] const LOGO3: &'static str = "   /---------|| | |  |    \\@@@@@@@@@@@@@_ |  |_| |_| |_| |_| | |  |_| |_| |_| |_| | ";
#[rustfmt::skip] const LOGO4: &'static str = "  + ========  +-+ |  |                  | |__________________| |__________________| ";

#[rustfmt::skip] const LWHL11: &'static str = " _|--O========O~\\-+  |__________________| |__________________| |__________________| ";
#[rustfmt::skip] const LWHL12: &'static str = "//// \\_/      \\_/       (O)       (O)        (O)        (O)       (O)        (O)    ";

#[rustfmt::skip] const LWHL21: &'static str = " _|--/~\\------/~\\-+  |__________________| |__________________| |__________________| ";
#[rustfmt::skip] const LWHL22: &'static str = "//// O========O_/       (O)       (O)        (O)        (O)       (O)        (O)    ";

#[rustfmt::skip] const LWHL31: &'static str = " _|--/~\\------/~\\-+  |__________________| |__________________| |__________________| ";
#[rustfmt::skip] const LWHL32: &'static str = "//// \\O========O/       (O)       (O)        (O)        (O)       (O)        (O)    ";

#[rustfmt::skip] const LWHL41: &'static str = " _|--/~\\------/~\\-+  |__________________| |__________________| |__________________| ";
#[rustfmt::skip] const LWHL42: &'static str = "//// \\_O========O       (O)       (O)        (O)        (O)       (O)        (O)    ";

#[rustfmt::skip] const LWHL51: &'static str = " _|--/~O========O-+  |__________________| |__________________| |__________________| ";
#[rustfmt::skip] const LWHL52: &'static str = "//// \\_/      \\_/       (O)       (O)        (O)        (O)       (O)        (O)    ";

#[rustfmt::skip] const LWHL61: &'static str = " _|--/O========O\\-+  |__________________| |__________________| |__________________| ";
#[rustfmt::skip] const LWHL62: &'static str = "//// \\_/      \\_/       (O)       (O)        (O)        (O)       (O)        (O)    ";

pub const FUNNEL: usize = 4;

pub fn get(frame: usize) -> Vec<&'static str> {
    let mut lines = vec![LOGO1, LOGO2, LOGO3, LOGO4];

    match frame % 6 {
        0 => push_whl_lines(&mut lines, LWHL11, LWHL12),
        1 => push_whl_lines(&mut lines, LWHL21, LWHL22),
        2 => push_whl_lines(&mut lines, LWHL31, LWHL32),
        3 => push_whl_lines(&mut lines, LWHL41, LWHL42),
        4 => push_whl_lines(&mut lines, LWHL51, LWHL52),
        5 => push_whl_lines(&mut lines, LWHL61, LWHL62),
        _ => panic!(),
    }

    lines
}

#[inline(always)]
fn push_whl_lines(vec: &mut Vec<&'static str>, l1: &'static str, l2: &'static str) {
    vec.push(l1);
    vec.push(l2);
}
