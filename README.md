# Instructions

To use this template, simply clone the repository into a fresh folder:

```
❯ mkdir new_project
❯ git clone https://github.com/qweenkie/ggez-template.git new_project
```

Then enter the directory, and run the init script (do not run any scripts you
download from the internet, unless you have read the script and have ensured
that it is safe):

```
❯ cd new_project
❯ ./init.sh
```

If you do not trust my *init.sh*, you can also set up your cargo project manually:

```
❯ cd new_project  # Enter the project directory
❯ cargo init      # Initialize a cargo (rust) project
❯ cargo add ggez  # Add the GGEZ framework as a dependency for the project

# Get rid of the useless files
❯ rm -f README.md
❯ rm -f init.sh
```
