# Trunk

As developers, it's very rare that we're working on a single project with a known, memorable set of tools. Some of our projects might be using PHPStan, others might be using Psalm. We might have older projects running PHP-CS-Fixer, but our newer ones are running Pint.

What if it didn't matter anymore? What if you could instead use a single command and have it work for all of your projects? Introducing **Trunk**.

Trunk provides a set of commands that handle the difficult things for you. You can use the same set of commands on all of your projects, regardless of the tools being used.

## Installation

**Composer (recommended)**

Install Trunk using Composer:

```
composer global require pxp/trunk
```

Installing this package will ask for a plugin named `pact-foundation/composer-downloads-plugin` is allowed to execute code. Answer **`y`** to this as it is responsible for downloading the appropriate binaries for your system.

Once installed, consult the [Commands](#commands) section below and start using Trunk!

## Commands

### `info`

The `info` command outputs a list of information about the current project. It is capable of telling you the following things:
* PHP version.
* Laravel/Symfony/WordPress version.
* Installed Composer packages w/ absolute version.
* Environment variables loaded and resolved from `.env` (if present).

```sh
trunk info
```

### `fmt`

The `fmt` command makes it easy to format PHP code in your project. It currently has support for the following formatters:
* [`php-cs-fixer`](https://github.com/PHP-CS-Fixer/PHP-CS-Fixer)
* [`pint`](https://laravel.com/docs/11.x/pint)

To format your entire project (or files configured for your editor), run the following command.

```sh
trunk fmt
```

If you wish to format a specific file or directory, you can pass the path as an argument to the command and it will be passed through to the underlying formatter (where appropriate).

```sh
trunk fmt ./src/Container/Container.php
```

If you want to perform a dry-run, you can pass the `--dry-run` (`-d`) flag.

```sh
trunk fmt --dry-run
```

### `check`

The `check` command is used to perform static analysis. It currently supports the following static analysers:
* [`phpstan`](https://phpstan.org/)
* [`psalm`](https://psalm.dev/)
* [`phan`](https://github.com/phan/phan)

To run the static analyser on your entire project, use the following command:

```sh
trunk check
```

If you wish to statically analyse a single file, use the following command:

```sh
trunk check ./src/Container/Container.php
```

Some projects might choose to use multiple static analysers for different reasons. If that is the case on your project, you can pass a comma-separated list of tools to run with the `--using` (`-u`) option. They will be executed in the order provided.

```sh
trunk check --using=phpstan,psalm
```

### `test`

The `test` command is used to execute any form of test. It currently supports the following test runners:
* [`phpunit`](https://phpunit.de/index.html)
* [`pest`](https://pestphp.com/)

To run your tests, use the following command:

```sh
trunk test
```

If you wish to execute a particular test, or directory of tests, use the following command:

```sh
trunk test ./path/to/Test.php
```

In certain scenarios, you might want to pass additional arguments to the underlying command. You can do this by placing additional arguments after a `--` symbol. All additional arguments will be forwarded to the underlying test runner.

```sh
trunk test -- --parallel
```