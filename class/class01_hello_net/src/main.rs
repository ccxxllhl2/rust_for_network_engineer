fn main() {
    // 欢迎来到这门课程.
    // 期望网络工程师能在这里学会使用Rust语言.
    /*
     * 这份教程是在网工版Python教程的基础上制作的,
     * 依然包含课程+闯关练习，
     * 所有内容都只围绕网络工程师。
     */

    /*
    欢迎 Fork 并自己尝试编写更多内容，同时请给个 Star🌟
    */

    // 在网络中检测到接口速率的基本单位是字节每秒，现在进行数值调整，期望输出兆字节每秒（MB/S）
    // 请问程序执行后确实是 MB/S 吗？
    let net_traffic = 20000000 / /* 1000 /  */ 1000;
    println!("Is `net_traffic` as MB/S or KB/S? Answer: net_traffic = {}", net_traffic);
}