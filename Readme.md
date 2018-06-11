Blue Yonder CSV to XML converter
================================

[![Build Status](https://travis-ci.org/blue-yonder/di-csv2xml.svg?branch=master)](https://travis-ci.org/blue-yonder/di-csv2xml)
[![Build Status](https://ci.appveyor.com/api/projects/status/{{status_id}})](https://ci.appveyor.com/project/blue-yonder/di-csv2xml)

This tool is intended to convert a `.csv` file into an `.xml` file ready to be sent to the
Blue Yonder Supply and Demand API. This tool has no schema information and therefore does not
perform any validation besides checking for valid `UTF8` encoding.

Installation
------------

You can download a binary executable (for OS-X, Windows and Linux) or build it yourself using:

```bash
git clone https://github.com/blue-yonder/di-csv2xml.git
cd di-csv2xml
cargo install
```

Is a previous version already installed? Not to worry, just use `cargo install --force`. The force
option will replace the old version with the new one.

You can install cargo from [here](https://rustup.rs) if it is not installed.

Usage
-----

```bash
di-csv2xml Category -i input.csv -o output.xml
```

converts this `input.csv` file

```csv
A,B,C,D
1,2,3,4
5,6,7,8
```

into this `output.xml`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<Category>
    <Record>
        <A>1</A>
        <B>2</B>
        <C>3</C>
        <D>4</D>
    </Record>
    <Record>
        <A>5</A>
        <B>6</B>
        <C>7</C>
        <D>8</D>
    </Record>
</Category>
```

Each line of the `input.csv` file is transformed into a separate XML-record. These are globally
embedded into a root-tag structure specified by the parameter `Category`.

Customer extensions are supported via the `CUEX_` prefix.

```csv
A,CUEX_B,C,CUEX_D
1,2,3,4
```

becomes

```xml
<?xml version="1.0" encoding="UTF-8"?>
<Category>
    <Record>
        <A>1</A>
        <C>3</C>
        <CustomerExtensions>
            <B>2</B>
            <D>4</D>
        </CustomerExtensions>
    </Record>
</Category>
```

For more information, please use `di-csv2xml --help`.

As this tool does not provide any schema validation, it is important to note that you get what you typed.
Any typo in the parameter `category` or the header column of the csv-file is directly translated into the
dedicated XML-tag, leading to potential errors when attempting to process the XML-file further.

Support
-------

This tool is provided as is under an MIT license without any warranty or SLA. You are free to use
it as part for any purpose, but the responsibility for operating it resides with you. We appreciate
your feedback though. Contributions on GitHub are welcome.