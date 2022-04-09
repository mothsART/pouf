complete -c pouf -n "__fish_use_subcommand" -s h -l help -d 'Print help information'
complete -c pouf -n "__fish_use_subcommand" -s V -l version -d 'Print version information'
complete -c pouf -n "__fish_use_subcommand" -f -a "lorem.word" -d 'give a fake word'
complete -c pouf -n "__fish_use_subcommand" -f -a "barecode.isbn" -d 'give an isbn code'
complete -c pouf -n "__fish_use_subcommand" -f -a "internet.mail" -d 'give a fake mail'
complete -c pouf -n "__fish_use_subcommand" -f -a "internet.ip" -d 'give a fake IP (Internet Protocol)'
complete -c pouf -n "__fish_use_subcommand" -f -a "internet.mac" -d 'give a fake mac adress'
complete -c pouf -n "__fish_use_subcommand" -f -a "internet.useragent" -d 'give a fake user agent'
complete -c pouf -n "__fish_use_subcommand" -f -a "http.code" -d 'give a fake HTTP code'
complete -c pouf -n "__fish_use_subcommand" -f -a "time.time" -d 'give a fake time'
complete -c pouf -n "__fish_use_subcommand" -f -a "time.date" -d 'give a fake date'
complete -c pouf -n "__fish_use_subcommand" -f -a "filesystem.mimetype" -d 'give a fake mime-type'
complete -c pouf -n "__fish_use_subcommand" -f -a "filesystem.semver" -d 'give a fake semver version'
complete -c pouf -n "__fish_use_subcommand" -f -a "administrative.healthinsurrancecode" -d 'give a Health insurrance code'
complete -c pouf -n "__fish_use_subcommand" -f -a "finance.bic" -d 'give a fake BIC (Business Identifier Code)'
complete -c pouf -n "__fish_use_subcommand" -f -a "auto.licenseplate" -d 'give a french automotive license plate'
complete -c pouf -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c pouf -n "__fish_seen_subcommand_from lorem.word" -s h -l help -d 'Print help information'
complete -c pouf -n "__fish_seen_subcommand_from barecode.isbn" -s h -l help -d 'Print help information'
complete -c pouf -n "__fish_seen_subcommand_from internet.mail" -s l -d 'give lang (ie: fr_FR)' -r
complete -c pouf -n "__fish_seen_subcommand_from internet.mail" -s n -d 'number of values' -r
complete -c pouf -n "__fish_seen_subcommand_from internet.mail" -s h -l help -d 'Print help information'
complete -c pouf -n "__fish_seen_subcommand_from internet.ip" -s 4 -l ipv4 -d 'give exclusivly IPv4'
complete -c pouf -n "__fish_seen_subcommand_from internet.ip" -s 6 -l ipv6 -d 'give exclusivly IPv6'
complete -c pouf -n "__fish_seen_subcommand_from internet.ip" -s h -l help -d 'Print help information'
complete -c pouf -n "__fish_seen_subcommand_from internet.mac" -s h -l help -d 'Print help information'
complete -c pouf -n "__fish_seen_subcommand_from internet.useragent" -s h -l help -d 'Print help information'
complete -c pouf -n "__fish_seen_subcommand_from http.code" -s h -l help -d 'Print help information'
complete -c pouf -n "__fish_seen_subcommand_from time.time" -s h -l help -d 'Print help information'
complete -c pouf -n "__fish_seen_subcommand_from time.date" -s h -l help -d 'Print help information'
complete -c pouf -n "__fish_seen_subcommand_from filesystem.mimetype" -s h -l help -d 'Print help information'
complete -c pouf -n "__fish_seen_subcommand_from filesystem.semver" -s s -l stable -d 'give exclusivly stable semver version (X.Y.Z)'
complete -c pouf -n "__fish_seen_subcommand_from filesystem.semver" -s u -l unstable -d 'give exclusivly unstable semver version (X-Y-Z-V.W)'
complete -c pouf -n "__fish_seen_subcommand_from filesystem.semver" -s h -l help -d 'Print help information'
complete -c pouf -n "__fish_seen_subcommand_from administrative.healthinsurrancecode" -s h -l help -d 'Print help information'
complete -c pouf -n "__fish_seen_subcommand_from finance.bic" -s h -l help -d 'Print help information'
complete -c pouf -n "__fish_seen_subcommand_from auto.licenseplate" -s h -l help -d 'Print help information'
