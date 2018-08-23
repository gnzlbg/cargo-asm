extern crate assert_cli;

fn lib_test(args: &[&str]) -> assert_cli::Assert {
    assert_cli::Assert::cargo_binary("cargo-asm")
        .with_args(&[
            "asm",
            "--manifest-path",
            "cargo-asm-test/lib_crate",
            "--no-color",
            "--debug-info",
        ])
        .with_args(args)
}

#[test]
fn sum_array() {
    let expected = if cfg!(target_os = "macos") {
        r#"lib_crate::sum_array (src/lib.rs:6):
 push    rbp
 mov     rbp, rsp
 test    rsi, rsi
 je      LBB13_1
 lea     r9, [4*rsi, -, 4]
 shr     r9, 2
 inc     r9
 cmp     r9, 8
 jae     LBB13_4
 xor     eax, eax
 mov     rcx, rdi
 jmp     LBB13_13
LBB13_1:
 xor     eax, eax
 pop     rbp
 ret
LBB13_4:
 movabs  r8, 9223372036854775800
 and     r8, r9
 lea     rcx, [r8, -, 8]
 mov     rdx, rcx
 shr     rdx, 3
 lea     eax, [rdx, +, 1]
 and     eax, 3
 cmp     rcx, 24
 jae     LBB13_6
 pxor    xmm0, xmm0
 xor     edx, edx
 pxor    xmm1, xmm1
 test    rax, rax
 jne     LBB13_9
 jmp     LBB13_11
LBB13_6:
 lea     rcx, [rax, -, 1]
 sub     rcx, rdx
 pxor    xmm0, xmm0
 xor     edx, edx
 pxor    xmm1, xmm1
LBB13_7:
 movdqu  xmm2, xmmword, ptr, [rdi, +, 4*rdx]
 paddd   xmm2, xmm0
 movdqu  xmm0, xmmword, ptr, [rdi, +, 4*rdx, +, 16]
 paddd   xmm0, xmm1
 movdqu  xmm1, xmmword, ptr, [rdi, +, 4*rdx, +, 32]
 movdqu  xmm3, xmmword, ptr, [rdi, +, 4*rdx, +, 48]
 movdqu  xmm4, xmmword, ptr, [rdi, +, 4*rdx, +, 64]
 paddd   xmm4, xmm1
 paddd   xmm4, xmm2
 movdqu  xmm2, xmmword, ptr, [rdi, +, 4*rdx, +, 80]
 paddd   xmm2, xmm3
 paddd   xmm2, xmm0
 movdqu  xmm0, xmmword, ptr, [rdi, +, 4*rdx, +, 96]
 paddd   xmm0, xmm4
 movdqu  xmm1, xmmword, ptr, [rdi, +, 4*rdx, +, 112]
 paddd   xmm1, xmm2
 add     rdx, 32
 add     rcx, 4
 jne     LBB13_7
 test    rax, rax
 je      LBB13_11
LBB13_9:
 lea     rcx, [rdi, +, 4*rdx, +, 16]
 neg     rax
LBB13_10:
 movdqu  xmm2, xmmword, ptr, [rcx, -, 16]
 paddd   xmm0, xmm2
 movdqu  xmm2, xmmword, ptr, [rcx]
 paddd   xmm1, xmm2
 add     rcx, 32
 inc     rax
 jne     LBB13_10
LBB13_11:
 paddd   xmm0, xmm1
 pshufd  xmm1, xmm0, 78
 paddd   xmm1, xmm0
 phaddd  xmm1, xmm1
 movd    eax, xmm1
 cmp     r9, r8
 je      LBB13_15
 lea     rcx, [rdi, +, 4*r8]
LBB13_13:
 lea     rdx, [rdi, +, 4*rsi]
LBB13_14:
 add     eax, dword, ptr, [rcx]
 add     rcx, 4
 cmp     rdx, rcx
 jne     LBB13_14
LBB13_15:
 pop     rbp
 ret
"#
    } else if cfg!(target_os = "linux") {
        r#"lib_crate::sum_array (src/lib.rs:6):
 push    rbp
 mov     rbp, rsp
 test    rsi, rsi
 je      .LBB16_1
 lea     r9, [4*rsi, -, 4]
 shr     r9, 2
 add     r9, 1
 cmp     r9, 8
 jae     .LBB16_4
 xor     eax, eax
 mov     rcx, rdi
 jmp     .LBB16_13
.LBB16_1:
 xor     eax, eax
 pop     rbp
 ret
.LBB16_4:
 movabs  r8, 9223372036854775800
 and     r8, r9
 lea     rcx, [r8, -, 8]
 mov     rdx, rcx
 shr     rdx, 3
 lea     eax, [rdx, +, 1]
 and     eax, 3
 cmp     rcx, 24
 jae     .LBB16_6
 pxor    xmm0, xmm0
 xor     edx, edx
 pxor    xmm1, xmm1
 test    rax, rax
 jne     .LBB16_9
 jmp     .LBB16_11
.LBB16_6:
 lea     rcx, [rax, -, 1]
 sub     rcx, rdx
 pxor    xmm0, xmm0
 xor     edx, edx
 pxor    xmm1, xmm1
.LBB16_7:
 movdqu  xmm2, xmmword, ptr, [rdi, +, 4*rdx]
 paddd   xmm2, xmm0
 movdqu  xmm0, xmmword, ptr, [rdi, +, 4*rdx, +, 16]
 paddd   xmm0, xmm1
 movdqu  xmm1, xmmword, ptr, [rdi, +, 4*rdx, +, 32]
 movdqu  xmm3, xmmword, ptr, [rdi, +, 4*rdx, +, 48]
 movdqu  xmm4, xmmword, ptr, [rdi, +, 4*rdx, +, 64]
 paddd   xmm4, xmm1
 paddd   xmm4, xmm2
 movdqu  xmm2, xmmword, ptr, [rdi, +, 4*rdx, +, 80]
 paddd   xmm2, xmm3
 paddd   xmm2, xmm0
 movdqu  xmm0, xmmword, ptr, [rdi, +, 4*rdx, +, 96]
 paddd   xmm0, xmm4
 movdqu  xmm1, xmmword, ptr, [rdi, +, 4*rdx, +, 112]
 paddd   xmm1, xmm2
 add     rdx, 32
 add     rcx, 4
 jne     .LBB16_7
 test    rax, rax
 je      .LBB16_11
.LBB16_9:
 lea     rcx, [rdi, +, 4*rdx]
 add     rcx, 16
 neg     rax
.LBB16_10:
 movdqu  xmm2, xmmword, ptr, [rcx, -, 16]
 paddd   xmm0, xmm2
 movdqu  xmm2, xmmword, ptr, [rcx]
 paddd   xmm1, xmm2
 add     rcx, 32
 add     rax, 1
 jne     .LBB16_10
.LBB16_11:
 paddd   xmm0, xmm1
 pshufd  xmm1, xmm0, 78
 paddd   xmm1, xmm0
 pshufd  xmm0, xmm1, 229
 paddd   xmm0, xmm1
 movd    eax, xmm0
 cmp     r9, r8
 je      .LBB16_15
 lea     rcx, [rdi, +, 4*r8]
.LBB16_13:
 lea     rdx, [rdi, +, 4*rsi]
.LBB16_14:
 add     eax, dword, ptr, [rcx]
 add     rcx, 4
 cmp     rdx, rcx
 jne     .LBB16_14
.LBB16_15:
 pop     rbp
 ret"#
    } else if cfg!(target_os = "windows") {
        r#"lib_crate::sum_array (src\lib.rs:6):
 push    rbp
 mov     rbp, rsp
 test    rdx, rdx
 je      .LBB14_1
 lea     r8, [4*rdx, -, 4]
 shr     r8, 2
 add     r8, 1
 cmp     r8, 8
 jae     .LBB14_4
 xor     eax, eax
 mov     r8, rcx
 jmp     .LBB14_13
.LBB14_1:
 xor     eax, eax
 pop     rbp
 ret
.LBB14_4:
 movabs  r9, 9223372036854775800
 and     r9, r8
 lea     r11, [r9, -, 8]
 mov     rax, r11
 shr     rax, 3
 lea     r10d, [rax, +, 1]
 and     r10d, 3
 cmp     r11, 24
 jae     .LBB14_6
 pxor    xmm0, xmm0
 xor     eax, eax
 pxor    xmm1, xmm1
 test    r10, r10
 jne     .LBB14_9
 jmp     .LBB14_11
.LBB14_6:
 lea     r11, [r10, -, 1]
 sub     r11, rax
 pxor    xmm0, xmm0
 xor     eax, eax
 pxor    xmm1, xmm1
.LBB14_7:
 movdqu  xmm2, xmmword, ptr, [rcx, +, 4*rax]
 paddd   xmm2, xmm0
 movdqu  xmm0, xmmword, ptr, [rcx, +, 4*rax, +, 16]
 paddd   xmm0, xmm1
 movdqu  xmm1, xmmword, ptr, [rcx, +, 4*rax, +, 32]
 movdqu  xmm3, xmmword, ptr, [rcx, +, 4*rax, +, 48]
 movdqu  xmm4, xmmword, ptr, [rcx, +, 4*rax, +, 64]
 paddd   xmm4, xmm1
 paddd   xmm4, xmm2
 movdqu  xmm2, xmmword, ptr, [rcx, +, 4*rax, +, 80]
 paddd   xmm2, xmm3
 paddd   xmm2, xmm0
 movdqu  xmm0, xmmword, ptr, [rcx, +, 4*rax, +, 96]
 paddd   xmm0, xmm4
 movdqu  xmm1, xmmword, ptr, [rcx, +, 4*rax, +, 112]
 paddd   xmm1, xmm2
 add     rax, 32
 add     r11, 4
 jne     .LBB14_7
 test    r10, r10
 je      .LBB14_11
.LBB14_9:
 lea     rax, [rcx, +, 4*rax]
 add     rax, 16
 neg     r10
.LBB14_10:
 movdqu  xmm2, xmmword, ptr, [rax, -, 16]
 paddd   xmm0, xmm2
 movdqu  xmm2, xmmword, ptr, [rax]
 paddd   xmm1, xmm2
 add     rax, 32
 add     r10, 1
 jne     .LBB14_10
.LBB14_11:
 paddd   xmm0, xmm1
 pshufd  xmm1, xmm0, 78
 paddd   xmm1, xmm0
 pshufd  xmm0, xmm1, 229
 paddd   xmm0, xmm1
 movd    eax, xmm0
 cmp     r8, r9
 je      .LBB14_15
 lea     r8, [rcx, +, 4*r9]
.LBB14_13:
 lea     rcx, [rcx, +, 4*rdx]
.LBB14_14:
 add     eax, dword, ptr, [r8]
 add     r8, 4
 cmp     rcx, r8
 jne     .LBB14_14
.LBB14_15:
 pop     rbp
 ret
"#
    } else {
        unimplemented!()
    };
    lib_test(&["lib_crate::sum_array"])
        .stdout()
        .is(expected)
        .unwrap();
}

#[test]
fn max_array() {
    let expected = if cfg!(target_os = "macos") {
        r#"lib_crate::bar::max_array (src/bar.rs:3):
 push    rbp
 mov     rbp, rsp
 mov     rax, -524288
LBB0_1:
 movupd  xmm0, xmmword, ptr, [rsi, +, rax, +, 524288]
 movupd  xmm1, xmmword, ptr, [rdi, +, rax, +, 524288]
 maxpd   xmm0, xmm1
 movupd  xmm1, xmmword, ptr, [rdi, +, rax, +, 524304]
 movupd  xmm2, xmmword, ptr, [rdi, +, rax, +, 524320]
 movupd  xmm3, xmmword, ptr, [rdi, +, rax, +, 524336]
 movupd  xmmword, ptr, [rdi, +, rax, +, 524288], xmm0
 movupd  xmm0, xmmword, ptr, [rsi, +, rax, +, 524304]
 maxpd   xmm0, xmm1
 movupd  xmmword, ptr, [rdi, +, rax, +, 524304], xmm0
 movupd  xmm0, xmmword, ptr, [rsi, +, rax, +, 524320]
 maxpd   xmm0, xmm2
 movupd  xmm1, xmmword, ptr, [rsi, +, rax, +, 524336]
 movupd  xmmword, ptr, [rdi, +, rax, +, 524320], xmm0
 maxpd   xmm1, xmm3
 movupd  xmmword, ptr, [rdi, +, rax, +, 524336], xmm1
 add     rax, 64
 jne     LBB0_1
 pop     rbp
 ret
"#
    } else if cfg!(target_os = "linux") {
        r#"lib_crate::bar::max_array (src/bar.rs:3):
 push    rbp
 mov     rbp, rsp
 mov     rax, -524288
.LBB0_1:
 movupd  xmm0, xmmword, ptr, [rsi, +, rax, +, 524288]
 movupd  xmm1, xmmword, ptr, [rsi, +, rax, +, 524304]
 movupd  xmm2, xmmword, ptr, [rdi, +, rax, +, 524288]
 maxpd   xmm0, xmm2
 movupd  xmm2, xmmword, ptr, [rdi, +, rax, +, 524304]
 movupd  xmm3, xmmword, ptr, [rdi, +, rax, +, 524320]
 movupd  xmm4, xmmword, ptr, [rdi, +, rax, +, 524336]
 movupd  xmmword, ptr, [rdi, +, rax, +, 524288], xmm0
 maxpd   xmm1, xmm2
 movupd  xmmword, ptr, [rdi, +, rax, +, 524304], xmm1
 movupd  xmm0, xmmword, ptr, [rsi, +, rax, +, 524320]
 maxpd   xmm0, xmm3
 movupd  xmm1, xmmword, ptr, [rsi, +, rax, +, 524336]
 movupd  xmmword, ptr, [rdi, +, rax, +, 524320], xmm0
 maxpd   xmm1, xmm4
 movupd  xmmword, ptr, [rdi, +, rax, +, 524336], xmm1
 add     rax, 64
 jne     .LBB0_1
 pop     rbp
 ret
"#
    } else if cfg!(target_os = "windows") {
        r#"lib_crate::bar::max_array (src\bar.rs:3):
 push    rbp
 mov     rbp, rsp
 mov     rax, -524288
.LBB0_1:
 movupd  xmm0, xmmword, ptr, [rdx, +, rax, +, 524288]
 movupd  xmm1, xmmword, ptr, [rdx, +, rax, +, 524304]
 movupd  xmm2, xmmword, ptr, [rcx, +, rax, +, 524288]
 maxpd   xmm0, xmm2
 movupd  xmm2, xmmword, ptr, [rcx, +, rax, +, 524304]
 movupd  xmm3, xmmword, ptr, [rcx, +, rax, +, 524320]
 movupd  xmm4, xmmword, ptr, [rcx, +, rax, +, 524336]
 movupd  xmmword, ptr, [rcx, +, rax, +, 524288], xmm0
 maxpd   xmm1, xmm2
 movupd  xmmword, ptr, [rcx, +, rax, +, 524304], xmm1
 movupd  xmm0, xmmword, ptr, [rdx, +, rax, +, 524320]
 maxpd   xmm0, xmm3
 movupd  xmm1, xmmword, ptr, [rdx, +, rax, +, 524336]
 movupd  xmmword, ptr, [rcx, +, rax, +, 524320], xmm0
 maxpd   xmm1, xmm4
 movupd  xmmword, ptr, [rcx, +, rax, +, 524336], xmm1
 add     rax, 64
 jne     .LBB0_1
 pop     rbp
 ret"#
    } else {
        unimplemented!()
    };
    lib_test(&["lib_crate::bar::max_array"])
        .stdout()
        .is(expected)
        .unwrap();
}

#[test]
fn sum_array_rust() {
    let expected = if cfg!(target_os = "macos") {
        r#" pub fn sum_array(x: &[i32]) -> i32 {
 push    rbp
 mov     rbp, rsp
     if self.ptr == self.end { (libcore/slice/mod.rs:2390)
     test    rsi, rsi
     je      LBB13_1
     intrinsics::offset(self, count) (libcore/ptr.rs:621)
     lea     r9, [4*rsi, -, 4]
     shr     r9, 2
     inc     r9
     cmp     r9, 8
     jae     LBB13_4
     xor     eax, eax
     mov     rcx, rdi
     jmp     LBB13_13
LBB13_1:
     xor     eax, eax
 }
 pop     rbp
 ret
LBB13_4:
     intrinsics::offset(self, count) (libcore/ptr.rs:621)
     movabs  r8, 9223372036854775800
     and     r8, r9
     lea     rcx, [r8, -, 8]
     mov     rdx, rcx
     shr     rdx, 3
     lea     eax, [rdx, +, 1]
     and     eax, 3
     cmp     rcx, 24
     jae     LBB13_6
     pxor    xmm0, xmm0
     xor     edx, edx
     pxor    xmm1, xmm1
     test    rax, rax
     jne     LBB13_9
     jmp     LBB13_11
LBB13_6:
     lea     rcx, [rax, -, 1]
     sub     rcx, rdx
     pxor    xmm0, xmm0
     xor     edx, edx
     pxor    xmm1, xmm1
LBB13_7:
     movdqu  xmm2, xmmword, ptr, [rdi, +, 4*rdx]
 x.iter().fold(0, |sum, next| sum + *next)
 paddd   xmm2, xmm0
 movdqu  xmm0, xmmword, ptr, [rdi, +, 4*rdx, +, 16]
 paddd   xmm0, xmm1
 movdqu  xmm1, xmmword, ptr, [rdi, +, 4*rdx, +, 32]
 movdqu  xmm3, xmmword, ptr, [rdi, +, 4*rdx, +, 48]
 movdqu  xmm4, xmmword, ptr, [rdi, +, 4*rdx, +, 64]
 paddd   xmm4, xmm1
 paddd   xmm4, xmm2
 movdqu  xmm2, xmmword, ptr, [rdi, +, 4*rdx, +, 80]
 paddd   xmm2, xmm3
 paddd   xmm2, xmm0
 movdqu  xmm0, xmmword, ptr, [rdi, +, 4*rdx, +, 96]
 paddd   xmm0, xmm4
 movdqu  xmm1, xmmword, ptr, [rdi, +, 4*rdx, +, 112]
 paddd   xmm1, xmm2
 add     rdx, 32
 add     rcx, 4
 jne     LBB13_7
 test    rax, rax
 je      LBB13_11
LBB13_9:
 lea     rcx, [rdi, +, 4*rdx, +, 16]
 neg     rax
LBB13_10:
 movdqu  xmm2, xmmword, ptr, [rcx, -, 16]
 paddd   xmm0, xmm2
 movdqu  xmm2, xmmword, ptr, [rcx]
 paddd   xmm1, xmm2
 add     rcx, 32
 inc     rax
 jne     LBB13_10
LBB13_11:
 paddd   xmm0, xmm1
 pshufd  xmm1, xmm0, 78
 paddd   xmm1, xmm0
 phaddd  xmm1, xmm1
 movd    eax, xmm1
 cmp     r9, r8
     intrinsics::offset(self, count) (libcore/ptr.rs:621)
     je      LBB13_15
     lea     rcx, [rdi, +, 4*r8]
LBB13_13:
     lea     rdx, [rdi, +, 4*rsi]
LBB13_14:
 x.iter().fold(0, |sum, next| sum + *next)
 add     eax, dword, ptr, [rcx]
     intrinsics::offset(self, count) (libcore/ptr.rs:621)
     add     rcx, 4
     if self.ptr == self.end { (libcore/slice/mod.rs:2390)
     cmp     rdx, rcx
     jne     LBB13_14
LBB13_15:
 }
 pop     rbp
 ret"#
    } else if cfg!(target_os = "linux") {
        r#" pub fn sum_array(x: &[i32]) -> i32 {
 push    rbp
 mov     rbp, rsp
     if self.ptr == self.end { (libcore/slice/mod.rs:1178)
     test    rsi, rsi
     je      .LBB16_1
     lea     r9, [4*rsi, -, 4]
     shr     r9, 2
     add     r9, 1
     cmp     r9, 8
     jae     .LBB16_4
     xor     eax, eax
     mov     rcx, rdi
     jmp     .LBB16_13
.LBB16_1:
     xor     eax, eax
 }
 pop     rbp
 ret
.LBB16_4:
     if self.ptr == self.end { (libcore/slice/mod.rs:1178)
     movabs  r8, 9223372036854775800
     and     r8, r9
     lea     rcx, [r8, -, 8]
     mov     rdx, rcx
     shr     rdx, 3
     lea     eax, [rdx, +, 1]
     and     eax, 3
     cmp     rcx, 24
     jae     .LBB16_6
     pxor    xmm0, xmm0
     xor     edx, edx
     pxor    xmm1, xmm1
     test    rax, rax
     jne     .LBB16_9
     jmp     .LBB16_11
.LBB16_6:
     lea     rcx, [rax, -, 1]
     sub     rcx, rdx
     pxor    xmm0, xmm0
     xor     edx, edx
     pxor    xmm1, xmm1
.LBB16_7:
     movdqu  xmm2, xmmword, ptr, [rdi, +, 4*rdx]
 x.iter().fold(0, |sum, next| sum + *next)
 paddd   xmm2, xmm0
 movdqu  xmm0, xmmword, ptr, [rdi, +, 4*rdx, +, 16]
 paddd   xmm0, xmm1
 movdqu  xmm1, xmmword, ptr, [rdi, +, 4*rdx, +, 32]
 movdqu  xmm3, xmmword, ptr, [rdi, +, 4*rdx, +, 48]
 movdqu  xmm4, xmmword, ptr, [rdi, +, 4*rdx, +, 64]
 paddd   xmm4, xmm1
 paddd   xmm4, xmm2
 movdqu  xmm2, xmmword, ptr, [rdi, +, 4*rdx, +, 80]
 paddd   xmm2, xmm3
 paddd   xmm2, xmm0
 movdqu  xmm0, xmmword, ptr, [rdi, +, 4*rdx, +, 96]
 paddd   xmm0, xmm4
 movdqu  xmm1, xmmword, ptr, [rdi, +, 4*rdx, +, 112]
 paddd   xmm1, xmm2
 add     rdx, 32
 add     rcx, 4
 jne     .LBB16_7
 test    rax, rax
 je      .LBB16_11
.LBB16_9:
 lea     rcx, [rdi, +, 4*rdx]
 add     rcx, 16
 neg     rax
.LBB16_10:
 movdqu  xmm2, xmmword, ptr, [rcx, -, 16]
 paddd   xmm0, xmm2
 movdqu  xmm2, xmmword, ptr, [rcx]
 paddd   xmm1, xmm2
 add     rcx, 32
 add     rax, 1
 jne     .LBB16_10
.LBB16_11:
 paddd   xmm0, xmm1
 pshufd  xmm1, xmm0, 78
 paddd   xmm1, xmm0
 pshufd  xmm0, xmm1, 229
 paddd   xmm0, xmm1
 movd    eax, xmm0
 cmp     r9, r8
     if self.ptr == self.end { (libcore/slice/mod.rs:1178)
     je      .LBB16_15
     lea     rcx, [rdi, +, 4*r8]
.LBB16_13:
     intrinsics::offset(self, count) (libcore/ptr.rs:622)
     lea     rdx, [rdi, +, 4*rsi]
.LBB16_14:
 x.iter().fold(0, |sum, next| sum + *next)
 add     eax, dword, ptr, [rcx]
     intrinsics::offset(self, count) (libcore/ptr.rs:622)
     add     rcx, 4
     if self.ptr == self.end { (libcore/slice/mod.rs:1178)
     cmp     rdx, rcx
     jne     .LBB16_14
.LBB16_15:
 }
 pop     rbp
 ret"#
    } else if cfg!(target_os = "windows") {
        r#"pub fn sum_array(x: &[i32]) -> i32 {
 push    rbp
 mov     rbp, rsp
     if self.ptr == self.end { (libcore\slice\mod.rs:1178)
     test    rdx, rdx
     je      .LBB14_1
     lea     r8, [4*rdx, -, 4]
     shr     r8, 2
     add     r8, 1
     cmp     r8, 8
     jae     .LBB14_4
     xor     eax, eax
     mov     r8, rcx
     jmp     .LBB14_13
.LBB14_1:
 }
 xor     eax, eax
 pop     rbp
 ret
.LBB14_4:
     if self.ptr == self.end { (libcore\slice\mod.rs:1178)
     movabs  r9, 9223372036854775800
     and     r9, r8
     lea     r11, [r9, -, 8]
     mov     rax, r11
     shr     rax, 3
     lea     r10d, [rax, +, 1]
     and     r10d, 3
     cmp     r11, 24
     jae     .LBB14_6
     pxor    xmm0, xmm0
     xor     eax, eax
     pxor    xmm1, xmm1
     test    r10, r10
     jne     .LBB14_9
     jmp     .LBB14_11
.LBB14_6:
     lea     r11, [r10, -, 1]
     sub     r11, rax
     pxor    xmm0, xmm0
     xor     eax, eax
     pxor    xmm1, xmm1
.LBB14_7:
 x.iter().fold(0, |sum, next| sum + *next)
 movdqu  xmm2, xmmword, ptr, [rcx, +, 4*rax]
 paddd   xmm2, xmm0
 movdqu  xmm0, xmmword, ptr, [rcx, +, 4*rax, +, 16]
 paddd   xmm0, xmm1
 movdqu  xmm1, xmmword, ptr, [rcx, +, 4*rax, +, 32]
 movdqu  xmm3, xmmword, ptr, [rcx, +, 4*rax, +, 48]
 movdqu  xmm4, xmmword, ptr, [rcx, +, 4*rax, +, 64]
 paddd   xmm4, xmm1
 paddd   xmm4, xmm2
 movdqu  xmm2, xmmword, ptr, [rcx, +, 4*rax, +, 80]
 paddd   xmm2, xmm3
 paddd   xmm2, xmm0
 movdqu  xmm0, xmmword, ptr, [rcx, +, 4*rax, +, 96]
 paddd   xmm0, xmm4
 movdqu  xmm1, xmmword, ptr, [rcx, +, 4*rax, +, 112]
 paddd   xmm1, xmm2
 add     rax, 32
 add     r11, 4
 jne     .LBB14_7
 test    r10, r10
 je      .LBB14_11
.LBB14_9:
 lea     rax, [rcx, +, 4*rax]
 add     rax, 16
 neg     r10
.LBB14_10:
 movdqu  xmm2, xmmword, ptr, [rax, -, 16]
 paddd   xmm0, xmm2
 movdqu  xmm2, xmmword, ptr, [rax]
 paddd   xmm1, xmm2
 add     rax, 32
 add     r10, 1
 jne     .LBB14_10
.LBB14_11:
 paddd   xmm0, xmm1
 pshufd  xmm1, xmm0, 78
 paddd   xmm1, xmm0
 pshufd  xmm0, xmm1, 229
 paddd   xmm0, xmm1
 movd    eax, xmm0
 cmp     r8, r9
     if self.ptr == self.end { (libcore\slice\mod.rs:1178)
     je      .LBB14_15
     lea     r8, [rcx, +, 4*r9]
.LBB14_13:
     intrinsics::offset(self, count) (libcore\ptr.rs:622)
     lea     rcx, [rcx, +, 4*rdx]
.LBB14_14:
 x.iter().fold(0, |sum, next| sum + *next)
 add     eax, dword, ptr, [r8]
     intrinsics::offset(self, count) (libcore\ptr.rs:622)
     add     r8, 4
     if self.ptr == self.end { (libcore\slice\mod.rs:1178)
     cmp     rcx, r8
     jne     .LBB14_14
.LBB14_15:
 }
 pop     rbp
 ret
"#
    } else {
        unimplemented!()
    };
    lib_test(&["lib_crate::sum_array", "--rust"])
        .stdout()
        .is(expected)
        .unwrap();
}

#[test]
fn max_array_rust() {
    let expected = if cfg!(target_os = "macos") {
        r#"pub fn max_array(x: &mut[f64; 65536], y: &[f64; 65536]) {
 push    rbp
 mov     rbp, rsp
 mov     rax, -524288
LBB0_1:
 x[i] = if y[i] > x[i] { y[i] } else { x[i] };
 movupd  xmm0, xmmword, ptr, [rsi, +, rax, +, 524288]
 movupd  xmm1, xmmword, ptr, [rdi, +, rax, +, 524288]
 maxpd   xmm0, xmm1
 movupd  xmm1, xmmword, ptr, [rdi, +, rax, +, 524304]
 movupd  xmm2, xmmword, ptr, [rdi, +, rax, +, 524320]
 movupd  xmm3, xmmword, ptr, [rdi, +, rax, +, 524336]
 movupd  xmmword, ptr, [rdi, +, rax, +, 524288], xmm0
 movupd  xmm0, xmmword, ptr, [rsi, +, rax, +, 524304]
 maxpd   xmm0, xmm1
 movupd  xmmword, ptr, [rdi, +, rax, +, 524304], xmm0
 movupd  xmm0, xmmword, ptr, [rsi, +, rax, +, 524320]
 maxpd   xmm0, xmm2
 movupd  xmm1, xmmword, ptr, [rsi, +, rax, +, 524336]
 movupd  xmmword, ptr, [rdi, +, rax, +, 524320], xmm0
 maxpd   xmm1, xmm3
 movupd  xmmword, ptr, [rdi, +, rax, +, 524336], xmm1
 add     rax, 64
 jne     LBB0_1
 }
 pop     rbp
 ret"#
    } else if cfg!(target_os = "linux") {
        r#"pub fn max_array(x: &mut[f64; 65536], y: &[f64; 65536]) {
 push    rbp
 mov     rbp, rsp
 mov     rax, -524288
.LBB0_1:
 x[i] = if y[i] > x[i] { y[i] } else { x[i] };
 movupd  xmm0, xmmword, ptr, [rsi, +, rax, +, 524288]
 movupd  xmm1, xmmword, ptr, [rsi, +, rax, +, 524304]
 movupd  xmm2, xmmword, ptr, [rdi, +, rax, +, 524288]
 maxpd   xmm0, xmm2
 movupd  xmm2, xmmword, ptr, [rdi, +, rax, +, 524304]
 movupd  xmm3, xmmword, ptr, [rdi, +, rax, +, 524320]
 movupd  xmm4, xmmword, ptr, [rdi, +, rax, +, 524336]
 movupd  xmmword, ptr, [rdi, +, rax, +, 524288], xmm0
 maxpd   xmm1, xmm2
 movupd  xmmword, ptr, [rdi, +, rax, +, 524304], xmm1
 movupd  xmm0, xmmword, ptr, [rsi, +, rax, +, 524320]
 maxpd   xmm0, xmm3
 movupd  xmm1, xmmword, ptr, [rsi, +, rax, +, 524336]
 movupd  xmmword, ptr, [rdi, +, rax, +, 524320], xmm0
 maxpd   xmm1, xmm4
 movupd  xmmword, ptr, [rdi, +, rax, +, 524336], xmm1
 add     rax, 64
 jne     .LBB0_1
 }
 pop     rbp
 ret"#
    } else if cfg!(target_os = "windows") {
        r#"pub fn max_array(x: &mut[f64; 65536], y: &[f64; 65536]) {
 push    rbp
 mov     rbp, rsp
 mov     rax, -524288
.LBB0_1:
 x[i] = if y[i] > x[i] { y[i] } else { x[i] };
 movupd  xmm0, xmmword, ptr, [rdx, +, rax, +, 524288]
 movupd  xmm1, xmmword, ptr, [rdx, +, rax, +, 524304]
 movupd  xmm2, xmmword, ptr, [rcx, +, rax, +, 524288]
 maxpd   xmm0, xmm2
 movupd  xmm2, xmmword, ptr, [rcx, +, rax, +, 524304]
 movupd  xmm3, xmmword, ptr, [rcx, +, rax, +, 524320]
 movupd  xmm4, xmmword, ptr, [rcx, +, rax, +, 524336]
 movupd  xmmword, ptr, [rcx, +, rax, +, 524288], xmm0
 maxpd   xmm1, xmm2
 movupd  xmmword, ptr, [rcx, +, rax, +, 524304], xmm1
 movupd  xmm0, xmmword, ptr, [rdx, +, rax, +, 524320]
 maxpd   xmm0, xmm3
 movupd  xmm1, xmmword, ptr, [rdx, +, rax, +, 524336]
 movupd  xmmword, ptr, [rcx, +, rax, +, 524320], xmm0
 maxpd   xmm1, xmm4
 movupd  xmmword, ptr, [rcx, +, rax, +, 524336], xmm1
 add     rax, 64
 jne     .LBB0_1
 }
 pop     rbp
 ret"#
    } else {
        unimplemented!()
    };
    lib_test(&["lib_crate::bar::max_array", "--rust"])
        .stdout()
        .is(expected)
        .unwrap();
}

#[test]
fn trait_method() {
    let expected = if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
        r#"fn addd(&self) -> usize {
 push    rbp
 mov     rbp, rsp
 self.x + self.y
 mov     rax, qword, ptr, [rdi, +, 8]
 add     rax, qword, ptr, [rdi]
 }
 pop     rbp
 ret
"#
    } else if cfg!(target_os = "windows") {
        r#"fn addd(&self) -> usize {
 push    rbp
 mov     rbp, rsp
 self.x + self.y
 mov     rax, qword, ptr, [rcx, +, 8]
 add     rax, qword, ptr, [rcx]
 }
 pop     rbp
 ret
"#
    } else {
        unimplemented!()
    };
    lib_test(&[
        "<lib_crate::baz::Foo as lib_crate::baz::Addd>::addd",
        "--rust",
    ]).stdout()
        .is(expected)
        .unwrap();
}

#[test]
fn generic_function() {
    let expected = if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
        r#"pub fn generic_add<T: ::std::ops::Add<T,Output=T>>(x: T, y: T) -> T { x + y }
 push    rbp
 mov     rbp, rsp
     fn add(self, other: $t) -> $t { self + other } (libcore/ops/arith.rs:110)
     lea     rax, [rdi, +, rsi]
 pub fn generic_add<T: ::std::ops::Add<T,Output=T>>(x: T, y: T) -> T { x + y }
 pop     rbp
 ret
"#
    } else if cfg!(target_os = "windows") {
        r#"pub fn generic_add<T: ::std::ops::Add<T,Output=T>>(x: T, y: T) -> T { x + y }
 push    rbp
 mov     rbp, rsp
     fn add(self, other: $t) -> $t { self + other } (libcore\ops\arith.rs:110)
     lea     rax, [rcx, +, rsi]
 pub fn generic_add<T: ::std::ops::Add<T,Output=T>>(x: T, y: T) -> T { x + y }
 pop     rbp
 ret
"#
    } else {
        unimplemented!()
    };

    lib_test(&["lib_crate::bar::generic_add", "--rust"])
        .stdout()
        .is(expected)
        .unwrap();
}

#[test]
fn inherent_method() {
    let expected = if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
        r#"pub fn foo_add(&self) -> usize {
 push    rbp
 mov     rbp, rsp
 self.x + self.y
 mov     rax, qword, ptr, [rdi, +, 8]
 add     rax, qword, ptr, [rdi]
 }
 pop     rbp
 ret
"#
    } else if cfg!(target_os = "windows") {
        r#"pub fn foo_add(&self) -> usize {
 push    rbp
 mov     rbp, rsp
 self.x + self.y
 mov     rax, qword, ptr, [rcx, +, 8]
 add     rax, qword, ptr, [rcx]
 }
 pop     rbp
 ret
"#
    } else {
        unimplemented!()
    };
    lib_test(&["lib_crate::baz::Foo::foo_add", "--rust"])
        .stdout()
        .is(expected)
        .unwrap();
}

#[test]
fn completions() {
    lib_test(&["maxarray"])
        .stderr().contains("could not find function at path \"maxarray\" in the generated assembly.")
        .stderr().contains("lib_crate::bar::max_array")
        .stderr().contains("lib_crate::sum_array")
        .fails()
        .unwrap();
}

#[test]
fn cargo_features() {
    lib_test(&["lib_crate::bar::tiger_add"])
        .stderr().contains("could not find function at path \"lib_crate::bar::tiger_add\" in the generated assembly.")
        .fails()
        .unwrap();

    lib_test(&["lib_crate::bar::tiger_add", "--features=tiger"])
        .stdout()
        .contains("lib_crate::bar::tiger_add")
        .succeeds()
        .unwrap();

    lib_test(&["lib_crate::bar::cat_tiger_add", "--features=tiger"])
        .stderr().contains("could not find function at path \"lib_crate::bar::cat_tiger_add\" in the generated assembly.")
        .fails()
        .unwrap();

    lib_test(&["lib_crate::bar::cat_tiger_add", "--features=tiger,cat"])
        .stdout()
        .contains("lib_crate::bar::cat_tiger_add")
        .succeeds()
        .unwrap();
}
