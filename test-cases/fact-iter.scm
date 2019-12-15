(begin

  (define factorial (lambda (n) 
                      (fact-iter 1 1 n)))

  (define fact-iter (lambda (product counter max-count)
                      (if (> counter max-count)
                          product
                          (fact-iter (* counter product)
                                     (+ counter 1)
                                     max-count))))
  
  (factorial 10)
  )
