#[derive(clap::Parser)]
pub struct CmdFlags {
    #[arg(long)]
    pub only_migration: bool,
}
