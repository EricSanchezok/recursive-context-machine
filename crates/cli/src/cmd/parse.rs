use crate::args::ParseArgs;
use crate::rcm;

pub async fn run(args: ParseArgs) -> anyhow::Result<()> {
    let source = std::fs::read_to_string(&args.file)
        .map_err(|error| anyhow::anyhow!("failed to read {}: {error}", args.file.display()))?;
    let file = rcm::parse(&source).map_err(anyhow::Error::msg)?;
    let json = serde_json::to_string_pretty(&file).map_err(anyhow::Error::msg)?;
    println!("{json}");
    Ok(())
}
