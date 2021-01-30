#include <cstdio>
#include <string>
#include <cstring>
#include <unistd.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <argp.h>
using namespace std;

#define RED "\e[1;91m"
#define CYN "\e[1;96m"
#define YEL "\e[1;93m"
#define reset "\e[0m"

#define MSG 0
#define ERR 1
#define WARN 2

const char *argp_program_version = "version 0.1";
const char *argp_program_bug_address = "report bugs to Avery <averylapine@gmail.com>";

struct plage {
    bool inside_package;
}; struct plage la_plage;

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

void enter_cache()
{
	string home = getenv("HOME");
	string str_cache = home + "/.cache/plage";
	char cache[100];
	strcpy(cache, str_cache.c_str());

	int i = chdir(cache);
	if (i != 0) {
		print_message("failed to enter cache", WARN);
		print_message("creating cache", MSG);
		if (mkdir(cache, 755) != 0) {
			print_message("failed to create cache directory", ERR);
			exit(1);
		}
		chdir(cache);
	}
}

int change_directory(char *directory)
{
	if (la_plage.inside_package)
		return 0;

	int i = chdir(directory);
	if (i != 0) {
		print_message("can not change into package directory", ERR);
		exit(1);
	}

	la_plage.inside_package = true;
	return 0;
}

int clone_package(const string& package)
{
	string url = "https://aur.archlinux.org/" + package + ".git";
	char command[256];
	strcpy(command, url.c_str());
	int status;

	print_message("executing git", MSG);
	if (fork() == 0)
		execl("/usr/bin/git", "git", "clone", command, NULL);

	wait(&status);
	if (status != 0)
		print_message("Git exited with an error", WARN);

	return 0;
}

int makepkg_exec(const string& package, const char *flags)
{
	if (!pkg_check("PKGBUILD")) {
		print_message("PKGBUILD not found", ERR);
		exit(1);
	}

	int status;

	print_message("executing makepkg", MSG);
	if (fork() == 0)
		execl("/usr/bin/makepkg", "makepkg", flags, NULL);

	wait(&status);
	if (status != 0) {
		print_message("makepkg error", ERR);
		exit(1);
	}
	return 0;
}

int remove_package(const string& package)
{
	if (geteuid() != 0) {
		print_message("you must be root", ERR);
		exit(1);
	}

	char charpkg[50];
	strcpy(charpkg, package.c_str());

	int status;
	if (fork() == 0) {
		execl("/usr/bin/pacman", "pacman", "-R", charpkg, NULL);
	}

	wait(&status);
	if (status != 0) {
		print_message("pacman exited with an error", ERR);
		exit(1);
	}
	return 0;
}

static int parse_opt(int key, char *arg, struct argp_state *state)
{
	switch (key) {
		case 'd':
			clone_package(arg);
			break;
		case 'b':
			change_directory(arg);
			makepkg_exec(arg, "-s");
			break;
		case 'i':
			change_directory(arg);
			makepkg_exec(arg, "-i");
			break;
		case 'r':
			remove_package(arg);
			break;
		case 'q':
			clone_package(arg);
			change_directory(arg);
			makepkg_exec(arg, "-si");
			break;
	}
	return 0;
}

int main(int argc, char **argv)
{
	if (argc == 1) {
		print_message("not enough arguments", ERR);
		return 1;
	}

	enter_cache();

	int ret;

	struct argp_option options[] =
		{
			{"download", 'd', "package", 0, "Download a package"},
			{"build", 'b', "package", 0, "Build a package"},
			{"install", 'i', "package", 0, "Install a package"},
			{"remove", 'r', "package", 0, "Remove a package"},
			{"quick", 'q', "package", 0, "Download, build and install a package"},
			{0}
		};

	struct argp argp = {options, parse_opt};
	ret = argp_parse(&argp, argc, argv, 0, 0, 0);

	return ret;
}
