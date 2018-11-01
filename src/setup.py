'''Support for python distutils. Allows installation using `python setup.py install`.'''

import sys
import os
import subprocess

from setuptools import setup, Distribution
from setuptools.command.build_py import build_py


class RustBuildCommand(build_py):
    """Custom build command."""

    def run(self):
        command = ['cargo', 'build', '--release']
        if not sys.stdout.isatty():
            command.append('--color=always')
        cli_dir = os.path.abspath(os.path.dirname(__file__))
        return_code = subprocess.Popen(command, cwd=cli_dir).wait()
        if return_code != 0:
            sys.exit(return_code)
        build_py.run(self)


class BinaryDistribution(Distribution):
    '''
    A hack to override wheel's autodetection for binary content.
    '''
    def has_ext_modules(self):
        return True


setup(
    cmdclass={
        'build_ext': RustBuildCommand
    },
    name='{{{name}}}',
    version='{{{version}}}',
    url = '{{{url}}}',
    author = '{{{author}}}',
    description = '{{{description}}}',
    distclass=BinaryDistribution,
    zip_safe=False,
    setup_requires=['wheel'],
    data_files=[('bin', ['target/release/{{{name}}}{{{executable_file_ending}}}'])]
)