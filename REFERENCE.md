# JUK2 - Reference

# Configuration Options

## Summary

TODO: table

## Description

# Interface

## Interface Modes

TODO

## Keybindings

TODO

# Command Reference

## Arguments

Some commands have key-value arguments, they are given as:
```
<key>=<value>
```
where key names are strings. Values are parsed on a per-key basis: each key has their value requirements listed in its description.

## Displacement Arguments

Allowed values for displacement arguments depend on the current reference frame (config: `frame`) and the current unit (config: `unit`).

That is:
- `frame` = `"rel"`, `unit` = `"steps"`:
    - Data type: integer
    - Range: \[-50000, 50000\]

- `frame` = `"rel"`, `unit` = `"mm"`:
    - Data type: floating point
    - Range: \[-1000, 1000\]

- `frame` = `"abs"`, `unit` = `"steps"`:
    - Data type: integer
    - Range: \[0, 50000\]

- `frame` = `"abs"`, `unit` = `"mm"`:
    - Data type: floating point
    - Range: \[0, 1000\]

## Motion Arguments

Allowed values for acceleration and velocity arguments depend on the current unit (config: `unit`).

- `unit` = `"steps"`
    - Data type: floating-point
    - Range: \[0, 25000\]

- `unit` = `"mm"`
    - Data type: floating-point
    - Range: \[0, 500\]

The acceleration is given as unit per second squared, velocity as unit per second.

## Argument Relations

If a command has required arguments, their description has an argument relation statement.

Let an imaginary command have argument keys:
- `foo`
- `bar`
- `baz`

The following list shows relation statements and their meaning (if a key is missing it is optional)

- `foo`, `bar`, `baz` are required:
```
* ( foo bar baz )
```

- At least one of `foo`, `bar`, `baz`:
```
|| ( foo bar baz )
```

- Exactly one of `foo`, `bar`, `baz`:
```
^^ ( foo bar baz )
```

## Commands List

#### `move` - Move in a Line

Examples:
```
move x=120 y=-400
move x=40.0 y=60.0 z=0.0
move x=1250 a=12000 v=8000
move x=-100.0 z=50.0 a=0
```

**Keys:**
- `x`: X axis displacement.
- `y`: Y axis displacement.
- `z`: Z axis displacement.
- `a`: Movement acceleration.
- `v`: Movement velocity.

**Relation:**
```
|| ( x y z )
```

#### `arc` - Move in an Arc

Examples:
```
arc x=-100 y=100 r=100 dir=pos
arc x=20.0 y=10.0 r=40.0 dir=neg
arc x=120 y=80 r=240 a=0 v=50
arc x=-10.0 y=-10.0 r=10.0 dir=neg a=500.0 v=100.0
```

**Keys:**
- `x`: X axis displacement.
- `y`: Y axis displacement.
- `z`: Z axis displacement. The arc is done in the XY plane. Use this to achieve helical motion.
- `r`: Arc radius. Adheres to the same rules as displacement, but must not be negative.
- `dir`: direction of the arc. `"pos"` means the direction where the angle of the radius vector increases, `"neg"` - where the angle decreases. Allowed values: `"pos"`, `"neg"`. Default is: `"pos"`
- `a`: Movement acceleration.
- `v`: Movement velocity.

**Relation:**
```
* ( x y r )
```

#### `home` - Perform Homing

Examples:
```
home
home axis=x
home axis=zy
home axis=xyz
```

**Keys:**
- `axis`: the axis (axes) to perform homing on. The only axes are `"x"`, `"y"`, `"z"`, so any combination of these three may be used. Default is: `"xyz"`.


#### `set` - Alter the configuration

Examples:
```
set unit=mm
set mmps=0.0125 posX=0 posY=500
```

**Keys:**

*see the configuration reference for the list of keys and their description*

Only the keys marked as read-write can be used.

**Relation:**
```
|| ( <keys> )
```

#### `get` - View the configuration

Examples:
```
get posX
get frame
get estatus
```

This commands functions differently than the others, namely the keys do not accept a value. This means that the following is an error:
```
get pos=loremipsum
```

**Keys:**

*see the configuration reference for the list of keys and their description*

Additionally the following keys are available:
- `pos`: absolute position of every axis in steps
- `version`: version information
- `license`: license information

**Relation:**
```
^^ ( <keys> )
```
