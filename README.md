# Trunk

Trunk is a command-line application that makes it silly-simple to manage and develop PHP applications.

## Commands

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