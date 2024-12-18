# Reminder

A little cli tool to help keep track of roughly how much filament is remaining
your 3d printer. 

## Installation
Clone the sources into a directory. 

Then install the package:
```
cargo install --path .
```

This will create the required directory for the database and install. *Note*
make sure the default cargo install path for binaries is added to your
environment path.

## Usage

### Create Spool
To use the tool you first need to create a new spool, which is what will be slowly
removed from. You can add the length or the weight or both. The application is configured
for standard 1.75mm fillament for the conversion. In the example it will create a 
spool of 1000 grams and roughly 330 meters long with the name `PLA Black`. When
the spool is complete a new spool can be created and prints will start using that
spool to calculate.

```shell
remainder -w 1000 -l 330 create-spool "PLA Black"
```

### Create print
When printing a new print the information can be entered and like with the spool
if only one is entered it will be converted to the other unit (weight -> length or length -> weight)
The option after the command is the amount of time the print takes in seconds.
Of **Note** is that the tool will always use the most recent spool as the one
which is loaded in the printer

```shell
remainder -w 89.5 add-print 1150
```

### Checking Stats
To see how much remaining fillament is on the spool or the lifetime statistics
for the printer the `check-remaining` and `lifetime-stats` commands can be used
they have no special inputs.
