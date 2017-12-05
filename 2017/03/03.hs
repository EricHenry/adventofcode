import System.IO
import Data.List
import Data.List.Split
import Data.Char
import Data.Ix


input = 265149

-- part 1
{-
main = do
  let sq = (toInteger . last . takeWhile (\x -> x*x <= input)) [1,3..]
  let startX = (sq `div` 2)
  let startCoor = (startX, (-startX))
  let cornerCoors = [ (startX + 1, (startX + 1) * (-1)), ((startX + 1) * (-1), startX + 1), ((startX + 1) * (-1), (startX + 1) * (-1)), (startX + 1, startX + 1)]
  let corners = reverse [(((sq*sq) + sq * x) + x) | x <- [1..4]]
  let zipped = zip3 corners cornerCoors (reverse [1..4])
  let closestCorner = (minimum . takeWhile (\(n, c, i) -> n > input)) zipped
  let (x, y) = findCoor closestCorner input
  let manhatDist = ((abs x) + (abs y))

  print sq
  print corners
  print cornerCoors
  print zipped
  print closestCorner
  print (x, y)
  print manhatDist
-}

findCoor:: (Integer, (Integer, Integer), Int) -> Integer -> (Integer, Integer)
findCoor (n, (x,y), corner) input
  | corner == 1 = (x, y - (n - input))
  | corner == 2 = (x + (n - input), y)
  | corner == 3 = (x, y + (n - input))
  | otherwise   = (x - (n - input), y)

