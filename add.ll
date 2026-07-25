; ModuleID = 'add.7337c6e4dd1d58ec-cgu.0'
source_filename = "add.7337c6e4dd1d58ec-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; add::add
; Function Attrs: nonlazybind uwtable
define i32 @_RNvCs9TiJ8oAt8wO_3add3add(i32 %a, i32 %b) unnamed_addr #0 {
start:
  %_0 = add i32 %a, %b
  ret i32 %_0
}

; add::sub
; Function Attrs: nonlazybind uwtable
define i32 @_RNvCs9TiJ8oAt8wO_3add3sub(i32 %a, i32 %b) unnamed_addr #0 {
start:
  %_0 = sub i32 %a, %b
  ret i32 %_0
}

; add::add2
; Function Attrs: nonlazybind uwtable
define { i32, i32 } @_RNvCs9TiJ8oAt8wO_3add4add2(i32 %a.0, i32 %a.1) unnamed_addr #0 {
start:
  %_2 = add i32 %a.0, %a.1
  %0 = insertvalue { i32, i32 } poison, i32 %_2, 0
  %1 = insertvalue { i32, i32 } %0, i32 %a.1, 1
  ret { i32, i32 } %1
}

attributes #0 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }

!llvm.module.flags = !{!0, !1, !2}
!llvm.ident = !{!3}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{i32 7, !"uwtable", i32 2}
!3 = !{!"rustc version 1.99.0-nightly (9f36de775 2026-07-19)"}
