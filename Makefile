build:
	g++ -o plage plage.cpp

install:
	install -Dm755 plage /usr/bin/plage

clean:
	rm plage
