; ModuleID = 'main'
source_filename = "main"

define i32 @main(i32 %0) {
entry:
  br label %.tmp.2

.tmp.2:                                           ; preds = %"no branch", %entry
  %counter4 = phi i32 [ %add, %"no branch" ], [ 0, %entry ]
  %_variable5 = phi i32 [ %subtract, %"no branch" ], [ 10, %entry ]
  %compare = icmp eq i32 %_variable5, 0
  br i1 %compare, label %.tmp.3, label %"no branch"

.tmp.3:                                           ; preds = %.tmp.2
  ret i32 %counter4

"no branch":                                      ; preds = %.tmp.2
  %add = add i32 %_variable5, %counter4
  %subtract = add i32 %_variable5, -1
  br label %.tmp.2
}
