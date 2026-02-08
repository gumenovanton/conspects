# INHERITED WIDGET
- widget to provide state to its children

# HOW TO WORK WITH IT
// STEP #1 - create a class that extends from Ingerited Widget
class DataProviderInherited extends InheritedWidget{
    // stata to share
    final int value;

    DataProviderInherited({
       Key?:key,
       required this.value,
       required child
    }): super(key:key, child:child);

    // method for check if shared state is changed, then update subscribed widgets
    @override
    bool updateShouldNotify(DataProviderInherited oldWidget){
        return value != oldWidget.value;
    }
}

// STEP #2 - in widget that changes state(here value), fe page
// wrap child widgets that has to be dependent on this state, in DataProviderInherited
// and pass state(value) in value param of DataProviderInherited
class HomePage extends StatefulWidget{
    const HomePage ({super.key});

    @override
    State<HomePage> createState()=> _HomePageState();
}

class _HomePageState extends State<HomePage>{
    var _value = 0;

    void _increment(){
        _value+=1;
        setState((){});
    }
    @override
    Widget build(BuildContext context){
        return Column(
            children:[
                TextButton('+', onPressed: _increment()),
                // when _value will change, this widget will be rebuilt, and will be created new DataProviderInherited
                // then will be executed updateShouldNotified() where state of old DataProviderInherited
                // will be compared with state of new DataProviderInherited
                // and all subscribed widgets will be rebuilt, it doesn't matter if it const or not
                DataProviderInherited(
                    value: _value;
                    child: SomeConsumerWIdget(),
                ),
            ],
        );
    }
}

// STEP #3 - go to dependent widget, that should be updated when state will be changed
// that can be a slw
class SomeConsumerWidget extends StatelessWidget{
    const SomeConsumerWidget({Key? key}):super(key:key);

    @override
    Widget build(BuildContext context){
        // here wi say, give me the value, and if state that passed in inherited vidget will be changed,
        // execute build again to rebuild me
        final value = context.dependOnInheritedWidgetOfExactType<DataProviderInherited>()?.value ?? 0;

        return Text('$value');
    }
}
