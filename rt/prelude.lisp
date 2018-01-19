(define foo (lambda (a b)
  (+ a (* 2 b))))

(foo 5 6)
; ((lambda (a b) (+ a (* 2 b))) 5 6)
; (+ 5 (* 2 6))
