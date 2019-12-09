(begin 
  (define map (lambda f xs)
    (if (null? xs)
        (list)
        (cons (f (car xs)) (cdr xs))))

  (define (square x) (mul x x))

  (map square (list 1 2 3)))
