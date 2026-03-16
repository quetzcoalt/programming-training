import Data.Char (toLower)

isPalindrom :: String -> Bool
isPalindrom str = map toLower str == map toLower (reverse str)
