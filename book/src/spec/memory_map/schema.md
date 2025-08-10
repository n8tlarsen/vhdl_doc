# JSON Schema for Memory Map Descriptors
``` json
{
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "title": "MemoryMap",
    "type": "object",
    "properties": {
        "protocol": {
            "$ref": "#/$defs/Protocol"
        },
        "name": {
            "type": "string"
        },
        "address": {
            "anyOf": [
                {
                    "$ref": "#/$defs/Address"
                },
                {
                    "type": "null"
                }
            ]
        },
        "access": {
            "anyOf": [
                {
                    "$ref": "#/$defs/Access"
                },
                {
                    "type": "null"
                }
            ]
        },
        "type": {
            "$ref": "#/$defs/FieldType"
        },
        "contains": {
            "anyOf": [
                {
                    "$ref": "#/$defs/OneOrMoreField"
                },
                {
                    "type": "null"
                }
            ]
        },
        "value": {
            "anyOf": [
                {
                    "$ref": "#/$defs/Value"
                },
                {
                    "type": "null"
                }
            ]
        }
    },
    "required": [
        "protocol",
        "name",
        "type"
    ],
    "$defs": {
        "Protocol": {
            "type": "object",
            "properties": {
                "name": {
                    "description": "An optional name for the protocol",
                    "type": [
                        "string",
                        "null"
                    ]
                },
                "addressMax": {
                    "description": "Maximum address in terms of dataMin",
                    "$ref": "#/$defs/Address"
                },
                "dataMin": {
                    "description": "Minimum addressable data size in bytes",
                    "type": "integer",
                    "format": "uint8",
                    "minimum": 0,
                    "maximum": 255
                }
            },
            "required": [
                "addressMax",
                "dataMin"
            ]
        },
        "Address": {
            "anyOf": [
                {
                    "type": "string"
                },
                {
                    "type": "integer",
                    "format": "uint64",
                    "minimum": 0
                }
            ]
        },
        "Access": {
            "type": "string",
            "enum": [
                "r",
                "w",
                "rw"
            ]
        },
        "FieldType": {
            "oneOf": [
                {
                    "type": "string",
                    "enum": [
                        "set"
                    ]
                },
                {
                    "type": "object",
                    "properties": {
                        "string": {
                            "type": "integer",
                            "format": "uint64",
                            "minimum": 0
                        }
                    },
                    "required": [
                        "string"
                    ],
                    "additionalProperties": false
                },
                {
                    "type": "object",
                    "properties": {
                        "vector": {
                            "type": "integer",
                            "format": "uint64",
                            "minimum": 0
                        }
                    },
                    "required": [
                        "vector"
                    ],
                    "additionalProperties": false
                },
                {
                    "type": "object",
                    "properties": {
                        "unsigned": {
                            "type": "integer",
                            "format": "uint64",
                            "minimum": 0
                        }
                    },
                    "required": [
                        "unsigned"
                    ],
                    "additionalProperties": false
                },
                {
                    "type": "object",
                    "properties": {
                        "signed": {
                            "type": "integer",
                            "format": "uint64",
                            "minimum": 0
                        }
                    },
                    "required": [
                        "signed"
                    ],
                    "additionalProperties": false
                },
                {
                    "type": "object",
                    "properties": {
                        "ufixed": {
                            "type": "array",
                            "prefixItems": [
                                {
                                    "type": "integer",
                                    "format": "int64"
                                },
                                {
                                    "type": "integer",
                                    "format": "int64"
                                }
                            ],
                            "minItems": 2,
                            "maxItems": 2
                        }
                    },
                    "required": [
                        "ufixed"
                    ],
                    "additionalProperties": false
                },
                {
                    "type": "object",
                    "properties": {
                        "sfixed": {
                            "type": "array",
                            "prefixItems": [
                                {
                                    "type": "integer",
                                    "format": "int64"
                                },
                                {
                                    "type": "integer",
                                    "format": "int64"
                                }
                            ],
                            "minItems": 2,
                            "maxItems": 2
                        }
                    },
                    "required": [
                        "sfixed"
                    ],
                    "additionalProperties": false
                }
            ]
        },
        "OneOrMoreField": {
            "anyOf": [
                {
                    "$ref": "#/$defs/Field"
                },
                {
                    "type": "array",
                    "items": {
                        "$ref": "#/$defs/Field"
                    }
                }
            ]
        },
        "Field": {
            "type": "object",
            "properties": {
                "name": {
                    "type": "string"
                },
                "address": {
                    "anyOf": [
                        {
                            "$ref": "#/$defs/Address"
                        },
                        {
                            "type": "null"
                        }
                    ]
                },
                "access": {
                    "anyOf": [
                        {
                            "$ref": "#/$defs/Access"
                        },
                        {
                            "type": "null"
                        }
                    ]
                },
                "type": {
                    "$ref": "#/$defs/FieldType"
                },
                "contains": {
                    "anyOf": [
                        {
                            "$ref": "#/$defs/OneOrMoreField"
                        },
                        {
                            "type": "null"
                        }
                    ]
                },
                "value": {
                    "anyOf": [
                        {
                            "$ref": "#/$defs/Value"
                        },
                        {
                            "type": "null"
                        }
                    ]
                }
            },
            "required": [
                "name",
                "type"
            ]
        },
        "Value": {
            "anyOf": [
                {
                    "type": "string"
                },
                {
                    "type": "integer",
                    "format": "uint64",
                    "minimum": 0
                },
                {
                    "type": "integer",
                    "format": "int64"
                },
                {
                    "type": "number",
                    "format": "double"
                }
            ]
        }
    }
}
```
