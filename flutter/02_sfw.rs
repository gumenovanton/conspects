# STATEFUL WIDGET
- has state
- life cycle looks like this:
    > create widget
    > createState()
    > state gets context
    > initState()
    > didChangeDependencies() - run when inheritedWidget changes
    > deactivate()
    > dispose() - here i close streams, controllers etc
- if sfw updates, then flutter launch didUpdateWidget()
- if hot reload, then flutter launch reasemble()
