# JUK2 - Reference

# Configuration Options

## Summary

| Option   | Data Type   | Values                 | Default Value |
|----------|-------------|------------------------|---------------|
| `accel`  | `f32`       | \[`0.0`, `50000.0`\]   | *TODO*        |
| `frame`  | `enum`      | `rel`, `abs`           | `rel`         |
| `led`    | `u32`       | \[`000000`, `ffffff`\] | `00ff00`      |
| `limits` | `[bool; 6]` | key is read-only       | *undefined*   |
| `mmpsX`  | `f32`       | \(`0.0`, `1.0`\]       | `0.0125625`   |
| `mmpsY`  | `f32`       | \(`0.0`, `1.0`\]       | `0.0125625`   |
| `mmpsZ`  | `f32`       | \(`0.0`, `1.0`\]       | *TODO*        |
| `posX`   | `u32`       | \[`0`, `50000`\]       | `0`           |
| `posY`   | `u32`       | \[`0`, `50000`\]       | `0`           |
| `posZ`   | `u32`       | \[`0`, `50000`\]       | `0`           |
| `unit`   | `enum`      | `steps`, `mm`          | `steps`       |
| `vel`    | `f32`       | \[`0.0`, `50000.0`\]   | *TODO*        |

## Description

#### `accel` - Default Acceleration

Data type: `f32`
Value range: \[`0.0`, `50000.0`\]
Default value: *TODO*

Default acceleration value in `steps / s^2` used when the acceleration motion parameter is ommited.

Example:
```
set accel=12000.0
```

#### `frame` - Displacement Reference Frame

Data type: `enum`
Variants: `rel`, `abs`
Default value: `rel`

Reference frame to use when moving. `rel` means relative to the current position, `abs` - relative to the origin of the \[`posX`, `posY`, `posZ`\] vector.

Example:
```
set frame=abs
```

#### `led` - RGB LED Colour

Data type: `u32`
Value range: \[`000000`, `ffffff`\]
Default value: `00ff00`

RGB8 value in HTML notation.

Example:
```
set led=ff00ff
```

#### `limits` - Limit Switch Status

Data type: `[bool; 6]`
Default value: *undefined*

Limit switch status for each axis and each direction. The data type doesn't matter - the software will display the output in a friendly format.

This configuration key is read-only and cannot be used with the `set` command.

Example:
```
get limits
```

#### `mmps{X,Y,Z}` - Millimeters per Step Coefficient

Data type: `f32`
Value range: \(`0.0`, `1.0`\]
Default value:
- `mmps{X,Y}`: `0.0125625`
- `mmpsZ`: *TODO*

Millimeters per step coefficient for an axis. To make a move when the value of `unit` is `mm`, the displacement value is divided by its axis `mmps` value and rounded to the nearest integer.

Example:
```
set mmpsX=0.02
set mmpsY=0.0125
set mmpsZ=0.005
```

#### `pos{X,Y,Z}` - Current Position

Data type: `u32`
Value range: \[`0`, `50000`\]
Default value: `0`

Current position of the end-effector in the unit of steps. Each axis' value is set to 0 after homing that axis. Changing this value does not move the end-effector, it only changes the reference frame.

Example:
```
set posX=100
set posY=43
set posZ=0
```

#### `unit` - Displacement Units

Data type: `enum`
Variants: `steps`, `mm`
Default value: `steps`

Units to use for the displacement values. `steps` is in the unit of motor steps, `mm` is for millimeters.

Example:
```
set unit=mm
```

#### `vel` - Default Velocity

Data type: `f32`
Value range: \[`0.0`, `50000.0`\]
Default value: *TODO*

Default velocity value in `steps / s` used when the velocity motion parameter is ommited.

Example:
```
set vel=5000.0
```

# Interface

## Interface Modes

The interface can be in one of two states: text or binary. After reset the state is set to text, the user may communicate with the machine using a command line interface. Binary mode is used for communicating with a PC using messages in a binary format. To switch between the modes the user has to press `CTRL + Space` once or twice, depending on the current state of the input buffer. The user is also notified about the switch by a message on the terminal when said switch happens. The external RGB indicator colour shows the current mode: green means text mode, magenta - binary mode.

## Keybindings

While the interface is in text mode, the following key combinations are recognised:
- `?`: Displays help based on the current command, if the command name is not entered, displays the command list
- `CTRL + X`: Cancels the current movement and flushes the execution queue
- `CTRL + C`: Cancels the current input at the command line
- `CTRL + D`: Issues a software reset
- `CTRL + Space`: Switch between text mode and binary mode
- `CTRL + G`: Easter egg

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
- `frame` = `rel`, `unit` = `steps`:
    - Data type: integer
    - Range: \[-50000, 50000\]

- `frame` = `rel`, `unit` = `mm`:
    - Data type: floating-point
    - Range: \[-1000, 1000\]

- `frame` = `abs`, `unit` = `steps`:
    - Data type: integer
    - Range: \[0, 50000\]

- `frame` = `abs`, `unit` = `mm`:
    - Data type: floating-point
    - Range: \[0, 1000\]

## Motion Arguments

Allowed values for acceleration and velocity arguments depend on the current unit (config: `unit`).

- `unit` = `steps`
    - Data type: floating-point
    - Range: \[0, 50000\]

- `unit` = `mm`
    - Data type: floating-point
    - Range: the values must be non-negative and after conversion to steps, must fit into the range for `unit` = `steps`.

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
- `dir`: direction of the arc. `pos` means the direction where the angle of the radius vector increases, `neg` - where the angle decreases. Allowed values: `pos`, `neg`. Default is: `pos`
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
- `axis`: the axis (axes) to perform homing on. The only axes are `x`, `y`, `z`, so any combination of these three may be used. Default is: `xyz`.


#### `set` - Alter the configuration

Examples:
```
set unit=mm
set mmpsX=0.0125 posX=0 posY=500
```

**Keys:**

*see the configuration reference for the list of keys and their description*

**Relation:**
```
|| ( <keys> )
```

#### `get` - View the configuration

Examples:
```
get posX
get frame
get status
```

This commands functions differently than the others, namely the keys do not accept a value. This means that the following is an error:
```
get pos=loremipsum
```

**Keys:**

*see the configuration reference for the list of keys and their description*

Additionally the following keys are available:
- `pos`: absolute position of every axis in steps
- `mmps`: millimeters per step coefficient for every axis
- `status`: combined `pos`, `mmps` and limit switch information
- `version`: version information
- `license`: license information

**Relation:**
```
^^ ( <keys> )
```
