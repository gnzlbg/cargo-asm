extern crate assert_cli;

fn lib_test(args: &[&str]) -> assert_cli::Assert {
    assert_cli::Assert::main_binary()
        .with_args(&["asm", "--project-path", "cargo-asm-test/lib_crate", "--no-color"])
        .with_args(args)
}

#[test]
fn sum_array() {
    lib_test(&["lib_crate::sum_array"])
        .stdout().is(r#"lib_crate::sum_array (src/lib.rs:6):
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
"#)
        .unwrap();
}

#[test]
fn max_array() {
    lib_test(&["lib_crate::bar::max_array"])
        .stdout().is(r#"lib_crate::bar::max_array (src/bar.rs:3):
 push    rbp
 mov     rbp, rsp
 mov     rax, -524288
LBB0_1:
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
 jne     LBB0_1
 pop     rbp
 ret
"#)
        .unwrap();
}

#[test]
fn sum_array_rust() {
    lib_test(&["lib_crate::sum_array", "--rust"])
        .stdout().is(r#" pub fn sum_array(x: &[i32]) -> i32 {
 push    rbp
 mov     rbp, rsp
     if self.ptr == self.end { (libcore/slice/mod.rs:1178)
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
 }
 pop     rbp
 ret
LBB13_4:
     if self.ptr == self.end { (libcore/slice/mod.rs:1178)
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
     if self.ptr == self.end { (libcore/slice/mod.rs:1178)
     je      LBB13_15
     lea     rcx, [rdi, +, 4*r8]
LBB13_13:
     intrinsics::offset(self, count) (libcore/ptr.rs:622)
     lea     rdx, [rdi, +, 4*rsi]
LBB13_14:
 x.iter().fold(0, |sum, next| sum + *next)
 add     eax, dword, ptr, [rcx]
     intrinsics::offset(self, count) (libcore/ptr.rs:622)
     add     rcx, 4
     if self.ptr == self.end { (libcore/slice/mod.rs:1178)
     cmp     rdx, rcx
     jne     LBB13_14
LBB13_15:
 }
 pop     rbp
 ret"#)
        .unwrap();
}

#[test]
fn max_array_rust() {
    lib_test(&["lib_crate::bar::max_array", "--rust"])
        .stdout().is(r#"pub fn max_array(x: &mut[f64; 65536], y: &[f64; 65536]) {
 push    rbp
 mov     rbp, rsp
 mov     rax, -524288
LBB0_1:
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
 jne     LBB0_1
 }
 pop     rbp
 ret"#).unwrap();
}

#[test]
fn trait_method() {
    lib_test(&["<lib_crate::baz::Foo as lib_crate::baz::Addd>::addd", "--rust"])
        .stdout().is(r#"fn addd(&self) -> usize {
 push    rbp
 mov     rbp, rsp
 self.x + self.y
 mov     rax, qword, ptr, [rdi, +, 8]
 add     rax, qword, ptr, [rdi]
 }
 pop     rbp
 ret
"#).unwrap();
}

#[test]
fn generic_function() {
    lib_test(&["lib_crate::bar::generic_add", "--rust"])
        .stdout().is(r#"pub fn generic_add<T: ::std::ops::Add<T,Output=T>>(x: T, y: T) -> T { x + y }
 push    rbp
 mov     rbp, rsp
     fn add(self, other: $t) -> $t { self + other } (libcore/ops/arith.rs:108)
     lea     rax, [rdi, +, rsi]
 pub fn generic_add<T: ::std::ops::Add<T,Output=T>>(x: T, y: T) -> T { x + y }
 pop     rbp
 ret
"#).unwrap();
}

#[test]
fn inherent_method() {
    lib_test(&["lib_crate::baz::Foo::foo_add", "--rust"])
        .stdout().is(r#"pub fn foo_add(&self) -> usize {
 push    rbp
 mov     rbp, rsp
 self.x + self.y
 mov     rax, qword, ptr, [rdi, +, 8]
 add     rax, qword, ptr, [rdi]
 }
 pop     rbp
 ret
"#).unwrap();
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
