# Shisp Bootstrap
  This is an attempt at a Boostrap Compiler for Shisp -- A Lisp that Compiles to POSIX Shell.


## Shisp Language Special Forms
### let 
**Usage: (let (assignments) body...)**
Creates a new environment and defines and sets the variables within the environment.

### define
**Usage: (define varname varval)**
Creates a new variable within the current scope. 
 
### setvar
**Usage: (set! varname val)**
Sets the variable to the value.

### defun
**Usage: (defun name (arglist) body...)**
Defines a function

### demac
**Usage: (demac name (arglist) body...)**
Defines a macro

### depun
**Usage: (depun name (arglist) body...)**
Defines a pure function.

### shell-literal
**Usage: (shell-literal literals...)**
Everything within this is not parsed as Shisp, but is instead passed to the compiler literally.

### quote
**Usage: (quote atom)**
**Usage: 'atom**
Quotes a specific atom

###Quasiquote
**Usage: (quasiquote atom)**
**Usage: `atom**

###Unquote
**Usage: (unquote atom)**
**Usage: ,atom**

###Unquote-splice
**Usage: (unquote-splice atom)**
**Usage: ,@atom**

### Cond
**Usage: (cond (cond1 expr1) (cond2 expr2) ... (condN exprN))**
