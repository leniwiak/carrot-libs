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
- FILEINFO: Supports 'resolve_symlink' argument on every function
- INPUT: There is a function 'get_with_default()' that let's you create text input with some text on start
- KINDER: Removed time related functions
- INPUT: Migrate function ask() from many carrot-utils to this library
