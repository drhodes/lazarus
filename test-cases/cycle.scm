(begin
  (define mkCycle (lambda ()
                    (define x 1)
                    (set! x (list x))
                    (quote ok)))
  (define dec (lambda (n) (- n 1)))
  
  (define calln (lambda (f n)
                  (if (eq? n 0) (quote ok)
                      (begin                        
                        (f)
                        (calln f (dec n))))))
  
  (define monster1 (lambda () (calln mkCycle 50)))
  (define monster2 (lambda () (calln monster1 100)))
  (define monster3 (lambda () (calln monster2 100)))
  (monster3)
  )
