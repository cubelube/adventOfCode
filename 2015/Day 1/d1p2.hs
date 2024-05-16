import Data.List
import System.IO

p2 :: String -> Int
p2 input = length.takeWhile(> -1).reverse $ foldl(\z@(y:ys) x -> if x == '(' then y + 1:z else y - 1:z) [0] input

main :: IO ()
main = do
    handle <- openFile "d1input.txt" ReadMode
    contents <- hGetContents handle
    print(p2 contents)
    hClose handle
