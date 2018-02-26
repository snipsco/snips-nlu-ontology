from __future__ import print_function

import io
import os
import sys

from setuptools import setup, find_packages
from setuptools.dist import Distribution
from wheel.bdist_wheel import bdist_wheel

from rust_build import build_rust_cmdclass, RustInstallLib

script_args = [i for i in sys.argv[1:] if "--debug" not in i
               and "--use-workspace-build" not in i]
debug = "--debug" in sys.argv
use_workspace_build = "--use-workspace-build" in sys.argv


class RustDistribution(Distribution):
    def __init__(self, *attrs):
        Distribution.__init__(self, *attrs)
        self.cmdclass["install_lib"] = RustInstallLib
        self.cmdclass["bdist_wheel"] = RustBdistWheel
        self.cmdclass["build_rust"] = build_rust_cmdclass(
            debug=debug, use_workspace_build=use_workspace_build)

        print("Building in {} mode".format("debug" if debug else "release"))
        if use_workspace_build:
            print("Using workspace build")


class RustBdistWheel(bdist_wheel):
    def finalize_options(self):
        bdist_wheel.finalize_options(self)
        self.root_is_pure = False


packages = [p for p in find_packages() if
            "tests" not in p and "debug" not in p]

PACKAGE_NAME = "snips_nlu_ontology"
ROOT_PATH = os.path.dirname(os.path.abspath(__file__))
PACKAGE_PATH = os.path.join(ROOT_PATH, PACKAGE_NAME)
README = os.path.join(ROOT_PATH, "README.rst")
VERSION = "__version__"

with io.open(os.path.join(PACKAGE_PATH, VERSION)) as f:
    version = f.readline()

with io.open(README, "rt", encoding="utf8") as f:
    readme = f.read()

required = [
    "future==0.16.0",
    "wheel==0.30.0"
]

setup(name=PACKAGE_NAME,
      description="Python wrapper of the Snips NLU ontology",
      long_description=readme,
      version=version,
      license="Apache 2.0",
      author="Kevin Lefevre, Adrien Ball",
      author_email="kevin.lefevre@snips.ai, adrien.ball@snips.ai",
      classifiers=[
          "Programming Language :: Python :: 2",
          "Programming Language :: Python :: 2.7",
          "Programming Language :: Python :: 3",
          "Programming Language :: Python :: 3.4",
          "Programming Language :: Python :: 3.5",
          "Programming Language :: Python :: 3.6",
      ],
      install_requires=required,
      packages=packages,
      include_package_data=True,
      distclass=RustDistribution,
      zip_safe=False,
      script_args=script_args)
