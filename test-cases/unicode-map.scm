(begin
  (define map (lambda (f xs)
                (if (eq? #t (null? xs)) (list)
                    (cons (f (car xs))
                          (map f (cdr xs))))))
  
  (define ε (lambda (x) (* x x)))
  
  (map ε (list 1 2 3 4 5))
  )

