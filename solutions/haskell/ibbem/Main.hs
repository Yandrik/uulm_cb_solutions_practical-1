module Main where

import Control.Monad
import Data.List
import Flow
import Parser

testCases :: [(String, Maybe Float)]
testCases =
  [ ("2 + 5 / 3 * 2", Just 5.33333),
    ("(2 + 5) / 3", Just 2.33333),
    ("((10 - 213) * 25) + 27", Just (-5048)),
    ("(7 - 3) * 4^3", Just 256),
    ("((1 + 1) * 1 - 2", Nothing),
    ("a + 1 * 3", Nothing)
  ]

tests :: [(String, Maybe Float, Either String Float)]
tests = map (\(input, expected) -> (input, expected, run input)) testCases

faillingTests :: [(String, Maybe Float, Either String Float)]
faillingTests = filter (not . isPassing) tests
  where
    isPassing (_, Just expected, Right actual) = abs (actual - expected) < 1e-3
    isPassing (_, Nothing, Left _) = True
    isPassing _ = False

printTestResults :: [(String, Maybe Float, Either String Float)] -> IO ()
printTestResults =
  map (\(input, expected, actual) -> show input ++ "\t" ++ show expected ++ "\t" ++ show actual)
    .> intercalate "\n"
    .> putStrLn

main = forever $ getLine >>= run .> either putStrLn print
