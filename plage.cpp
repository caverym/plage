#include <cstdio>
#include <string>
#include <cstring>
#include <unistd.h>
#include <sys/wait.h>
using namespace std;

int clone_aur(int argc, char *argv[])
{
	string url = "https://aur.archlinux.org/";
	string name = argv[2];
	string full = url + name + ".git";
	char command[100];
	strcpy(command, full.c_str());

	pid_t git;
	if (fork() == 0)
		execl("/usr/bin/git", "git", "clone", command, NULL);
	
	git = wait(NULL);
	return 0;
}

int move_in(char *name)
{
	chdir(name);
	return 0;
}

int make_the_package(char keys[10])
{
	pid_t makepkg;
	if (fork() == 0)
		execl("/usr/bin/makepkg", "makepkg", keys, NULL);

	makepkg = wait(NULL);
	return 0;
}

int main(int argc, char *argv[])
{
	if (argc == 1)
		return 1;

	clone_aur(argc, argv);
	move_in(argv[2]);
	make_the_package(argv[1]);

	return 0;
}
