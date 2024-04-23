sq :: Int -> Int
sq x = x * x

sqlist :: [Int] -> [Int]
sqlist  = map sq -- l = map(int, input().split())

iseven :: Int -> Bool
iseven x = (mod x 2 ==0)

filteven :: [Int] -> [Int]
filteven = filter iseven

divisors :: Int -> [Int]
divisors n = [m | m <- [1..n], mod n m == 0] -- div, mod....  op n m - n op m

factorial :: Int -> Int
factorial 0 = 1
factorial n = n * factorial (n - 1)

isPrime :: Int -> Bool
isPrime n = (divisors n == [1, n])

-- merge sort -- nlogn
-- merge function -- 2 lists (sorted) -- merge -- sorted
-- merge(l1, l2) = l3 -- sorted

-- l1 = [2,8,10], l2 = [3,7,9]
-- x = [2], xs = [8,10] -- y = [3], ys = [7,9]
-- [2]: merge([8, 10] [3,7,9])

merge :: [Int] -> [Int] -> [Int] -- merge(l1, l2) -> l3
merge [] ys = ys
merge xs [] = xs
merge (x: xs) (y: ys) | x <= y = x:(merge xs (y: ys))
                      | otherwise = y:(merge (x: xs) ys)

-- msort(l) = l (sorted)
msort [] = []
msort [x] = [x]

-- front(l) -- msort(L) == merge(msort(L[left, mid]), msort(L[mid+1, right]))
msort l = merge(msort (front l)) (msort (back l))
  where
    front l = take(div (length l) 2) l -- take(len(l) // 2) l -- l[0, len(l) // 2]
    back l = drop(div (length l) 2) l -- drop(len(l) // 2) l -- l[(len(l) + 1) // 2, len(l)]


isort [] = []
isort [x] = [x]
isort (x:xs) = insert x (isort xs)
  where
    insert x [] = [x]
    insert x (y: l) | x <= y = x:y:l
                    | otherwise = y: (insert x l)
