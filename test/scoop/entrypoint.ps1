Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
irm get.scoop.sh | iex

scoop bucket add lune https://github.com/CompeyDev/lune-packaging.git
scoop install lune
