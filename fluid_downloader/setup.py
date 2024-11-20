from setuptools import setup, find_packages

setup(
    name='fluid_downloader',
    version='0.1.0',
    packages=find_packages(),
    install_requires=[
        'selenium>=4.0.0',
        'python-dotenv>=0.15.0',
        'pandas>=2.2.3', 
        'openpyxl>=3.0.0',
    ],
    entry_points={
        'console_scripts': [
            'fluid_downloader=fluid_downloader:main',
        ],
    },
    author='Your Name',
    author_email='your_email@example.com',
    description='A script to download data from a secured website using Selenium',
    python_requires='>=3.12',
)
