#!/usr/bin/env python3

import os
import importlib

# Enumerate runners
paths = [file.split(".")[0] for file in os.listdir("runners") if not file.startswith("__")]

for runner in paths: 
    print(runner)
    importlib.import_module(f"..{runner}", 'runners.subpkg')

    test = 