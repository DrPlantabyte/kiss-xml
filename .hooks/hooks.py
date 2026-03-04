#!/usr/bin/env python3
"""script for installing and running hg/git hooks"""

import argparse
import enum
import platform
import re
import shlex
import subprocess
import sys
from pathlib import Path
from typing import Iterable


class OsType(enum.Enum):
	"""Enum of OSes"""

	WINDOWS = 'windows'
	MAC = 'mac'
	LINUX = 'linux'


HOOKS = {'precommit': lambda os_type, files: run_precommit_hooks(os_type, files)}


def main(argsv: list[str] | None = None):
	"""Run this script as a CLI app via ArgParse"""
	# CLI
	arg_parser = argparse.ArgumentParser(description='Install or run repository hooks')
	arg_parser.add_argument('--install', action='store_true', help='install hooks into this Hg/Git repo')
	arg_parser.add_argument('--all', '-a', action='store_true', help='run hooks on all files')
	arg_parser.add_argument(
		'--modified',
		'-m',
		action='store_true',
		help='run hooks on added and modified files (as per `hg status -ma` or `git diff --name-only --diff-filter=AM HEAD`)',
	)
	arg_parser.add_argument('--hook', '-x', choices=HOOKS.keys(), default=None, type=str, help='which hook to run')
	arg_parser.add_argument('files', type=Path, nargs='*')
	args = arg_parser.parse_args(argsv)
	# identify the repo
	os_type = get_os_type()
	hg_repo = is_hg_repo()
	git_repo = is_git_repo()
	# install, if requested
	if args.install:
		err('Installing format and linter pre-commit hooks...')
		if not (hg_repo or git_repo):
			err('ERROR: This does not appear to be a Git nor Hg repository. Cannot install hooks.')
			exit(1)
		if hg_repo:
			install_hg_hooks(os_type)
		if git_repo:
			install_git_hooks(os_type)
		err('...Done')
	# run requested hook
	if args.hook is not None:
		root_dir = hg_repo_root() if hg_repo else git_repo_root()
		files = set(args.files)
		if args.all:
			files.update(root_dir.rglob('*'))
		if args.modified:
			if hg_repo:
				files.update(hg_modified_files())
			if git_repo:
				files.update(git_modified_files())
		print_list = '\n\t'.join([str(p) for p in files])
		err(f'Running {args.hook} hooks on files...\n\t{print_list}')
		hook_fn = HOOKS[args.hook]
		success = hook_fn(os_type, expand_dirs(files))
		if success:
			err('...SUCCESS!')
		else:
			err('...FAILURE!')
			err(f'One or more {args.hook} hooks failed.')
			exit(1)
	# done
	exit(0)


def err(*args, **wkargs):
	"""print to stderr"""
	print(*args, **wkargs, file=sys.stderr)


def expand_dirs(files: Iterable[Path]) -> set[Path]:
	"""expand directories in file list to files, returning a flattened set of files"""
	output = set()
	for f in files:
		if f.is_dir():
			output.update(f.rglob('*'))
		else:
			output.add(f)
	return output


def get_os_type() -> OsType:
	"""Gets the type of OS"""
	if platform.system() == 'Windows':
		return OsType.WINDOWS
	if platform.system() == 'Linux':
		return OsType.LINUX
	if platform.system() == 'Darwin':
		return OsType.MAC
	raise NotImplementedError(f'Unsupported OS platform: {platform.system()}')


def is_hg_repo() -> bool:
	"""returns True if CWD is in a hg repo, False otherwise"""
	try:
		_ = hg_repo_root()
		return True
	except subprocess.CalledProcessError:
		return False


def is_git_repo() -> bool:
	"""returns True if CWD is in a git repo, False otherwise"""
	try:
		_ = git_repo_root()
		return True
	except subprocess.CalledProcessError:
		return False


def run_precommit_hooks(os_type: OsType, files: Iterable[Path]) -> bool:
	"""Runs all of the pre-commit hooks on the provided files and returns True if all passed, otherwise False"""
	shell_prefix = (
		['powershell', '-NoProfile', '-ExecutionPolicy', 'Bypass', '-Command'] if os_type == OsType.WINDOWS else []
	)
	hooks_commands_filters = [
		('rust format (Rust)', ['rustfmt', '--check'], ['rustfmt'], r'^.*\.rs$'),
	]
	hook_results: dict[str, bool] = {}
	for hook_name, cmd, on_fail_cmd, regex_filter in hooks_commands_filters:
		file_list = [p for p in files if re.match(regex_filter, p.name)]
		err(f'Running {hook_name} on {len(file_list)} files...')
		if len(file_list) == 0:
			err('\t...skipped')
		else:
			err(f'\t{shlex.join(cmd)} @FILES')
			result = subprocess.run(
				cmd + [str(p) for p in file_list],
				stdout=sys.stderr,
				stderr=sys.stderr,
				text=True,
				check=False,
			)
			success = result.returncode == 0
			hook_results[hook_name] = success
			if not success and on_fail_cmd is not None:
				err(f'\t{shlex.join(on_fail_cmd)} @FILES')
				_ = subprocess.run(
					on_fail_cmd + [str(p) for p in file_list],
					stdout=sys.stderr,
					stderr=sys.stderr,
					text=True,
					check=False,
				)
	for hook_name, _, _, _ in hooks_commands_filters:
		if hook_name not in hook_results:
			result_display = 'SKIP'
		else:
			result_display = 'PASS' if hook_results[hook_name] else 'FAIL'
		err(f'{hook_name}{"." * (40 - len(hook_name))}{result_display}')
	return all(hook_results.values())


def hg_repo_root(cwd: Path | None = None) -> Path:
	"""
	Return the absolute path to the Mercurial repository root.

	Raises `FileNotFoundError` if `hg` is not installed and `subprocess.CalledProcessError` if the current working
	directory is not in a Mercurial repo
	"""
	result = subprocess.run(
		['hg', 'root'],
		cwd=str(cwd) if cwd else None,
		stdout=subprocess.PIPE,
		stderr=subprocess.PIPE,
		text=True,
		check=True,
	)
	root = Path(result.stdout.strip())
	return root.resolve()


def git_repo_root(cwd: Path | None = None) -> Path:
	"""
	Return the absolute path to the Git repository root.

	Raises `FileNotFoundError` if `git` is not installed and `subprocess.CalledProcessError` if the current working
	directory is not in a Git repo
	"""
	result = subprocess.run(
		['git', 'rev-parse', '--show-toplevel'],
		cwd=str(cwd) if cwd else None,
		stdout=subprocess.PIPE,
		stderr=subprocess.PIPE,
		text=True,
		check=True,
	)
	root = Path(result.stdout.strip())
	return root.resolve()


def install_hg_hooks(os_type: OsType):
	"""install hooks for hg"""
	hg_dir = hg_repo_root()
	# read the hgrc file, if it exists
	hgrc_file = hg_dir / '.hg' / 'hgrc'
	hooks_dir = hg_dir / '.hooks'
	precommit_script_path = hooks_dir / 'hooks.py'
	if hgrc_file.exists():
		hgrc_content = hgrc_file.read_text()
	else:
		hgrc_content = ''
	# find the [hooks] section, if it exists
	hook_matches = list(re.finditer(r'^\s*\[\s*hooks\s*\]', hgrc_content))
	if len(hook_matches) > 0:
		insert_index = hook_matches[-1].end(0)
	else:
		hgrc_content = hgrc_content + '\n[hooks]'
		insert_index = len(hgrc_content)
	# figure out which command to use
	if os_type == OsType.WINDOWS:
		command = f'python.exe {str(precommit_script_path.relative_to(hg_dir))} --modified --hook=precommit'
	else:
		command = f'python3 {str(precommit_script_path.relative_to(hg_dir))} --modified --hook=precommit'
	# add the hook entry, if it is not already there
	hook_entry = f'\nprecommit.format-and-lint = {command}'
	changed = False
	if hook_entry not in hgrc_content:
		err(f"installing hook '{hook_entry}'")
		hgrc_content = hgrc_content[:insert_index] + hook_entry + '\n' + hgrc_content[insert_index:]
		changed = True
	else:
		err(f"hook '{hook_entry}' already installed")
	if changed:
		err(f'Writing precommit hooks to {str(hgrc_file)}...')
		err('```')
		err(f'[hooks]\n{hook_entry}')
		err('```')
		hgrc_file.write_text(hgrc_content)
	# set execute bit
	if os_type != OsType.WINDOWS:
		precommit_script_path.chmod(mode=0o755)


def install_git_hooks(os_type: OsType):
	"""install hooks for git"""
	# Frickin' Git! Your hooks support on Windows is so dodgy!
	# hook files are required to start with a #! to determine which shell to use, but cannot find windows shells
	# so on windows, one must use #!/bin/sh to bypass its attempt to search for a shell
	# but then the shell it uses is whatever posix-wrapper Git was compiled with (usually cygwin),
	# so it might not even be a fully featured shell and you have no control over what you get.
	# Therefore the only plausible thing to do is make the hook file a one-line command that works in any and all
	# Windows and posix-wrapper shells (eg cygwin), eg `python.exe actual-hooks-as-python-script`
	git_dir = git_repo_root()
	repo_hooks_dir = git_dir / '.hooks'
	git_hook_dir = git_dir / '.git' / 'hooks'
	precommit_script_path = repo_hooks_dir / 'hooks.py'
	precommit_hook_path = git_hook_dir / 'pre-commit'
	if os_type == OsType.WINDOWS:
		git_precommit_content = """#!/bin/sh
python.exe .hooks/hooks.py --modified --hook=precommit
"""
	else:
		git_precommit_content = """#!/bin/sh
python3 .hooks/hooks.py --modified --hook=precommit
"""
	err(f'Writing precommit hooks to {str(precommit_hook_path)}...')
	err('```')
	err(git_precommit_content)
	err('```')
	precommit_hook_path.write_text(git_precommit_content, encoding='utf-8')
	# set execute bit
	if os_type != OsType.WINDOWS:
		precommit_hook_path.chmod(mode=0o755)
		precommit_script_path.chmod(mode=0o755)


def hg_modified_files(cwd: Path | None = None) -> list[Path]:
	"""returns a list of added and modified files tracked by hg"""
	result = subprocess.run(
		['hg', 'status', '-am'],
		cwd=str(cwd) if cwd else None,
		stdout=subprocess.PIPE,
		stderr=sys.stderr,
		text=True,
		check=True,
	)
	return [Path(ln[2:].rstrip('\r')) for ln in result.stdout.strip().split('\n')]


def git_modified_files(cwd: Path | None = None) -> list[Path]:
	"""returns a list of added and modified files tracked by git"""
	result = subprocess.run(
		['git', 'diff', '--name-only', '--diff-filter=AM', 'HEAD'],
		cwd=str(cwd) if cwd else None,
		stdout=subprocess.PIPE,
		stderr=sys.stderr,
		text=True,
		check=True,
	)
	return [Path(ln.rstrip('\r')) for ln in result.stdout.strip().split('\n')]


if __name__ == '__main__':
	main()
