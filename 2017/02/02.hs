import System.IO
import Data.List
import Data.List.Split
import Data.Char

-- part 1
-- main = do
--   content <- readFile "input.txt"
--   let linesOfFiles = lines content
--   let rows = [ (splitOn "\t") x | x <- linesOfFiles ]
--   let rowsAsInts = strToInt rows
--   let maxAndMin = [(maximum r, minimum r) | r <- rowsAsInts]
--   let sum = foldl (\a (max, min) -> a + max - min) 0 maxAndMin
--   print sum

-- part 2
main = do
  content <- readFile "input.txt"
  let linesOfFiles = lines content
  let rows = [ (splitOn "\t") x | x <- linesOfFiles ]
  let divisible = map evenDivide $ strToInt rows
  let sum = foldl (\a [x] -> a + x) 0 divisible
  print sum


strToInt:: [[[Char]]] -> [[Int]]
strToInt rs = [[read x::Int | x <- r] | r <- rs]

evenDivide:: [Int] -> [Int]
evenDivide xs = [x `div` y | x <- xs, y <- xs, noRemainder x y]

noRemainder:: Int -> Int -> Bool
noRemainder x y = x `mod` y == 0 && x /= y
