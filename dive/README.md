# Dive OS
Kernel and boot information may be on their own partition.  Main Partition (ext4):
```
/                                               # Root Directory '/'
├── dive/                                       # Dive OS info, program.  Includes libraries.
│   ├── 0.2.3/                                  # A version of dive.  Named by semver.
│   │   ├── dive.mut                            # Mutable config data for dive OS (usernames, keys).
│   │   ├── dive.res                            # Immutable Resources for dive OS.
│   │   ├── dive.run                            # .app transpiled for this arch.
│   │   └── dive.app                            # Program executable (PORTABLE, includes data).
│   ├── dive/                                   # Icon data (dive Theme) shared between programs.
│   │   ├── dive-0.2.3.res                      # Resource file for dive: Named by semver.
│   │   └── custom_theme-0.2.3.res              # A custom theme (also named by semver).
│   └── library_name/                           # Library folder.
│       └── 0.2.3                               # Shared object file.  Named by semver.
└── program_name/                               # Application info, program.
    ├── 0.2.3/                                  # A version of dive.  Named by semver.
    │   ├── program_name.res                    # Immutable Resources for dive OS.
    │   ├── program_name.run                    # .app transpiled for this arch.
    │   └── program_name.app                    # Program executable (PORTABLE, includes data).
    └── username/                               # User's folder, named by username.
        └── filename.ext                        # File.  Extension is file type for this program.
```

## Harp
`harp` is Dive OS's software manager.
```
harp Get "package name" # download & install a package
harp Del "package name" # remove & uninstall a package
harp New                # update packages (get new versions of packages)
```
