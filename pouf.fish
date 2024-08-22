# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_pouf_global_optspecs
	string join \n h/help V/version
end

function __fish_pouf_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_pouf_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_pouf_using_subcommand
	set -l cmd (__fish_pouf_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c pouf -n "__fish_pouf_needs_command" -s h -l help -d 'Print help'
complete -c pouf -n "__fish_pouf_needs_command" -s V -l version -d 'Print version'
complete -c pouf -n "__fish_pouf_needs_command" -f -a "template" -d 'generate file with template'
complete -c pouf -n "__fish_pouf_needs_command" -f -a "address.city" -d 'give a city name (English only)'
complete -c pouf -n "__fish_pouf_needs_command" -f -a "address.country" -d 'give a country name and code (English only)'
complete -c pouf -n "__fish_pouf_needs_command" -f -a "address.street" -d 'give a street name (English only)'
complete -c pouf -n "__fish_pouf_needs_command" -f -a "administrative.healthinsurrancecode" -d 'give a Health insurrance code (French only)'
complete -c pouf -n "__fish_pouf_needs_command" -f -a "auto.licenseplate" -d 'give an automotive license plate (French only)'
complete -c pouf -n "__fish_pouf_needs_command" -f -a "barcode.isbn" -d 'give an isbn code'
complete -c pouf -n "__fish_pouf_needs_command" -f -a "color" -d 'give a fake color (hexadécimal, rgb, rgba, hsl and hsla representation)'
complete -c pouf -n "__fish_pouf_needs_command" -f -a "filesystem.mimetype" -d 'give a fake mime-type'
complete -c pouf -n "__fish_pouf_needs_command" -f -a "filesystem.semver" -d 'give a fake semver version'
complete -c pouf -n "__fish_pouf_needs_command" -f -a "finance.bic" -d 'give a fake BIC (Business Identifier Code)'
complete -c pouf -n "__fish_pouf_needs_command" -f -a "http.code" -d 'give a fake HTTP code'
complete -c pouf -n "__fish_pouf_needs_command" -f -a "internet.ip" -d 'give a fake IP (Internet Protocol)'
complete -c pouf -n "__fish_pouf_needs_command" -f -a "internet.mac" -d 'give a fake mac adress'
complete -c pouf -n "__fish_pouf_needs_command" -f -a "internet.mail" -d 'give a fake mail'
complete -c pouf -n "__fish_pouf_needs_command" -f -a "internet.useragent" -d 'give a fake user agent'
complete -c pouf -n "__fish_pouf_needs_command" -f -a "lorem.word" -d 'give a fake word (in Latin)'
complete -c pouf -n "__fish_pouf_needs_command" -f -a "people.name" -d 'give a fake name'
complete -c pouf -n "__fish_pouf_needs_command" -f -a "time.time" -d 'give a fake time'
complete -c pouf -n "__fish_pouf_needs_command" -f -a "time.date" -d 'give a fake date'
complete -c pouf -n "__fish_pouf_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c pouf -n "__fish_pouf_using_subcommand template" -s i -l input -d 'give an input template file (tera : https://tera.netlify.app/)' -r -F
complete -c pouf -n "__fish_pouf_using_subcommand template" -s n -l number -d 'number of values' -r
complete -c pouf -n "__fish_pouf_using_subcommand template" -s h -l help -d 'Print help'
complete -c pouf -n "__fish_pouf_using_subcommand address.city" -s n -l number -d 'number of values' -r
complete -c pouf -n "__fish_pouf_using_subcommand address.city" -s h -l help -d 'Print help'
complete -c pouf -n "__fish_pouf_using_subcommand address.country" -s n -l number -d 'number of values' -r
complete -c pouf -n "__fish_pouf_using_subcommand address.country" -s h -l help -d 'Print help'
complete -c pouf -n "__fish_pouf_using_subcommand address.street" -s n -l number -d 'number of values' -r
complete -c pouf -n "__fish_pouf_using_subcommand address.street" -s h -l help -d 'Print help'
complete -c pouf -n "__fish_pouf_using_subcommand administrative.healthinsurrancecode" -s n -l number -d 'number of values' -r
complete -c pouf -n "__fish_pouf_using_subcommand administrative.healthinsurrancecode" -s h -l help -d 'Print help'
complete -c pouf -n "__fish_pouf_using_subcommand auto.licenseplate" -s n -l number -d 'number of values' -r
complete -c pouf -n "__fish_pouf_using_subcommand auto.licenseplate" -s h -l help -d 'Print help'
complete -c pouf -n "__fish_pouf_using_subcommand barcode.isbn" -s n -l number -d 'number of values' -r
complete -c pouf -n "__fish_pouf_using_subcommand barcode.isbn" -s h -l help -d 'Print help'
complete -c pouf -n "__fish_pouf_using_subcommand color" -s n -l number -d 'number of values' -r
complete -c pouf -n "__fish_pouf_using_subcommand color" -s d -l hexa -d 'give a fake hexadecimal color'
complete -c pouf -n "__fish_pouf_using_subcommand color" -s r -l rgb -d 'give a fake rgb color'
complete -c pouf -n "__fish_pouf_using_subcommand color" -s a -l rgba -d 'give a fake rgba color'
complete -c pouf -n "__fish_pouf_using_subcommand color" -s t -l hsl -d 'give a fake hsl (tsl) color'
complete -c pouf -n "__fish_pouf_using_subcommand color" -s l -l hsla -d 'give a fake hsla (tsl) color'
complete -c pouf -n "__fish_pouf_using_subcommand color" -s h -l help -d 'Print help'
complete -c pouf -n "__fish_pouf_using_subcommand filesystem.mimetype" -s n -l number -d 'number of values' -r
complete -c pouf -n "__fish_pouf_using_subcommand filesystem.mimetype" -s h -l help -d 'Print help'
complete -c pouf -n "__fish_pouf_using_subcommand filesystem.semver" -s n -l number -d 'number of values' -r
complete -c pouf -n "__fish_pouf_using_subcommand filesystem.semver" -s s -l stable -d 'give exclusivly stable semver version (X.Y.Z)'
complete -c pouf -n "__fish_pouf_using_subcommand filesystem.semver" -s u -l unstable -d 'give exclusivly unstable semver version (X-Y-Z-V.W)'
complete -c pouf -n "__fish_pouf_using_subcommand filesystem.semver" -s h -l help -d 'Print help'
complete -c pouf -n "__fish_pouf_using_subcommand finance.bic" -s n -l number -d 'number of values' -r
complete -c pouf -n "__fish_pouf_using_subcommand finance.bic" -s h -l help -d 'Print help'
complete -c pouf -n "__fish_pouf_using_subcommand http.code" -s l -l lang -d 'give lang (ie: fr_FR)' -r -f -a "{fr\t'',fr_FR\t'',en\t''}"
complete -c pouf -n "__fish_pouf_using_subcommand http.code" -s n -l number -d 'number of values' -r
complete -c pouf -n "__fish_pouf_using_subcommand http.code" -s h -l help -d 'Print help'
complete -c pouf -n "__fish_pouf_using_subcommand internet.ip" -s n -l number -d 'number of values' -r
complete -c pouf -n "__fish_pouf_using_subcommand internet.ip" -s 4 -l ipv4 -d 'give exclusivly IPv4'
complete -c pouf -n "__fish_pouf_using_subcommand internet.ip" -s 6 -l ipv6 -d 'give exclusivly IPv6'
complete -c pouf -n "__fish_pouf_using_subcommand internet.ip" -s h -l help -d 'Print help'
complete -c pouf -n "__fish_pouf_using_subcommand internet.mac" -s n -l number -d 'number of values' -r
complete -c pouf -n "__fish_pouf_using_subcommand internet.mac" -s h -l help -d 'Print help'
complete -c pouf -n "__fish_pouf_using_subcommand internet.mail" -s n -l number -d 'number of values' -r
complete -c pouf -n "__fish_pouf_using_subcommand internet.mail" -s h -l help -d 'Print help'
complete -c pouf -n "__fish_pouf_using_subcommand internet.useragent" -s n -l number -d 'number of values' -r
complete -c pouf -n "__fish_pouf_using_subcommand internet.useragent" -s h -l help -d 'Print help'
complete -c pouf -n "__fish_pouf_using_subcommand lorem.word" -s n -l number -d 'number of values' -r
complete -c pouf -n "__fish_pouf_using_subcommand lorem.word" -s h -l help -d 'Print help'
complete -c pouf -n "__fish_pouf_using_subcommand people.name" -s l -l lang -d 'give lang (ie: fr_FR)' -r -f -a "{fr\t'',fr_FR\t'',en\t''}"
complete -c pouf -n "__fish_pouf_using_subcommand people.name" -s n -l number -d 'number of values' -r
complete -c pouf -n "__fish_pouf_using_subcommand people.name" -s t -l title -d 'give a fake name title'
complete -c pouf -n "__fish_pouf_using_subcommand people.name" -s w -l with-title -d 'give a fake name with her title'
complete -c pouf -n "__fish_pouf_using_subcommand people.name" -s f -l firstname -d 'give a fake firstname'
complete -c pouf -n "__fish_pouf_using_subcommand people.name" -s z -l lastname -d 'give a fake lastname'
complete -c pouf -n "__fish_pouf_using_subcommand people.name" -s h -l help -d 'Print help'
complete -c pouf -n "__fish_pouf_using_subcommand time.time" -s n -l number -d 'number of values' -r
complete -c pouf -n "__fish_pouf_using_subcommand time.time" -s h -l help -d 'Print help'
complete -c pouf -n "__fish_pouf_using_subcommand time.date" -s n -l number -d 'number of values' -r
complete -c pouf -n "__fish_pouf_using_subcommand time.date" -s h -l help -d 'Print help'
complete -c pouf -n "__fish_pouf_using_subcommand help; and not __fish_seen_subcommand_from template address.city address.country address.street administrative.healthinsurrancecode auto.licenseplate barcode.isbn color filesystem.mimetype filesystem.semver finance.bic http.code internet.ip internet.mac internet.mail internet.useragent lorem.word people.name time.time time.date help" -f -a "template" -d 'generate file with template'
complete -c pouf -n "__fish_pouf_using_subcommand help; and not __fish_seen_subcommand_from template address.city address.country address.street administrative.healthinsurrancecode auto.licenseplate barcode.isbn color filesystem.mimetype filesystem.semver finance.bic http.code internet.ip internet.mac internet.mail internet.useragent lorem.word people.name time.time time.date help" -f -a "address.city" -d 'give a city name (English only)'
complete -c pouf -n "__fish_pouf_using_subcommand help; and not __fish_seen_subcommand_from template address.city address.country address.street administrative.healthinsurrancecode auto.licenseplate barcode.isbn color filesystem.mimetype filesystem.semver finance.bic http.code internet.ip internet.mac internet.mail internet.useragent lorem.word people.name time.time time.date help" -f -a "address.country" -d 'give a country name and code (English only)'
complete -c pouf -n "__fish_pouf_using_subcommand help; and not __fish_seen_subcommand_from template address.city address.country address.street administrative.healthinsurrancecode auto.licenseplate barcode.isbn color filesystem.mimetype filesystem.semver finance.bic http.code internet.ip internet.mac internet.mail internet.useragent lorem.word people.name time.time time.date help" -f -a "address.street" -d 'give a street name (English only)'
complete -c pouf -n "__fish_pouf_using_subcommand help; and not __fish_seen_subcommand_from template address.city address.country address.street administrative.healthinsurrancecode auto.licenseplate barcode.isbn color filesystem.mimetype filesystem.semver finance.bic http.code internet.ip internet.mac internet.mail internet.useragent lorem.word people.name time.time time.date help" -f -a "administrative.healthinsurrancecode" -d 'give a Health insurrance code (French only)'
complete -c pouf -n "__fish_pouf_using_subcommand help; and not __fish_seen_subcommand_from template address.city address.country address.street administrative.healthinsurrancecode auto.licenseplate barcode.isbn color filesystem.mimetype filesystem.semver finance.bic http.code internet.ip internet.mac internet.mail internet.useragent lorem.word people.name time.time time.date help" -f -a "auto.licenseplate" -d 'give an automotive license plate (French only)'
complete -c pouf -n "__fish_pouf_using_subcommand help; and not __fish_seen_subcommand_from template address.city address.country address.street administrative.healthinsurrancecode auto.licenseplate barcode.isbn color filesystem.mimetype filesystem.semver finance.bic http.code internet.ip internet.mac internet.mail internet.useragent lorem.word people.name time.time time.date help" -f -a "barcode.isbn" -d 'give an isbn code'
complete -c pouf -n "__fish_pouf_using_subcommand help; and not __fish_seen_subcommand_from template address.city address.country address.street administrative.healthinsurrancecode auto.licenseplate barcode.isbn color filesystem.mimetype filesystem.semver finance.bic http.code internet.ip internet.mac internet.mail internet.useragent lorem.word people.name time.time time.date help" -f -a "color" -d 'give a fake color (hexadécimal, rgb, rgba, hsl and hsla representation)'
complete -c pouf -n "__fish_pouf_using_subcommand help; and not __fish_seen_subcommand_from template address.city address.country address.street administrative.healthinsurrancecode auto.licenseplate barcode.isbn color filesystem.mimetype filesystem.semver finance.bic http.code internet.ip internet.mac internet.mail internet.useragent lorem.word people.name time.time time.date help" -f -a "filesystem.mimetype" -d 'give a fake mime-type'
complete -c pouf -n "__fish_pouf_using_subcommand help; and not __fish_seen_subcommand_from template address.city address.country address.street administrative.healthinsurrancecode auto.licenseplate barcode.isbn color filesystem.mimetype filesystem.semver finance.bic http.code internet.ip internet.mac internet.mail internet.useragent lorem.word people.name time.time time.date help" -f -a "filesystem.semver" -d 'give a fake semver version'
complete -c pouf -n "__fish_pouf_using_subcommand help; and not __fish_seen_subcommand_from template address.city address.country address.street administrative.healthinsurrancecode auto.licenseplate barcode.isbn color filesystem.mimetype filesystem.semver finance.bic http.code internet.ip internet.mac internet.mail internet.useragent lorem.word people.name time.time time.date help" -f -a "finance.bic" -d 'give a fake BIC (Business Identifier Code)'
complete -c pouf -n "__fish_pouf_using_subcommand help; and not __fish_seen_subcommand_from template address.city address.country address.street administrative.healthinsurrancecode auto.licenseplate barcode.isbn color filesystem.mimetype filesystem.semver finance.bic http.code internet.ip internet.mac internet.mail internet.useragent lorem.word people.name time.time time.date help" -f -a "http.code" -d 'give a fake HTTP code'
complete -c pouf -n "__fish_pouf_using_subcommand help; and not __fish_seen_subcommand_from template address.city address.country address.street administrative.healthinsurrancecode auto.licenseplate barcode.isbn color filesystem.mimetype filesystem.semver finance.bic http.code internet.ip internet.mac internet.mail internet.useragent lorem.word people.name time.time time.date help" -f -a "internet.ip" -d 'give a fake IP (Internet Protocol)'
complete -c pouf -n "__fish_pouf_using_subcommand help; and not __fish_seen_subcommand_from template address.city address.country address.street administrative.healthinsurrancecode auto.licenseplate barcode.isbn color filesystem.mimetype filesystem.semver finance.bic http.code internet.ip internet.mac internet.mail internet.useragent lorem.word people.name time.time time.date help" -f -a "internet.mac" -d 'give a fake mac adress'
complete -c pouf -n "__fish_pouf_using_subcommand help; and not __fish_seen_subcommand_from template address.city address.country address.street administrative.healthinsurrancecode auto.licenseplate barcode.isbn color filesystem.mimetype filesystem.semver finance.bic http.code internet.ip internet.mac internet.mail internet.useragent lorem.word people.name time.time time.date help" -f -a "internet.mail" -d 'give a fake mail'
complete -c pouf -n "__fish_pouf_using_subcommand help; and not __fish_seen_subcommand_from template address.city address.country address.street administrative.healthinsurrancecode auto.licenseplate barcode.isbn color filesystem.mimetype filesystem.semver finance.bic http.code internet.ip internet.mac internet.mail internet.useragent lorem.word people.name time.time time.date help" -f -a "internet.useragent" -d 'give a fake user agent'
complete -c pouf -n "__fish_pouf_using_subcommand help; and not __fish_seen_subcommand_from template address.city address.country address.street administrative.healthinsurrancecode auto.licenseplate barcode.isbn color filesystem.mimetype filesystem.semver finance.bic http.code internet.ip internet.mac internet.mail internet.useragent lorem.word people.name time.time time.date help" -f -a "lorem.word" -d 'give a fake word (in Latin)'
complete -c pouf -n "__fish_pouf_using_subcommand help; and not __fish_seen_subcommand_from template address.city address.country address.street administrative.healthinsurrancecode auto.licenseplate barcode.isbn color filesystem.mimetype filesystem.semver finance.bic http.code internet.ip internet.mac internet.mail internet.useragent lorem.word people.name time.time time.date help" -f -a "people.name" -d 'give a fake name'
complete -c pouf -n "__fish_pouf_using_subcommand help; and not __fish_seen_subcommand_from template address.city address.country address.street administrative.healthinsurrancecode auto.licenseplate barcode.isbn color filesystem.mimetype filesystem.semver finance.bic http.code internet.ip internet.mac internet.mail internet.useragent lorem.word people.name time.time time.date help" -f -a "time.time" -d 'give a fake time'
complete -c pouf -n "__fish_pouf_using_subcommand help; and not __fish_seen_subcommand_from template address.city address.country address.street administrative.healthinsurrancecode auto.licenseplate barcode.isbn color filesystem.mimetype filesystem.semver finance.bic http.code internet.ip internet.mac internet.mail internet.useragent lorem.word people.name time.time time.date help" -f -a "time.date" -d 'give a fake date'
complete -c pouf -n "__fish_pouf_using_subcommand help; and not __fish_seen_subcommand_from template address.city address.country address.street administrative.healthinsurrancecode auto.licenseplate barcode.isbn color filesystem.mimetype filesystem.semver finance.bic http.code internet.ip internet.mac internet.mail internet.useragent lorem.word people.name time.time time.date help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
