# MaybeSQL Syntax

## Basic Principles
Like SQL, MaybeSQL allows you to use newline characters to increase readability of your queries, and a `;` to end the query statement.
This file uses:
- `[]-` to indicate an optional value.
- `[]+` to indicate a value that can be specified between 1 and infinite times.
- `#` to indicate a comment

## Datatypes
A collection can be made of each datatype through the following syntax:
```
[datatype](max_size)
```
Where `max_size` is the maximum number of elements the collection can hold.
Datatypes in MaybeSQL always follow a syntax of `LABEL(options)`, where options may be left empty.

### String Datatypes
1. `STRING(size)`: A string of size `size` bytes.
2. `OPTIONS(opt1, ...)`: A object that can have only 1 value from the given options.
3. `CHAR()`: Equivalent to `STRING(1)`

### Numeric Datatypes
1. `BYTES(size)`: `size` Bytes stored
2. `UINT(size)`: An unsigned integer of `size` bytes
  - Minimum value for `size` is 1
  - Maximum value for `size` is 64
3. `INT(size)`: A signed integer of `size` bytes:
  - Minimum value for `size` is 1
  - Maximum value for `size` is 64
4. `FLOAT(size)`: A float of size `size` bytes 
  - Minimum value for `size` is 4
  - Maximum value for `size` is 32
5. `TIMESTAMP()`: A unix timestamp.

## Constraints
1. `EXISTS`: Forces the element to be a non-null value
2. `UNIQUE`: Forces the element to be unique, i.e., there should be no other entries in the table with the same values for a field constrained by `UNIQUE`
3. `PKEY`: A primary key, used to uniquely identify records.
4. `FKEY (ON table_name)`: A foreign key, used to refer to a `PKEY` from another table.
5. `SUCHTHAT (condition)`: Ensures that every value in the specified constrained field has a value satisfying `condition`
6. `DEFAULT (value)`: Gives a default value `value` to the specified constrained field.
7. `INC`: Automatically increments the specified constrained field to always produce unique values.

The `AUTO` keyword is used to automatically infer a default value.

## Constants
MaybeSQL defines the following constants:
1. `U8MAX`: 255
2. `U16MAX`: 65535
3. `U32MAX`: 4294967295
4. `U64MAX`: 18446744073709551615

## Modes
MaybeSQL uses 2 sets of 2 modes each to control which action you want to place more importance on:
Group 1:
1. `FADD`: Faster inserts
2. `FREAD`: Faster reads
Group 2:
3. `FDELETE`: Faster deletions
4. `LMEM`: Lower memory usage

1 mode can be used from each group, resulting in 4 different configurations.
These modes are defined when you make a table.

## Creating Databases
Databases are stored in the form of directories, and can be created with:
```
DATABASE database_name;

# Example:
DATABASE my_database;
```

## Creating Tables
Tables are stored in seperate files. They are created with:
```
TABLE table_name ON database_name STRUCTURED (
    [DATATYPE field_name,]+
) [CONSTRAINED (
    [ON field_name constraint]+
)]-
[MODE [[mode1]+]-;

# Example
TABLE orders ON my_database STRUCTURED (
    UINT(8) order_id,
    STRING(255) item_name,
) CONSTRAINED (
    ON order_id EXISTS PKEY INC,
    ON item_name EXISTS
)
MODE FADD LMEM;

TABLE users ON my_database STRUCTURED (
    UINT(8) user_id,
    [UINT(8)](U32MAX) orders_placed,
) CONSTRAINED (
    ON user_id EXISTS PKEY INC,
    ON orders_placed FKEY (ON orders) DEFAULT (AUTO)
)
MODE FREAD FDELETE;
```
Elements in a collection datatype can be constrained by treating the collection as the field being constraint. The specified constrains will be applied to its items.

## Inserting data into tables
Data is inserted into tables with:
```
INSERT STRUCTURED ([value,]+) ON table_name ON database_name;

# Example
INSERT STRUCTURED (...) ON users STRUCTURED (...) ON my_database;

INSERT STRUCTURED ("Ice cream") ON orders STRUCTURED (item_name) ON my_database;
INSERT STRUCTURED ([1]) ON users ON STRUCTURED (orders_placed) ON my_database;
```

