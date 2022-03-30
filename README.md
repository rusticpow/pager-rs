# pager-rs
Paging study project

Pager is for columnar store database, so it is a bit different from regular row databases.

Size of page - 4096 bytes. First 8 bytes allocated for header.

Leaf page header structure:

|Offset|Size|Description|
|---|---|---|
|0  |1  |Page type|
|1  |3  |Column Id|
|4  |4  |Reserved |

Last 4096 - 8 = 4088 is for body (payload).
Body has two types of data: row local id and column data. Row identifiers fills body from start to end while column data from end to start.



