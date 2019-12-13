(begin
  (define point
    (lambda (x y)      
      
      (define dispatch (lambda (methodname)
                         (define m? (lambda (sym) (eq? sym methodname)))
                         (if (m? (quote foo)) 1
                             (if (m? (quote as-list))
                                 (list x y)
                                 (quote unknownmethod)))))
      dispatch))
    
  ((point 1 2) (quote as-list)))

      
