# Another Backup Tool

I'm starting my adventure into the Rust Lang world, so here's a simple command line backup tool to practice Rust coding.

## TODO
- [x] config action - Basic config functionality.
- [ ] backup action - Basic backup functionality.
- [ ] recover action - Basic recover functionality.
- [ ] at least one additional remote storage(ftp or cloud) as an alternative to local storage.

## Print help
```bash
That is another backup tool

Usage: abt [OPTIONS] <COMMAND>

Commands:
  config   Create/Update config file
  backup   Copy files from the source directory to the backup directory
  restore  Restore files from the backup directory to the source directory
  help     Print this message or the help of the given subcommand(s)

Options:
      --debug    Turn debugging information on
  -h, --help     Print help
  -V, --version  Print version
```