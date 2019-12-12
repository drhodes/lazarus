(begin

  (define map (lambda (f xs)
                (cons (f (car xs))
                      (map f (cdr xs)))))

  (define square (lambda (x) (mul x x)))

  (map square (list 1 2 3 4))
  
  )

