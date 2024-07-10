# Release 0.2.4:

### Added on 2.04.2024

- INPUT: Deal with char boundaries

# Release 0.2.5:

### Added on 2.04.2024

- INPUT: CTRL+ARROW, DEL, CTRL+DEL, CTRL+BACKSPACE keys support

# Release 0.2.6:

### Added on 4.05.2024

- INPUT: Move some code from GET() to DETECT() - a new function responsible just for detecting keys
- ANSI: New library containing some usefull ansi codes (NEW!)

### Added on 13.05.2024

- INPUT: Added secure prompt

# Release 0.2.7:

### Added on 13.05.2024

- SYSTEM: Access system information (NEW!)

# Release 0.3.0:

### Added on 29.05.2024

- ANSI: Renamed to CLI

### Added on 30.05.2024

- *: Properly return errors
- *: Libraries can use Strings or &strs without any issue thanks to this: <S: AsRef<str>>(whatever:S) and this: whatever.as_ref(). 
- FILEINFO: Supports 'resolve_symlink' argument on every function
- INPUT: There is a function 'get_with_default()' that let's you create text input with some text on start
- KINDER: Removed time related functions
- INPUT: Migrate function ask() from many carrot-utils to this library
- SYSTEM: New getpref(), getpref_or_exit() and ~~setpref()~~ functions allow programs to quickly retrieve information from system config files.

# Release 0.3.1:

### Added on 2.06.2024

- Configuration file management was moved from carrot-libs to programs itself

# Release 0.3.2:

### Added on 11.07.2024

- SYSTEM: Added a list of allowed simple characters for usernames, variables, etc.
- INPUT: Custom message for ask() (This was actually added before, but I don't exactly know when)

# Waiting features

