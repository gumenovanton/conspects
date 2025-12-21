# CONTEXT
- object that give me an opportunity to get:
    > size - only when widget is built
    > widget - widget itself, in sfw i can get widget from store
    > inheritedWidget - i can get any iherited widget up the widget tree
    > ingeritedElement - useless thing
    > renderObject - useless thing
    > element - usless thing
    > state - of any widget up the widget tree

# CONTEXT METHODS
- size - i can only launch in when widget is built, fe in Button onTap method
- widget - rarely used
- dependOnInheritedElement() - subscrybe on InheritedWidget changes, but do not receive InheritedWidget itself, rarely used, can execute it only in didChangeDependencies() or build()
- dependOnInheritedWidgetOfExactType() - gets an InheritedWidget and subscrybe on InheritedWidget changes, used because i can get a value, that changes, can execute it only in didChangeDependencies() or build()
- getElementForInheritedWidgetOfExactType() - gets the value of InheritedVidget, but not subscribe on it, rare used, can execute it only in initState(), didChangeDependencies(), build(), dispose()
- findAncestorStateOfType() - gets closest State of Widget up the Widget tree, can execute it only in initState() didChangeDependencies() and build(), do not use
- findRootAncestorStateOfType() - gets closest to root State of Widget up the Widget tree, can execute it only in initState() didChangeDependencies() and build(), very expencive, do not use
- findAncestorRenderObjectOfType() - useless
- findAncestorWidgetElementOfExactType() - useless
- findRenderObject() - useless
- findAncestonElements() - useless
- visitChildElements() - useless
