use aoc_lib::TracingAlloc;
use color_eyre::Result;

mod days;

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc;

fn main() -> Result<()> {
    color_eyre::install()?;
    aoc_lib::run(&ALLOC, 2023, days::DAYS)?;

    Ok(())
}
