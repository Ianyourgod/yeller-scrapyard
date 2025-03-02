; ModuleID = 'main'
source_filename = "main"

define i32 @main() {
entry:
  %.tmp.10 = alloca i32, align 4
  %.tmp.12 = alloca i32, align 4
  %.tmp.0 = alloca i32, align 4
  %.tmp.5 = alloca i32, align 4
  %.tmp.4 = alloca i32, align 4
  %.tmp.6 = alloca i32, align 4
  %.tmp.3 = alloca i32, align 4
  %.tmp.1 = alloca i32, align 4
  %.tmp.8 = alloca i32, align 4
  %.tmp.9 = alloca i32, align 4
  %.tmp.11 = alloca i32, align 4
  %.tmp.13 = alloca i32, align 4
  %.tmp.2 = alloca i32, align 4
  %.tmp.7 = alloca i32, align 4
  %call = call i32 @putchar(i32 72)
  store i32 %call, i32* %.tmp.0, align 4
  %call1 = call i32 @putchar(i32 101)
  store i32 %call1, i32* %.tmp.1, align 4
  %call2 = call i32 @putchar(i32 108)
  store i32 %call2, i32* %.tmp.2, align 4
  %call3 = call i32 @putchar(i32 108)
  store i32 %call3, i32* %.tmp.3, align 4
  %call4 = call i32 @putchar(i32 111)
  store i32 %call4, i32* %.tmp.4, align 4
  %call5 = call i32 @putchar(i32 44)
  store i32 %call5, i32* %.tmp.5, align 4
  %call6 = call i32 @putchar(i32 32)
  store i32 %call6, i32* %.tmp.6, align 4
  %call7 = call i32 @putchar(i32 87)
  store i32 %call7, i32* %.tmp.7, align 4
  %call8 = call i32 @putchar(i32 111)
  store i32 %call8, i32* %.tmp.8, align 4
  %call9 = call i32 @putchar(i32 114)
  store i32 %call9, i32* %.tmp.9, align 4
  %call10 = call i32 @putchar(i32 108)
  store i32 %call10, i32* %.tmp.10, align 4
  %call11 = call i32 @putchar(i32 100)
  store i32 %call11, i32* %.tmp.11, align 4
  %call12 = call i32 @putchar(i32 33)
  store i32 %call12, i32* %.tmp.12, align 4
  %call13 = call i32 @putchar(i32 10)
  store i32 %call13, i32* %.tmp.13, align 4
  ret i32 0

"after term":                                     ; No predecessors!
  ret i32 0
}

declare i32 @putchar(i32)
