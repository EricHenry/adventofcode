import Data.List
import System.IO

{-
  part 1
main = do
  contents <- readFile "captcha-input.txt"
  let intArr = strToIntArr contents
  let shifted = shift 1 intArr
  let zipped = zip intArr shifted
  print (sum (duplicates zipped))
-}

{-
  part 2
-}
main = do
  contents <- readFile "captcha-input.txt"
  let intArr = strToIntArr contents
  let half = (length intArr) `div` 2
  let shifted = shift half intArr
  let zipped = zip intArr shifted
  print (sum (duplicates zipped))

{-
  . == compose
  : == prepend oporator
  :"" takes in the [char] and turns it into a [String]
-}
strToIntArr :: String -> [Int]
strToIntArr s = map (read . (:"")) s

shift :: Int -> [Int] -> [Int]
shift c xs = (drop c xs) ++ (take c xs)

duplicates :: [(Int, Int)] -> [Int]
duplicates xs = [ x | (x,y) <- xs, x == y ]
