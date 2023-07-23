function increment_version() {
 local v=$1
 if [ -z $2 ]; then 
    local rgx='^((?:[0-9]+\.)*)([0-9]+)($)'
 else 
    local rgx='^((?:[0-9]+\.){'$(($2-1))'})([0-9]+)(\.|$)'
    for (( p=`grep -o "\."<<<".$v"|wc -l`; p<$2; p++)); do 
       v+=.0; done; fi
 val=`echo -e "$v" | perl -pe 's/^.*'$rgx'.*$/$2/'`
 echo "$v" | perl -pe s/$rgx.*$'/${1}'`printf %0${#val}s $(($val+1))`/
}

function get_current_version() {
  current_version=`cat PKGBUILD.git | grep pkgver | head -n 1 | cut -d '=' -f 2`

  printf current_version
}

log_prefix=\x1b[34m[\u001b[0m\x1b[31m
log_suffix=\x1b[34m\x1b[34m]\u001b[0m

function log() {
   log_identifier=$1
   log_msg=$2

   echo -e "$logprefix$log_identifier$log_suffix $log_msg"
}