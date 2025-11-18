#[allow(deprecated)]
pub fn console_putchar(c: usize) {
    sbi_rt::legacy::console_putchar(c);
}

#[allow(deprecated)]
pub fn shutdown(failure: bool) -> ! {
    use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};
    if !failure {
        system_reset(Shutdown, NoReason);  // 正常关机：Shutdown 类型 + NoReason 原因
    } else {
        system_reset(Shutdown, SystemFailure);  // 异常关机：Shutdown 类型 + SystemFailure 原因
    }
    unreachable!()  // SBI 调用后应该不会返回，防止编译警告
}
