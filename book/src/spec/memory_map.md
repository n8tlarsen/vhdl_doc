# Memory Map Descriptors

`vhdl_doc` memory map descriptors may be written in either json or toml formats.
The content may be embedded directly in the vhdl file or linked to an external `.json` or `.toml`
file.

#### Embedded Memory Map
```vhdl
library ieee;
use ieee.std_logic_1164.all;

--! \memorymap toml
--! 
--! \end memorymap
entity example is
    port (
        clock : in std_logic;
        reset : in std_logic
    );
end entity example;

```
#### Externally Linked Memory Map
```vhdl
library ieee;
use ieee.std_logic_1164.all;

--! \memorymap external example.toml
entity example is
    port (
        clock : in std_logic;
        reset : in std_logic
    );
end entity example;
```


