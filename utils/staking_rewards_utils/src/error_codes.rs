#[repr(u16)]
pub enum ErrorCodes {
    Abort = 35,
    TimedOut,
    TransferFailed,
    InvalidParameter,
    NotKeeper,
    NotLiquidityTransformer,
    DivisionByZero,
    Underflow,
    Overflow,
    LiquidityGuardDisabled,
    StakeInactive, // Insert more error codes here as per need.
}
