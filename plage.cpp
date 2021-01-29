#include <cstdio>
#include <string>
#include <cstring>
#include <unistd.h>
#include <sys/stat.h>
#include <sys/wait.h>
using namespace std;

#define RED "\e[1;91m"
#define CYN "\e[1;96m"
#define YEL "\e[1;93m"
#define reset "\e[0m"

#define MSG 0
#define ERR 1
#define WARN 2

// TYPE 0 is message, 1 is error, 2 is warning
int print_message(const char *message, int type)
{

	switch (type) {
	case MSG:
		return fprintf(stdout, CYN "Plage:" reset " %s\n", message);
	case ERR:
		return fprintf(stderr, RED "Error:" reset " %s\n", message);
	case WARN:
		return fprintf(stderr, YEL "Warning:" reset " %s\n", message);
	}
	return 1;
}

inline bool pkg_check(const string& name)
{
	struct stat buffer;
	return (stat(name.c_str(), &buffer) == 0);
}

int to_cache()
{
	string home = getenv("HOME");
	string cache = home + "/.cache/plage";
	char char_cache[100];
	strcpy(char_cache, cache.c_str());

	int i = chdir(char_cache);
	if (i != 0) {
		print_message("making cache directory", MSG);
		mkdir(char_cache, 755);
		chdir(char_cache);
	}

	print_message("entered cache directory", MSG);
	return 0;
}

int clone_aur(char **argv)
{
	string url = "https://aur.archlinux.org/";
	string name = argv[2];
	string full = url + name + ".git";
	char command[100];
	strcpy(command, full.c_str());
	int status;

	print_message("executing git", MSG);
	if (fork() == 0)
		execl("/usr/bin/git", "git", "clone", command, NULL);

	wait(&status);

	if (status != 0)
		print_message("Git exited with an error", WARN);

	return 0;
}

int move_in(char *name)
{
	int i = chdir(name);
	if (i != 0) {
		print_message("can not change into AUR directory", ERR);
		exit(1);
	}
	return 0;
}

int make_the_package(char keys[10], const string& name)
{
	if (!pkg_check(name)) {
		print_message("failed to find PKGBUILD", ERR);
		exit(1);
	}

	print_message("executing makepkg", 0);
	if (fork() == 0)
		execl("/usr/bin/makepkg", "makepkg", keys, NULL);

	wait(nullptr);
	return 0;
}

int parse_the_args(const string& arg1, const string& arg2)
{
	if (arg1.length() > 10) {
		print_message("first argument invalid", ERR);
		exit(1);
	}

	return 0;
}

int main(int argc, char *argv[])
{
	if (argc == 1) {
		print_message("not enough arguments", ERR);
		exit(1);
	}
	if (argc > 3) {
		print_message("too many arguments", ERR);
		exit(1);
	}
	if (argv[2] == nullptr) {
		print_message("not enough arguments", ERR);
		exit(1);
	}

	parse_the_args(argv[1], argv[2]);
	to_cache();
	clone_aur(&*argv);
	move_in(argv[2]);
	make_the_package(argv[1], "PKGBUILD");

	print_message("exiting", MSG);

	return 0;
}
