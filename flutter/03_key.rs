# KEY
- used to mark a widget
- i can create GlobalKey
- or i can create LocalKey

# TYPE OF GLOBAL KEYS I CAN CREATE
- GlobalKey() - i can create this key in global scope
- GlobalObjectKey(object) - almost the same as a GlobalKey, but i can put any object as param, and if two global keys with one object, they will be equal

# TYPE OF LOCAL KEYS I CAN CREATE
- UniqueKey() - just unique key
- ValueKey(value) - a key that gets a value, two Value Keys that receives equal values are equal, used when key can be recreated, usualy used in ListView to keep normal order of elements
- ObjectKey(object) - similar to Value Key but receives an object
- PageStorageKey() - every page has a storage. Widgets like listView, TextField automaticaly stores in it theirs scroll position. And i manualy can put into it some state that i want to keep when i move from page to page
