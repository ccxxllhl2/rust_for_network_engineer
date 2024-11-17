fn main() {
    // Variables can be type annotated.
    let if_enable: bool = true;

    let cpu_usage: f32 = 0.5;  // Regular annotation
    let pkg_per_sec   = 527i32; // Suffix annotation

    // Or a default will be used.
    let G1_0_1_TX_B   = 1527.34; // `f64`
    let cisco_device_num = 7;   // `i32`

    // A type can also be inferred from context.
    let mut total_pkg = 1; // Type i64 is inferred from another line.
    total_pkg = 4294967296i64;

    // A mutable variable's value can be changed.
    let mut pkg_num = 12; // Mutable `i32`
    pkg_num = 21;

    // Error! The type of a variable can't be changed.
    //pkg_num = true;

    // Variables can be overwritten with shadowing.
    //let pkg_num = true;

    /* Compound types - Array and Tuple */

    // Array signature consists of Type T and length as [T; length].
    let all_cpu_usage: [f32; 4] = [0.15, 0.27, 0.86, 0.55];

    // Tuple is a collection of values of different types 
    // and is constructed using parentheses ().
    let some_data = (5u32, 1u8, true, -5.04f32);
}