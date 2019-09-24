#[rustfmt::skip] const D51STR1: &'static str = "      ====        ________                ___________ ";
#[rustfmt::skip] const D51STR2: &'static str = "  _D _|  |_______/        \\__I_I_____===__|_________| ";
#[rustfmt::skip] const D51STR3: &'static str = "   |(_)---  |   H\\________/ |   |        =|___ ___|       _________________         ";
#[rustfmt::skip] const D51STR4: &'static str = "   /     |  |   H  |  |     |   |         ||_| |_||      _|                \\_____A  ";
#[rustfmt::skip] const D51STR5: &'static str = "  |      |  |   H  |__--------------------| [___] |    =|                        |  ";
#[rustfmt::skip] const D51STR6: &'static str = "  | ________|___H__/__|_____/[][]~\\_______|       |    -|                        |  ";
#[rustfmt::skip] const D51STR7: &'static str = "  |/ |   |-----------I_____I [][] []  D   |=======|__ __|________________________|_ ";

#[rustfmt::skip] const D51WHL11: &'static str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ |__________________________|_ ";
#[rustfmt::skip] const D51WHL12: &'static str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/           |_D__D__D_|  |_D__D__D_|   ";
#[rustfmt::skip] const D51WHL13: &'static str = "  \\_/      \\O=====O=====O=====O_/      \\_/                \\_/   \\_/    \\_/   \\_/    ";

#[rustfmt::skip] const D51WHL21: &'static str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ |__________________________|_ ";
#[rustfmt::skip] const D51WHL22: &'static str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/           |_D__D__D_|  |_D__D__D_|   ";
#[rustfmt::skip] const D51WHL23: &'static str = "  \\_/      \\_O=====O=====O=====O/      \\_/                \\_/   \\_/    \\_/   \\_/    ";

#[rustfmt::skip] const D51WHL31: &'static str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ |__________________________|_ ";
#[rustfmt::skip] const D51WHL32: &'static str = " |/-=|___|=   O=====O=====O=====O|_____/~\\___/           |_D__D__D_|  |_D__D__D_|   ";
#[rustfmt::skip] const D51WHL33: &'static str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/                \\_/   \\_/    \\_/   \\_/    ";

#[rustfmt::skip] const D51WHL41: &'static str = "__/ =| o |=-~O=====O=====O=====O\\ ____Y___________|__ |__________________________|_ ";
#[rustfmt::skip] const D51WHL42: &'static str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/           |_D__D__D_|  |_D__D__D_|   ";
#[rustfmt::skip] const D51WHL43: &'static str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/                \\_/   \\_/    \\_/   \\_/    ";

#[rustfmt::skip] const D51WHL51: &'static str = "__/ =| o |=-O=====O=====O=====O \\ ____Y___________|__ |__________________________|_ ";
#[rustfmt::skip] const D51WHL52: &'static str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/           |_D__D__D_|  |_D__D__D_|   ";
#[rustfmt::skip] const D51WHL53: &'static str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/                \\_/   \\_/    \\_/   \\_/    ";

#[rustfmt::skip] const D51WHL61: &'static str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ |__________________________|_ ";
#[rustfmt::skip] const D51WHL62: &'static str = " |/-=|___|=O=====O=====O=====O   |_____/~\\___/           |_D__D__D_|  |_D__D__D_|   ";
#[rustfmt::skip] const D51WHL63: &'static str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/                \\_/   \\_/    \\_/   \\_/    ";

pub const FUNNEL: usize = 7;

pub fn get(frame: usize) -> Vec<&'static str> {
    let mut lines = vec![
        D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7,
    ];

    match frame % 6 {
        0 => push_whl_lines(&mut lines, D51WHL11, D51WHL12, D51WHL13),
        1 => push_whl_lines(&mut lines, D51WHL21, D51WHL22, D51WHL23),
        2 => push_whl_lines(&mut lines, D51WHL31, D51WHL32, D51WHL33),
        3 => push_whl_lines(&mut lines, D51WHL41, D51WHL42, D51WHL43),
        4 => push_whl_lines(&mut lines, D51WHL51, D51WHL52, D51WHL53),
        5 => push_whl_lines(&mut lines, D51WHL61, D51WHL62, D51WHL63),
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
) {
    vec.push(l1);
    vec.push(l2);
    vec.push(l3);
}
