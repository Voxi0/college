fn main() {
    // Should be a string since it's text
    let username: String = String::new();

    // Unsigned 8-bit integer because this number can't be negative and also u8 can represent upto
    // 255 which should be more than enough
    let num_logins: u8 = 0;

    // Account balance may be negative e.g. overdrafts or something so we use a signed integer and a
    // 32-bit integer at that since it can grow pretty large and an i8 would be too small
    let acc_balance: i32 = 0;

    // Can only be true or false, yes or no
    let acc_active: bool = true;

    // Can't be negative so we use unsigned here and with this being a 16-bit type, we can represent
    // up to 65,535 which should easily be plentiful enough
    let avg_session_length: u16 = 0;

    // I would use `type()` to show the datatype of each and every variable but I mean, since I am
    // just setting the types by myself manually, it's just unnecessary

    // For the stretch, if I had changed `avg_session_length` to an f32 instead, we could represent
    // even more numbers with even more precision since floats don't have to be whole numbers
    // allowing a bit more flexibility if required

    // And if acc_balance was set to `u32` then we could represent more numbers than an `i32`
    // actually since unsigned numbers are positive only without having to use an extra bit or so to
    // represent negative values. However, obviously we'd lose the ability for the variable to hold
    // a negative number which would absolutely crash the program
}
