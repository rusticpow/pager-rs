# pager-rs
Paging study project

Pager is for columnar store database, so it is a bit different from regular row databases.

Size of page - 4096 bytes. First 12 bytes allocated for header.

Leaf page header structure:

|Offset|Size|Description|
|---|---|---|
|0  |1  |Page type|
|1  |4  |Column Id|
|5  |7  |Reserved |

Last 4096 - 12 = 4084 is for body (payload).
Body has two types of data: row local id and column data. Row identifiers fills body from start to end while column data from end to start.



