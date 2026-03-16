hasUniqueChar :: String -> Bool
hasUniqueChar "" = True
hasUniqueChar (x : xs)
  | countLetters (x : xs) x > 1 = False
  | otherwise = hasUniqueChar xs

countLetters :: String -> Char -> Int
countLetters str c = length $ filter (== c) str
