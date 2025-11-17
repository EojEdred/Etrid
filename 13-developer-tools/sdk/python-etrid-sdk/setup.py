"""
Ëtrid Python SDK - Setup Configuration
"""

from setuptools import setup, find_packages

with open("README.md", "r", encoding="utf-8") as fh:
    long_description = fh.read()

setup(
    name="etrid-sdk",
    version="0.1.0",
    author="Ëtrid Foundation",
    author_email="dev@etrid.io",
    description="Python SDK for Ëtrid Protocol blockchain",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/etrid/etrid-protocol",
    packages=find_packages(),
    classifiers=[
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Topic :: Software Development :: Libraries :: Python Modules",
        "License :: OSI Approved :: Apache Software License",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
    ],
    python_requires=">=3.8",
    install_requires=[
        "substrate-interface>=1.7.0",
        "py-scale-codec>=1.2.0",
        "scalecodec>=0.11.0",
        "websocket-client>=1.5.0",
        "requests>=2.31.0",
    ],
    extras_require={
        "dev": [
            "pytest>=7.4.0",
            "pytest-asyncio>=0.21.0",
            "pytest-cov>=4.1.0",
            "black>=23.7.0",
            "pylint>=2.17.0",
            "mypy>=1.4.0",
        ],
    },
)
