use super::super::cli::arg::CliOptions;
use super::types::{CobenType, ImmunityType};
use formatx::formatx;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Debug)]
pub enum Chance {
    Immunity(ImmunityType),
    Coben(CobenType),
}

impl Chance {
    // pub fn is_immunity(&self) -> bool {
    //     self.is_immunity_and(|&_| true)
    // }
    // pub fn is_immunity_and(&self, f: impl FnOnce(&ImmunityType) -> bool) -> bool {
    //     matches!(self, Chance::Immunity(i) if f(i))
    // }
    // pub fn is_chance(&self) -> bool {
    //     self.is_chance_and(|&_| true)
    // }
    // pub fn is_chance_and(&self, f: impl FnOnce(&CobenType) -> bool) -> bool {
    //     matches!(self, Chance::Coben(c) if f(c))
    // }
    pub fn display_with_opts(&self, opts: CliOptions) -> String {
        match self {
            Chance::Immunity(i) => formatx!(opts.immunity_fmt, i).unwrap(),
            Chance::Coben(c) => formatx!(opts.coben_fmt, {
                let precision = opts.coben_precision;
                let divisor: CobenType = 10.0_f64.powi(precision as i32);
                let fnew: CobenType = (c * divisor).round() / divisor;
                let min: CobenType = 10.0_f64.powi(-(precision as i32));
                format!(
                    "{sign}{num:.precision$}%",
                    sign = if fnew < min { "<" } else { " " },
                    num = fnew.max(min),
                )
            })
            .unwrap(),
        }
    }
}

impl Display for Chance {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            self.display_with_opts(CliOptions {
                immunity_fmt: String::from("Immune ({})"),
                coben_fmt: String::from("{} COBEN"),
                coben_precision: 2,
            })
        )
    }
}
