import Data.List
import Data.List.Split
import System.IO

-- part 1
-- main = do
--   content <- readFile "input.txt"
--   let pwds = map (\ps -> words ps) $ lines content
--   let validPwds = [ps | ps <- pwds, noDups ps]
--   print $ length pwds
--   print $ length validPwds

-- part 2
main = do
  content <- readFile "input.txt"
  let pwds = map (\ps -> words ps) $ lines content
  let hasDups = [[not $ dupsp2 perms p | p <- ps,
                        let perms = concatMap permutations $ delete p $ dropWhile (\x -> x /= p) ps ] | ps <- pwds]
  let validPwds = filter (\w -> not $ elem False w) hasDups
  print $ length pwds
  print $ length validPwds

noDups :: [String] -> Bool
noDups ws = (length $ filter (dups ws) ws) == 0

dups :: [String] -> [Char] -> Bool
dups xs x = (length $ elemIndices x xs) > 1

dupsp2 :: [String] -> [Char] -> Bool
dupsp2 xs x = (length $ elemIndices x xs) > 0