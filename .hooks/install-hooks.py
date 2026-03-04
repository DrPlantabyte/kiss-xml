#!/usr/bin/env python3
"""script for installing hg/git hooks"""

import os
import pathlib

os.environ['PYTHONPATH'] = str(pathlib.Path(__file__).parent)
import hooks

hooks.main(['--install'])
