pub mod add;
pub mod delete;
pub mod list;
pub mod list_ssh;
pub mod show;
pub mod switch;
pub mod update;
pub mod add_ssh;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "Git Account Switcher")]
#[command(about = "A tool to easily switch between Git accounts")]
#[command(
    long_about = "A tool to easily switch between Git accounts. Manage multiple Git accounts and switch between them quickly."
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug)]
pub enum Commands {
    #[command(name = "add", about = "Add a new Git profile")]
    Add(add::AddCommand),

    #[command(name = "delete", about = "Delete an existing Git profile")]
    Delete(delete::DeleteCommand),

    #[command(name = "list", about = "Show all saved Git profiles")]
    List(list::ListCommand),

    #[command(
        name = "list-ssh",
        about = "List configured SSH hosts from ~/.ssh/config"
    )]
    ListSsh(list_ssh::ListSshCommand),

    #[command(name = "show", about = "Show current Git configuration")]
    Show(show::ShowCommand),

    #[command(name = "switch", about = "Switch to a different Git profile")]
    Switch(switch::SwitchCommand),

    #[command(name = "update", about = "Update an existing Git profile")]
    Update(update::UpdateCommand),
    #[command(name = "add-ssh", about = "Add a new SSH host to ~/.ssh/config")]
    AddSsh(add_ssh::AddSshCommand),
}
