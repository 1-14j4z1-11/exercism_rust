#[rustfmt::skip] static C51STR1: &'static str = "        ___                                             ";
#[rustfmt::skip] static C51STR2: &'static str = "       _|_|_  _     __       __             ___________ ";
#[rustfmt::skip] static C51STR3: &'static str = "    D__/   \\_(_)___|  |__H__|  |_____I_Ii_()|_________| ";
#[rustfmt::skip] static C51STR4: &'static str = "     | `---'   |:: `--'  H  `--'         |  |___ ___|      _________________         ";
#[rustfmt::skip] static C51STR5: &'static str = "    +|~~~~~~~~++::~~~~~~~H~~+=====+~~~~~~|~~||_| |_||     _|                \\_____A  ";
#[rustfmt::skip] static C51STR6: &'static str = "    ||        | ::       H  +=====+      |  |::  ...|   =|                        |  ";
#[rustfmt::skip] static C51STR7: &'static str = "|    | _______|_::-----------------[][]-----|       |   -|                        |  ";

#[rustfmt::skip] static C51WH11: &'static str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|____|________________________|_ ";
#[rustfmt::skip] static C51WH12: &'static str = "------'|oOo|==[]=-     ||      ||      |  ||=======_|__|__________________________|_ ";
#[rustfmt::skip] static C51WH13: &'static str = "/~\\____|___|/~\\_|   O=======O=======O  |__|+-/~\\_|        |_D__D__D_|  |_D__D__D_|   ";
#[rustfmt::skip] static C51WH14: &'static str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/           \\_/   \\_/    \\_/   \\_/    ";

#[rustfmt::skip] static C51WH21: &'static str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|____|________________________|_ ";
#[rustfmt::skip] static C51WH22: &'static str = "------'|oOo|===[]=-    ||      ||      |  ||=======_|__|__________________________|_ ";
#[rustfmt::skip] static C51WH23: &'static str = "/~\\____|___|/~\\_|    O=======O=======O |__|+-/~\\_|        |_D__D__D_|  |_D__D__D_|   ";
#[rustfmt::skip] static C51WH24: &'static str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/           \\_/   \\_/    \\_/   \\_/    ";

#[rustfmt::skip] static C51WH31: &'static str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|____|________________________|_ ";
#[rustfmt::skip] static C51WH32: &'static str = "------'|oOo|===[]=- O=======O=======O  |  ||=======_|__|__________________________|_ ";
#[rustfmt::skip] static C51WH33: &'static str = "/~\\____|___|/~\\_|      ||      ||      |__|+-/~\\_|        |_D__D__D_|  |_D__D__D_|   ";
#[rustfmt::skip] static C51WH34: &'static str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/           \\_/   \\_/    \\_/   \\_/    ";

#[rustfmt::skip] static C51WH41: &'static str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|____|________________________|_ ";
#[rustfmt::skip] static C51WH42: &'static str = "------'|oOo|==[]=- O=======O=======O   |  ||=======_|__|__________________________|_ ";
#[rustfmt::skip] static C51WH43: &'static str = "/~\\____|___|/~\\_|      ||      ||      |__|+-/~\\_|        |_D__D__D_|  |_D__D__D_|   ";
#[rustfmt::skip] static C51WH44: &'static str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/           \\_/   \\_/    \\_/   \\_/    ";

#[rustfmt::skip] static C51WH51: &'static str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|____|________________________|_ ";
#[rustfmt::skip] static C51WH52: &'static str = "------'|oOo|=[]=- O=======O=======O    |  ||=======_|__|__________________________|_ ";
#[rustfmt::skip] static C51WH53: &'static str = "/~\\____|___|/~\\_|      ||      ||      |__|+-/~\\_|        |_D__D__D_|  |_D__D__D_|   ";
#[rustfmt::skip] static C51WH54: &'static str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/           \\_/   \\_/    \\_/   \\_/    ";

#[rustfmt::skip] static C51WH61: &'static str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|____|________________________|_ ";
#[rustfmt::skip] static C51WH62: &'static str = "------'|oOo|=[]=-      ||      ||      |  ||=======_|__|__________________________|_ ";
#[rustfmt::skip] static C51WH63: &'static str = "/~\\____|___|/~\\_|  O=======O=======O   |__|+-/~\\_|        |_D__D__D_|  |_D__D__D_|   ";
#[rustfmt::skip] static C51WH64: &'static str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/           \\_/   \\_/    \\_/   \\_/    ";

pub const FUNNEL: usize = 7;

pub fn get(frame: usize) -> Vec<&'static str> {
    let mut lines = vec![
        C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7,
    ];

    match frame % 6 {
        0 => push_whl_lines(&mut lines, C51WH11, C51WH12, C51WH13, C51WH14),
        1 => push_whl_lines(&mut lines, C51WH21, C51WH22, C51WH23, C51WH24),
        2 => push_whl_lines(&mut lines, C51WH31, C51WH32, C51WH33, C51WH34),
        3 => push_whl_lines(&mut lines, C51WH41, C51WH42, C51WH43, C51WH44),
        4 => push_whl_lines(&mut lines, C51WH51, C51WH52, C51WH53, C51WH54),
        5 => push_whl_lines(&mut lines, C51WH61, C51WH62, C51WH63, C51WH64),
        _ => panic!(),
    }

    lines
}

#[inline(always)]
fn push_whl_lines(
    vec: &mut Vec<&'static str>,
    l1: &'static str,
    l2: &'static str,
    l3: &'static str,
    l4: &'static str,
) {
    vec.push(l1);
    vec.push(l2);
    vec.push(l3);
    vec.push(l4);
}
