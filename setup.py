import os.path as op

from setuptools import find_packages, setup


def _read_md_as_rst(file):
    """Read MarkDown file and convert it to ReStructuredText."""
    from pypandoc import convert
    return convert(file, 'rst')


def _read_md_as_md(file):
    """Read MarkDown file."""
    with open(op.join(op.dirname(__file__), file)) as ifh:
        return ifh.read()


def read_md(file):
    """Read MarkDown file and try to convert it to ReStructuredText if you can."""
    try:
        return _read_md_as_rst(file)
    except ImportError:
        print("WARNING: pypandoc module not found, could not convert Markdown to RST!")
        return _read_md_as_md(file)


setup(
    name='biorustlings',
    version='0.0.1',
    author='kimlab.org',
    author_email='alex.strokach@utoronto.ca',
    url="https://gitlab.com/kimlab/biorustlings",
    description="Routines for parsing biological data, written in Rust.",
    long_description=read_md("README.md"),
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python :: 3 :: Only",
        "Topic :: Scientific/Engineering :: Bio-Informatics",
    ],
    license='MIT',
    packages=['biorustlings.' + x for x in find_packages('kmtools')],
    package_data={})
