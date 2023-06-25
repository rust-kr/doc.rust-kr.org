// ANCHOR: here
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --생략--
    // ANCHOR_END: here
    Ok(())
    // ANCHOR: here
}

fn function2() -> IoResult<()> {
    // --생략--
    // ANCHOR_END: here
    Ok(())
    // ANCHOR: here
}
// ANCHOR_END: here
