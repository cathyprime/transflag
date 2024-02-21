all: pack

pack: transflag
	upx --brute transflag

transflag:
	go build -ldflags="-s -w" -o transflag main.go

install: pack
	go install transflag

clean:
	rm transflag

.PHONY: pack install
