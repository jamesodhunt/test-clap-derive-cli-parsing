use anyhow::Result;

mod args;
mod handlers;

fn main() -> Result<()> {
    args::handle()
}
