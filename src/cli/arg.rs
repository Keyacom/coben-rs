use clap::Parser;

#[derive(Parser)]
#[command(about = "COBEN calculation CLI tool")]
pub struct CliOptions {
    #[arg(
        short,
        long,
        help = "Formatting for the immunity status. Must contain exactly one '{}' (braces can be escaped by doubling).",
        default_value = "Immune ({})",
    )]
    pub immunity_fmt: String,
    #[arg(
        short,
        long,
        help = "Formatting for the COBEN status. Must contain exactly one '{}' (braces can be escaped by doubling). The '%' sign is included automatically.",
        default_value = "{} COBEN",
    )]
    pub coben_fmt: String,
    #[arg(
        short = 'p',
        long = "precision",
        help = "Precision for COBEN formatting",
        default_value_t = 2
    )]
    pub coben_precision: usize,
}
