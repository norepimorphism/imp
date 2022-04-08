% IMP(1) imp 0.1.0
% norepimorphism
% April 2022

# NAME
imp - evaluate mathematical expressions

# SYNOPSIS
**imp** [*OPTION*]

# DESCRIPTION
**imp** evaluates mathematical expressions, solves equations, and generates graphs from a LISP-like language called IMPL. IMPL code may either be imported from a script file or entered interactively in a shell interface.

# OPTIONS
**-i**, **--in**
: Reads IMPL code from the given script file.

**-c**, **--config**
: Reads settings from the given TOML configuration file.

**-V**, **--version**
: Displays the software version.

# EXIT VALUES
**0**
: Success

**1**
: Argument error

**2**
: Configuration error

# BUGS
If you encounter any bugs, please create an issue for each at <https://github.com/norepimorphism/imp>. Thanks!

# COPYRIGHT
Copyright (C) 2022 norepimorphism.

This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.

# SEE ALSO
