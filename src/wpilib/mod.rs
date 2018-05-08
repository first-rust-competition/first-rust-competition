// no code in this module should have any `unsafe` calls.
// Anything needing an unsafe call should be abstracted using either `hal_call!` or something else in the `hal` module.

pub mod digital_out;
