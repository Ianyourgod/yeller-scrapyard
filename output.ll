; ModuleID = 'main'
source_filename = "main"

define i32 @main() {
entry:
  %.tmp.0 = alloca i32, align 4
  %call = call i32 @fib(i32 5)
  store i32 %call, i32* %.tmp.0, align 4
  %.tmp.01 = load i32, i32* %.tmp.0, align 4
  ret i32 %.tmp.01

"after term":                                     ; No predecessors!
  ret i32 0
}

define i32 @fib(i32 %0) {
entry:
  %.tmp.6 = alloca i32, align 4
  %.tmp.3 = alloca i32, align 4
  %.tmp.7 = alloca i32, align 4
  %.tmp.9 = alloca i32, align 4
  %count = alloca i32, align 4
  %.tmp.4 = alloca i32, align 4
  %.tmp.8 = alloca i32, align 4
  %.tmp.10 = alloca i32, align 4
  %.tmp.1 = alloca i32, align 4
  %friendly = alloca i32, align 4
  %count1 = alloca i32, align 4
  store i32 %0, i32* %count1, align 4
  store i32 0, i32* %friendly, align 4
  %count2 = load i32, i32* %count1, align 4
  %equal = icmp eq i32 %count2, 0
  %extend = zext i1 %equal to i32
  store i32 %extend, i32* %.tmp.1, align 4
  %.tmp.13 = load i32, i32* %.tmp.1, align 4
  %compare = icmp eq i32 %.tmp.13, 0
  br i1 %compare, label %.tmp.2, label %"no branch"

.tmp.2:                                           ; preds = %"after term", %entry
  %count4 = load i32, i32* %count1, align 4
  %subtract = sub i32 %count4, 1
  store i32 %subtract, i32* %.tmp.3, align 4
  %.tmp.35 = load i32, i32* %.tmp.3, align 4
  %equal6 = icmp eq i32 %.tmp.35, 0
  %extend7 = zext i1 %equal6 to i32
  store i32 %extend7, i32* %.tmp.4, align 4
  %.tmp.48 = load i32, i32* %.tmp.4, align 4
  %compare9 = icmp eq i32 %.tmp.48, 0
  br i1 %compare9, label %.tmp.5, label %"no branch10"

"no branch":                                      ; preds = %entry
  ret i32 1

"after term":                                     ; No predecessors!
  br label %.tmp.2

.tmp.5:                                           ; preds = %"after term11", %.tmp.2
  %count12 = load i32, i32* %count1, align 4
  %subtract13 = sub i32 %count12, 1
  store i32 %subtract13, i32* %.tmp.6, align 4
  %.tmp.614 = load i32, i32* %.tmp.6, align 4
  %call = call i32 @fib(i32 %.tmp.614)
  store i32 %call, i32* %.tmp.7, align 4
  %count15 = load i32, i32* %count1, align 4
  %subtract16 = sub i32 %count15, 2
  store i32 %subtract16, i32* %.tmp.8, align 4
  %.tmp.817 = load i32, i32* %.tmp.8, align 4
  %call18 = call i32 @fib(i32 %.tmp.817)
  store i32 %call18, i32* %.tmp.9, align 4
  %.tmp.719 = load i32, i32* %.tmp.7, align 4
  %.tmp.920 = load i32, i32* %.tmp.9, align 4
  %add = add i32 %.tmp.719, %.tmp.920
  store i32 %add, i32* %.tmp.10, align 4
  %.tmp.1021 = load i32, i32* %.tmp.10, align 4
  ret i32 %.tmp.1021

"no branch10":                                    ; preds = %.tmp.2
  ret i32 1

"after term11":                                   ; No predecessors!
  br label %.tmp.5

"after term22":                                   ; No predecessors!
  ret i32 0
}
