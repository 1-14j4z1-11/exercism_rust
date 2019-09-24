#[rustfmt::skip] static SMOKE_11: &'static str = "                (@@) (  ) (@)  ( )  @@   ()   @    O    @    O    @   ";
#[rustfmt::skip] static SMOKE_12: &'static str = "           (   )                                                      ";
#[rustfmt::skip] static SMOKE_13: &'static str = "       (@@@@)                                                         ";
#[rustfmt::skip] static SMOKE_14: &'static str = "    (    )                                                            ";
#[rustfmt::skip] static SMOKE_15: &'static str = "                                                                      ";
#[rustfmt::skip] static SMOKE_16: &'static str = "  (@@@)                                                               ";

#[rustfmt::skip] static SMOKE_21: &'static str = "                (  ) (@@) ( )  (@)  ()   @@   O    @    O    @    O   ";
#[rustfmt::skip] static SMOKE_22: &'static str = "           (@@@)                                                      ";
#[rustfmt::skip] static SMOKE_23: &'static str = "       (    )                                                         ";
#[rustfmt::skip] static SMOKE_24: &'static str = "    (@@@@)                                                            ";
#[rustfmt::skip] static SMOKE_25: &'static str = "                                                                      ";
#[rustfmt::skip] static SMOKE_26: &'static str = "  (   )                                                               ";

const FRAME_SKIP: usize = 4;

pub fn get(frame: usize) -> Vec<&'static str> {
    match frame % (FRAME_SKIP * 4) / FRAME_SKIP {
        0 => vec![SMOKE_11, SMOKE_12, SMOKE_13, SMOKE_14, SMOKE_15, SMOKE_16],
        1 => vec![
            &SMOKE_11[1..],
            &SMOKE_12[1..],
            &SMOKE_13[1..],
            &SMOKE_14[1..],
            &SMOKE_15[1..],
            &SMOKE_16[1..],
        ],
        2 => vec![SMOKE_21, SMOKE_22, SMOKE_23, SMOKE_24, SMOKE_25, SMOKE_26],
        3 => vec![
            &SMOKE_21[1..],
            &SMOKE_22[1..],
            &SMOKE_23[1..],
            &SMOKE_24[1..],
            &SMOKE_25[1..],
            &SMOKE_26[1..],
        ],
        _ => panic!(),
    }
}
