
(define second 
    (lambda 
        (l) 
        (car 
            (cdr l))))

(define not 
    (lambda 
        (x) 
        (cond 
            (x 0) 
            (1 1))))

(equal 
    (quote  
        (b 2))
    (second 
        (quote 
            (
                (a 1) 
                (b 2) 
                (c 3)))))