use aoc_lib::TracingAlloc;
use color_eyre::Result;

mod days;

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc;

#[cfg(feature = "testing")]
type ErrType<'a> = chumsky::extra::Default;

#[cfg(not(feature = "testing"))]
type ErrType<'a> = chumsky::extra::Err<chumsky::error::Rich<'a, char>>;

trait Parser<'a, T>: chumsky::Parser<'a, &'a str, T, ErrType<'a>> {}
impl<'a, O, T> Parser<'a, O> for T where T: chumsky::Parser<'a, &'a str, O, ErrType<'a>> {}

fn main() -> Result<()> {
    color_eyre::install()?;
    aoc_lib::run(&ALLOC, 2023, days::DAYS)?;

    Ok(())
}
