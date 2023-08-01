Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
iex "& {$(irm get.scoop.sh)} -RunAsAdmin"

scoop bucket add lune https://github.com/CompeyDev/lune-packaging.git
scoop install lune
