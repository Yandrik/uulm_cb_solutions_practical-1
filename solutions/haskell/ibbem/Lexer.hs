module Lexer where

import Control.Monad.Except
import Data.Char

data Token = Plus | Minus | Mult | Div | Exp | LParen | RParen | Number Float
  deriving (Show, Eq)

lexString :: String -> Either String [Token]
lexString ('+' : cs) = (Plus :) <$> lexString cs
lexString ('-' : cs) = (Minus :) <$> lexString cs
lexString ('*' : cs) = (Mult :) <$> lexString cs
lexString ('/' : cs) = (Div :) <$> lexString cs
lexString ('^' : cs) = (Exp :) <$> lexString cs
lexString ('(' : cs) = (LParen :) <$> lexString cs
lexString (')' : cs) = (RParen :) <$> lexString cs
lexString (c : cs)
  | isSpace c = lexString cs
  | isDigit c =
    let (digits, cs') = break (not . isDigit) cs
     in (Number (fromIntegral $ foldl (\n d -> 10 * n + d) 0 $ map digitToInt $ c : digits) :)
          <$> lexString cs'
  | otherwise = throwError $ "Invalid character: " ++ [c]
lexString [] = pure []
