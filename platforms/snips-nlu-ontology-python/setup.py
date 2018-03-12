from __future__ import print_function

import io
import os
import sys

from setuptools import setup, find_packages
from setuptools_rust import Binding, RustExtension

packages = [p for p in find_packages() if "tests" not in p]

PACKAGE_NAME = "snips_nlu_ontology"
ROOT_PATH = os.path.dirname(os.path.abspath(__file__))
PACKAGE_PATH = os.path.join(ROOT_PATH, PACKAGE_NAME)
README = os.path.join(ROOT_PATH, "README.rst")
VERSION = "__version__"

RUST_EXTENSION_NAME = 'snips_nlu_ontology.dylib.libsnips_nlu_ontology_rs'
CARGO_ROOT_PATH = os.path.join(ROOT_PATH, 'snips-nlu-ontology-rs')
CARGO_FILE_PATH = os.path.join(CARGO_ROOT_PATH, 'Cargo.toml')
CARGO_TARGET_DIR = os.path.join(CARGO_ROOT_PATH, 'target')
os.environ['CARGO_TARGET_DIR'] = CARGO_TARGET_DIR

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
      rust_extensions=[RustExtension(RUST_EXTENSION_NAME, CARGO_FILE_PATH,
                                     debug="develop" in sys.argv,
                                     binding=Binding.NoBinding)],
      install_requires=required,
      packages=packages,
      include_package_data=True,
      zip_safe=False)
