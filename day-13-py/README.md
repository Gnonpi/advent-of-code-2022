# day-13-py

Your awesome project

## Installation

Install the dependencies with [Poetry](https://python-poetry.org/):
```commandline
$ poetry install --no-dev
```

## Running tests

Install the dev dependencies with:
```commandline
$ poetry install
```

Run the tests with [pytest]():
```commandline
$ poetry run pytest
```

### Hooks

Git hooks were used to fix our code.
To install and run them, you can do:
```commandline
$ poetry run pre-commit install
$ poetry run pre-commit run --all-files
```