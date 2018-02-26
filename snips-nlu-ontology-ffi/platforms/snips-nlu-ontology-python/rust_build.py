# Largely inspired from https://github.com/novocaine/rust-python-ext
# MIT LICENCE - Copyright (c) 2015 James Salter


from __future__ import print_function

import glob
import os
import os.path
import shutil
import subprocess
import sys
from distutils.cmd import Command
from distutils.command.install_lib import install_lib

RUST_CRATE_NAME = "snips-nlu-ontology-rs"
ROOT_PATH = os.path.dirname(os.path.abspath(__file__))


class RustBuildCommand(Command):
    """Command for building rust crates via cargo.

    Don't use this directly; use the build_rust_cmdclass factory method.
    """

    description = "build rust crates into Python extensions"

    user_options = []

    def _unpack_classargs(self):
        for k, v in self.__class__.args.items():
            setattr(self, k, v)

    def initialize_options(self):
        self._unpack_classargs()

    def finalize_options(self):
        pass

    def run(self):
        # Execute cargo.
        try:
            cargo_root_path = os.path.join(ROOT_PATH, RUST_CRATE_NAME)
            if self.use_workspace_build:
                cargo_root_path = os.path.join(cargo_root_path, "../../../..")

            target_tuple = os.environ.get("CARGO_TARGET")
            os.environ["CARGO_TARGET_DIR"] = os.path.join(cargo_root_path,
                                                          "target")
            # TODO Switch to make build-<TARGET>
            args = (["cargo", "build"] + list(self.extra_cargo_args or []))
            if not self.debug:
                args.append("--release")
            if target_tuple:
                args.append("--target=%s" % target_tuple)
            if not self.quiet:
                print(" ".join(args), file=sys.stderr)
            output = subprocess.check_output(args, cwd=cargo_root_path)
        except subprocess.CalledProcessError as e:
            msg = "cargo failed with code: %d\n%s" % (e.returncode, e.output)
            raise Exception(msg)
        except OSError:
            raise Exception("Unable to execute 'cargo' - this package "
                            "requires rust to be installed and cargo to be on "
                            "the PATH")

        if not self.quiet:
            print(output, file=sys.stderr)

        # Find the shared library that cargo hopefully produced and copy
        # it into the build directory as if it were produced by build_cext.
        if self.debug:
            suffix = "debug"
        else:
            suffix = "release"

        if target_tuple:
            target_dir = os.path.join(cargo_root_path, "target", target_tuple,
                                      suffix, "deps")
        else:
            target_dir = os.path.join(cargo_root_path, "target", suffix,
                                      "deps")

        if sys.platform == "win32":
            wildcard_so = "*snips_nlu_ontology_ffi*.dll"
        elif sys.platform == "darwin":
            wildcard_so = "*snips_nlu_ontology_ffi*.dylib"
        else:
            wildcard_so = "*snips_nlu_ontology_ffi*.so"

        try:
            dylib_path = glob.glob(os.path.join(target_dir, wildcard_so))[0]
        except IndexError:
            raise Exception("rust build failed; unable to find any .dylib in "
                            "%s" % target_dir)
        package_name = self.distribution.metadata.name

        dylib_resource_path = os.path.join(ROOT_PATH, package_name, "dylib",
                                           os.path.basename(dylib_path))
        print("from: ", dylib_path)
        print("to: ", dylib_resource_path)
        shutil.copyfile(dylib_path, dylib_resource_path)


def build_rust_cmdclass(debug=False, use_workspace_build=False,
                        extra_cargo_args=None, quiet=False):
    """
    Build a Command subclass suitable for passing to the cmdclass argument of
    distutils.

    :param debug: if True --debug is passed to cargo, otherwise --release
    :param use_workspace_build: if True, will use snips-nlu-ontology workspace
        target directory instead of local target directory
    :param extra_cargo_args: list of extra arguments to be passed to cargo
    :param quiet: If True, doesn't echo cargo's output
    """

    # Manufacture a once-off command class here and set the params on it as
    # class members, which it can retrieve later in initialize_options.
    # This is clumsy, but distutils doesn't give you an appropriate
    # hook for passing params to custom command classes (and it does the
    # instantiation).

    _args = locals()

    class RustBuildCommandImpl(RustBuildCommand):
        args = _args

    return RustBuildCommandImpl


class RustInstallLib(install_lib):
    """
    A replacement install_lib cmdclass that remembers to build_rust
    during install_lib.

    Typically you want to use this so that the usual 'setup.py install'
    just works.
    """

    def build(self):
        install_lib.build(self)
        if not self.skip_build:
            self.run_command('build_rust')
