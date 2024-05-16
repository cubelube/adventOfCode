import Data.List
import System.IO

p1 :: String -> Integer
p1 = foldl (\x y -> if y == '(' then x + 1 else x - 1) 0

main :: IO ()
main = do
    handle <- openFile "d1input.txt" ReadMode
    contents <- hGetContents handle
    print(p1 contents)
    hClose handle
