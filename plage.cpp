#include <cstdio>
#include <string>
#include <cstring>
#include <unistd.h>
#include <sys/stat.h>
#include <sys/wait.h>
using namespace std;

#define RED "\e[1;91m"
#define CYN "\e[1;96m"
#define reset "\e[0m"

int print_error(const char *message)
{
	return fprintf(stderr, RED "Error:" reset " %s\n", message);
}

int print_message(const char *message)
{
	return fprintf(stdout, CYN "Plage:" reset " %s\n", message);
}

int to_cache()
{
	string home = getenv("HOME");
	string cache = home + "/.cache/plage";
	char char_cache[100];
	strcpy(char_cache, cache.c_str());

	mkdir(char_cache, 755);
	int i = chdir(char_cache);
	if (i != 0) {
		print_error("Unable to make or change to `~/.cache/plage` directory");
		exit(1);
	}

	print_message("Entered cache directory");
	return 0;
}

int clone_aur(char **argv)
{
	string url = "https://aur.archlinux.org/";
	string name = argv[2];
	string full = url + name + ".git";
	char command[100];
	strcpy(command, full.c_str());

	print_message("executing git");
	if (fork() == 0)
		execl("/usr/bin/git", "git", "clone", command, NULL);

	wait(nullptr);
	return 0;
}

int move_in(char *name)
{
	chdir(name);
	return 0;
}

int make_the_package(char keys[10])
{
	print_message("executing makepkg");
	if (fork() == 0)
		execl("/usr/bin/makepkg", "makepkg", keys, NULL);

	wait(nullptr);
	return 0;
}

int parse_the_args(const string& arg1, const string& arg2)
{
	if (arg1.length() > 10) {
		print_error("first argument too long");
		exit(1);
	}

	return 0;
}

int main(int argc, char *argv[])
{
	if (argc == 1) {
		print_error("not enough arguments");
		exit(1);
	}

	parse_the_args(argv[1], argv[2]);
	to_cache();
	clone_aur(&*argv);
	move_in(argv[2]);
	make_the_package(argv[1]);

	return 0;
}
