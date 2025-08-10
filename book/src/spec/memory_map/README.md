# Memory Map Descriptors

`vhdl_doc` can generate memory maps tables using descriptors written in either json or toml formats.
The content may be embedded directly in the vhdl file or linked to an external `.json` or `.toml`
file.

#### Embedded TOML Memory Map
```vhdl
library ieee;
use ieee.std_logic_1164.all;

--! \memorymap toml
--! 
--! name = "Example Memory Map"
--! type = "set"
--! 
--! [protocol]
--! name       = "Example Protocol"
--! addressMax = 0xFFFF
--! dataMin    = 1
--! 
--! [contains]
--! name    = "Description String"
--! address = 0x0000
--! value   = "My Great Memory Map"
--! access  = "r"
--! type    = { string = 20 }
--!
--! \end memorymap
entity example is
    port (
        clock : in std_logic;
        reset : in std_logic
    );
end entity example;

```

#### Embedded JSON Memory Map
```vhdl
library ieee;
use ieee.std_logic_1164.all;

--! \memorymap json
--! 
--! {
--!     "protocol": {
--!         "name": "Example Protocol",
--!         "addressMax": 65535,
--!         "dataMin": 1
--!     },
--!     "name": "Example Memory Map",
--!     "type": "set",
--!     "contains": {
--!         "name": "Description String",
--!         "address": 0,
--!         "access": "r",
--!         "type": { "string": 20 },
--!         "value": "My Great Memory Map"
--!     }
--! }
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

--! \memorymap path example.toml
entity example is
    port (
        clock : in std_logic;
        reset : in std_logic
    );
end entity example;
```

