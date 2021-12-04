module Parser where

import Control.Monad.Except
import Control.Monad.State
import Data.Functor
import Flow
import Lexer

type Parser a = StateT [Token] (Either String) a

nextToken :: Parser (Maybe Token)
nextToken = do
  ts <- get
  case ts of
    [] -> pure Nothing
    (t : ts') -> put ts' $> Just t

matchLookahead :: [(Token, Parser a)] -> Parser a -> Parser a
matchLookahead cases def = do
  ts <- get
  case ts of
    (t : ts') -> case lookup t cases of
      Just action -> put ts' >> action
      Nothing -> def
    [] -> def

expr = multExpr >>= expr'

expr' value =
  matchLookahead
    [ (Plus, multExpr >>= (value +) .> expr')
    , (Minus, multExpr >>= (value -) .> expr')
    ]
    (pure value)

multExpr = potExpr >>= multExpr'

multExpr' value =
  matchLookahead
    [ (Mult, potExpr >>= (value *) .> multExpr')
    , (Div, potExpr >>= (value /) .> multExpr')
    ]
    (pure value)

potExpr = do
  value <- term
  matchLookahead
    [(Exp, (value **) <$> potExpr)]
    (pure value)

term = do
  t <- nextToken
  case t of
    Just LParen -> do
      value <- expr
      t' <- nextToken
      case t' of
        Just RParen -> pure value
        _ -> throwError $ "Expected a closing parenthesis, but got " ++ show t'
    Just (Number n) -> pure n
    _ -> throwError "Missing operand (expected either a number or a parenthesised expression)"

run :: String -> Either String Float
run input = do
  (result, inputRest) <- lexString input >>= runStateT expr
  if null inputRest
    then pure result
    else throwError $ "There are trailing tokens: " ++ show inputRest
