urlify :: String -> String
urlify "" = ""
urlify (x : xs)
  | x == ' ' = "%20" ++ urlify xs
  | otherwise = x : urlify xs
