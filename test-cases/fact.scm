(begin
  (define fact (lambda (n)
                 (if (eq? n 0) 1
                     (mul n (fact (dec n))))))
  (fact 4)
  )
